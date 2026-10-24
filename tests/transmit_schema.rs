//! Explicit base-schema loading and declarative layout construction boundaries.
#[path = "support/transmit.rs"]
mod transmit_support;
use bessemer::parasolid::*;
use transmit_support::{BASE_TEXT, NEUTRAL, base, list_mut, parse};

fn changed_schema(from: &str, to: &str) -> bessemer::Result<BaseSchema> {
    let text = std::str::from_utf8(BASE_TEXT).unwrap().replace(from, to);
    BaseSchema::from_bytes(text.as_bytes())
}

#[test]
fn schema_counts_transmitted_fields_and_schema_only_declarations_are_retained() {
    let schema = base();
    assert_eq!(schema.number().value(), 13006);
    assert_eq!(schema.node_types().len(), 6);
    assert_eq!(schema.node_types()[&200].fields.len(), 13);
    assert_eq!(
        schema.node_types()[&201].fields[0].kind,
        FieldKind::SchemaOnlyQ
    );
    assert_eq!(
        schema.node_types()[&201].fields[1].kind,
        FieldKind::SchemaOnlyT
    );
    let rebuilt = BaseSchema::new(schema.number(), schema.node_types().clone()).unwrap();
    assert_eq!(rebuilt, schema);
    assert!(changed_schema("202 6 38 0", "202 7 38 0").is_err());
    assert!(changed_schema("202 6 38 0", "202 6 39 0").is_err());
    assert!(changed_schema("1 13 1", "1 14 1").is_err());
}

#[test]
fn skipped_fields_are_not_parsed_as_unknown_wire_kinds() {
    let schema = changed_schema("byte; u; 1 0 0", "byte; unknown_untransmitted; 0 0 0").unwrap();
    assert_eq!(schema.node_types()[&200].fields.len(), 12);
    assert!(changed_schema("byte; u; 1 0 0", "byte; unknown_transmitted; 1 0 0").is_err());
}

#[test]
fn spaces_in_field_names_do_not_turn_field_rows_into_node_headers() {
    let schema = changed_schema("current_pmark; p; 1 4 0", "current pmark; p; 1 4 0").unwrap();
    assert_eq!(schema.node_types()[&3].fields[0].name, "current pmark");
    assert_eq!(schema.node_types()[&3].fields[0].kind, FieldKind::Pointer);
    assert_eq!(schema.node_types()[&3].fields[0].pointer_class, 4);
}

#[test]
fn malformed_schema_headers_duplicates_and_out_of_shape_columns_fail() {
    for (from, to) in [
        ("KEY=SCH_13006;", "KEY=SCH_13006x;"),
        ("KEY=SCH_13006;", "KEY=SCH_13006"),
        ("**END_OF_HEADER", "**END_HEADER"),
        ("\nT\n", "\nB\n"),
        ("\n1\n", "\n2\n"),
        ("101 WORLD", "3 WORLD"),
        ("current_pmark; p; 1 4 0", "current_pmark; p; 1 4"),
        ("current_pmark; p; 1 4 0", "current_pmark p 1 4 0"),
        ("current_pmark; p; 1 4 0", "current_pmark; p; 1 65536 0"),
    ] {
        assert!(changed_schema(from, to).is_err(), "{from:?} -> {to:?}");
    }
}

#[test]
fn base_construction_rejects_invalid_fixed_extents_and_nonfinal_variable_fields() {
    let schema = base();
    let mut types = schema.node_types().clone();
    types.get_mut(&200).unwrap().fields[0].extent = FieldExtent::Fixed(1);
    assert!(BaseSchema::new(schema.number(), types).is_err());
    let mut types = schema.node_types().clone();
    types.get_mut(&200).unwrap().fields[0].extent = FieldExtent::Variable;
    assert!(BaseSchema::new(schema.number(), types).is_err());
    let mut types = schema.node_types().clone();
    types.get_mut(&200).unwrap().fields[0].pointer_class = 4;
    assert!(BaseSchema::new(schema.number(), types).is_err());
    assert!(BaseSchema::new(schema.number(), Default::default()).is_err());
}

#[test]
fn explicit_new_type_definitions_must_be_short_latin1_and_belong_to_new_codes() {
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    let LayoutDefinition::Defined { name, .. } = contents.layouts.get_mut(&230).unwrap() else {
        panic!("expected definition");
    };
    *name = "λ".into();
    assert!(source.with_contents(contents).is_err());
    let mut contents = source.contents().clone();
    let LayoutDefinition::Defined { name, .. } = contents.layouts.get_mut(&230).unwrap() else {
        panic!("expected definition");
    };
    *name = "x".repeat(256);
    assert!(source.with_contents(contents).is_err());
    let mut contents = source.contents().clone();
    let definition = contents.layouts.remove(&230).unwrap();
    contents.layouts.insert(200, definition);
    assert!(source.with_contents(contents).is_err());
}

#[test]
fn schema_only_base_fields_can_be_dropped_by_an_explicit_edited_layout() {
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    contents.layouts.clear();
    contents.layouts.insert(
        201,
        LayoutDefinition::Edited(vec![LayoutEdit::Drop, LayoutEdit::Drop]),
    );
    let records = &mut list_mut(&mut contents).records;
    records.clear();
    records.push(NodeRecord {
        node_type: 201,
        index: NodeReference::Node(NodeId::new(1).unwrap()),
        length: None,
        fields: Vec::new(),
    });
    let constructed = source.with_contents(contents).unwrap();
    let encoded = constructed.to_bytes().unwrap();
    assert!(parse(&encoded).layout_fields(201).unwrap().is_empty());
}

#[test]
fn explicit_descriptor_field_count_254_does_not_collide_with_the_base_marker() {
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    let field = FieldDefinition {
        name: "byte".into(),
        kind: FieldKind::UnsignedByte,
        pointer_class: 0,
        extent: FieldExtent::Scalar,
    };
    let LayoutDefinition::Defined { fields, .. } = contents.layouts.get_mut(&230).unwrap() else {
        panic!("expected definition");
    };
    *fields = vec![field.clone(); 254];
    let record = &mut list_mut(&mut contents).records[3];
    record.length = None;
    record.fields = vec![FieldValues::Bytes(vec![128]); 254];
    let constructed = source.with_contents(contents.clone()).unwrap();
    let bytes = constructed.to_bytes().unwrap();
    assert!(
        bytes
            .windows(7)
            .any(|span| span == [0, 230, 254, 3, b'N', b'E', b'W'])
    );
    assert_eq!(parse(&bytes).layout_fields(230).unwrap().len(), 254);
    let LayoutDefinition::Defined { fields, .. } = contents.layouts.get_mut(&230).unwrap() else {
        panic!("expected definition");
    };
    fields.push(field);
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("254")
    );
}

#[test]
fn maximum_fixed_extent_uses_the_full_u16_range_without_losing_elements() {
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    let field = FieldDefinition {
        name: "bytes".into(),
        kind: FieldKind::UnsignedByte,
        pointer_class: 0,
        extent: FieldExtent::Fixed(65534),
    };
    let LayoutDefinition::Defined { fields, .. } = contents.layouts.get_mut(&230).unwrap() else {
        panic!("expected definition");
    };
    *fields = vec![field];
    let record = &mut list_mut(&mut contents).records[3];
    record.length = None;
    record.fields = vec![FieldValues::Bytes(vec![165; 65534])];
    let constructed = source.with_contents(contents).unwrap();
    let bytes = constructed.to_bytes().unwrap();
    let reopened = parse(&bytes);
    assert_eq!(reopened.contents(), constructed.contents());
    assert_eq!(
        reopened.layout_fields(230).unwrap()[0].extent,
        FieldExtent::Fixed(65534)
    );
}
