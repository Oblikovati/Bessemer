use super::super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::entity_visual::EntityVisualStoredEdit;
use crate::model_archives::{
    CompressedKernelContext, ModelEditCompletion, ModelEntityVisualStoredState,
    PartResolvedFeaturesArchive,
};

struct NativeVisualSpan {
    offset: usize,
    words: [u8; 17],
}

fn native_source_spans(cursor: &mut ArchiveCursor<'_>, count: usize) -> Vec<NativeVisualSpan> {
    (0..count)
        .map(|_| NativeVisualSpan {
            offset: cursor.u32().unwrap() as usize,
            words: cursor.take(17).unwrap().try_into().unwrap(),
        })
        .collect()
}

macro_rules! native_rgb_editor {
    ($source:expr) => {
        |state: &ModelEntityVisualStoredState, rgb| {
            let result = $source
                .edit_entity_visual_state(state, EntityVisualStoredEdit::ColorRgb(rgb))
                .unwrap();
            assert_eq!(result.completion(), ModelEditCompletion::Changed);
            assert_eq!(result.changes().len(), 1);
            let index = state.source_field().index() + 3;
            assert_eq!(
                result.changes()[0].parents(),
                [$source.field_handle(index).unwrap()]
            );
            assert!(
                result
                    .archive()
                    .entity_visual_state(state.source_field())
                    .is_err()
            );
            result.archive().to_bytes().unwrap()
        }
    };
}

fn native_visual_edit_projection(
    raw: &[u8],
    states: &[ModelEntityVisualStoredState],
    spans: &[NativeVisualSpan],
    edit: impl Fn(&ModelEntityVisualStoredState, [u8; 3]) -> Vec<u8>,
) {
    assert_eq!(states.len(), spans.len());
    for (state, span) in states.iter().zip(spans) {
        assert_eq!(&raw[span.offset..span.offset + 17], &span.words);
        assert_native_source_words(state, &span.words);
        let color = u32::from_le_bytes(span.words[5..9].try_into().unwrap()) ^ 0x00a53cf0;
        let rgb = [color as u8, (color >> 8) as u8, (color >> 16) as u8];
        let mut expected = raw.to_vec();
        expected[span.offset + 5..span.offset + 9].copy_from_slice(&color.to_le_bytes());
        assert_eq!(
            edit(state, rgb),
            expected,
            "native visual offset {}",
            span.offset
        );
    }
}

fn assert_native_source_words(state: &ModelEntityVisualStoredState, raw: &[u8; 17]) {
    let words = state.words();
    assert_eq!(
        words.layer_number,
        u16::from_le_bytes(raw[0..2].try_into().unwrap())
    );
    assert_eq!(
        words.by_layer_flags,
        u16::from_le_bytes(raw[2..4].try_into().unwrap())
    );
    assert_eq!(words.layer_attribute_cache, raw[4]);
    assert_eq!(
        words.color_word,
        u32::from_le_bytes(raw[5..9].try_into().unwrap())
    );
    assert_native_source_width(state, raw);
}

fn native_visual_edit_case(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
    configuration_spans: &[NativeVisualSpan],
    feature_spans: &[NativeVisualSpan],
) {
    let source = configuration_for_case(case, kernel).unwrap();
    native_visual_edit_projection(
        case.configuration,
        &source.entity_visual_states().unwrap(),
        configuration_spans,
        native_rgb_editor!(source),
    );
    assert_eq!(source.to_bytes().unwrap(), case.configuration);
    let features =
        PartResolvedFeaturesArchive::from_bytes(case.features, source, case.manager, LIMITS)
            .unwrap();
    native_visual_edit_projection(
        case.features,
        &features.entity_visual_states().unwrap(),
        feature_spans,
        native_rgb_editor!(features),
    );
    assert_eq!(features.to_bytes().unwrap(), case.features);
}

fn next_native_visual_edit_case(
    cursor: &mut ArchiveCursor<'_>,
    oracle: &mut ArchiveCursor<'_>,
    index: usize,
    kernel: Option<CompressedKernelContext>,
) -> [usize; 2] {
    assert_eq!(oracle.u32().unwrap() as usize, index);
    let counts = [
        oracle.u32().unwrap() as usize,
        oracle.u32().unwrap() as usize,
    ];
    let configurations = native_source_spans(oracle, counts[0]);
    let features = native_source_spans(oracle, counts[1]);
    native_visual_edit_case(&next_case(cursor), kernel, &configurations, &features);
    println!("VISUAL_NATIVE_EDIT\t{index}\t{}\t{}", counts[0], counts[1]);
    counts
}

#[test]
#[ignore = "requires complete906 native packets, original Source span oracle and explicit kernel schema"]
fn native_visual_rgb_edits_match_source_offsets_for_every_physical_record() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let spans = std::fs::read("target/entity-visual-native-ordered-spans.bin").unwrap();
    let (mut cursor, mut oracle) = (ArchiveCursor::new(&bytes), ArchiveCursor::new(&spans));
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(oracle.take(4).unwrap(), b"BVS1");
    assert_eq!((cursor.u32().unwrap(), oracle.u32().unwrap()), (906, 906));
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut totals = [0; 2];
    for index in 0..906 {
        let counts = next_native_visual_edit_case(&mut cursor, &mut oracle, index, kernel.clone());
        for (total, count) in totals.iter_mut().zip(counts) {
            *total += count;
        }
    }
    assert_eq!((cursor.remaining(), oracle.remaining()), (0, 0));
    assert_eq!(totals, [9586, 127773]);
    println!("VISUAL_NATIVE_EDIT_TOTAL\t{}\t{}", totals[0], totals[1]);
}

fn assert_native_source_width(state: &ModelEntityVisualStoredState, raw: &[u8; 17]) {
    let words = state.words();
    assert_eq!(
        words.line_style,
        u16::from_le_bytes(raw[9..11].try_into().unwrap())
    );
    assert_eq!(
        words.standard_width,
        i16::from_le_bytes(raw[11..13].try_into().unwrap())
    );
    assert_eq!(
        words.custom_width.unwrap().bits(),
        u32::from_le_bytes(raw[13..17].try_into().unwrap())
    );
}
