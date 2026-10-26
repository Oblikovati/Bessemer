use super::{ArchiveClass, ArchiveCursor, ObjectReader, ObjectRecord, ObjectWriter};

const PART_TAG: &[u8] = b"\xff\xff\x01\x00\x08\x00moPart_c";
const NAME_TAG: &[u8] = b"\xff\xff\x01\x00\x0c\x00moNodeName_c";

fn external_class() -> ArchiveClass {
    ArchiveClass {
        name: "moExtObject_c".into(),
        schema: 1,
    }
}

fn read_import_probe(objects: &mut ObjectReader, bytes: &[u8]) -> ObjectRecord {
    let mut cursor = ArchiveCursor::new(bytes);
    objects.object(&mut cursor).unwrap()
}

fn serialized_with_header_import() -> Vec<u8> {
    let mut writer = ObjectWriter::new();
    let mut output = Vec::new();
    let part = ArchiveClass {
        name: "moPart_c".into(),
        schema: 1,
    };
    writer.object(&part, &mut output).unwrap();
    assert_eq!(writer.reserve().unwrap().value(), 3);
    writer.object(&external_class(), &mut output).unwrap();
    output
}

#[test]
fn header_import_takes_one_object_slot_without_consuming_class_bytes() {
    let mut objects = ObjectReader::new();
    let root = read_import_probe(&mut objects, PART_TAG);
    assert!(matches!(root, ObjectRecord::New { index, .. } if index.value() == 2));
    let imported = objects
        .append_loaded_object(Some(external_class()))
        .unwrap();
    assert_eq!(imported.value(), 3);
    let node = read_import_probe(&mut objects, NAME_TAG);
    assert!(matches!(node, ObjectRecord::New { index, .. } if index.value() == 5));
    let alias = read_import_probe(&mut objects, &[3, 0]);
    assert!(matches!(alias, ObjectRecord::Reference { index, class }
        if index.value() == 3 && class == external_class()));
}

#[test]
fn imported_nulls_duplicates_and_class_flags_keep_separate_slots() {
    let mut objects = ObjectReader::new();
    for expected in 1..=2 {
        let index = objects
            .append_loaded_object(Some(external_class()))
            .unwrap();
        assert_eq!(index.value(), expected);
    }
    let null = objects.append_loaded_object(None).unwrap();
    assert_eq!(null.value(), 3);
    let alias = read_import_probe(&mut objects, &[2, 0]);
    assert!(matches!(alias, ObjectRecord::Reference { index, .. } if index.value() == 2));
    let null_alias = read_import_probe(&mut objects, &[3, 0]);
    assert!(matches!(null_alias, ObjectRecord::Null));
    assert!(objects.object(&mut ArchiveCursor::new(&[1, 0x80])).is_err());
}

#[test]
fn imported_classes_are_not_registered_as_serialized_class_slots() {
    let output = serialized_with_header_import();
    let expected = [PART_TAG, b"\xff\xff\x01\x00\x0d\x00moExtObject_c"].concat();
    assert_eq!(output, expected);
    let mut reader = ObjectReader::new();
    read_import_probe(&mut reader, PART_TAG);
    reader.append_loaded_object(Some(external_class())).unwrap();
    let loaded = read_import_probe(&mut reader, &output[PART_TAG.len()..]);
    assert!(matches!(loaded, ObjectRecord::New { index, .. } if index.value() == 5));
}

#[test]
fn wide_aliases_to_imported_objects_and_nulls_are_resolved() {
    let mut objects = ObjectReader::new();
    objects
        .append_loaded_object(Some(external_class()))
        .unwrap();
    objects.append_loaded_object(None).unwrap();
    let alias = read_import_probe(&mut objects, &[0xff, 0x7f, 1, 0, 0, 0]);
    assert!(matches!(alias, ObjectRecord::Reference { index, .. } if index.value() == 1));
    let null = read_import_probe(&mut objects, &[0xff, 0x7f, 2, 0, 0, 0]);
    assert!(matches!(null, ObjectRecord::Null));
    assert!(
        objects
            .object(&mut ArchiveCursor::new(&[0xff, 0x7f, 3, 0, 0, 0]))
            .is_err()
    );
}
