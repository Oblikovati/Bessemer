use super::*;
use std::io::Write;

pub(super) fn export_root_case(case: &ResolvedCorpusCase<'_>, graph: &ModelResolvedRootGraph) {
    let Some(path) = std::env::var_os("BESSEMER_ROOT_RESULT_PACKET") else {
        return;
    };
    let mut packet = Vec::new();
    put_word(&mut packet, case.version.value());
    put_frame(&mut packet, case.label.as_bytes());
    put_word(&mut packet, u32::try_from(graph.roots().len()).unwrap());
    for root in graph.roots() {
        put_root(&mut packet, graph, root);
    }
    append_packet(path, &packet);
}

pub(crate) fn append_packet(path: impl AsRef<std::path::Path>, packet: &[u8]) {
    let mut output = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    output.write_all(packet).unwrap();
}

fn put_root(
    packet: &mut Vec<u8>,
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
) {
    put_word(packet, u32::try_from(root.ordinal).unwrap());
    put_tag(packet, &root.tag);
    put_target(packet, graph, root);
}

pub(crate) fn put_tag(packet: &mut Vec<u8>, tag: &crate::model_archives::ModelObjectTag) {
    put_word(packet, u32::from(tag.framing.tag));
    put_word(packet, u32::from(tag.framing.extended.is_some()));
    if let Some(extension) = tag.framing.extended {
        put_word(packet, extension);
    }
    put_object_record(packet, &tag.record);
}

fn put_target(
    packet: &mut Vec<u8>,
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
) {
    match &root.target {
        ModelResolvedRootTarget::Null => {
            put_word(packet, 0);
            put_word(packet, 0);
        }
        ModelResolvedRootTarget::Inherited(index) => {
            put_word(packet, 1);
            put_word(packet, index.value());
        }
        ModelResolvedRootTarget::Local { definition } => {
            put_word(packet, 2);
            let field = graph.source().resolve_field(definition).unwrap();
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                panic!("expected local definition");
            };
            put_word(packet, root_record_identity(&tag.record).0.value());
        }
    }
}

fn put_object_record(packet: &mut Vec<u8>, record: &ModelObjectRecord) {
    let kind = match record {
        ModelObjectRecord::Null => 0,
        ModelObjectRecord::New { .. } => 1,
        ModelObjectRecord::Reference { .. } => 2,
    };
    put_word(packet, kind);
    if kind == 0 {
        put_word(packet, 0);
        put_frame(packet, &[]);
        put_word(packet, 0);
        return;
    }
    let (index, class) = root_record_identity(record);
    put_word(packet, index.value());
    put_frame(packet, class.name.as_bytes());
    put_word(packet, u32::from(class.schema));
}

pub(crate) fn put_word(packet: &mut Vec<u8>, word: u32) {
    packet.extend_from_slice(&word.to_le_bytes());
}

pub(crate) fn put_frame(packet: &mut Vec<u8>, bytes: &[u8]) {
    put_word(packet, u32::try_from(bytes.len()).unwrap());
    packet.extend_from_slice(bytes);
}

#[test]
fn exported_null_root_has_the_literal_nine_zero_word_shape() {
    let configuration = crate::model_archives::context_snapshot_tests::owned_configuration();
    let source = PartResolvedFeaturesArchive::from_bytes(
        &[0x85, 0xff, 0xff, 0xff, 1, 0, 0, 0],
        configuration,
        include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
        LIMITS,
    )
    .unwrap();
    let graph = source.root_graph().unwrap();
    let mut packet = Vec::new();
    put_root(&mut packet, &graph, &graph.roots()[0]);
    assert_eq!(packet, vec![0; 36]);
}
