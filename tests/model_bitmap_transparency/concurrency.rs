use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use bessemer::BessemerError;
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

type BitmapMixedOutcome = (
    ModelVisualOperation,
    ModelVisualOperation,
    [usize; 2],
    BessemerError,
    BessemerError,
);
type BitmapMixedTrace = Option<ZlibEncoder<Vec<u8>>>;

fn bitmap_mixed_failures(
    foreign: &PartResolvedFeaturesArchive,
    changed: &PartResolvedFeaturesEdit,
    state: &ModelBitmapTransparencyState,
    request: ModelBitmapTransparencyEdit,
) -> (BessemerError, BessemerError) {
    let field = &state.source_fields()[0];
    let expected_foreign = foreign.resolve_field(field).unwrap_err();
    let expected_stale = changed.archive().resolve_field(field).unwrap_err();
    let rejected = foreign
        .edit_bitmap_transparency(state, request)
        .unwrap_err();
    let stale = changed
        .archive()
        .edit_bitmap_transparency(state, request)
        .unwrap_err();
    assert_eq!(rejected, expected_foreign);
    assert_eq!(stale, expected_stale);
    (rejected, stale)
}

fn bitmap_unchanged_result(
    source: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
    expected: (usize, i32),
) -> PartResolvedFeaturesEdit {
    let unchanged = source
        .edit_bitmap_transparency(state, setting_request(role, expected.1))
        .unwrap();
    verify_bitmap_result(
        source,
        &source.to_bytes().unwrap(),
        state,
        role,
        expected.1,
        expected,
        &unchanged,
    );
    assert_eq!(unchanged.archive().fields(), source.fields());
    unchanged
}

fn capture_bitmap_mixed(
    source: &PartResolvedFeaturesArchive,
    changed: &PartResolvedFeaturesEdit,
    unchanged: &PartResolvedFeaturesEdit,
    failures: (BessemerError, BessemerError),
) -> BitmapMixedOutcome {
    let (rejected, stale) = failures;
    (
        model_visual_operation(source, changed),
        model_visual_operation(source, unchanged),
        [changed.unchanged_fields(), unchanged.unchanged_fields()],
        rejected,
        stale,
    )
}

fn bitmap_mixed_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
    expected: (usize, i32),
) -> BitmapMixedOutcome {
    let value = !expected.1;
    let request = setting_request(role, value);
    let changed = source.edit_bitmap_transparency(state, request).unwrap();
    verify_bitmap_result(
        source,
        &source.to_bytes().unwrap(),
        state,
        role,
        value,
        expected,
        &changed,
    );
    let unchanged = bitmap_unchanged_result(source, state, role, expected);
    let failures = bitmap_mixed_failures(foreign, &changed, state, request);
    capture_bitmap_mixed(source, &changed, &unchanged, failures)
}

fn bitmap_mixed_owner(
    case: usize,
    owner: usize,
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
) -> Vec<BitmapMixedOutcome> {
    (0..3)
        .map(|role| {
            bitmap_mixed_outcome(
                source,
                foreign,
                state,
                role,
                bitmap_scalar(case, owner, role),
            )
        })
        .collect()
}

fn bitmap_mixed_outcomes(
    case: usize,
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    states: &[ModelBitmapTransparencyState],
) -> Vec<BitmapMixedOutcome> {
    states
        .iter()
        .enumerate()
        .flat_map(|(owner, state)| bitmap_mixed_owner(case, owner, source, foreign, state))
        .collect()
}

fn bitmap_concurrent_case(
    case: usize,
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    raw: &[u8],
) -> Vec<BitmapMixedOutcome> {
    let original = source.fields().to_vec();
    let states = source.bitmap_transparency_states().unwrap();
    assert_eq!(states.len(), 2);
    let serial = bitmap_mixed_outcomes(case, source, foreign, &states);
    crate::visual_workers::assert_eight_visual_workers(&serial, || {
        bitmap_mixed_outcomes(case, source, foreign, &states)
    });
    assert_eq!(source.fields(), original);
    assert_eq!(foreign.fields(), original);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
    serial
}

fn append_bitmap_concurrent_case(
    output: &mut Option<Vec<u8>>,
    trace: &mut BitmapMixedTrace,
    case: usize,
    raw: &[u8],
    serial: &[BitmapMixedOutcome],
) {
    append_bitmap_export(output, raw);
    for (changed, ..) in serial {
        append_bitmap_export(output, &changed.bytes);
    }
    if let Some(trace) = trace {
        writeln!(trace, "{case}\t{serial:?}").unwrap();
    }
}

fn bitmap_concurrent_matrix(output: &mut Option<Vec<u8>>, trace: &mut BitmapMixedTrace) {
    let (configuration, manager) = bitmap_context();
    let original = configuration.to_bytes().unwrap();
    for (case, raw) in bitmap_images().into_iter().enumerate() {
        let source = bitmap_archive(raw, &configuration, &manager);
        let foreign = bitmap_archive(raw, &configuration, &manager);
        let serial = bitmap_concurrent_case(case, &source, &foreign, raw);
        append_bitmap_concurrent_case(output, trace, case, raw, &serial);
        println!("BITMAP_CONCURRENT\t{case}\t8\t6");
    }
    assert_eq!(configuration.to_bytes().unwrap(), original);
}

#[test]
fn every_bitmap_setting_case_isolates_mixed_requests_across_eight_workers() {
    let output_path = std::env::var_os("BESSEMER_BITMAP_CONCURRENT_OUTPUT");
    let trace_path = std::env::var_os("BESSEMER_BITMAP_CONCURRENT_TRACE");
    let mut output = output_path
        .as_ref()
        .map(|_| [b"BTC1".as_slice(), &731_u32.to_le_bytes()].concat());
    let mut trace = trace_path
        .as_ref()
        .map(|_| ZlibEncoder::new(Vec::new(), Compression::fast()));
    bitmap_concurrent_matrix(&mut output, &mut trace);
    if let (Some(path), Some(bytes)) = (output_path, output) {
        std::fs::write(path, bytes).unwrap();
    }
    if let (Some(path), Some(trace)) = (trace_path, trace) {
        std::fs::write(path, trace.finish().unwrap()).unwrap();
    }
}
