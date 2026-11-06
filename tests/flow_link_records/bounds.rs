use super::*;
use bessemer::FailureClass;

fn read_bytes(stored: &[u8], options: FlowLinkRecordOptions) -> bessemer::Result<FlowLinkRecords> {
    let source = carrier(stored, 1_000_000);
    source.record_402_links(&source.latest(402).unwrap().unwrap().id, options)
}

#[test]
fn every_owned_proper_prefix_and_extension_rejects_without_partial_links() {
    for case in cases().into_iter().skip(2) {
        for end in 0..case.stored.len() {
            assert!(
                read_bytes(&case.stored[..end], options(&case)).is_err(),
                "{} {end}",
                case.name
            );
        }
        let mut extra = case.stored.clone();
        extra.push(0);
        assert!(read_bytes(&extra, options(&case)).is_err());
    }
}

#[test]
fn exact_and_one_below_decoded_entry_and_shared_array_budgets_are_independent() {
    for case in cases() {
        let mut exact = options(&case);
        exact.maximum_decoded_bytes = case.decoded.len();
        exact.maximum_entries = case.entries.len().max(1);
        exact.maximum_array_elements = array_elements(&case).max(1);
        assert!(read_bytes(&case.stored, exact).is_ok());
        let mut below = exact;
        below.maximum_decoded_bytes -= 1;
        assert!(read_bytes(&case.stored, below).is_err());
        below = exact;
        below.maximum_entries = case.entries.len().saturating_sub(1);
        assert!(read_bytes(&case.stored, below).is_err());
        below = exact;
        below.maximum_array_elements = array_elements(&case).saturating_sub(1);
        assert!(read_bytes(&case.stored, below).is_err());
    }
}

fn raw_case(version: i32) -> LinkCase {
    cases()
        .into_iter()
        .find(|c| {
            c.name == format!("v{version}-full") && !c.compressed && c.pointer_count.is_none()
        })
        .unwrap()
}

#[test]
fn unsupported_negative_and_impossible_counts_reject_before_allocation() {
    let case = raw_case(9);
    for (offset, value) in [
        (0, -1),
        (0, 10),
        (4, -1),
        (4, i32::MAX),
        (case.tail.numeric.as_ref().unwrap().start as usize, -1),
        (
            case.tail.final_tail.as_ref().unwrap().start as usize,
            i32::MAX,
        ),
    ] {
        let mut raw = case.stored.clone();
        raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        let mut unlimited = options(&case);
        unlimited.maximum_entries = usize::MAX;
        unlimited.maximum_array_elements = usize::MAX;
        assert!(read_bytes(&raw, unlimited).is_err());
    }
}

#[test]
fn sentinel_and_context_limits_apply_again_to_edits_and_structure_stays_fixed() {
    let case = raw_case(9);
    let mut bounded = options(&case);
    bounded.global_pointer_count = Some(FlowGlobalPointerCount::try_from(2).unwrap());
    let source = read_bytes(&case.stored, bounded).unwrap();
    let field = reference_field();
    for value in [-2, 2, i32::MAX] {
        assert!(source
            .replace_field(field, FlowLinkRecordValue::Signed32(value))
            .is_err());
    }
    assert!(source
        .replace_field(field, FlowLinkRecordValue::Signed32(-1))
        .is_ok());
    assert!(links(&case)
        .replace_field(field, FlowLinkRecordValue::Signed32(i32::MIN))
        .is_ok());
    check_structural_edits(&source);
    assert_eq!(source.decoded_bytes(), case.decoded);
}

#[test]
fn missing_fields_invalid_indices_and_widths_are_typed_failures() {
    let source = links(&raw_case(0));
    for field in missing_fields() {
        check_failure(&source, field, FailureClass::Missing);
    }
    for field in invalid_entry_fields().into_iter().chain([
        FlowLinkRecordField::TailMember { index: 11 },
        FlowLinkRecordField::Discarded { index: 0 },
        FlowLinkRecordField::Conditional { index: 0 },
    ]) {
        check_failure(&source, field, FailureClass::Invalid);
    }
    check_wrong_widths(&source);
}

fn missing_fields() -> [FlowLinkRecordField; 4] {
    [
        FlowLinkRecordField::Control,
        FlowLinkRecordField::NumericWide { index: 0 },
        FlowLinkRecordField::NumericIndex { index: 0 },
        FlowLinkRecordField::FinalValue { index: 0 },
    ]
}

fn invalid_entry_fields() -> [FlowLinkRecordField; 4] {
    [
        FlowLinkRecordField::EntryReference {
            entry: usize::MAX,
            component: 0,
        },
        FlowLinkRecordField::EntryReference {
            entry: 0,
            component: 2,
        },
        FlowLinkRecordField::EntryWide {
            entry: 0,
            index: usize::MAX,
        },
        FlowLinkRecordField::EntryInteger {
            entry: 0,
            index: usize::MAX,
        },
    ]
}

fn check_failure(source: &FlowLinkRecords, field: FlowLinkRecordField, class: FailureClass) {
    assert_eq!(
        source
            .replace_field(field, FlowLinkRecordValue::Signed32(0))
            .unwrap_err()
            .class,
        class
    );
}

fn check_wrong_widths(source: &FlowLinkRecords) {
    for (field, value) in [
        (
            FlowLinkRecordField::EntryCode { entry: 0 },
            FlowLinkRecordValue::Signed32(0),
        ),
        (
            FlowLinkRecordField::EntryWide { entry: 0, index: 0 },
            FlowLinkRecordValue::Byte(0),
        ),
        (
            FlowLinkRecordField::TailMember { index: 0 },
            FlowLinkRecordValue::WideBits(0),
        ),
    ] {
        assert!(source.replace_field(field, value).is_err());
    }
}

#[test]
fn foreign_stale_wrong_record_and_bad_compressed_streams_reject() {
    let case = raw_case(0);
    let source = links(&case);
    let foreign = carrier(&case.stored, 1_000_000);
    assert!(foreign
        .record_402_links(source.record(), options(&case))
        .is_err());
    check_stale_identity(&source, &case);
    let wrong = source.source().records().nth(2).unwrap().id;
    assert!(source
        .source()
        .record_402_links(&wrong, options(&case))
        .is_err());
    check_bad_streams();
}

fn check_bad_streams() {
    let case = cases()
        .into_iter()
        .find(|c| c.name == "v9-full" && c.compressed)
        .unwrap();
    let mut checksum = case.stored.clone();
    *checksum.last_mut().unwrap() ^= 1;
    let mut concatenated = case.stored.clone();
    concatenated.extend_from_slice(&case.stored);
    for raw in [Vec::new(), vec![0x1f, 0x8b, 0], checksum, concatenated] {
        assert!(read_bytes(&raw, options(&case)).is_err());
    }
}

#[test]
fn all_public_link_types_are_send_and_sync() {
    fn assert_shared<T: Send + Sync>() {}
    assert_shared::<FlowLinkRecordOptions>();
    assert_shared::<FlowLinkRecords>();
    assert_shared::<FlowLinkEntry<'static>>();
    assert_shared::<FlowLinkIntegers<'static>>();
    assert_shared::<FlowLinkTail<'static>>();
    assert_shared::<FlowLinkNumericTail<'static>>();
    assert_shared::<FlowLinkFinalTail<'static>>();
    assert_shared::<FlowLinkRecordCoverage>();
    assert_shared::<FlowLinkRecordDiagnostic>();
    assert_shared::<FlowLinkRecordEdit>();
    assert_shared::<FlowLinkRecordField>();
    assert_shared::<FlowLinkRecordValue>();
}

fn check_structural_edits(source: &FlowLinkRecords) {
    assert!(source
        .replace_field(
            FlowLinkRecordField::Control,
            FlowLinkRecordValue::Signed32(0)
        )
        .is_err());
    assert!(source
        .replace_field(
            FlowLinkRecordField::TailMember { index: 6 },
            FlowLinkRecordValue::Signed32(4)
        )
        .is_err());
}

fn check_stale_identity(source: &FlowLinkRecords, case: &LinkCase) {
    let changed = source
        .replace_field(
            FlowLinkRecordField::EntryCode { entry: 0 },
            FlowLinkRecordValue::Byte(0),
        )
        .unwrap();
    assert!(changed
        .result()
        .source()
        .record_402_links(source.record(), options(case))
        .is_err());
}

fn reference_field() -> FlowLinkRecordField {
    FlowLinkRecordField::EntryReference {
        entry: 0,
        component: 0,
    }
}

#[test]
fn measured_compressed_growth_obeys_the_original_physical_payload_limit() {
    let mut case = raw_zero_words();
    case.stored = compressed(&case.decoded, flate2::Compression::best());
    let source = carrier(&case.stored, case.stored.len());
    let links = source
        .record_402_links(&source.latest(402).unwrap().unwrap().id, options(&case))
        .unwrap();
    let mut changed = case.decoded.clone();
    let start = changed.len() - 8192;
    changed[start..start + 8].copy_from_slice(&0x123456789abcdef0_u64.to_le_bytes());
    assert!(compressed(&changed, flate2::Compression::fast()).len() > case.stored.len());
    assert!(links
        .replace_field(
            FlowLinkRecordField::FinalValue { index: 0 },
            FlowLinkRecordValue::WideBits(0x123456789abcdef0)
        )
        .is_err());
    assert_eq!(links.decoded_bytes(), case.decoded);
    assert_eq!(links.source().as_bytes(), source.as_bytes());
}

fn raw_zero_words() -> LinkCase {
    let mut case = cases()
        .into_iter()
        .find(|c| c.name == "v9-zero" && c.compressed)
        .unwrap();
    let offset = case.decoded.len() - 4;
    case.decoded[offset..].copy_from_slice(&1024_i32.to_le_bytes());
    case.decoded.resize(case.decoded.len() + 8192, 0);
    case
}

fn compressed(raw: &[u8], compression: flate2::Compression) -> Vec<u8> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), compression);
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
