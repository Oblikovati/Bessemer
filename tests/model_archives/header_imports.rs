use super::{ArchiveClass, ArchiveCursor, ObjectReader, ObjectRecord};
use crate::Document;
use crate::header::{ArchiveVersion, DocumentHeader, HeaderObject, HeaderReference};

const NATIVE_PART: &[u8] = include_bytes!("../fixtures/blank-part.SLDPRT");
fn owned_configuration_bytes(name: &str) -> Vec<u8> {
    let document = Document::from_bytes(NATIVE_PART).unwrap();
    document.read_stream(name).unwrap()
}

fn owned_configuration_header() -> DocumentHeader {
    let bytes = owned_configuration_bytes("Contents/Config-0-ModelHeader");
    let version = ArchiveVersion::new(19000);
    DocumentHeader::from_bytes(&bytes, version, 1000).unwrap()
}

fn owned_header_root(header: &DocumentHeader) -> &crate::header::HeaderRecord {
    let contents = header.contents();
    let HeaderObject::Header(root) = &contents.objects[&contents.root].object else {
        panic!("owned header root; expected moHeader_c");
    };
    root
}

fn owned_header_self_class(header: &DocumentHeader) -> ArchiveClass {
    let root = owned_header_root(header);
    assert_eq!(root.external_references, HeaderReference::Null);
    let Some(HeaderReference::Object(identity)) = root.self_reference else {
        panic!(
            "owned self reference {:?}; expected an object",
            root.self_reference
        );
    };
    assert_ne!(identity.value(), 3);
    let node = &header.contents().objects[&identity];
    assert!(matches!(node.object, HeaderObject::External(_)));
    ArchiveClass {
        name: "moExtObject_c".into(),
        schema: node.schema,
    }
}

fn read_owned_node(map: &mut ObjectReader, class: ArchiveClass) -> ObjectRecord {
    let bytes = owned_configuration_bytes("Contents/Config-0");
    let mut cursor = ArchiveCursor::new(&bytes);
    let root = map.object(&mut cursor).unwrap();
    assert!(matches!(root, ObjectRecord::New { index, class }
        if index.value() == 2 && class.name == "moPart_c"));
    assert_eq!(cursor.i32().unwrap(), 2903);
    assert_eq!(map.append_loaded_object(Some(class)).unwrap().value(), 3);
    assert_eq!(cursor.i32().unwrap(), 0);
    assert_eq!(cursor.u32().unwrap(), 19000);
    map.object(&mut cursor).unwrap()
}

#[test]
fn native_header_object_is_imported_at_configuration_map_slot_three() {
    let header = owned_configuration_header();
    let before = header.to_bytes().unwrap();
    let mut map = ObjectReader::new();
    let class = owned_header_self_class(&header);
    let name = read_owned_node(&mut map, class.clone());
    assert!(matches!(name, ObjectRecord::New { index, .. } if index.value() == 5));
    let alias = map.object(&mut ArchiveCursor::new(&[3, 0])).unwrap();
    assert!(
        matches!(alias, ObjectRecord::Reference { index, class: resolved }
        if index.value() == 3 && resolved == class)
    );
    assert_eq!(header.to_bytes().unwrap(), before);
}
