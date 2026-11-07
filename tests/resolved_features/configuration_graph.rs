//! Postload field fixtures isolate graph policy from the separately tested complete wire parser.
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::{
    ArchiveClass, ArchiveMapIndex, FramedObjectRecord, ObjectFraming, ObjectRecord,
};

struct GraphFixture {
    fields: Vec<ModelField>,
    manager: ObjectRecord,
}

fn part_record(node: usize, definition: bool) -> ObjectRecord {
    let index = ArchiveMapIndex::new(u32::try_from(node).unwrap() + 4).unwrap();
    let class = ArchiveClass {
        name: "moPartConfiguration_c".into(),
        schema: 1,
    };
    if definition {
        ObjectRecord::New { index, class }
    } else {
        ObjectRecord::Reference { index, class }
    }
}

fn tag(record: ObjectRecord) -> ModelFieldValue {
    ModelFieldValue::ObjectTag(Box::new(FramedObjectRecord {
        framing: ObjectFraming {
            tag: 0,
            extended: None,
        },
        record,
    }))
}

fn push(fields: &mut Vec<ModelField>, path: String, value: ModelFieldValue) {
    fields.push(ModelField {
        path: path.into(),
        value,
    });
}

fn fixture(children: &[Vec<usize>], roots: &[usize], selected: usize) -> GraphFixture {
    let manager = ObjectRecord::New {
        index: ArchiveMapIndex::new(2).unwrap(),
        class: ArchiveClass {
            name: "moConfigurationMgr_c".into(),
            schema: 1,
        },
    };
    let mut fields = Vec::new();
    push(&mut fields, "root.manager".into(), tag(manager.clone()));
    for name in ["configuration_358", "configuration_360"] {
        push(
            &mut fields,
            format!("root.manager.{name}"),
            tag(part_record(selected, false)),
        );
    }
    push(
        &mut fields,
        "root.manager.configurations.count.count_word".into(),
        ModelFieldValue::U16(roots.len().try_into().unwrap()),
    );
    for (index, node) in roots.iter().enumerate() {
        push(
            &mut fields,
            format!("root.manager.configurations[{index}]"),
            tag(part_record(*node, false)),
        );
    }
    for (node, children) in children.iter().enumerate() {
        append_node(&mut fields, node, children);
    }
    GraphFixture { fields, manager }
}

fn append_node(fields: &mut Vec<ModelField>, node: usize, children: &[usize]) {
    let path = format!("root.nodes[{node}]");
    push(fields, path.clone(), tag(part_record(node, true)));
    push(
        fields,
        format!("{path}.parent_698"),
        tag(ObjectRecord::Null),
    );
    push(fields, format!("{path}.raw_77c"), ModelFieldValue::I32(0));
    push(
        fields,
        format!("{path}.feature.has_unknown_1e0"),
        ModelFieldValue::U8(0),
    );
    push(
        fields,
        format!("{path}.children_6a0.count.count_word"),
        ModelFieldValue::U16(children.len().try_into().unwrap()),
    );
    for (index, child) in children.iter().enumerate() {
        push(
            fields,
            format!("{path}.children_6a0[{index}]"),
            tag(part_record(*child, false)),
        );
    }
}

fn validate(graph: &GraphFixture) -> crate::Result<()> {
    super::configuration_graph::validate(&graph.fields, &graph.manager)
}

#[test]
fn postload_graph_accepts_shared_children_multiple_roots_and_a_selected_descendant() {
    let graph = fixture(&[vec![1, 2], vec![3], vec![3], vec![]], &[0], 3);
    assert!(validate(&graph).is_ok());
    assert!(validate(&fixture(&[vec![1], vec![]], &[0, 1], 1)).is_ok());
    let chain: Vec<_> = (0..1501)
        .map(|node| if node == 1500 { vec![] } else { vec![node + 1] })
        .collect();
    assert!(validate(&fixture(&chain, &[0], 1500)).is_ok());
}

#[test]
fn postload_graph_rejects_duplicate_empty_null_roots_cycles_and_unreachable_selected_nodes() {
    for graph in [
        fixture(&[vec![]], &[0, 0], 0),
        fixture(&[vec![]], &[], 0),
        fixture(&[vec![], vec![]], &[0], 1),
        fixture(&[vec![0]], &[0], 0),
        fixture(&[vec![1], vec![0]], &[0], 0),
    ] {
        assert!(validate(&graph).is_err());
    }
    let mut null = fixture(&[vec![]], &[0], 0);
    replace(
        &mut null,
        "root.manager.configurations[0]",
        tag(ObjectRecord::Null),
    );
    assert!(validate(&null).is_err());
}

fn replace(graph: &mut GraphFixture, path: &str, replacement: ModelFieldValue) {
    graph
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = replacement;
}

#[test]
fn postload_graph_rejects_alias_mismatch_parent_links_and_unverified_node_fields() {
    for (path, value) in [
        ("root.manager.configuration_360", tag(part_record(1, false))),
        ("root.nodes[1].parent_698", tag(part_record(0, false))),
        ("root.nodes[1].raw_77c", ModelFieldValue::I32(1)),
        (
            "root.nodes[1].feature.has_unknown_1e0",
            ModelFieldValue::U8(1),
        ),
    ] {
        let mut graph = fixture(&[vec![1], vec![]], &[0], 0);
        replace(&mut graph, path, value);
        assert!(validate(&graph).is_err(), "{path}");
    }
    let mut missing = fixture(&[vec![1], vec![]], &[0], 0);
    missing
        .fields
        .retain(|field| field.path.as_ref() != "root.nodes[1].raw_77c");
    assert!(validate(&missing).is_err());
}

#[test]
fn postload_graph_rejects_a_nonpart_child_and_does_not_mutate_inputs() {
    let mut graph = fixture(&[vec![1], vec![]], &[0], 0);
    let unrelated = ObjectRecord::Reference {
        index: ArchiveMapIndex::new(5).unwrap(),
        class: ArchiveClass {
            name: "moBaseConfiguration_c".into(),
            schema: 1,
        },
    };
    replace(&mut graph, "root.nodes[0].children_6a0[0]", tag(unrelated));
    let before = graph.fields.clone();
    assert!(validate(&graph).is_err());
    assert_eq!(graph.fields, before);
}
