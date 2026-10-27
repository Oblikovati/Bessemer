use super::annotation_view_tests::LIMITS;
use super::configuration_manager::SavedHelperState;
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/sheet-metal/cases.bin");
type SheetRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SheetRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SMT1");
    assert_eq!(cursor.u32().unwrap(), 316);
    let slots: Vec<_> = (0..316).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn state(kind: u8) -> SavedHelperState {
    if kind & 4 == 0 {
        return SavedHelperState::Inactive;
    }
    SavedHelperState::Active
}

fn decoder<'a>(
    version: u32,
    bytes: &'a [u8],
    selected: Option<SavedHelperState>,
    limits: ModelFieldLimits,
) -> crate::Result<ModelDecoder<'a>> {
    let reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    Ok(match selected {
        Some(selected) => reader.with_saved_helper_state(selected),
        None => reader,
    })
}

pub(super) fn decode_graph(
    version: u32,
    bytes: &[u8],
    selected: Option<SavedHelperState>,
    limits: ModelFieldLimits,
) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = decoder(version, bytes, selected, limits)?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn sheet_features_templates_fixed_faces_and_components_match_independent_fields_bytes_and_slots() {
    for (kind, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) =
            decode_graph(version, bytes, Some(state(kind)), LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn sheet_records_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], Some(state(kind)), LIMITS).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), Some(state(kind)), LIMITS).is_err());
        assert!(decode_graph(0x27cb, bytes, Some(state(kind)), LIMITS).is_err());
    }
}

#[test]
fn template_requires_verified_context_before_body_and_ordinary_default_matches_inactive() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.1 == 19000) {
        if kind == 0 {
            assert_eq!(
                decode_graph(version, bytes, None, LIMITS).unwrap(),
                decode_graph(version, bytes, Some(state(kind)), LIMITS).unwrap()
            );
        }
        if kind & 3 != 1 {
            continue;
        }
        let mut reader = decoder(version, bytes, None, LIMITS).unwrap();
        let error = reader
            .object("object", &mut ObjectReader::new(), ModelObjectRole::Any)
            .unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(
            error.cause.contains("saved helper state None"),
            "{}",
            error.cause
        );
        assert_eq!(
            reader.remaining(),
            bytes.len() - 6 - "moTemplateSheetMetal_c".len()
        );
    }
}

fn populated_default() -> Vec<ModelField> {
    records()
        .into_iter()
        .filter(|record| record.0 == 0 && record.1 == 19000)
        .map(|(_, version, bytes, _, _)| {
            decode_graph(version, bytes, Some(SavedHelperState::Inactive), LIMITS)
                .unwrap()
                .0
        })
        .find(|fields| {
            fields.iter().any(|field| {
                field.path.ends_with("array_50.element_size")
                    && field.value == ModelFieldValue::U32(16)
            })
        })
        .unwrap()
}

#[test]
fn dynamic_payload_and_table_extents_reject_invalid_sizes_and_bounds() {
    let fields = populated_default();
    let prefix = "root.object.payload.common_bend_800.allowance_08";
    for (label, value) in [
        ("array_50.element_size", 7u32),
        ("array_50.count", u32::MAX),
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("{prefix}.{label}"),
            &value.to_le_bytes(),
        );
        let error =
            decode_graph(19000, &changed, Some(SavedHelperState::Inactive), LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("sheet-metal array"), "{}", error.cause);
    }
    let table = fields
        .iter()
        .position(|field| field.path.as_ref() == format!("{prefix}.raw_table[0]"))
        .unwrap();
    let start = super::encode_fields(&fields[..table], 30000).unwrap().len();
    let bytes = super::encode_fields(&fields, 30000).unwrap();
    let error = decode_graph(
        19000,
        &bytes[..start + 40],
        Some(SavedHelperState::Inactive),
        LIMITS,
    )
    .unwrap_err();
    assert!(
        error.cause.contains("table shape [2, 1, 3]"),
        "{}",
        error.cause
    );
}

#[test]
fn strided_payload_words_and_table_values_charge_the_exact_entry_budget() {
    let record = records()
        .into_iter()
        .rev()
        .find(|record| record.0 == 0)
        .unwrap();
    let exact = ModelFieldLimits {
        maximum_entries: 16,
        ..LIMITS
    };
    decode_graph(record.1, record.2, Some(SavedHelperState::Inactive), exact).unwrap();
    let error = decode_graph(
        record.1,
        record.2,
        Some(SavedHelperState::Inactive),
        ModelFieldLimits {
            maximum_entries: 15,
            ..LIMITS
        },
    )
    .unwrap_err();
    assert!(
        error.cause.contains("sheet-metal allowance table"),
        "{}",
        error.cause
    );
}

#[test]
fn sheet_pointer_roles_keep_unrelated_root_targets_out_of_successful_results() {
    let fields = populated_default();
    for label in [
        "payload.common_bend_800.dimension_10",
        "payload.common_bend_800.feature_18",
        "payload.fixed_face_910",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2u16.to_le_bytes(),
        );
        let error =
            decode_graph(19000, &changed, Some(SavedHelperState::Inactive), LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moSheetMetal_c"), "{}", error.cause);
    }
}
