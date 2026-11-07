use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/replay.rs"]
mod replay;
const KINDS: [FlowImsArrayKind; 4] = [
    FlowImsArrayKind::First,
    FlowImsArrayKind::Second,
    FlowImsArrayKind::FirstExtra,
    FlowImsArrayKind::SecondExtra,
];
const VERSIONS: [FlowImsDefinitionVersion; 4] = [
    FlowImsDefinitionVersion::V0,
    FlowImsDefinitionVersion::V1,
    FlowImsDefinitionVersion::V2,
    FlowImsDefinitionVersion::V3,
];
fn options() -> FlowResultDefinitionOptions {
    FlowResultDefinitionOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_objects: 100_000,
        maximum_values: 1_000_000,
    }
}
fn open(
    raw: &[u8],
    options: FlowResultDefinitionOptions,
) -> bessemer::Result<FlowResultDefinitions> {
    let source = state_carrier::carrier(500, raw);
    let id = source.records().nth(1).unwrap().id;
    source.record_500_ims_definitions(&id, options)
}
fn raw_count(row: &DefinitionCase) -> usize {
    row.groups[0]
        .objects
        .iter()
        .flat_map(|obj| obj.body.arrays.iter().flatten())
        .map(|array| array.count as usize)
        .sum()
}
fn check_case(row: &DefinitionCase) -> FlowResultDefinitions {
    let view = open(&row.stored, options()).unwrap();
    assert_eq!(view.version(), row.version);
    assert_eq!(view.options(), options());
    assert_eq!(view.decoded_bytes(), row.decoded);
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.stored_count(), row.groups[0].objects.len());
    for (ordinal, group) in row.groups.iter().enumerate() {
        assert_eq!(
            view.collection_interval(ordinal).unwrap(),
            (group.start as usize, group.end as usize)
        );
    }
    for (ordinal, object) in row.groups[0].objects.iter().enumerate() {
        check_object(view.definition(ordinal).unwrap(), object, ordinal);
    }
    check_coverage(&view, row);
    check_diagnostics(&view);
    view
}
fn check_object(view: FlowImsDefinition<'_>, object: &DefinitionObject, ordinal: usize) {
    assert_eq!(view.ordinal(), ordinal);
    assert_eq!(view.identifier(), object.identifier);
    assert_eq!(
        view.identifier_interval(),
        (5 + 4 * ordinal, 9 + 4 * ordinal)
    );
    assert_eq!(view.guid_bytes(), object.guid);
    assert_eq!(
        view.body_interval(),
        (object.body.start as usize, object.body.end as usize)
    );
    assert_eq!(view.version(), VERSIONS[object.body.version as usize]);
    assert_eq!(view.allocation_byte(), object.body.allocation);
    for (kind, array) in KINDS.into_iter().zip(&object.body.arrays) {
        check_array(view, kind, array.as_ref());
    }
}
fn check_array(
    view: FlowImsDefinition<'_>,
    kind: FlowImsArrayKind,
    expected: Option<&DefinitionArray>,
) {
    let Some(expected) = expected else {
        assert!(view.array(kind).is_none() && view.shared_array_interval(kind).is_none());
        return;
    };
    assert_eq!(
        view.shared_array_interval(kind),
        Some((expected.start as usize, expected.end as usize))
    );
    let array = view.array(kind).unwrap();
    assert_eq!(array.as_bytes(), expected.raw);
    assert_eq!(array.word_count(), expected.count as usize);
    assert_eq!(
        array.payload_interval(),
        (expected.start as usize + 12, expected.end as usize)
    );
    for (index, raw) in expected.raw.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(array.raw_word(index), Some(u32::from_le_bytes(*raw)));
    }
    assert!(array.raw_word(array.word_count()).is_none() && array.raw_word(usize::MAX).is_none());
}
fn check_coverage(view: &FlowResultDefinitions, row: &DefinitionCase) {
    let proof = view.coverage();
    let arrays: Vec<_> = row.groups[0]
        .objects
        .iter()
        .flat_map(|obj| obj.body.arrays.iter().flatten())
        .collect();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactZlibTable);
    assert_eq!(proof.stored_bytes, row.stored.len());
    assert_eq!(proof.decoded_bytes, row.decoded.len());
    assert_eq!(proof.stored_objects, row.groups[0].objects.len());
    assert_eq!(proof.stored_arrays, arrays.len());
    assert_eq!(
        proof.empty_arrays,
        arrays.iter().filter(|array| array.count == 0).count()
    );
    assert_eq!(proof.raw_words, raw_count(row));
    assert_eq!(proof.achieved_error, 0);
}
fn check_diagnostics(view: &FlowResultDefinitions) {
    use FlowSimpleTableDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn all_owned_complete_definitions_match_every_field_interval_presence_and_raw_word() {
    for row in cases() {
        check_case(&row);
    }
}

fn snapshot(raw: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(
        raw,
        FlowLimits {
            maximum_wire_bytes: 4_000_000,
            maximum_payload_bytes: 2_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}
