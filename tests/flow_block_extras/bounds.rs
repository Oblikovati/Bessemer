use super::*;
use bessemer::FailureClass;

fn read_bytes(raw: &[u8], options: FlowBlockExtraOptions) -> bessemer::Result<FlowBlockExtras> {
    let source = carrier(raw, 5_000_000);
    source.record_408_extras(&source.latest(408).unwrap().unwrap().id, options)
}

fn owned(name: &str) -> ExtraCase {
    cases()
        .into_iter()
        .find(|c| c.name == name && !c.compressed)
        .unwrap()
}

#[test]
fn all_small_owned_proper_prefixes_and_extensions_reject_without_partial_extras() {
    for case in cases()
        .into_iter()
        .skip(1)
        .filter(|case| case.stored.len() <= 4096)
    {
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
fn exact_and_one_below_all_five_resource_axes_are_independent() {
    let cases = cases();
    let frames = oracle::frames();
    for (case, frame) in cases.iter().zip(&frames) {
        let mut exact = options(case);
        exact.maximum_decoded_bytes = case.decoded.len();
        exact.maximum_blocks = (case.leaves as usize).max(1);
        exact.maximum_entities = frame.scopes.len();
        exact.maximum_fields = frame.fields.len();
        exact.maximum_values = frame.fields.iter().map(|f| f.layout.element_count).sum();
        assert!(read_bytes(&case.stored, exact).is_ok());
        check_below_limits(case, exact);
    }
}

fn check_below_limits(case: &ExtraCase, exact: FlowBlockExtraOptions) {
    for axis in 0..5 {
        let mut below = exact;
        match axis {
            0 => below.maximum_decoded_bytes -= 1,
            1 => below.maximum_blocks = (case.leaves as usize).saturating_sub(1),
            2 => below.maximum_entities -= 1,
            3 => below.maximum_fields -= 1,
            4 => below.maximum_values -= 1,
            _ => panic!("axis {axis}; expected 0..4"),
        }
        assert!(
            read_bytes(&case.stored, below).is_err(),
            "{} axis {axis}",
            case.name
        );
    }
}

#[test]
fn delegated_versions_negative_and_impossible_counts_reject_before_allocation() {
    let case = owned("v8-rich");
    let frame = oracle::frames()
        .into_iter()
        .find(|frame| frame.name == case.name)
        .unwrap();
    for (offset, value) in [(0, -1), (0, 9)].into_iter().chain(count_mutations(&frame)) {
        let mut raw = case.stored.clone();
        raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        let mut unlimited = options(&case);
        unlimited.maximum_entities = usize::MAX;
        unlimited.maximum_fields = usize::MAX;
        unlimited.maximum_values = usize::MAX;
        assert!(read_bytes(&raw, unlimited).is_err());
    }
    for count in [0_u32, 1, 3] {
        let mut wrong = options(&case);
        wrong.leaf_count = FlowLeafCount::try_from(count).unwrap();
        assert!(read_bytes(&case.stored, wrong).is_err());
    }
}

fn count_mutations(frame: &ExpectedExtraFrame) -> Vec<(usize, i32)> {
    frame
        .fields
        .iter()
        .filter(|field| {
            field.kind.is_structural() && field.encoding == FlowBlockExtraEncoding::Signed32
        })
        .flat_map(|field| {
            [
                (field.layout.first_byte, -1),
                (field.layout.first_byte, i32::MAX),
            ]
        })
        .collect()
}

#[test]
fn structural_fields_empty_arrays_wrong_types_and_branch_changes_reject() {
    let case = owned("v8-rich");
    let source = extras(&case);
    for field in source.fields() {
        if field.is_editable() {
            check_nonstructural_failure(&source, field);
        } else {
            check_structural_failure(&source, field);
        }
    }
    check_branch_changes(&source);
    assert_eq!(source.decoded_bytes(), case.decoded);
}

fn wrong_value(encoding: FlowBlockExtraEncoding) -> FlowBlockExtraValue {
    match encoding {
        FlowBlockExtraEncoding::Byte => FlowBlockExtraValue::Signed32(0),
        FlowBlockExtraEncoding::Signed32 => FlowBlockExtraValue::WideBits(0),
        FlowBlockExtraEncoding::WideBits => FlowBlockExtraValue::Byte(0),
    }
}

#[test]
fn empty_interleaved_arrays_have_bounded_offsets_and_reject_element_edits() {
    let raw = zero_mixed_record();
    let mut context = options(&owned("v8-rich"));
    context.leaf_count = FlowLeafCount::try_from(1).unwrap();
    let source = read_bytes(&raw, context).unwrap();
    for field in source.fields().filter(|f| {
        matches!(
            f.kind(),
            FlowBlockExtraFieldKind::MixedWords | FlowBlockExtraFieldKind::MixedWide
        )
    }) {
        check_empty_interleaved(&source, field);
    }
}

fn zero_mixed_record() -> Vec<u8> {
    let mut raw = 8_i32.to_le_bytes().to_vec();
    raw.extend_from_slice(&(-1_i32).to_le_bytes());
    raw.extend_from_slice(&[0, 1, 0, 1]);
    raw.resize(raw.len() + 20 + 4 + 16, 0);
    raw.extend_from_slice(&[0, 0, 0, 0, 2, 0, 0]);
    raw
}

#[test]
fn foreign_stale_and_wrong_record_ids_and_invalid_compression_fail_explicitly() {
    let case = owned("v8-rich");
    let source = extras(&case);
    let field = source
        .fields()
        .find(|f| f.kind() == FlowBlockExtraFieldKind::LastByte)
        .unwrap();
    assert!(extras(&case).field(&field.id()).is_err());
    check_stale_field(&source, field);
    let wrong = source.source().records().nth(2).unwrap().id;
    assert!(
        source
            .source()
            .record_408_extras(&wrong, options(&case))
            .is_err()
    );
    check_bad_streams();
}

fn check_bad_streams() {
    let case = cases()
        .into_iter()
        .find(|c| c.name == "v8-rich" && c.compressed)
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
fn all_public_extra_types_are_send_and_sync() {
    fn assert_shared<T: Send + Sync>() {}
    assert_shared::<FlowBlockExtrasEnabled>();
    assert_shared::<FlowBlockExtraOptions>();
    assert_shared::<FlowBlockExtras>();
    assert_shared::<FlowBlockExtraFieldId>();
    assert_shared::<FlowBlockExtraField<'static>>();
    assert_shared::<FlowBlockExtraScope<'static>>();
    assert_shared::<FlowBlockExtraEntity>();
    assert_shared::<FlowBlockExtraFieldKind>();
    assert_shared::<FlowBlockExtraEncoding>();
    assert_shared::<FlowBlockExtraValue>();
    assert_shared::<FlowBlockExtraFieldLayout>();
    assert_shared::<FlowBlockExtraCoverage>();
    assert_shared::<FlowBlockExtraDiagnostic>();
    assert_shared::<FlowBlockExtraEdit>();
}

#[test]
fn measured_compressed_growth_fails_at_the_original_carrier_limit_without_mutation() {
    let decoded = compressed_growth_record();
    let stored = compressed(&decoded, flate2::Compression::best());
    let source = carrier(&stored, stored.len());
    let mut context = options(&owned("v8-rich"));
    context.compression = FlowLegacyTableTransport::Zlib;
    context.leaf_count = FlowLeafCount::try_from(1).unwrap();
    let extras = source
        .record_408_extras(&source.latest(408).unwrap().unwrap().id, context)
        .unwrap();
    let field = extras
        .fields()
        .find(|f| f.kind() == FlowBlockExtraFieldKind::FinalWords)
        .unwrap();
    check_growth_size(&decoded, &stored, field);
    check_growth_rejection(&extras, field);
    assert_eq!(extras.source().as_bytes(), source.as_bytes());
    assert_eq!(extras.decoded_bytes(), decoded);
}

fn compressed_growth_record() -> Vec<u8> {
    let mut raw = 8_i32.to_le_bytes().to_vec();
    raw.extend_from_slice(&(-1_i32).to_le_bytes());
    raw.extend_from_slice(&[0, 0, 0, 0, 0, 1, 0, 1]);
    raw.extend_from_slice(&0_i32.to_le_bytes());
    raw.extend_from_slice(&2048_i32.to_le_bytes());
    raw.resize(raw.len() + 8192, 0);
    raw.extend_from_slice(&[0, 0]);
    raw
}

fn compressed(raw: &[u8], compression: flate2::Compression) -> Vec<u8> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), compression);
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}

fn check_structural_failure(source: &FlowBlockExtras, field: FlowBlockExtraField<'_>) {
    let error = source
        .replace_field(&field.id(), 0, field.value(0).unwrap())
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
}

fn check_nonstructural_failure(source: &FlowBlockExtras, field: FlowBlockExtraField<'_>) {
    assert!(
        source
            .replace_field(&field.id(), usize::MAX, FlowBlockExtraValue::Byte(0))
            .is_err()
    );
    assert!(
        source
            .replace_field(&field.id(), 0, wrong_value(field.encoding()))
            .is_err()
    );
}

fn check_branch_changes(source: &FlowBlockExtras) {
    for kind in [
        FlowBlockExtraFieldKind::OuterFlag,
        FlowBlockExtraFieldKind::NestedFlag,
        FlowBlockExtraFieldKind::FinalFlag,
    ] {
        let id = source.fields().find(|f| f.kind() == kind).unwrap().id();
        assert!(
            source
                .replace_field(&id, 0, FlowBlockExtraValue::Byte(2))
                .is_err()
        );
    }
}

fn check_empty_interleaved(source: &FlowBlockExtras, field: FlowBlockExtraField<'_>) {
    assert_eq!(
        (
            field.layout().first_byte,
            field.layout().element_count,
            field.layout().stride_bytes
        ),
        (36, 0, 12)
    );
    assert!(field.value(0).is_none() && field.contiguous_bytes().is_none());
    assert!(
        source
            .replace_field(&field.id(), 0, FlowBlockExtraValue::Signed32(0))
            .is_err()
    );
}

fn check_stale_field(source: &FlowBlockExtras, field: FlowBlockExtraField<'_>) {
    let edit = source
        .replace_field(&field.id(), 0, FlowBlockExtraValue::Byte(0))
        .unwrap();
    assert!(edit.result().field(&field.id()).is_err());
    assert!(
        edit.result()
            .replace_field(&field.id(), 0, FlowBlockExtraValue::Byte(1))
            .is_err()
    );
}

fn check_growth_size(decoded: &[u8], stored: &[u8], field: FlowBlockExtraField<'_>) {
    let mut changed = decoded.to_vec();
    let offset = field.element_offset(0).unwrap();
    changed[offset..offset + 4].copy_from_slice(&0x12345678_i32.to_le_bytes());
    assert!(compressed(&changed, flate2::Compression::fast()).len() > stored.len());
}

fn check_growth_rejection(extras: &FlowBlockExtras, field: FlowBlockExtraField<'_>) {
    assert!(
        extras
            .replace_field(&field.id(), 0, FlowBlockExtraValue::Signed32(0x12345678))
            .is_err()
    );
}
