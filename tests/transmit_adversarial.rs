//! Fail-closed framing, allocation limits, layout validation, and malformed generated inputs.
#[path = "support/transmit.rs"]
mod transmit_support;
use bessemer::{FailureClass, parasolid::*};
use transmit_support::{CASES, DELTAS, LIMITS, NEUTRAL, base, list_mut, parse};

fn read(bytes: &[u8]) -> bessemer::Result<TransmitFile> {
    TransmitFile::from_bytes(bytes, &base(), LIMITS)
}
fn user_field_offset() -> usize {
    let key = b"SCH_3701250_37102_13006";
    NEUTRAL
        .windows(key.len())
        .position(|span| span == key)
        .unwrap()
        + key.len()
        + 2
}

#[test]
fn truncations_are_classified_against_independent_complete_boundaries() {
    let initial = include_bytes!("fixtures/parasolid/neutral-initial-deltas.bin");
    assert_eq!(&DELTAS[..initial.len()], initial);
    for (bytes, complete_boundary) in [(NEUTRAL, None), (DELTAS, Some(initial.len()))] {
        for end in 0..bytes.len() {
            let parsed = read(&bytes[..end]);
            if Some(end) == complete_boundary {
                assert!(parsed.is_ok());
            } else {
                assert!(parsed.is_err(), "prefix {end}");
            }
        }
    }
    let mut bytes = NEUTRAL.to_vec();
    bytes.push(0);
    assert!(read(&bytes).unwrap_err().cause.contains("trailing"));
    assert!(read(b"XY").is_err());
}

#[test]
fn record_and_element_budgets_have_exact_boundaries() {
    let limits = TransmitLimits {
        maximum_records: 6,
        maximum_field_elements: 3,
    };
    assert!(TransmitFile::from_bytes(NEUTRAL, &base(), limits).is_ok());
    assert!(
        TransmitFile::from_bytes(
            NEUTRAL,
            &base(),
            TransmitLimits {
                maximum_records: 5,
                ..limits
            }
        )
        .unwrap_err()
        .cause
        .contains("record count")
    );
    assert!(
        TransmitFile::from_bytes(
            NEUTRAL,
            &base(),
            TransmitLimits {
                maximum_field_elements: 2,
                ..limits
            }
        )
        .unwrap_err()
        .cause
        .contains("element budget")
    );
    assert!(
        TransmitFile::from_bytes(
            DELTAS,
            &base(),
            TransmitLimits {
                maximum_records: 9,
                maximum_field_elements: 3
            }
        )
        .is_ok()
    );
}

#[test]
fn nonzero_user_fields_and_schema_only_wire_kinds_are_unsupported() {
    let mut bytes = NEUTRAL.to_vec();
    let offset = user_field_offset();
    bytes[offset..offset + 4].copy_from_slice(&1u32.to_be_bytes());
    let error = read(&bytes).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("user_field_size 1"));
    let mut bytes = NEUTRAL.to_vec();
    let offset = user_field_offset() + 4;
    bytes[offset..offset + 2].copy_from_slice(&201u16.to_be_bytes());
    assert_eq!(read(&bytes).unwrap_err().class, FailureClass::Unsupported);
}

#[test]
fn extended_indices_preserve_equivalent_bytes_and_canonicalize_only_on_forced_rebuild() {
    let mut bytes = NEUTRAL.to_vec();
    let index = user_field_offset() + 4 + 3;
    assert_eq!(&bytes[index..index + 2], &[0, 2]);
    bytes.splice(index..index + 2, [255, 253, 0, 0]);
    let source = parse(&bytes);
    assert_eq!(
        source
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    let rebuilt = TransmitFile::new(&base(), source.contents().clone()).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), NEUTRAL);
    let mut invalid = NEUTRAL.to_vec();
    invalid[index..index + 2].fill(0);
    assert!(
        read(&invalid)
            .unwrap_err()
            .cause
            .contains("stored node index 0")
    );
}

#[test]
fn malformed_layout_operations_counts_and_extents_are_rejected() {
    let prefix = [0, 200, 14, b'C', b'D', b'I'];
    let offset = NEUTRAL
        .windows(prefix.len())
        .position(|span| span == prefix)
        .unwrap();
    let mut count = NEUTRAL.to_vec();
    count[offset + 2] = 15;
    assert!(read(&count).unwrap_err().cause.contains("expected 15"));
    let mut operation = NEUTRAL.to_vec();
    operation[offset + 3] = b'X';
    assert!(
        read(&operation)
            .unwrap_err()
            .cause
            .contains("expected C,D,I,A,Z")
    );
    let mut contents = parse(NEUTRAL).contents().clone();
    contents
        .layouts
        .insert(200, LayoutDefinition::Edited(vec![LayoutEdit::Copy; 100]));
    assert!(
        TransmitFile::new(&base(), contents)
            .unwrap_err()
            .cause
            .contains("exhausted")
    );
}

#[test]
fn wrong_schema_field_shapes_and_pointer_flag_modes_fail_without_changing_source() {
    let source = parse(NEUTRAL);
    let wrong = BaseSchema::new(SchemaNumber::new(13005), base().node_types().clone()).unwrap();
    assert!(
        TransmitFile::from_bytes(NEUTRAL, &wrong, LIMITS)
            .unwrap_err()
            .cause
            .contains("13005")
    );
    let mut contents = source.contents().clone();
    list_mut(&mut contents).records[2].length = None;
    assert!(source.with_contents(contents).is_err());
    let mut contents = source.contents().clone();
    list_mut(&mut contents).records[2].fields[0] = FieldValues::Bytes(vec![1]);
    assert!(source.with_contents(contents).is_err());
    let mut contents = source.contents().clone();
    let FieldValues::Pointers(pointers) = &mut list_mut(&mut contents).records[2].fields[1] else {
        panic!("expected pointer");
    };
    pointers[0].delta_flag = Some(1);
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("flag presence false")
    );
    assert_eq!(source.to_bytes().unwrap(), NEUTRAL);
}

#[test]
fn invalid_section_order_orphan_layouts_and_terminator_records_are_rejected() {
    let source = parse(DELTAS);
    let mut contents = source.contents().clone();
    let TransmitBody::Deltas(sections) = &mut contents.body else {
        panic!("expected deltas");
    };
    sections[0].changes = None;
    assert!(source.with_contents(contents).is_err());
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    contents.layouts.insert(
        232,
        LayoutDefinition::Defined {
            name: "empty".into(),
            description: String::new(),
            fields: Vec::new(),
            variable: 0,
        },
    );
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("orphan")
    );
    let mut contents = source.contents().clone();
    list_mut(&mut contents).records[0].node_type = 1;
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("terminator")
    );
    let mut contents = source.contents().clone();
    contents.layouts.remove(&200);
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("declared layout")
    );
}

#[test]
fn generated_byte_mutations_are_failures_or_stable_complete_typed_archives() {
    for original in &CASES[..4] {
        for index in 0..original.len() {
            let mut bytes = original.to_vec();
            bytes[index] ^= 255;
            let Ok(parsed) = read(&bytes) else {
                continue;
            };
            let rebuilt = TransmitFile::new(&base(), parsed.contents().clone()).unwrap();
            let reopened = parse(&rebuilt.to_bytes().unwrap());
            assert_eq!(reopened.contents(), parsed.contents(), "mutation {index}");
        }
    }
}

#[test]
fn oversized_variable_lengths_are_rejected_before_allocation_even_without_an_element_budget() {
    let ending = b"CCCCCCCCCCCZ";
    let length = NEUTRAL
        .windows(ending.len())
        .position(|span| span == ending)
        .unwrap()
        + ending.len();
    let mut bytes = NEUTRAL.to_vec();
    bytes[length..length + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    let limits = TransmitLimits {
        maximum_field_elements: usize::MAX,
        ..LIMITS
    };
    let error = TransmitFile::from_bytes(&bytes, &base(), limits).unwrap_err();
    assert!(error.cause.contains("4294967295"));
    assert!(error.cause.contains("at least 1 bytes"));
}
