//! Complete forming-tool operations through the public document and edit APIs (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::{ArchiveString, TextEncoding};
use bessemer::model_archives::ModelFieldValue;

const INSTANCE: &[u8] = include_bytes!("fixtures/forming-tools/owned-instance.bin");
const EMBEDDED: &[u8] = include_bytes!("fixtures/forming-tools/owned-embedded.bin");

#[test]
fn public_forming_tool_streams_reopen_and_own_their_context_and_source_bytes() {
    contract::check_document(INSTANCE, 111);
    contract::check_document(EMBEDDED, 266);
}

#[test]
fn public_instance_edit_matches_the_independent_word_golden_and_rejects_wrong_types() {
    let path = "root.features[0].payload.raw_9d8";
    contract::check_value_edit(
        INSTANCE,
        include_bytes!("fixtures/forming-tools/owned-instance-edited.bin"),
        path,
        ModelFieldValue::I32(123456),
        None,
    );
    contract::check_rejected_edit(INSTANCE, path);
}

#[test]
fn variable_length_embedded_name_edit_derives_sizes_records_history_and_reverts() {
    let document = contract::document_with(EMBEDDED);
    let source = contract::parsed_with_kernel(&document, None);
    let mut fields = source.fields().to_vec();
    let path = "root.features[0].payload.tool_908.part_08.node.name.name";
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
    let edit = source.edit_fields(fields).unwrap();
    let expected = include_bytes!("fixtures/forming-tools/owned-embedded-edited.bin");
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_eq!(edit.changes().len(), 2);
    let reverted = edit
        .archive()
        .edit_fields(source.fields().to_vec())
        .unwrap();
    assert_eq!(reverted.archive().to_bytes().unwrap(), EMBEDDED);
    assert_eq!(source.to_bytes().unwrap(), EMBEDDED);
    assert_eq!(
        document
            .read_stream("Contents/Config-0-ResolvedFeatures")
            .unwrap(),
        EMBEDDED
    );
}
