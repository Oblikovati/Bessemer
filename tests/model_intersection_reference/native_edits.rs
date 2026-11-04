use super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    CompressedKernelContext, ModelEditCompletion, ModelFieldValue,
    ModelIntersectionReferenceStoredEdit as Edit, ModelIntersectionReferenceStoredState as State,
    ModelIntersectionReferenceStoredWords as Words, PartResolvedFeaturesArchive,
};

#[path = "native_documents.rs"]
mod documents;

type NativeReferenceCheck =
    fn(&ResolvedCorpusCase<'_>, &NativeReferenceOracle, Option<CompressedKernelContext>);

struct NativeReferenceSpan {
    offset: usize,
    words: Words,
}

struct NativeReferenceOracle {
    index: u32,
    version: u32,
    label: String,
    spans: Vec<NativeReferenceSpan>,
}

fn read_native_reference_span(cursor: &mut ArchiveCursor<'_>) -> NativeReferenceSpan {
    let offset = cursor.u32().unwrap() as usize;
    let raw = cursor.take(14).unwrap();
    NativeReferenceSpan {
        offset,
        words: Words {
            raw_state: u16::from_le_bytes(raw[..2].try_into().unwrap()),
            raw_word_20: i32::from_le_bytes(raw[2..6].try_into().unwrap()),
            span_count_state: i32::from_le_bytes(raw[6..10].try_into().unwrap()),
            backward_offset_state: i32::from_le_bytes(raw[10..14].try_into().unwrap()),
        },
    }
}

fn read_native_reference_oracle(cursor: &mut ArchiveCursor<'_>) -> NativeReferenceOracle {
    let index = cursor.u32().unwrap();
    let version = cursor.u32().unwrap();
    let label_length = cursor.u32().unwrap() as usize;
    let groups = cursor.u32().unwrap() as usize;
    let label = std::str::from_utf8(cursor.take(label_length).unwrap())
        .unwrap()
        .to_owned();
    let spans = (0..groups)
        .map(|_| read_native_reference_span(cursor))
        .collect();
    NativeReferenceOracle {
        index,
        version,
        label,
        spans,
    }
}

fn verify_native_reference_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &State,
    span: &NativeReferenceSpan,
) -> crate::model_archives::PartResolvedFeaturesEdit {
    let expected = [
        span.words.span_count_state ^ 0x1357_9bdf,
        span.words.backward_offset_state ^ 0x2468_ace0,
    ];
    let request = Edit::SpanAndBackwardStates {
        span_count_state: expected[0],
        backward_offset_state: expected[1],
    };
    let result = source
        .edit_intersection_reference_state(state, request)
        .unwrap();
    verify_native_reference_payload(source, raw, state, span, expected, result.archive());
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    verify_native_reference_history(source, state, &result);
    verify_native_reference_revision(state, &result);
    result
}

fn verify_native_reference_revision(
    state: &State,
    result: &crate::model_archives::PartResolvedFeaturesEdit,
) {
    assert!(
        result
            .archive()
            .intersection_reference_state(&state.source_fields()[0])
            .is_err()
    );
}

fn verify_native_reference_payload(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &State,
    span: &NativeReferenceSpan,
    expected: [i32; 2],
    edited: &PartResolvedFeaturesArchive,
) {
    assert_eq!(state.words(), span.words);
    let mut literal = raw.to_vec();
    literal[span.offset + 6..span.offset + 10].copy_from_slice(&expected[0].to_le_bytes());
    literal[span.offset + 10..span.offset + 14].copy_from_slice(&expected[1].to_le_bytes());
    assert_eq!(edited.to_bytes().unwrap(), literal);
    let mut fields = source.fields().to_vec();
    for (field, value) in state.source_fields()[2..].iter().zip(expected) {
        fields[field.index()].value = ModelFieldValue::I32(value);
    }
    assert_eq!(edited.fields(), fields);
}

fn verify_native_reference_history(
    source: &PartResolvedFeaturesArchive,
    state: &State,
    result: &crate::model_archives::PartResolvedFeaturesEdit,
) {
    assert_eq!(result.changes().len(), 2);
    for (change, parent) in result.changes().iter().zip(&state.source_fields()[2..]) {
        assert_eq!(change.parents(), std::slice::from_ref(parent));
        assert_eq!(
            change.path(),
            source.resolve_field(parent).unwrap().path.as_ref()
        );
        assert!(
            result
                .archive()
                .resolve_field(change.result().unwrap())
                .is_ok()
        );
    }
}

fn verify_native_reference_case(
    case: &ResolvedCorpusCase<'_>,
    oracle: &NativeReferenceOracle,
    kernel: Option<CompressedKernelContext>,
) {
    assert_eq!(case.label, oracle.label);
    assert_eq!(case.version.value(), oracle.version);
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source =
        PartResolvedFeaturesArchive::from_bytes(case.features, configuration, case.manager, LIMITS)
            .unwrap();
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), oracle.spans.len(), "{}", case.label);
    for (state, span) in states.iter().zip(&oracle.spans) {
        verify_native_reference_edit(&source, case.features, state, span);
    }
    assert_eq!(source.to_bytes().unwrap(), case.features);
}

fn run_native_reference_cohort(
    probe: &[u8],
    spans: &[u8],
    kernel: Option<CompressedKernelContext>,
    verify: NativeReferenceCheck,
) {
    let (mut cursor, mut oracle) = (ArchiveCursor::new(probe), ArchiveCursor::new(spans));
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(oracle.take(4).unwrap(), b"BIR1");
    assert_eq!((cursor.u32().unwrap(), oracle.u32().unwrap()), (906, 8));
    let expected: Vec<_> = (0..8)
        .map(|_| read_native_reference_oracle(&mut oracle))
        .collect();
    verify_native_reference_selection(&mut cursor, &expected, kernel, verify);
    assert_eq!(expected.iter().map(|entry| entry.spans.len()).sum::<usize>(), 68);
    assert_eq!((cursor.remaining(), oracle.remaining()), (0, 0));
}

fn verify_native_reference_selection(
    cursor: &mut ArchiveCursor<'_>,
    expected: &[NativeReferenceOracle],
    kernel: Option<CompressedKernelContext>,
    verify: NativeReferenceCheck,
) {
    let mut selected = expected.iter().peekable();
    for index in 0..906 {
        let case = next_case(cursor);
        if selected.peek().is_some_and(|entry| entry.index == index) {
            let entry = selected.next().unwrap();
            verify(&case, entry, kernel.clone());
            println!("INTERSECTION_NATIVE_EDIT\t{index}\t{}", entry.spans.len());
        }
    }
    assert!(selected.next().is_none());
}

#[test]
#[ignore = "Requires retained BRF1, exact C-3307 span packet and explicit native kernel schema"]
fn every_native_intersection_reference_matches_source_words_and_isolated_literal_edits() {
    let probe = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let spans = std::fs::read("target/intersection-reference-native-source-spans.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    run_native_reference_cohort(&probe, &spans, kernel, verify_native_reference_case);
}
