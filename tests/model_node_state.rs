//! Complete archive-backed node projections and stored-bit changes against independent literals (#178).
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use bessemer::{Document, FailureClass};

#[path = "model_node_state/configuration.rs"]
mod configuration_tests;
#[path = "model_node_state/edits.rs"]
mod edits;
#[path = "model_node_state/exports.rs"]
mod exports;
#[path = "model_node_state/ownership.rs"]
mod ownership;

const OWNED: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const INPUTS: &[u8] = include_bytes!("fixtures/model-node-state/owned.bin");
const ORACLE: &str = include_str!("fixtures/model-node-state/nodes.tsv");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

const FLAGS: &[(ModelNodeStoredFlag, usize, u32, bool)] = &[
    (ModelNodeStoredFlag::NeedsUpdate, 0, 1, false),
    (ModelNodeStoredFlag::UpdateError, 0, 2, false),
    (ModelNodeStoredFlag::UpdateWarning, 0, 4, false),
    (ModelNodeStoredFlag::SatNotSolved, 0, 0x100, false),
    (ModelNodeStoredFlag::UpdateLocked, 0, 0x4000, false),
    (ModelNodeStoredFlag::UpdatePending, 0, 0x8000, false),
    (ModelNodeStoredFlag::UpdateSecurity, 0, 0x20000, false),
    (ModelNodeStoredFlag::NeedsIdToBeSet, 4, 0x40000000, false),
    (ModelNodeStoredFlag::ReadOnly, 4, 0x10000000, true),
    (ModelNodeStoredFlag::SharedParameter, 4, 0x20000000, true),
    (
        ModelNodeStoredFlag::GlobalVariableParameter,
        4,
        0x80000000,
        true,
    ),
    (ModelNodeStoredFlag::SharedThickness, 12, 1, true),
];

struct NodeCase<'a> {
    class: &'a str,
    words: [u32; 3],
    offsets: [usize; 2],
    raw: &'a [u8],
}

fn configuration() -> (PartConfigurationArchive, Vec<u8>) {
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

fn cases() -> Vec<NodeCase<'static>> {
    let mut cursor = INPUTS;
    let result = ORACLE
        .lines()
        .map(|line| case_from_line(line, &mut cursor))
        .collect::<Vec<_>>();
    assert!(cursor.is_empty());
    assert_eq!(result.len(), 992);
    result
}

fn case_from_line(line: &'static str, cursor: &mut &'static [u8]) -> NodeCase<'static> {
    let columns: Vec<_> = line.split('\t').collect();
    assert_eq!(columns.len(), 7);
    let input = *cursor;
    let size = u32::from_le_bytes(input[..4].try_into().unwrap()) as usize;
    let raw = &input[4..4 + size];
    *cursor = &input[4 + size..];
    assert_eq!(size, columns[6].parse::<usize>().unwrap());
    NodeCase {
        class: columns[0],
        words: std::array::from_fn(|i| columns[i + 1].parse().unwrap()),
        offsets: [columns[4].parse().unwrap(), columns[5].parse().unwrap()],
        raw,
    }
}

fn feature_archive(
    case: &NodeCase<'_>,
    source: &PartConfigurationArchive,
    manager: &[u8],
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_bytes(case.raw, source.clone(), manager, LIMITS).unwrap()
}

fn node_handle(archive: &PartResolvedFeaturesArchive, ordinal: usize) -> ModelFieldHandle {
    let path = format!("root.features[{ordinal}].node.unknown_0c");
    let index = archive
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap();
    archive.field_handle(index).unwrap()
}

fn is_parameter(case: &NodeCase<'_>) -> bool {
    !matches!(
        case.class,
        "moFavoriteFeatItemData_c" | "moHistoryFeatItemData_c"
    )
}

fn expected_words(case: &NodeCase<'_>, ordinal: usize) -> [u32; 3] {
    if ordinal == 0 {
        return case.words;
    }
    [
        case.words[0] ^ 0x80000008,
        case.words[1] ^ 0x40000000,
        case.words[2] ^ 0xfffffffe,
    ]
}

fn assert_projection(node: &ModelNodeStoredState, case: &NodeCase<'_>, ordinal: usize) {
    let words = expected_words(case, ordinal);
    assert_eq!(node.update_state() as u32, words[0]);
    assert_eq!(node.flags_28(), words[1]);
    assert_eq!(node.flags_2c(), Some(words[2]));
    assert_eq!(
        node.parameter_class(),
        is_parameter(case).then_some(case.class)
    );
    assert_unresolved_words(node, case, words);
    assert_flags(node, case, words);
}

fn assert_unresolved_words(node: &ModelNodeStoredState, case: &NodeCase<'_>, words: [u32; 3]) {
    let first_mask = if is_parameter(case) {
        0x0fffffff
    } else {
        0xbfffffff
    };
    let second_mask = if is_parameter(case) {
        0xfffffffe
    } else {
        u32::MAX
    };
    assert_eq!(node.update_state_unresolved_bits(), words[0] & 0xfffd3ef8);
    assert_eq!(node.flags_28_unresolved_bits(), words[1] & first_mask);
    assert_eq!(
        node.flags_2c_unresolved_bits(),
        Some(words[2] & second_mask)
    );
}

fn assert_flags(node: &ModelNodeStoredState, case: &NodeCase<'_>, words: [u32; 3]) {
    for &(flag, offset, mask, parameter) in FLAGS {
        if parameter && !is_parameter(case) {
            assert_eq!(
                node.stored_flag(flag).unwrap_err().class,
                FailureClass::Unsupported
            );
            continue;
        }
        let word = words[match offset {
            0 => 0,
            4 => 1,
            12 => 2,
            _ => panic!("unknown oracle offset"),
        }];
        assert_eq!(node.stored_flag(flag).unwrap(), word & mask != 0);
    }
}

#[test]
fn every_complete_owned_node_and_loaded_class_matches_independent_words_and_roles() {
    let (source, manager) = configuration();
    for case in cases() {
        let archive = feature_archive(&case, &source, &manager);
        assert_eq!(archive.to_bytes().unwrap(), case.raw);
        assert_eq!(archive.object_slots(), (97, 100));
        for ordinal in 0..2 {
            assert_projection(
                &archive.node_state(&node_handle(&archive, ordinal)).unwrap(),
                &case,
                ordinal,
            );
        }
    }
}
