use super::*;

fn root_state(archive: &PartConfigurationArchive) -> (ModelFieldHandle, ModelNodeStoredState) {
    let index = archive
        .fields()
        .iter()
        .position(|field| field.path.ends_with(".node.unknown_0c"))
        .unwrap();
    let handle = archive.field_handle(index).unwrap();
    let node = archive.node_state(&handle).unwrap();
    (handle, node)
}

fn root_word_oracle() -> Vec<(usize, u32)> {
    include_str!("../fixtures/model-node-state/configuration-nodes.tsv")
        .lines()
        .take(4)
        .map(|line| {
            let columns: Vec<_> = line.split('\t').collect();
            let hex = columns[3].as_bytes();
            let bytes = std::array::from_fn(|index| {
                u8::from_str_radix(
                    std::str::from_utf8(&hex[index * 2..index * 2 + 2]).unwrap(),
                    16,
                )
                .unwrap()
            });
            (columns[0].parse().unwrap(), u32::from_le_bytes(bytes))
        })
        .collect()
}

#[test]
fn configuration_node_flags_match_original_reader_offsets_and_complete_edits() {
    let (source, _) = configuration();
    let (handle, node) = root_state(&source);
    let words = root_word_oracle();
    assert_eq!(node.update_state() as u32, words[0].1);
    assert_eq!(node.flags_28(), words[1].1);
    assert_eq!(node.flags_2c(), Some(words[3].1));
    assert_eq!(node.parameter_class(), None);
    assert_configuration_options(&source, &node, &handle, &words);
}

fn assert_configuration_options(
    source: &PartConfigurationArchive,
    node: &ModelNodeStoredState,
    handle: &ModelFieldHandle,
    words: &[(usize, u32)],
) {
    for &(flag, relative, mask, parameter) in FLAGS {
        if parameter {
            assert!(source.edit_node_flag(node, flag, true).is_err());
            continue;
        }
        for enabled in [false, true] {
            assert_configuration_edit(source, node, handle, flag, (relative, mask, enabled), words);
        }
    }
}

fn assert_configuration_edit(
    source: &PartConfigurationArchive,
    node: &ModelNodeStoredState,
    handle: &ModelFieldHandle,
    flag: ModelNodeStoredFlag,
    option: (usize, u32, bool),
    words: &[(usize, u32)],
) {
    let (relative, mask, enabled) = option;
    let raw = source.to_bytes().unwrap();
    let offset = words[if relative == 0 { 0 } else { 1 }].0;
    let expected = edits::changed_word(&raw, offset, mask, enabled);
    let edit = source.edit_node_flag(node, flag, enabled).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    let selected = handle.index() + relative / 4;
    edits::assert_unselected_fields(source.fields(), edit.archive().fields(), selected);
    assert_eq!(edit.coverage().wire_bytes, raw.len());
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_configuration_history(source, &edit, selected);
}

fn assert_configuration_history(
    source: &PartConfigurationArchive,
    edit: &PartConfigurationEdit,
    selected: usize,
) {
    if source.fields() == edit.archive().fields() {
        assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
        assert!(edit.changes().is_empty());
        assert!(
            edit.archive()
                .resolve_field(&source.field_handle(selected).unwrap())
                .is_ok()
        );
        return;
    }
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
    assert_eq!(edit.changes().len(), 1);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.parents(), &[source.field_handle(selected).unwrap()]);
    assert_eq!(
        change.result(),
        Some(&edit.archive().field_handle(selected).unwrap())
    );
}

#[test]
fn configuration_node_edits_reopen_the_document_preserve_other_streams_and_revert() {
    let original = Document::from_bytes(OWNED).unwrap();
    let mut document = original.clone();
    let edited = update_document_node(&mut document, true);
    assert_document_reopen(&original, &document);
    let reopened = read_configuration(&document);
    assert_eq!(reopened.fields(), edited.fields());
    assert!(
        root_state(&reopened)
            .1
            .stored_flag(ModelNodeStoredFlag::NeedsUpdate)
            .unwrap()
    );
    update_document_node(&mut document, false);
    assert_eq!(document.to_bytes().unwrap(), OWNED);
}

fn read_configuration(document: &Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn update_document_node(document: &mut Document, enabled: bool) -> PartConfigurationArchive {
    let source = read_configuration(document);
    let (_, node) = root_state(&source);
    let edited = source
        .edit_node_flag(&node, ModelNodeStoredFlag::NeedsUpdate, enabled)
        .unwrap();
    document
        .replace_stream("Contents/Config-0", edited.archive().to_bytes().unwrap())
        .unwrap();
    edited.archive().clone()
}

fn assert_document_reopen(original: &Document, edited: &Document) {
    let reread = Document::from_bytes(&edited.to_bytes().unwrap()).unwrap();
    for entry in original.streams() {
        if entry.name != "Contents/Config-0" {
            assert_eq!(
                reread.read_stream(entry.name).unwrap(),
                original.read_stream(entry.name).unwrap()
            );
        }
    }
}
