use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use bessemer::entity_visual::EntityVisualStoredEdit as Edit;
use bessemer::{BessemerError, FailureClass};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

type ModelTrace = Option<ZlibEncoder<Vec<u8>>>;

#[derive(Debug, PartialEq, Eq)]
struct ModelVisualMixedOutcome {
    changed: ModelVisualOperation,
    rejected: BessemerError,
    unchanged: ModelVisualOperation,
}

fn shared_model_rgb(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEntityVisualStoredState,
) -> PartResolvedFeaturesEdit {
    let color = state.words().color_word ^ 0x00a53cf0;
    let rgb = [color as u8, (color >> 8) as u8, (color >> 16) as u8];
    let result = source
        .edit_entity_visual_state(state, Edit::ColorRgb(rgb))
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(
        result.changes()[0].parents(),
        [source
            .field_handle(state.source_field().index() + 3)
            .unwrap()]
    );
    assert!(
        result
            .archive()
            .entity_visual_state(state.source_field())
            .is_err()
    );
    result
}

fn shared_model_width_rejection(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEntityVisualStoredState,
) -> BessemerError {
    let request = Edit::LineWidth {
        standard: 0,
        custom: None,
    };
    let error = source.edit_entity_visual_state(state, request).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    error
}

fn shared_model_noop(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEntityVisualStoredState,
) -> PartResolvedFeaturesEdit {
    let result = source
        .edit_entity_visual_state(state, Edit::ColorWord(state.words().color_word))
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
    assert!(result.changes().is_empty());
    assert!(result.archive().resolve_field(state.source_field()).is_ok());
    result
}

fn shared_model_outcomes(
    source: &PartResolvedFeaturesArchive,
    states: &[ModelEntityVisualStoredState],
) -> Vec<ModelVisualMixedOutcome> {
    states
        .iter()
        .map(|state| {
            let changed = shared_model_rgb(source, state);
            let rejected = shared_model_width_rejection(source, state);
            let unchanged = shared_model_noop(source, state);
            ModelVisualMixedOutcome {
                changed: model_visual_operation(source, &changed),
                rejected,
                unchanged: model_visual_operation(source, &unchanged),
            }
        })
        .collect()
}

fn focused_model_states(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) -> Vec<ModelEntityVisualStoredState> {
    visual_group_positions(source.fields(), suffix)
        .into_iter()
        .enumerate()
        .map(|(ordinal, index)| {
            let offset = columns[8 + ordinal].parse::<usize>().unwrap();
            let expected = visual_literal_values(&raw[offset..offset + 17]);
            let actual: Vec<_> = source.fields()[index..index + 7]
                .iter()
                .map(|field| field.value.clone())
                .collect();
            assert_eq!(actual, expected);
            source
                .entity_visual_state(&source.field_handle(index).unwrap())
                .unwrap()
        })
        .collect()
}

fn verify_model_serial_change(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelEntityVisualStoredState,
    offset: usize,
    serial: &ModelVisualMixedOutcome,
) {
    let color = u32::from_le_bytes(raw[offset + 5..offset + 9].try_into().unwrap()) ^ 0x00a53cf0;
    let mut expected = raw.to_vec();
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    assert_eq!(serial.changed.bytes, expected);
    let mut fields = source.fields().to_vec();
    fields[state.source_field().index() + 3].value = ModelFieldValue::U32(color);
    assert_eq!(serial.changed.fields, fields);
    assert_eq!(serial.unchanged.bytes, raw);
    assert_eq!(serial.unchanged.fields, source.fields());
    assert_eq!(
        serial.changed.coverage.mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(serial.changed.coverage.wire_bytes, raw.len());
    assert_eq!(serial.changed.coverage.typed_fields, source.fields().len());
    assert_eq!(
        serial.changed.fidelity,
        ModelEditFidelity::ExactStoredFields
    );
}

fn model_concurrent_case(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) -> Vec<ModelVisualMixedOutcome> {
    let original_fields = source.fields().to_vec();
    let states = focused_model_states(source, raw, columns, suffix);
    let serial = shared_model_outcomes(source, &states);
    for (ordinal, (state, outcome)) in states.iter().zip(&serial).enumerate() {
        verify_model_serial_change(
            source,
            raw,
            state,
            columns[8 + ordinal].parse().unwrap(),
            outcome,
        );
    }
    crate::visual_workers::assert_eight_visual_workers(&serial, || {
        shared_model_outcomes(source, &states)
    });
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(source.fields(), original_fields);
    serial
}

fn append_model_concurrent_projection(
    output: &mut Option<Vec<u8>>,
    trace: &mut ModelTrace,
    index: usize,
    raw: &[u8],
    serial: &[ModelVisualMixedOutcome],
) {
    if let Some(output) = output {
        for bytes in std::iter::once(raw).chain(
            serial
                .iter()
                .map(|outcome| outcome.changed.bytes.as_slice()),
        ) {
            output.extend_from_slice(&u32::try_from(bytes.len()).unwrap().to_le_bytes());
            output.extend_from_slice(bytes);
        }
    }
    if let Some(trace) = trace {
        writeln!(trace, "{index}\t{serial:?}").unwrap();
    }
}

fn model_concurrent_corpus(
    corpus: VisualCarrierCorpus,
    output: &mut Option<Vec<u8>>,
    trace: &mut ModelTrace,
    first: usize,
) -> usize {
    let (configuration, manager) = visual_model_context();
    let original_configuration = configuration.to_bytes().unwrap();
    let mut cursor = corpus.packet;
    let mut count = 0;
    for row in corpus.words.lines() {
        let columns: Vec<_> = row.split('\t').collect();
        let raw = visual_case_bytes(&mut cursor, &columns);
        let source = concurrent_model_archive(raw, &configuration, &manager, corpus.final_slots);
        let serial = model_concurrent_case(&source, raw, &columns, corpus.suffix);
        append_model_concurrent_projection(output, trace, first + count, raw, &serial);
        println!("VISUAL_MODEL_CONCURRENT\t{}\t8\t2", first + count);
        count += 1;
    }
    assert!(cursor.is_empty());
    assert_eq!(configuration.to_bytes().unwrap(), original_configuration);
    count
}

fn all_model_concurrent_corpora(output: &mut Option<Vec<u8>>, trace: &mut ModelTrace) {
    let mut first = 0;
    for corpus in [
        folder_visual_corpus(),
        sketch_visual_corpus(),
        nested_display_visual_corpus(),
    ] {
        let count = model_concurrent_corpus(corpus, output, trace, first);
        assert_eq!(count, 3831);
        first += count;
    }
    assert_eq!(first, 11493);
}

#[test]
fn all_owned_model_visual_matrices_isolate_shared_snapshots_across_eight_workers() {
    let output_path = std::env::var_os("BESSEMER_MODEL_VISUAL_CONCURRENT_OUTPUT");
    let trace_path = std::env::var_os("BESSEMER_MODEL_VISUAL_CONCURRENT_TRACE");
    let mut output = output_path
        .as_ref()
        .map(|_| 11493_u32.to_le_bytes().to_vec());
    let mut trace = trace_path
        .as_ref()
        .map(|_| ZlibEncoder::new(Vec::new(), Compression::fast()));
    all_model_concurrent_corpora(&mut output, &mut trace);
    if let (Some(path), Some(output)) = (output_path, output) {
        std::fs::write(path, output).unwrap();
    }
    if let (Some(path), Some(trace)) = (trace_path, trace) {
        std::fs::write(path, trace.finish().unwrap()).unwrap();
    }
}

fn concurrent_model_archive(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
    manager: &[u8],
    slots: usize,
) -> PartResolvedFeaturesArchive {
    let source =
        PartResolvedFeaturesArchive::from_bytes(raw, configuration.clone(), manager, VISUAL_LIMITS)
            .unwrap();
    assert_eq!(source.object_slots(), (97, slots));
    assert_eq!(source.root_count(), 3);
    source
}
