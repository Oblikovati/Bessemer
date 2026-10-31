//! Complete element selector, legacy rewrite and named topology reference comparisons.
use bessemer::display_lists::DisplayEditCompletion;
use bessemer::simulation::*;
use std::io::Read;

#[path = "common/simulation_element_checks.rs"]
mod checks;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut output)
        .unwrap();
    output
}

#[test]
fn paired_gen_record_snapshot_preserves_unsigned_counts_without_weakening_existing_accessor() {
    let owned = packet::owned();
    let mut cursor = Cursor(&owned);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 9);
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let image = SimulationGenArchive::from_bytes(cursor.span(), &master).unwrap();
    let original = image.to_bytes();
    let mut parts = image.parts();
    std::sync::Arc::make_mut(&mut parts.blocks[6])[23] = 255;
    let changed = SimulationGenArchive::from_parts(parts, &master).unwrap();
    record_snapshot_guards(&changed);
    assert_eq!(image.to_bytes(), original);
}

#[test]
fn every_unsigned_selector_combination_matches_actual_reference_and_closed_forms() {
    let bytes = inflate(include_bytes!("fixtures/simulation-elements/selectors.bin"));
    assert_eq!(&bytes[..4], b"SEL1");
    assert_eq!(bytes.len(), 4 + 256 * 256 * 256);
    let mut raw = [0xabu8; 140];
    raw[26..28].copy_from_slice(&0x1234u16.to_le_bytes());
    for (index, encoded) in bytes[4..].iter().enumerate() {
        raw[20] = (index >> 16) as u8;
        raw[23] = (index >> 8) as u8;
        raw[24] = index as u8;
        assert_selector(&raw, *encoded);
    }
}

#[test]
fn owned_legacy_topology_and_beam_projections_match_actual_reference_and_literal_bytes() {
    let bytes = inflate(include_bytes!("fixtures/simulation-elements/owned.bin"));
    let mut cursor = Cursor(&bytes);
    assert_eq!(cursor.take(4), b"ELO1");
    assert_eq!(cursor.word(), 1200);
    for _ in 0..1200 {
        legacy_case(&mut cursor);
    }
    owned_topology(&mut cursor);
    owned_predicates(&mut cursor);
    assert!(cursor.0.is_empty());
}

fn legacy_case(cursor: &mut Cursor<'_>) {
    let original = cursor.span();
    let version = cursor.word() as i32;
    let analysis = cursor.word() as i32;
    let golden = cursor.span();
    let record = SimulationElementRecord::from_bytes(original).unwrap();
    checks::classification(&record, cursor.span());
    let context = SimulationElementLegacyContext::new(version, analysis).unwrap();
    assert_eq!(context.version(), version);
    assert_eq!(context.analysis_code(), analysis);
    let projected = record.project_legacy(context);
    assert_legacy_contract(&projected, context);
    assert_legacy_completion(&record, &projected, original == golden);
    assert_eq!(projected.record.to_bytes().as_slice(), golden);
    assert_eq!(record.to_bytes().as_slice(), original);
}

#[test]
#[ignore = "requires exported approved local native corpus"]
fn every_native_record_matches_reference_histograms_topology_and_first_record_test() {
    let bytes = std::fs::read("target/simulation-elements-native.bin").unwrap();
    let mut cursor = Cursor(&bytes);
    assert_eq!(cursor.take(4), b"ELN1");
    assert_eq!(cursor.word(), 3);
    let mut totals = [0usize; 3];
    let mut projected = b"ELP1".to_vec();
    for _ in 0..3 {
        native_pair(&mut cursor, &mut totals, &mut projected);
    }
    assert_eq!(totals, [25106, 25094, 87852]);
    assert!(cursor.0.is_empty());
    std::fs::write(
        "target/simulation-elements-native-projections.bin",
        projected,
    )
    .unwrap();
}

fn native_record(
    image: &SimulationGenArchive,
    index: usize,
    expected: &mut Cursor<'_>,
    totals: &mut [usize; 3],
    projected: &mut Vec<u8>,
) {
    let record = image.element_record(index).unwrap();
    let actual = checks::classification(&record, expected.take(24));
    projected.extend(actual.into_iter().flat_map(u32::to_le_bytes));
    let fields = record.classification();
    totals[0] += 1;
    totals[1] += usize::from(u32::from(fields.stored_code) != fields.selected_code);
    assert_eq!(
        record.to_bytes().as_slice(),
        &image.block(6).unwrap()[index * 140..(index + 1) * 140]
    );
    if expected.word() != 0 {
        let topology = checks::topology(&record, Some(image.layout().counts[3]), expected.span());
        projected.extend_from_slice(&topology);
        totals[2] += 6;
    } else {
        assert_no_topology(&record, image.layout().counts[3]);
    }
}

fn record_snapshot_guards(image: &SimulationGenArchive) {
    let record = image.element_record(0).unwrap();
    assert_eq!(record.classification().accessor_node_count, 255);
    assert!(image.element(0).is_err());
    assert_eq!(
        record.to_bytes().as_slice(),
        &image.block(6).unwrap()[..140]
    );
    let count = image.layout().counts[4] as usize;
    assert!(image.element_record(count).is_err());
    assert!(image.element_record(usize::MAX).is_err());
}

fn assert_selector(raw: &[u8; 140], encoded: u8) {
    let record = SimulationElementRecord::from_bytes(raw).unwrap();
    let fields = record.classification();
    let expected = if encoded == 255 {
        u32::MAX
    } else {
        u32::from(encoded)
    };
    assert_eq!(
        fields.selected_code,
        expected,
        "inputs {:02x?}",
        [raw[20], raw[23], raw[24]]
    );
    assert_eq!(fields.selected_signed(), expected as i32);
    assert_eq!(fields.stored_code, 0x1234);
    assert_eq!(fields.selector_family, raw[20]);
    assert_eq!(fields.accessor_node_count, raw[23]);
    assert_eq!(fields.selector_variant, raw[24]);
    assert_eq!(record.to_bytes(), *raw);
}

fn owned_topology(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 16);
    for _ in 0..16 {
        let raw = cursor.span();
        let bound = (cursor.word() != 0).then_some(cursor.word());
        let record = SimulationElementRecord::from_bytes(raw).unwrap();
        checks::topology(&record, bound, cursor.span());
        assert_eq!(record.to_bytes().as_slice(), raw);
    }
}

fn owned_predicates(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 216);
    for _ in 0..216 {
        let raw = cursor.span();
        let record = SimulationElementRecord::from_bytes(raw).unwrap();
        assert_eq!(record.beam_first_record_predicate(), cursor.word() != 0);
        assert_eq!(record.to_bytes().as_slice(), raw);
    }
}

fn assert_legacy_contract(
    projected: &SimulationElementLegacyProjection,
    context: SimulationElementLegacyContext,
) {
    assert_eq!(projected.result, *projected.record.id());
    assert_eq!(projected.context, context);
    assert_eq!(
        projected.rewritten_code,
        projected.record.classification().stored_code
    );
    assert_eq!(projected.requested_pattern_error_bits, 0);
    assert_eq!(
        projected.validation,
        SimulationElementLegacyCoverage::ExactCompleteRecordWithStoredCodeReplacement
    );
    assert!(
        projected
            .diagnostics
            .contains(&SimulationElementDiagnostic::LoopVisitationNotProven)
    );
}

fn assert_legacy_completion(
    parent: &SimulationElementRecord,
    projected: &SimulationElementLegacyProjection,
    unchanged: bool,
) {
    assert_eq!(projected.parent, *parent.id());
    let completion = if unchanged {
        DisplayEditCompletion::Unchanged
    } else {
        DisplayEditCompletion::Changed
    };
    assert_eq!(projected.completion, completion);
    assert_eq!(projected.parent == projected.result, unchanged);
    let noop = projected.record.project_legacy(projected.context);
    assert_eq!(noop.completion, DisplayEditCompletion::Unchanged);
    assert_eq!(noop.result, projected.result);
}

fn native_pair(cursor: &mut Cursor<'_>, totals: &mut [usize; 3], projected: &mut Vec<u8>) {
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let raw = cursor.span();
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    let mut expected = Cursor(cursor.span());
    let count = expected.word() as usize;
    assert_eq!(count, image.layout().counts[4] as usize);
    for index in 0..count {
        native_record(&image, index, &mut expected, totals, projected);
    }
    assert_native_image_end(&image, raw, &mut expected, count);
}

fn assert_native_image_end(
    image: &SimulationGenArchive,
    raw: &[u8],
    expected: &mut Cursor<'_>,
    count: usize,
) {
    let first = image.element_record(0).unwrap();
    assert_eq!(first.beam_first_record_predicate(), expected.word() != 0);
    assert!(image.element_record(count).is_err());
    assert_eq!(image.to_bytes(), raw);
    assert!(expected.0.is_empty());
}

fn assert_no_topology(record: &SimulationElementRecord, bound: u32) {
    let original = record.to_bytes();
    assert!(record.solid_ten_node(Some(bound)).is_err());
    assert_eq!(record.to_bytes(), original);
}
