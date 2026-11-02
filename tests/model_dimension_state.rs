//! Complete stored dimension operations against literal carriers and original-reader byte partitions (#179).
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use bessemer::{Document, FailureClass};

#[path = "model_dimension_state/configuration.rs"]
mod configuration_tests;
#[path = "model_dimension_state/edits.rs"]
mod edits;
#[path = "model_dimension_state/exports.rs"]
mod exports;
#[path = "model_dimension_state/ownership.rs"]
mod ownership;

const OWNED: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const INPUTS: &[u8] = include_bytes!("fixtures/dimension-stored-state/owned.bin");
const CONFIGURATION: &[u8] = include_bytes!("fixtures/dimension-stored-state/configuration.bin");
const CONFIGURATION_FEATURES: &[u8] =
    include_bytes!("fixtures/dimension-stored-state/configuration-features.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

struct DimensionCase<'a> {
    class: &'a str,
    mask: u32,
    identifier: i32,
    offsets: [usize; 2],
    raw: &'a [u8],
}

fn base_configuration() -> (PartConfigurationArchive, Vec<u8>) {
    let document = Document::from_bytes(OWNED).unwrap();
    let archive = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    (archive, document.read_stream("Contents/CMgr").unwrap())
}

fn cases() -> Vec<DimensionCase<'static>> {
    let mut cursor = INPUTS;
    let result = include_str!("fixtures/dimension-stored-state/dimensions.tsv")
        .lines()
        .map(|line| dimension_case(line, &mut cursor))
        .collect::<Vec<_>>();
    assert_eq!(result.len(), 5440);
    assert!(cursor.is_empty());
    result
}

fn dimension_case(line: &'static str, cursor: &mut &'static [u8]) -> DimensionCase<'static> {
    let columns: Vec<_> = line.split('\t').collect();
    assert_eq!(columns.len(), 6);
    let length = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
    let raw = &cursor[4..4 + length];
    *cursor = &cursor[4 + length..];
    assert_eq!(length, columns[5].parse::<usize>().unwrap());
    DimensionCase {
        class: columns[0],
        mask: columns[1].parse().unwrap(),
        identifier: columns[2].parse().unwrap(),
        offsets: [columns[3].parse().unwrap(), columns[4].parse().unwrap()],
        raw,
    }
}

fn feature_archive(
    case: &DimensionCase<'_>,
    source: &PartConfigurationArchive,
    manager: &[u8],
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_bytes(case.raw, source.clone(), manager, LIMITS).unwrap()
}

fn dimension_handle(archive: &PartResolvedFeaturesArchive, ordinal: usize) -> ModelFieldHandle {
    let indices: Vec<_> = archive
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".mask"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(indices.len(), 2);
    archive.field_handle(indices[ordinal]).unwrap()
}

fn expected_words(case: &DimensionCase<'_>, ordinal: usize) -> (u32, i32) {
    if ordinal == 0 {
        return (case.mask, case.identifier);
    }
    (
        case.mask ^ 0x8000001e,
        ((case.identifier as u32) ^ 0x80000000) as i32,
    )
}

fn assert_view(view: &ModelDimensionStoredState, mask: u32, identifier: i32) {
    assert_eq!(view.mask() as u32, mask);
    assert_eq!(view.redundant_id(), identifier);
    assert_eq!(view.enforce_bend_angle_range(), mask & 2 != 0);
    assert_eq!(
        view.fillet_ref_point_change_number() as u32,
        (mask >> 2) & 7
    );
    assert_eq!(view.mask_unresolved_bits(), mask & 0xffffffe1);
}

#[test]
fn every_complete_owned_dimension_matches_original_values_and_literal_word_offsets() {
    let (source, manager) = base_configuration();
    for case in cases() {
        let archive = feature_archive(&case, &source, &manager);
        assert_eq!(archive.to_bytes().unwrap(), case.raw);
        assert_eq!(archive.object_slots(), (97, 100));
        for ordinal in 0..2 {
            let view = archive
                .dimension_state(&dimension_handle(&archive, ordinal))
                .unwrap();
            let (mask, identifier) = expected_words(&case, ordinal);
            assert_view(&view, mask, identifier);
            let start = case.offsets[ordinal];
            assert_eq!(&case.raw[start + 16..start + 20], mask.to_le_bytes());
            assert_eq!(&case.raw[start + 20..start + 24], identifier.to_le_bytes());
        }
    }
}
