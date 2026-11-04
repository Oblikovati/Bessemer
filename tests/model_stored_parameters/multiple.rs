use super::*;

const MULTIPLE_ORIGINAL: &[u8] =
    include_bytes!("../fixtures/stored-parameters/multiple/original.bin");
const MULTIPLE_EDITED: [&[u8]; 6] = [
    include_bytes!("../fixtures/stored-parameters/multiple/edited-0.bin"),
    include_bytes!("../fixtures/stored-parameters/multiple/edited-1.bin"),
    include_bytes!("../fixtures/stored-parameters/multiple/edited-2.bin"),
    include_bytes!("../fixtures/stored-parameters/multiple/edited-3.bin"),
    include_bytes!("../fixtures/stored-parameters/multiple/edited-4.bin"),
    include_bytes!("../fixtures/stored-parameters/multiple/edited-5.bin"),
];

#[test]
fn each_primary_edit_preserves_five_other_parameters_and_reverses_complete_documents() {
    let (document, source) = fixture(MULTIPLE_ORIGINAL);
    let parameters = source.stored_parameters().unwrap();
    assert_eq!(parameters.len(), 6);
    for (kind, parameter) in parameters.iter().enumerate() {
        assert_eq!(parameter.kind(), KINDS[kind]);
        let edit = apply_parameter(&source, parameter, primary_request(kind));
        assert_eq!(edit.archive().to_bytes().unwrap(), MULTIPLE_EDITED[kind]);
        assert_eq!(edit.changes().len(), 1);
        assert_multiple_values(&parameters, edit.archive(), kind);
        reverse_multiple_document(&document, &edit, kind, parameter.primary().clone());
    }
    assert_eq!(source.to_bytes().unwrap(), MULTIPLE_ORIGINAL);
}

fn assert_multiple_values(
    before: &[ModelParameterStoredState],
    changed: &PartResolvedFeaturesArchive,
    selected: usize,
) {
    let after = changed.stored_parameters().unwrap();
    assert_eq!(after.len(), before.len());
    for (kind, (original, current)) in before.iter().zip(&after).enumerate() {
        let expected = if kind == selected {
            edited_value(kind)
        } else {
            original.primary().clone()
        };
        assert_eq!(current.primary(), &expected);
        assert_eq!(current.alternate(), original.alternate());
        assert_eq!(current.alternate_presence(), original.alternate_presence());
        assert_eq!(current.node_name().text(), original.node_name().text());
        assert_eq!(
            current.parameter_name().text(),
            original.parameter_name().text()
        );
    }
}

fn reverse_multiple_document(
    document: &Document,
    edit: &PartResolvedFeaturesEdit,
    selected: usize,
    primary: ModelParameterValue,
) {
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
    let parameter = current.stored_parameters().unwrap().remove(selected);
    let reverse = apply_parameter(
        &current,
        &parameter,
        ModelParameterStoredEdit::Primary(primary),
    );
    assert_eq!(reverse.archive().to_bytes().unwrap(), MULTIPLE_ORIGINAL);
    replace_owned_resolved_stream(&mut output, reverse.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}
