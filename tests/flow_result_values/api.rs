use super::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/replay.rs"]
mod replay;
const ARRAYS: [FlowImsValueArray; 3] = [
    FlowImsValueArray::First,
    FlowImsValueArray::Main,
    FlowImsValueArray::Last,
];
const FLAGS: [FlowImsValueFlag; 3] = [
    FlowImsValueFlag::First,
    FlowImsValueFlag::Bl,
    FlowImsValueFlag::Last,
];
fn options() -> FlowImsValueOptions {
    FlowImsValueOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_objects: 100_000,
        maximum_shared_values: 1_000_000,
        maximum_bl_rows: 100_000,
        maximum_bl_values: 2_000_000,
    }
}
fn context(row: &ValueCase) -> FlowImsValueContext {
    FlowImsValueContext::new(&check_context(row), FlowImsFreshObjects::FreshDeclared).unwrap()
}
fn open(
    stored: &[u8],
    context: &FlowImsValueContext,
    options: FlowImsValueOptions,
) -> bessemer::Result<FlowImsValues> {
    let source = state_carrier::carrier(501, stored);
    let id = source.records().nth(1).unwrap().id;
    source.record_501_ims_values(&id, context, options)
}
fn check_case(row: &ValueCase) -> FlowImsValues {
    let context = context(row);
    let view = open(&row.frame.stored, &context, options()).unwrap();
    assert_eq!(view.context(), &context);
    assert_eq!(
        view.context().declaration(),
        FlowImsFreshObjects::FreshDeclared
    );
    assert_eq!(view.options(), options());
    assert_eq!(view.version(), row.frame.version);
    assert_eq!(view.decoded_bytes(), row.frame.decoded);
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.stored_count(), row.frame.objects.len());
    for (ordinal, object) in row.frame.objects.iter().enumerate() {
        check_object(view.object(ordinal).unwrap(), object, ordinal);
    }
    check_coverage(&view, row);
    check_diagnostics(&view);
    view
}
fn check_object(view: FlowImsValue<'_>, object: &ImsValueObject, ordinal: usize) {
    assert_eq!(view.ordinal(), ordinal);
    assert_eq!(view.identifier(), object.identifier);
    assert_eq!(view.definition().unwrap().identifier(), object.identifier);
    assert_eq!(view.version(), object.version);
    assert_eq!(
        view.body_interval(),
        (object.start as usize, object.end as usize)
    );
    for (flag, raw) in FLAGS
        .into_iter()
        .zip([object.first_flag, object.bl_flag, object.last_flag])
    {
        assert_eq!(view.flag(flag), raw);
    }
    check_flag_intervals(view, object);
    check_arrays(view, object);
    check_bl(view.bl(), object.bl.as_ref());
}
fn check_arrays(view: FlowImsValue<'_>, object: &ImsValueObject) {
    for (kind, array) in ARRAYS.into_iter().zip([
        object.first.as_ref(),
        Some(&object.main),
        object.last.as_ref(),
    ]) {
        check_array(view, kind, array);
    }
}
fn check_flag_intervals(view: FlowImsValue<'_>, object: &ImsValueObject) {
    let first = object.start as usize + 4;
    let bl = object.main.end as usize;
    let last = object.bl.as_ref().map_or(bl + 1, |body| body.end as usize);
    for (flag, offset) in FLAGS.into_iter().zip([first, bl, last]) {
        assert_eq!(view.flag_interval(flag), (offset, offset + 1));
    }
}
fn check_array(view: FlowImsValue<'_>, kind: FlowImsValueArray, expected: Option<&ValueArray>) {
    let Some(expected) = expected else {
        assert!(view.array(kind).is_none() && view.shared_array_interval(kind).is_none());
        return;
    };
    assert_eq!(
        view.shared_array_interval(kind),
        Some((expected.start as usize, expected.end as usize))
    );
    let array = view.array(kind).unwrap();
    assert_eq!(array.word_count(), expected.count as usize);
    assert_eq!(
        array.payload_interval(),
        (expected.start as usize + 12, expected.end as usize)
    );
    check_patterns(array, &expected.raw);
}
fn check_patterns(array: FlowTableWideArray<'_>, raw: &[u8]) {
    assert_eq!(array.as_bytes(), raw);
    let (words, remainder) = raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, word) in words.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*word)));
    }
    assert!(array.raw_bits(words.len()).is_none() && array.raw_bits(usize::MAX).is_none());
}
fn check_bl(view: Option<FlowImsBlRows<'_>>, expected: Option<&BlRows>) {
    let Some(expected) = expected else {
        assert!(view.is_none());
        return;
    };
    let bl = view.unwrap();
    assert_eq!(bl.version(), expected.version);
    assert_eq!(
        bl.body_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(bl.row_count(), expected.count as usize);
    assert_eq!(bl.native_state_after_load(), expected.native_state);
    for (ordinal, raw) in expected.raw.as_chunks::<144>().0.iter().enumerate() {
        let array = bl.row(ordinal).unwrap();
        assert_eq!(array.word_count(), 18);
        let start = expected.start as usize + 12 + ordinal * 144;
        assert_eq!(array.payload_interval(), (start, start + 144));
        check_patterns(array, raw);
    }
    assert!(bl.row(bl.row_count()).is_err() && bl.row(usize::MAX).is_err());
}
fn totals(row: &ValueCase) -> [usize; 3] {
    let objects = &row.frame.objects;
    let shared = objects
        .iter()
        .flat_map(|o| {
            [o.first.as_ref(), Some(&o.main), o.last.as_ref()]
                .into_iter()
                .flatten()
        })
        .map(|a| a.count as usize)
        .sum();
    let rows: usize = objects
        .iter()
        .filter_map(|o| o.bl.as_ref())
        .map(|b| b.count as usize)
        .sum();
    [shared, rows, rows * 18]
}
fn check_coverage(view: &FlowImsValues, row: &ValueCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactZlibTable);
    assert_eq!(
        &proof.definition_record,
        view.context().definitions().record()
    );
    assert_eq!(proof.fresh_objects, FlowImsFreshObjects::FreshDeclared);
    assert_eq!(proof.stored_bytes, row.frame.stored.len());
    assert_eq!(proof.decoded_bytes, row.frame.decoded.len());
    assert_eq!(proof.stored_objects, row.frame.objects.len());
    assert_eq!(
        [proof.shared_values, proof.bl_rows, proof.bl_values],
        totals(row)
    );
    assert_eq!(proof.achieved_error, 0);
    check_presence_coverage(&proof, row);
}
fn check_presence_coverage(proof: &FlowImsValueCoverage, row: &ValueCase) {
    let arrays = case_arrays(row);
    let bl: Vec<_> = row
        .frame
        .objects
        .iter()
        .filter_map(|o| o.bl.as_ref())
        .collect();
    assert_eq!(proof.shared_arrays, arrays.len());
    assert_eq!(
        proof.empty_shared_arrays,
        arrays.iter().filter(|a| a.count == 0).count()
    );
    assert_eq!(proof.bl_bodies, bl.len());
    assert_eq!(
        proof.empty_bl_bodies,
        bl.iter().filter(|b| b.count == 0).count()
    );
}
fn check_diagnostics(view: &FlowImsValues) {
    use FlowImsValueDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            FreshObjectsAssumed,
            DefinitionPairingAssumed,
            NativeRuntimeNotReproduced,
            EngineeringColumnsNotAssigned,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn all_complete_values_match_every_raw_field_row_interval_flag_and_retained_definition() {
    for row in cases() {
        check_case(&row);
    }
}

fn case_arrays(row: &ValueCase) -> Vec<&ValueArray> {
    row.frame
        .objects
        .iter()
        .flat_map(|o| {
            [o.first.as_ref(), Some(&o.main), o.last.as_ref()]
                .into_iter()
                .flatten()
        })
        .collect()
}
