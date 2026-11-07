use super::{ModelNodeStoredFlag, projection::project_node};
use crate::FailureClass;
use crate::archive_codec::{ArchiveRevision, ObjectFraming, ObjectRecord};
use crate::model_archives::{
    ModelClass, ModelField, ModelFieldHandle, ModelFieldValue, ModelObjectId, ModelObjectTag,
};

fn field(path: &str, value: ModelFieldValue) -> ModelField {
    ModelField {
        path: path.into(),
        value,
    }
}

fn prefix(class: &str) -> (Vec<ModelField>, ModelFieldHandle) {
    let fields = vec![
        field("root.item", definition(class)),
        field("root.item.node.unknown_0c", ModelFieldValue::I32(-1)),
        field("root.item.node.flags_28", ModelFieldValue::U32(u32::MAX)),
        field("root.item.node.id", ModelFieldValue::I32(-19)),
    ];
    let handle = ModelFieldHandle {
        revision: ArchiveRevision::default(),
        index: 1,
    };
    (fields, handle)
}

fn definition(class: &str) -> ModelFieldValue {
    let record = ObjectRecord::New {
        index: ModelObjectId::new(2).unwrap(),
        class: ModelClass {
            name: class.into(),
            schema: 1,
        },
    };
    let tag = ModelObjectTag {
        framing: ObjectFraming {
            tag: 0xffff,
            extended: None,
        },
        record,
    };
    ModelFieldValue::ObjectTag(Box::new(tag))
}

#[test]
fn absent_second_word_remains_absent_and_parameter_requests_fail_explicitly() {
    let (fields, handle) = prefix("moParameter_c");
    let node = project_node(&fields, &handle).unwrap();
    assert_eq!(node.flags_2c(), None);
    assert_eq!(node.flags_2c_unresolved_bits(), None);
    assert!(node.stored_flag(ModelNodeStoredFlag::ReadOnly).unwrap());
    assert_eq!(
        node.stored_flag(ModelNodeStoredFlag::SharedThickness)
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
    assert!(
        node.replacement(ModelNodeStoredFlag::SharedThickness, false)
            .is_err()
    );
}

#[test]
fn wrong_primitive_types_and_cross_scope_siblings_never_form_node_views() {
    let (fields, handle) = prefix("moParameter_c");
    for index in 1..4 {
        let mut wrong = fields.clone();
        wrong[index].value = ModelFieldValue::U16(0);
        assert_eq!(
            project_node(&wrong, &handle).unwrap_err().class,
            FailureClass::Invalid
        );
        wrong = fields.clone();
        wrong[index].path = "root.unrelated.field".into();
        assert!(project_node(&wrong, &handle).is_err());
    }
    assert!(project_node(&fields[..3], &handle).is_err());
    let bad = ModelFieldHandle {
        index: usize::MAX,
        ..handle
    };
    assert!(project_node(&fields, &bad).is_err());
}

#[test]
fn unrelated_classes_alias_only_owners_and_nested_inline_scopes_do_not_gain_parameter_roles() {
    for class in ["moFavoriteFeatItemData_c", "OtherParameter_c"] {
        let (fields, handle) = prefix(class);
        assert_eq!(
            project_node(&fields, &handle).unwrap().parameter_class(),
            None
        );
    }
    let (mut fields, handle) = prefix("moParameter_c");
    for value in [ModelFieldValue::U32(0), alias_parameter()] {
        fields[0].value = value;
        assert_eq!(
            project_node(&fields, &handle).unwrap().parameter_class(),
            None
        );
    }
}

#[test]
fn nested_inline_node_scope_does_not_inherit_parameter_roles_from_an_outer_definition() {
    let (mut fields, handle) = prefix("moParameter_c");
    for item in &mut fields[1..] {
        item.path = item.path.replace(".node.", ".inline.node.").into();
    }
    assert_eq!(
        project_node(&fields, &handle).unwrap().parameter_class(),
        None
    );
}

fn alias_parameter() -> ModelFieldValue {
    ModelFieldValue::ObjectTag(Box::new(ModelObjectTag {
        framing: ObjectFraming {
            tag: 2,
            extended: None,
        },
        record: ObjectRecord::Reference {
            index: ModelObjectId::new(2).unwrap(),
            class: ModelClass {
                name: "moParameter_c".into(),
                schema: 1,
            },
        },
    }))
}

#[test]
fn malformed_second_word_is_rejected_instead_of_becoming_absent_or_zero() {
    let (mut fields, handle) = prefix("moParameter_c");
    fields.push(field("root.item.node.unknown_2c", ModelFieldValue::I32(0)));
    assert!(project_node(&fields, &handle).is_err());
    fields[4].value = ModelFieldValue::U32(0);
    let node = project_node(&fields, &handle).unwrap();
    assert_eq!(node.flags_2c(), Some(0));
    assert!(
        !node
            .stored_flag(ModelNodeStoredFlag::SharedThickness)
            .unwrap()
    );
}
