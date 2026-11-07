use super::{ModelDimensionStoredEdit, projection::project_dimension};
use crate::FailureClass;
use crate::archive_codec::ArchiveRevision;
use crate::model_archives::{ModelField, ModelFieldHandle, ModelFieldValue};

fn group() -> (Vec<ModelField>, ModelFieldHandle) {
    let fields = [
        "am_object_id",
        "is_design_table_dimension",
        "mask",
        "unknown_5c",
    ]
    .into_iter()
    .zip([19, -31, -1, i32::MIN])
    .map(|(label, value)| ModelField {
        path: format!("root.dimension.{label}").into(),
        value: ModelFieldValue::I32(value),
    })
    .collect();
    (
        fields,
        ModelFieldHandle {
            revision: ArchiveRevision::default(),
            index: 2,
        },
    )
}

#[test]
fn dimension_selection_rejects_each_wrong_type_and_cross_scope_word() {
    let (fields, handle) = group();
    for index in 0..4 {
        let mut wrong = fields.clone();
        wrong[index].value = ModelFieldValue::U16(7);
        assert_eq!(
            project_dimension(&wrong, &handle).unwrap_err().class,
            FailureClass::Invalid
        );
        wrong = fields.clone();
        wrong[index].path = "root.tolerance.mask".into();
        assert!(project_dimension(&wrong, &handle).is_err());
    }
}

#[test]
fn incomplete_dimension_groups_and_invalid_indices_fail_without_panics() {
    let (fields, handle) = group();
    for length in 0..4 {
        assert!(project_dimension(&fields[..length], &handle).is_err());
    }
    for index in [0, 1, 3, usize::MAX] {
        let bad = ModelFieldHandle {
            index,
            ..handle.clone()
        };
        assert!(project_dimension(&fields, &bad).is_err());
    }
    let mut wrong = fields;
    wrong[2].path = "root.dimension.mask.suffix".into();
    assert!(project_dimension(&wrong, &handle).is_err());
}

#[test]
fn every_out_of_domain_counter_reports_the_offending_value_and_expected_range() {
    let (fields, handle) = group();
    let dimension = project_dimension(&fields, &handle).unwrap();
    for number in 8..=u8::MAX {
        let error = dimension
            .replacement(ModelDimensionStoredEdit::FilletRefPointChangeNumber(number))
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&number.to_string()));
        assert!(error.cause.contains("0..=7"));
    }
    assert_eq!(dimension.mask(), -1);
    assert_eq!(dimension.redundant_id(), i32::MIN);
}

#[test]
fn empty_dimension_scopes_and_wrong_word_values_are_diagnosed_explicitly() {
    let (mut fields, handle) = group();
    fields[0].value = ModelFieldValue::U16(65123);
    let error = project_dimension(&fields, &handle).unwrap_err();
    assert!(error.cause.contains("65123"));
    assert!(error.cause.contains("I32"));
    let (mut fields, handle) = group();
    for field in &mut fields {
        field.path = field.path.strip_prefix("root.dimension").unwrap().into();
    }
    let error = project_dimension(&fields, &handle).unwrap_err();
    assert!(error.cause.contains("empty dimension scope"));
}
