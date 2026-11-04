use super::*;

#[test]
fn primary_edits_match_six_independent_goldens_and_reverse_complete_documents() {
    for kind in 0..6 {
        let (document, source) = fixture(fixtures::ORIGINAL[kind][0]);
        let parameter = state(&source);
        let edit = source
            .edit_stored_parameter(
                &parameter,
                ModelParameterStoredEdit::Primary(edited_value(kind)),
            )
            .unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), fixtures::EDITED[kind]);
        assert_eq!(edit.completion(), ModelEditCompletion::Changed);
        assert_eq!(edit.changes().len(), 1);
        assert_document_reversal(
            &document,
            &source,
            &edit,
            ModelParameterStoredEdit::Primary(parameter.primary().clone()),
        );
        assert_eq!(source.to_bytes().unwrap(), fixtures::ORIGINAL[kind][0]);
    }
}

pub(super) fn assert_document_reversal(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    reverse_request: ModelParameterStoredEdit,
) {
    let original = document.to_bytes().unwrap();
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, edit.archive().to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = read_owned_resolved_document(&reopened, LIMITS);
    assert_eq!(current.fields(), edit.archive().fields());
    unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reverse = current
        .edit_stored_parameter(&state(&current), reverse_request)
        .unwrap();
    assert_eq!(
        reverse.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    replace_owned_resolved_stream(&mut output, reverse.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), original);
}

#[test]
fn noops_preserve_revision_history_and_all_stored_bits_for_every_graph() {
    for roots in fixtures::ORIGINAL {
        for raw in roots {
            let (_, source) = fixture(raw);
            let parameter = state(&source);
            let edit = source
                .edit_stored_parameter(
                    &parameter,
                    ModelParameterStoredEdit::Primary(parameter.primary().clone()),
                )
                .unwrap();
            assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
            assert!(edit.changes().is_empty());
            assert_eq!(edit.archive().to_bytes().unwrap(), raw);
            edit.archive()
                .resolve_field(parameter.definition())
                .unwrap();
        }
    }
}

#[test]
fn alternates_and_noncanonical_presence_bytes_match_literal_selected_byte_replacements() {
    for kind in 0..5 {
        for style in 1..4 {
            assert_alternate_edit(kind, style);
            assert_presence_edits(kind, style);
        }
    }
}

fn assert_alternate_edit(kind: usize, style: usize) {
    let raw = fixtures::ORIGINAL[kind][style];
    let (document, source) = fixture(raw);
    let parameter = state(&source);
    let expected = alternate_golden(raw, kind);
    let request = ModelParameterStoredEdit::Alternate(edited_value(kind));
    let edit = apply_parameter(&source, &parameter, request);
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_eq!(edit.changes().len(), 1);
    let reverse = ModelParameterStoredEdit::Alternate(parameter.alternate().unwrap().clone());
    assert_document_reversal(&document, &source, &edit, reverse);
}

fn alternate_golden(raw: &[u8], kind: usize) -> Vec<u8> {
    let mut expected = raw.to_vec();
    let bytes = if kind == 3 {
        12345i32.to_le_bytes().to_vec()
    } else {
        0x3ff4000000000000u64.to_le_bytes().to_vec()
    };
    let start = expected.len() - bytes.len();
    expected[start..].copy_from_slice(&bytes);
    expected
}

fn assert_presence_edits(kind: usize, style: usize) {
    let raw = fixtures::ORIGINAL[kind][style];
    let (_, source) = fixture(raw);
    let parameter = state(&source);
    for byte in [1, 2, 128, 255] {
        let mut expected = raw.to_vec();
        let offset = expected.len() - if kind == 3 { 5 } else { 9 };
        expected[offset] = byte;
        let request = ModelParameterStoredEdit::AlternatePresence(byte);
        let edit = apply_parameter(&source, &parameter, request);
        assert_eq!(edit.archive().to_bytes().unwrap(), expected);
        assert_eq!(state(edit.archive()).alternate_presence(), Some(byte));
        assert_eq!(state(edit.archive()).alternate(), parameter.alternate());
    }
    let invalid =
        source.edit_stored_parameter(&parameter, ModelParameterStoredEdit::AlternatePresence(0));
    assert!(invalid.is_err());
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
fn text_count_width_only_edit_is_changed_and_reversible_despite_equal_semantic_text() {
    let (document, source) = fixture(fixtures::ORIGINAL[5][2]);
    let parameter = state(&source);
    let value = ModelParameterValue::Text(ArchiveString::new("ABC", TextEncoding::Latin1).unwrap());
    assert_eq!(parameter.primary(), &value);
    let edit = source
        .edit_stored_parameter(&parameter, ModelParameterStoredEdit::Primary(value))
        .unwrap();
    let mut expected = fixtures::ORIGINAL[5][2].to_vec();
    let prefix = expected.len() - 6;
    assert_eq!(&expected[prefix..], b"\xff\x03\x00ABC");
    expected.splice(prefix..prefix + 3, [3]);
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.changes().len(), 1);
    assert_document_reversal(
        &document,
        &source,
        &edit,
        ModelParameterStoredEdit::Primary(parameter.primary().clone()),
    );
}

#[test]
fn configuration_parameters_use_the_same_typed_operation_and_remain_isolated() {
    let (_, features) = fixture(fixtures::ORIGINAL[0][0]);
    let snapshot = state(&features);
    let source = snapshot.source().configuration_archive();
    let original = source.to_bytes().unwrap();
    let parameters = source.stored_parameters().unwrap();
    assert!(!parameters.is_empty());
    for parameter in parameters {
        assert_configuration_parameter(source, &parameter);
    }
    assert_eq!(source.to_bytes().unwrap(), original);
}

fn assert_configuration_parameter(
    source: &PartConfigurationArchive,
    parameter: &ModelParameterStoredState,
) {
    let selected = source.stored_parameter(parameter.definition()).unwrap();
    assert_eq!(selected.primary(), parameter.primary());
    assert!(matches!(
        parameter.source(),
        ModelParameterSource::Configuration(_)
    ));
    assert_eq!(parameter.coverage(), source.coverage());
    for relation in parameter.history() {
        parameter.source().resolve_field(&relation.result).unwrap();
    }
    let request = ModelParameterStoredEdit::Primary(parameter.primary().clone());
    let noop = source.edit_stored_parameter(parameter, request).unwrap();
    assert_eq!(noop.completion(), ModelEditCompletion::Unchanged);
    assert_configuration_reversal(source, parameter);
}

fn assert_configuration_reversal(
    source: &PartConfigurationArchive,
    parameter: &ModelParameterStoredState,
) {
    let changed = source
        .edit_stored_parameter(parameter, primary_request(0))
        .unwrap();
    assert_eq!(changed.changes().len(), 1);
    let next = changed.archive().stored_parameters().unwrap();
    assert_eq!(next.len(), 1);
    let request = ModelParameterStoredEdit::Primary(parameter.primary().clone());
    let reversed = changed
        .archive()
        .edit_stored_parameter(&next[0], request)
        .unwrap();
    assert_eq!(
        reversed.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    let stale = changed.archive().stored_parameter(parameter.definition());
    assert!(stale.is_err());
}

#[test]
fn configuration_value_edit_reopens_and_reverses_the_complete_document() {
    let (document, features) = fixture(fixtures::ORIGINAL[0][0]);
    let snapshot = state(&features);
    let source = snapshot.source().configuration_archive();
    let original_configuration = source.to_bytes().unwrap();
    let parameter = source.stored_parameters().unwrap().remove(0);
    let edit = source
        .edit_stored_parameter(&parameter, primary_request(0))
        .unwrap();
    let mut output = document.clone();
    replace_configuration(&mut output, edit.archive());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = read_configuration(&reopened);
    assert_eq!(current.fields(), edit.archive().fields());
    assert_configuration_stream_isolation(&document, &reopened);
    reverse_configuration_document(&mut output, &current, parameter.primary().clone());
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
    assert_eq!(source.to_bytes().unwrap(), original_configuration);
}

fn replace_configuration(document: &mut Document, archive: &PartConfigurationArchive) {
    document
        .replace_stream("Contents/Config-0", archive.to_bytes().unwrap())
        .unwrap();
}

fn read_configuration(document: &Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        bessemer::header::ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn assert_configuration_stream_isolation(original: &Document, changed: &Document) {
    unselected_document_streams::assert_unselected_document_streams(
        original,
        changed,
        "Contents/Config-0",
    );
}

fn reverse_configuration_document(
    document: &mut Document,
    archive: &PartConfigurationArchive,
    original: ModelParameterValue,
) {
    let parameter = archive.stored_parameters().unwrap().remove(0);
    let reverse = archive
        .edit_stored_parameter(&parameter, ModelParameterStoredEdit::Primary(original))
        .unwrap();
    replace_configuration(document, reverse.archive());
}
