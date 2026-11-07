use super::*;
use crate::archive_codec::ArchiveString;
use crate::model_archives::{ModelObjectId, TextEncoding};

struct CorpusPrimaryRow<'a> {
    ordinal: usize,
    kind: u32,
    identity: ModelObjectId,
    offset: usize,
    original: &'a [u8],
    replacement: &'a [u8],
    text: &'a str,
}

fn corpus_blob<'a>(cursor: &mut ArchiveCursor<'a>) -> &'a [u8] {
    let count = cursor.u32().unwrap() as usize;
    assert!(count <= LIMITS.maximum_bytes);
    cursor.take(count).unwrap()
}

fn corpus_rows<'a>(cursor: &mut ArchiveCursor<'a>) -> Vec<CorpusPrimaryRow<'a>> {
    let count = cursor.u32().unwrap() as usize;
    assert!(count <= LIMITS.maximum_objects);
    (0..count)
        .map(|ordinal| {
            assert_eq!(cursor.u32().unwrap() as usize, ordinal);
            CorpusPrimaryRow {
                ordinal,
                kind: cursor.u32().unwrap(),
                identity: ModelObjectId::new(cursor.u32().unwrap()).unwrap(),
                offset: cursor.u32().unwrap() as usize,
                original: corpus_blob(cursor),
                replacement: corpus_blob(cursor),
                text: std::str::from_utf8(corpus_blob(cursor)).unwrap(),
            }
        })
        .collect()
}

fn corpus_value(row: &CorpusPrimaryRow<'_>, replacement: bool) -> ModelParameterValue {
    let raw = if replacement {
        row.replacement
    } else {
        row.original
    };
    match row.kind {
        0 => ModelParameterValue::Double(StoredDouble::from_bits(u64::from_le_bytes(
            raw.try_into().unwrap(),
        ))),
        1 => ModelParameterValue::Integer(i32::from_le_bytes(raw.try_into().unwrap())),
        2 => corpus_text_value(row, replacement),
        other => panic!("value kind {other}; expected double0/integer1/text2"),
    }
}

fn corpus_text_value(row: &CorpusPrimaryRow<'_>, replacement: bool) -> ModelParameterValue {
    assert!(
        replacement,
        "original CString uses the validated source value"
    );
    let encoding = if row.replacement.starts_with(&[255, 254, 255]) {
        TextEncoding::Utf16
    } else {
        TextEncoding::Latin1
    };
    ModelParameterValue::Text(ArchiveString::new(row.text, encoding).unwrap())
}

fn fresh_corpus_parameter(
    source: &ModelParameterSource,
    index: usize,
) -> ModelParameterStoredState {
    match source {
        ModelParameterSource::Configuration(archive) => archive
            .stored_parameter(&archive.field_handle(index).unwrap())
            .unwrap(),
        ModelParameterSource::ResolvedFeatures(archive) => archive
            .stored_parameter(&archive.field_handle(index).unwrap())
            .unwrap(),
    }
}

fn assert_corpus_parameter(
    source: &ModelParameterSource,
    parameter: &ModelParameterStoredState,
    row: &CorpusPrimaryRow<'_>,
) {
    let field = source.resolve_field(parameter.definition()).unwrap();
    assert!(matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ModelObjectRecord::New { index, .. } if *index == row.identity)));
    if row.kind != 2 {
        assert_eq!(parameter.primary(), &corpus_value(row, false));
    }
}

fn corpus_expected(original: &[u8], row: &CorpusPrimaryRow<'_>) -> Vec<u8> {
    let end = row.offset.checked_add(row.original.len()).unwrap();
    assert_eq!(&original[row.offset..end], row.original);
    let mut expected = original.to_vec();
    expected.splice(row.offset..end, row.replacement.iter().copied());
    expected
}

struct CorpusDocumentTrial<'a> {
    document: &'a Document,
    case: &'a ResolvedCorpusCase<'a>,
    context: u32,
    source: &'a ModelParameterSource,
    kernel: Option<CompressedKernelContext>,
}

fn check_corpus_primary(trial: &CorpusDocumentTrial<'_>, row: &CorpusPrimaryRow<'_>) {
    let field_index = corpus_definition_index(trial.source, row.ordinal);
    let parameter = fresh_corpus_parameter(trial.source, field_index);
    assert_corpus_parameter(trial.source, &parameter, row);
    let original = if trial.context == 0 {
        trial.case.configuration
    } else {
        trial.case.features
    };
    let changed = edit_native_value(trial.source, &parameter, corpus_value(row, true));
    assert_eq!(
        parameter_source_bytes(&changed),
        corpus_expected(original, row)
    );
    check_corpus_document(trial, row, &parameter, &changed);
    assert_eq!(parameter_source_bytes(trial.source), original);
}

fn corpus_definition_index(source: &ModelParameterSource, ordinal: usize) -> usize {
    parameter_source_fields(source).iter().enumerate()
        .filter(|(_, field)| matches!(&field.value, ModelFieldValue::ObjectTag(tag)
            if matches!(&tag.record, ModelObjectRecord::New { class, .. } if matches!(class.name.as_str(),
                "moDensityParameter_c" | "moLengthParameter_c" | "moAngleParameter_c" |
                "moIntegerParameter_c" | "moScalerParameter_c" | "moStringParameter_c"))))
        .nth(ordinal).unwrap().0
}

fn check_corpus_document(
    trial: &CorpusDocumentTrial<'_>,
    row: &CorpusPrimaryRow<'_>,
    parameter: &ModelParameterStoredState,
    changed: &ModelParameterSource,
) {
    let name = trial.case.label.rsplit_once("::").unwrap().1;
    let stream = if trial.context == 0 {
        name.to_owned()
    } else {
        format!("{name}-ResolvedFeatures")
    };
    let (output, reopened) =
        primary_output_document(trial.document, &stream, parameter_source_bytes(changed));
    let current =
        read_parameter_context(&reopened, trial.case, trial.context, trial.kernel.clone());
    assert_parameter_archives_equal(&current, changed);
    restore_corpus_document(trial, parameter, &current, output, &stream);
    println!(
        "PARAMETER_EDIT\t{}\t{}\t{}\t{}",
        trial.case.label, trial.context, row.ordinal, row.kind
    );
}

fn restore_corpus_document(
    trial: &CorpusDocumentTrial<'_>,
    parameter: &ModelParameterStoredState,
    current: &ModelParameterSource,
    mut output: Document,
    stream: &str,
) {
    let selected = fresh_corpus_parameter(current, parameter.definition().index());
    let reverse = edit_native_value(current, &selected, parameter.primary().clone());
    assert_parameter_archives_equal(&reverse, trial.source);
    output
        .replace_stream(stream, parameter_source_bytes(&reverse))
        .unwrap();
    assert_eq!(
        output.to_bytes().unwrap(),
        trial.document.to_bytes().unwrap()
    );
}

fn check_corpus_context(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    context: u32,
    rows: &[CorpusPrimaryRow<'_>],
    kernel: Option<CompressedKernelContext>,
) {
    let source = read_parameter_context(document, case, context, kernel.clone());
    let original = if context == 0 {
        case.configuration
    } else {
        case.features
    };
    assert_eq!(parameter_source_bytes(&source), original);
    let count = physical_parameter_count(parameter_source_fields(&source));
    assert_eq!(count, rows.len());
    let trial = CorpusDocumentTrial {
        document,
        case,
        context,
        source: &source,
        kernel,
    };
    for row in rows {
        check_corpus_primary(&trial, row);
    }
}

fn check_corpus_case(
    case: &ResolvedCorpusCase<'_>,
    oracle: &mut ArchiveCursor<'_>,
    kernel: Option<CompressedKernelContext>,
) -> usize {
    assert_eq!(oracle.u32().unwrap(), case.version.value());
    assert_eq!(
        std::str::from_utf8(corpus_blob(oracle)).unwrap(),
        case.label
    );
    let (document, original) = native_lookup_original_document(case);
    let mut count = 0;
    for context in 0..2 {
        let rows = corpus_rows(oracle);
        count += rows.len();
        check_corpus_context(&document, case, context, &rows, kernel.clone());
    }
    assert_eq!(document.to_bytes().unwrap(), original);
    count
}

#[test]
#[ignore = "requires retained native inputs, original files and kernel schema;43917 complete primary edits and typed reversals"]
fn all_native_parameter_primaries_match_original_spans_reopen_and_reverse_every_document() {
    let native = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let path = "tests/fixtures/stored-parameters/native-primary-corpus/spans.bin";
    let oracle_bytes = std::fs::read(path).unwrap();
    let mut cases = ArchiveCursor::new(&native);
    let mut oracle = ArchiveCursor::new(&oracle_bytes);
    assert_corpus_headers(&mut cases, &mut oracle);
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut edits = 0;
    for _ in 0..906 {
        edits += check_corpus_case(&next_case(&mut cases), &mut oracle, kernel.clone());
    }
    assert_eq!(
        (cases.remaining(), oracle.remaining(), edits),
        (0, 0, 43917)
    );
    println!("PARAMETER_EDIT_TOTAL\t906\t{edits}");
}

fn assert_corpus_headers(cases: &mut ArchiveCursor<'_>, oracle: &mut ArchiveCursor<'_>) {
    assert_eq!(
        (cases.take(4).unwrap(), cases.u32().unwrap()),
        (b"BRF1".as_slice(), 906)
    );
    assert_eq!(
        (oracle.take(4).unwrap(), oracle.u32().unwrap()),
        (b"BPP1".as_slice(), 906)
    );
}
