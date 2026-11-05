use super::*;
use crate::archive_codec::{ArchiveClass, ArchiveMapIndex, FramedObjectRecord, ObjectFraming};

fn equation_field(path: &str, value: ModelFieldValue) -> ModelField {
    ModelField {
        path: path.into(),
        value,
    }
}

fn sketch_owner(alias: bool) -> ModelField {
    let index = ArchiveMapIndex::new(2).unwrap();
    let class = ArchiveClass {
        name: "sgSketch".into(),
        schema: 7,
    };
    let record = if alias {
        ObjectRecord::Reference { index, class }
    } else {
        ObjectRecord::New { index, class }
    };
    equation_field(
        "sketch",
        ModelFieldValue::ObjectTag(Box::new(FramedObjectRecord {
            framing: ObjectFraming {
                tag: if alias { 2 } else { 0xffff },
                extended: None,
            },
            record,
        })),
    )
}

fn scoped_fields() -> Vec<ModelField> {
    vec![
        sketch_owner(false),
        equation_field("sketch.spline_count", ModelFieldValue::I32(1)),
        equation_field(
            "sketch.splines[0].stored_category_mask",
            ModelFieldValue::U16(1),
        ),
        equation_field("sketch.splines[0].flags_32c", ModelFieldValue::U32(4)),
        equation_field("sketch.splines[0].unknown_190", ModelFieldValue::I32(0)),
    ]
}

#[test]
fn physical_equation_scope_accepts_every_admitted_subtype() {
    for mask in (0..=0x1d).filter(|mask| mask & !0x1d == 0).chain([0x80]) {
        let mut fields = scoped_fields();
        fields[2].value = ModelFieldValue::U16(mask);
        let actual = physical_scope(&fields, 3).unwrap();
        assert_eq!(actual.prefix, "sketch.splines[0]");
        assert_eq!(actual.range, 2..5);
    }
}

#[test]
fn physical_scope_rejects_noncanonical_and_missing_selected_paths() {
    for path in [
        "splines[0].flags_32c",
        ".splines[0].flags_32c",
        "sketch.splines[00].flags_32c",
        "sketch.splines[-1].flags_32c",
        "sketch.splines[0].equation_word",
    ] {
        let mut fields = scoped_fields();
        fields[3].path = path.into();
        assert!(physical_scope(&fields, 3).is_err(), "{path}");
    }
    assert!(physical_scope(&scoped_fields(), 5).is_err());
}

#[test]
fn physical_scope_rejects_alias_owners_and_invalid_counts() {
    let mut fields = scoped_fields();
    fields[0] = sketch_owner(true);
    assert!(physical_scope(&fields, 3).is_err());
    for value in [
        ModelFieldValue::I32(-1),
        ModelFieldValue::I32(0),
        ModelFieldValue::U32(1),
    ] {
        let mut fields = scoped_fields();
        fields[1].value = value;
        assert!(physical_scope(&fields, 3).is_err());
    }
}

#[test]
fn physical_scope_rejects_missing_boundaries_and_wrong_prototypes() {
    for index in [0, 1, 2, 4] {
        let mut fields = scoped_fields();
        fields[index].path = "unrelated".into();
        assert!(physical_scope(&fields, 3).is_err(), "missing index {index}");
    }
    for value in [
        ModelFieldValue::U16(2),
        ModelFieldValue::U16(0x81),
        ModelFieldValue::I32(1),
    ] {
        let mut fields = scoped_fields();
        fields[2].value = value;
        assert!(physical_scope(&fields, 3).is_err());
    }
}

#[test]
fn repeated_paths_are_bound_to_the_nearest_physical_sketch_owner() {
    let mut fields = scoped_fields();
    fields.extend(scoped_fields());
    assert_eq!(physical_scope(&fields, 8).unwrap().range, 7..10);
    fields[5] = sketch_owner(true);
    assert!(physical_scope(&fields, 8).is_err());
    assert_eq!(physical_scope(&fields, 3).unwrap().range, 2..5);
}
