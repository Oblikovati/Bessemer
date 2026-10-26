use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use crate::archive_codec::{ArchiveClass, ArchiveCursor, MaterialRead, ObjectReader, ObjectRecord};
use crate::header::ArchiveVersion;
use crate::{Document, FailureClass, Result};

const NEW_NAME: &[u8] = b"\xff\xff\x34\x12\x0c\x00moNodeName_c";
const EMPTY_NAME: &[u8] = b"\xff\xfe\xff\x00";
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 1024,
    maximum_fields: 32,
    maximum_entries: 32,
    maximum_depth: 8,
    maximum_objects: 3,
};

fn external_object_class(schema: u16) -> ArchiveClass {
    ArchiveClass {
        name: "moExtObject_c".into(),
        schema,
    }
}

fn object_sequence() -> Vec<u8> {
    [
        NEW_NAME,
        EMPTY_NAME,
        &[1, 0x80],
        EMPTY_NAME,
        &[0xff, 0x7f, 1, 0, 0, 0x80],
        EMPTY_NAME,
        &[0xff, 0x7f, 2, 0, 0, 0],
        &[3, 0],
        &[0, 0],
        &[0xff, 0x7f, 0, 0, 0, 0],
    ]
    .concat()
}

fn read_name_fields(input: &[u8], limits: ModelFieldLimits) -> Result<Vec<ModelField>> {
    let mut decoder = ModelDecoder::new(input, ArchiveVersion::new(19000), limits)?;
    let mut map = ObjectReader::new();
    let mut index = 0;
    while decoder.remaining() != 0 {
        let label = format!("nodes[{index}]");
        let record = decoder.object_tag(
            &label,
            &mut map,
            ModelObjectRole::Declared(&["moNodeName_c"]),
        )?;
        if matches!(record, ObjectRecord::New { .. }) {
            decoder.scope(&label, |reader| reader.text("name"))?;
        }
        index += 1;
    }
    decoder.finish()
}

fn object_records(fields: &[ModelField]) -> Vec<ObjectRecord> {
    fields
        .iter()
        .filter_map(|field| match &field.value {
            ModelFieldValue::ObjectTag(value) => Some(value.record.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn class_declarations_small_and_wide_tags_keep_schema_and_aliases() {
    let bytes = object_sequence();
    let fields = read_name_fields(&bytes, LIMITS).unwrap();
    assert_eq!(encode_fields(&fields, bytes.len()).unwrap(), bytes);
    let records = object_records(&fields);
    assert_eq!(records.len(), 7);
    assert!(matches!(&records[0], ObjectRecord::New { index, class }
        if index.value() == 2 && class.schema == 0x1234));
    assert!(matches!(&records[1], ObjectRecord::New { index, .. } if index.value() == 3));
    assert!(matches!(&records[2], ObjectRecord::New { index, .. } if index.value() == 4));
    assert!(matches!(&records[3], ObjectRecord::Reference { index, .. } if index.value() == 2));
    assert!(matches!(&records[4], ObjectRecord::Reference { index, .. } if index.value() == 3));
    assert_eq!(records[5..], [ObjectRecord::Null, ObjectRecord::Null]);
    assert_eq!(
        read_name_fields(&encode_fields(&fields, 1024).unwrap(), LIMITS).unwrap(),
        fields
    );
}

#[test]
fn object_budget_counts_new_objects_and_excludes_nulls_and_aliases() {
    let bytes = object_sequence();
    assert!(read_name_fields(&bytes, LIMITS).is_ok());
    let limits = ModelFieldLimits {
        maximum_objects: 2,
        ..LIMITS
    };
    let error = read_name_fields(&bytes, limits).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("model objects 3; expected <= 2"));
}

#[test]
fn aliases_cannot_satisfy_the_wrong_declared_pointer_role() {
    let bytes = [NEW_NAME, &[2, 0]].concat();
    let mut decoder = ModelDecoder::new(&bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    let mut map = ObjectReader::new();
    decoder
        .object_tag("new", &mut map, ModelObjectRole::Any)
        .unwrap();
    let error = decoder
        .object_tag(
            "alias",
            &mut map,
            ModelObjectRole::Declared(&["moUnitsTable_c"]),
        )
        .unwrap_err();
    assert!(error.cause.contains("moNodeName_c"));
    assert!(error.cause.contains("moUnitsTable_c"));
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn imported_object_and_imported_null_aliases_preserve_their_physical_indices() {
    let mut map = ObjectReader::new();
    map.append_loaded_object(Some(external_object_class(7)))
        .unwrap();
    map.append_loaded_object(None).unwrap();
    let bytes = [1, 0, 2, 0];
    let mut decoder = ModelDecoder::new(&bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    let reference = decoder
        .object_tag("external", &mut map, ModelObjectRole::Any)
        .unwrap();
    let null = decoder
        .object_tag("null", &mut map, ModelObjectRole::Any)
        .unwrap();
    assert!(matches!(reference, ObjectRecord::Reference { index, class }
        if index.value() == 1 && class.schema == 7));
    assert_eq!(null, ObjectRecord::Null);
    assert_eq!(encode_fields(&decoder.finish().unwrap(), 4).unwrap(), bytes);
}

#[test]
fn malformed_map_tags_and_incomplete_headers_cannot_publish_fields() {
    for bytes in [&[1, 0][..], &[0, 0x80], &[0xff, 0x7f, 1], &[0xff, 0xff, 1]] {
        let mut decoder = ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
        let error = decoder
            .object_tag("bad", &mut ObjectReader::new(), ModelObjectRole::Any)
            .unwrap_err();
        assert_eq!(error.operation, "model archive");
        assert_eq!(decoder.finish().unwrap_err(), error);
    }
}

#[test]
fn malformed_stored_framing_is_rejected_instead_of_silently_reencoded() {
    let mut fields = read_name_fields(&object_sequence(), LIMITS).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &mut fields[0].value else {
        panic!("expected object tag")
    };
    tag.framing.extended = Some(0);
    assert!(encode_fields(&fields, 1024).is_err());
    let ModelFieldValue::ObjectTag(tag) = &mut fields[0].value else {
        panic!("expected object tag")
    };
    tag.framing.extended = None;
    tag.record = ObjectRecord::Null;
    let error = encode_fields(&fields, 1024).unwrap_err();
    assert!(error.cause.contains("expected a new object"));
}

fn owned_name_object() -> (Vec<u8>, ObjectReader) {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let configuration = document.read_stream("Contents/Config-0").unwrap();
    let mut map = ObjectReader::new();
    map.object(&mut ArchiveCursor::new(&configuration[..14]))
        .unwrap();
    map.append_loaded_object(Some(external_object_class(1)))
        .unwrap();
    (configuration[26..58].to_vec(), map)
}

#[test]
fn native_node_name_framing_resolves_after_root_and_header_import_slots() {
    let (bytes, mut map) = owned_name_object();
    let mut decoder = ModelDecoder::new(&bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    let record = decoder
        .object_tag(
            "node",
            &mut map,
            ModelObjectRole::Declared(&["moNodeName_c"]),
        )
        .unwrap();
    assert!(matches!(record, ObjectRecord::New { index, class }
        if index.value() == 5 && class.schema == 1));
    decoder.scope("node", |reader| reader.text("name")).unwrap();
    let fields = decoder.finish().unwrap();
    assert!(matches!(&fields[1].value, ModelFieldValue::Text(name) if name.text() == "Part1"));
    assert_eq!(encode_fields(&fields, 32).unwrap(), bytes);
}

#[test]
fn class_descriptors_keep_ascii_and_word_length_validation() {
    for name in [String::new(), "µ".into(), "x".repeat(65536)] {
        let mut fields = read_name_fields(&object_sequence(), LIMITS).unwrap();
        let ModelFieldValue::ObjectTag(tag) = &mut fields[0].value else {
            panic!("expected tag")
        };
        let ObjectRecord::New { class, .. } = &mut tag.record else {
            panic!("expected new object")
        };
        class.name = name;
        let error = encode_fields(&fields, 100000).unwrap_err();
        assert_eq!(error.operation, "model archive");
        assert!(error.cause.contains("expected"));
    }
}

#[test]
fn object_fields_own_class_descriptors_and_parallel_framing_is_deterministic() {
    let expected = object_sequence();
    let mut input = expected.clone();
    let fields = read_name_fields(&input, LIMITS).unwrap();
    input.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 1024).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
