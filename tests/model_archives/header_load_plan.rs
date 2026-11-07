use std::sync::Arc;

use super::header_imports::{ImportedHeaderMap, ModelHeaderLoadPlan};
use super::{ModelDecoder, ModelFieldLimits, header_order};
use crate::archive_codec::{ArchiveCursor, ArchiveMapIndex, ObjectReader, ObjectRecord};
use crate::header::{
    ArchiveVersion, DocumentHeader, HeaderContents, HeaderNode, HeaderObject, HeaderObjectId,
    HeaderRecord, HeaderReference, ObjectList,
};
use crate::{Document, FailureClass};

const VERSION: ArchiveVersion = ArchiveVersion::new(19000);
const ROOT_TAG: &[u8] = b"\xff\xff\x01\x00\x08\x00moPart_c";
const NAME_TAG: &[u8] = b"\xff\xff\x01\x00\x0c\x00moNodeName_c";
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 1024,
    maximum_fields: 32,
    maximum_entries: 64,
    maximum_depth: 8,
    maximum_objects: 16,
};

pub(super) fn native_header() -> DocumentHeader {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = document
        .read_stream("Contents/Config-0-ModelHeader")
        .unwrap();
    DocumentHeader::from_bytes(&bytes, VERSION, 1000).unwrap()
}

fn root_mut(contents: &mut HeaderContents) -> &mut HeaderRecord {
    let node = contents.objects.get_mut(&contents.root).unwrap();
    let HeaderObject::Header(root) = &mut node.object else {
        panic!("expected header")
    };
    root
}

fn duplicate_external(contents: &mut HeaderContents, original: HeaderReference) -> HeaderReference {
    let HeaderReference::Object(original) = original else {
        panic!("expected self object")
    };
    let identity = HeaderObjectId::new(10001).unwrap();
    let mut node = contents.objects[&original].clone();
    node.schema = 0x1234;
    contents.objects.insert(identity, node);
    HeaderReference::Object(identity)
}

fn install_external_list(
    contents: &mut HeaderContents,
    items: Vec<HeaderReference>,
    clear_self: bool,
) {
    let identity = HeaderObjectId::new(10000).unwrap();
    let root = root_mut(contents);
    root.external_references = HeaderReference::Object(identity);
    if clear_self {
        root.self_reference = Some(HeaderReference::Null);
    }
    let node = HeaderNode {
        schema: 0x2468,
        object: HeaderObject::ExternalList(ObjectList { items }),
    };
    contents.objects.insert(identity, node);
}

fn mixed_header(clear_self: bool) -> (Arc<DocumentHeader>, HeaderReference, HeaderReference) {
    let mut contents = native_header().contents().clone();
    let self_ref = root_mut(&mut contents).self_reference.unwrap();
    let other = duplicate_external(&mut contents, self_ref);
    let items = vec![HeaderReference::Null, self_ref, other, self_ref];
    install_external_list(&mut contents, items, clear_self);
    let header = Arc::new(DocumentHeader::new(VERSION, contents).unwrap());
    (header, self_ref, other)
}

pub(super) fn root_map() -> ObjectReader {
    let mut map = ObjectReader::new();
    map.object(&mut ArchiveCursor::new(ROOT_TAG)).unwrap();
    map
}

fn load_header(header: Arc<DocumentHeader>) -> (ImportedHeaderMap, ObjectReader) {
    let plan = ModelHeaderLoadPlan::new(header, VERSION, 64).unwrap();
    let mut decoder = ModelDecoder::new(&[], VERSION, LIMITS).unwrap();
    let mut map = root_map();
    let imported = plan.append(&mut decoder, &mut map).unwrap();
    assert!(decoder.finish().unwrap().is_empty());
    (imported, map)
}

fn imported_references(imported: &ImportedHeaderMap) -> Vec<HeaderReference> {
    imported
        .slots()
        .iter()
        .map(|(_, reference)| *reference)
        .collect()
}

fn assert_order_failure_shape(contents: &HeaderContents, expected: &str) {
    let error = header_order::ordered_imports(contents, 64).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains(expected));
}

#[test]
fn native_header_plan_preserves_graph_identity_and_the_next_class_slot() {
    let header = Arc::new(native_header());
    let before = header.to_bytes().unwrap();
    let expected = header_order::ordered_imports(header.contents(), 1).unwrap()[0];
    let (imported, mut map) = load_header(header);
    assert_eq!(imported.slots().len(), 1);
    assert_eq!(
        imported.origin(ArchiveMapIndex::nonzero(3).unwrap()),
        Some(expected)
    );
    assert_eq!(imported.origin(ArchiveMapIndex::nonzero(2).unwrap()), None);
    let node = map.object(&mut ArchiveCursor::new(NAME_TAG)).unwrap();
    assert!(matches!(node, ObjectRecord::New { index, .. } if index.value() == 5));
    assert_eq!(imported.header().to_bytes().unwrap(), before);
}

#[test]
fn external_items_keep_order_nulls_duplicates_and_distinct_runtime_schemas() {
    let (header, self_ref, other) = mixed_header(false);
    let (imported, mut map) = load_header(header);
    let references = imported_references(&imported);
    let expected = [self_ref, HeaderReference::Null, self_ref, other, self_ref];
    assert_eq!(references, expected);
    let null = map.object(&mut ArchiveCursor::new(&[4, 0])).unwrap();
    assert_eq!(null, ObjectRecord::Null);
    let distinct = map.object(&mut ArchiveCursor::new(&[6, 0])).unwrap();
    assert!(
        matches!(distinct, ObjectRecord::Reference { class, .. } if class.name == "moExtObject_c" && class.schema == 0x1234)
    );
    assert_eq!(
        imported.origin(ArchiveMapIndex::nonzero(3).unwrap()),
        imported.origin(ArchiveMapIndex::nonzero(7).unwrap())
    );
    let node = map.object(&mut ArchiveCursor::new(NAME_TAG)).unwrap();
    assert!(matches!(node, ObjectRecord::New { index, .. } if index.value() == 9));
}

#[test]
fn null_self_reference_is_skipped_but_null_external_items_take_slots() {
    let (header, self_ref, other) = mixed_header(true);
    let (imported, _) = load_header(header);
    let references = imported_references(&imported);
    assert_eq!(
        references,
        [HeaderReference::Null, self_ref, other, self_ref]
    );
    assert_eq!(
        imported.origin(ArchiveMapIndex::nonzero(3).unwrap()),
        Some(HeaderReference::Null)
    );
}

#[test]
fn absent_and_null_self_fields_do_not_synthesize_an_import() {
    let mut contents = native_header().contents().clone();
    for self_ref in [None, Some(HeaderReference::Null)] {
        root_mut(&mut contents).self_reference = self_ref;
        assert!(
            header_order::ordered_imports(&contents, 0)
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn import_limits_fail_before_appending_any_model_map_slots() {
    let (header, _, _) = mixed_header(false);
    assert!(ModelHeaderLoadPlan::new(header.clone(), VERSION, 4).is_err());
    let plan = ModelHeaderLoadPlan::new(header, VERSION, 5).unwrap();
    let limits = ModelFieldLimits {
        maximum_entries: 4,
        ..LIMITS
    };
    let mut decoder = ModelDecoder::new(&[], VERSION, limits).unwrap();
    let mut map = root_map();
    let error = plan.append(&mut decoder, &mut map).err().unwrap();
    assert!(error.cause.contains("header imports 5; expected <= 4"));
    assert_eq!(decoder.finish().unwrap_err(), error);
    let node = map.object(&mut ArchiveCursor::new(NAME_TAG)).unwrap();
    assert!(matches!(node, ObjectRecord::New { index, .. } if index.value() == 4));
}

#[test]
fn header_and_model_versions_are_explicit_and_cannot_be_mixed() {
    let header = Arc::new(native_header());
    let old = ModelHeaderLoadPlan::new(header.clone(), ArchiveVersion::new(0x27bc), 64)
        .err()
        .unwrap();
    assert_eq!(old.class, FailureClass::Unsupported);
    let wrong = ModelHeaderLoadPlan::new(header.clone(), ArchiveVersion::new(18000), 64)
        .err()
        .unwrap();
    assert!(wrong.cause.contains("expected equal outer versions"));
    let plan = ModelHeaderLoadPlan::new(header, VERSION, 64).unwrap();
    let mut decoder = ModelDecoder::new(&[], ArchiveVersion::new(18000), LIMITS).unwrap();
    let error = plan.append(&mut decoder, &mut root_map()).err().unwrap();
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn missing_raw_header_identities_fail_with_the_offending_index() {
    let mut contents = native_header().contents().clone();
    let missing = HeaderObjectId::new(90000).unwrap();
    assert!(header_order::imported_class(&contents, HeaderReference::Object(missing)).is_err());
    contents.root = missing;
    assert_order_failure_shape(&contents, "90000");
}

#[test]
fn wrong_raw_header_node_roles_fail_with_expected_class_shapes() {
    let mut contents = native_header().contents().clone();
    let users = root_mut(&mut contents).users;
    root_mut(&mut contents).external_references = users;
    assert_order_failure_shape(&contents, "expected moExtObjectList_c");
    let HeaderReference::Object(users) = users else {
        panic!("expected users")
    };
    contents.root = users;
    assert_order_failure_shape(&contents, "expected moHeader_c");
}

#[test]
fn header_class_projection_uses_the_stored_schema_and_actual_node_kind() {
    let mut contents = native_header().contents().clone();
    let users = root_mut(&mut contents).users;
    let descriptor = header_order::imported_class(&contents, users)
        .unwrap()
        .unwrap();
    assert_eq!(descriptor.name, "su_CStringArray");
    let HeaderReference::Object(identity) = users else {
        panic!("expected users")
    };
    assert_eq!(descriptor.schema, contents.objects[&identity].schema);
    assert_eq!(
        header_order::imported_class(&contents, HeaderReference::Null).unwrap(),
        None
    );
}

#[test]
fn import_count_arithmetic_handles_exact_limits_and_overflow() {
    assert_eq!(header_order::checked_import_count(0, false, 0).unwrap(), 0);
    assert_eq!(header_order::checked_import_count(4, true, 5).unwrap(), 5);
    assert!(header_order::checked_import_count(4, true, 4).is_err());
    assert!(header_order::checked_import_count(usize::MAX, true, usize::MAX).is_err());
    assert_eq!(
        header_order::checked_import_count(usize::MAX, false, usize::MAX).unwrap(),
        usize::MAX
    );
}

#[test]
fn imported_header_storage_outlives_the_original_arc_and_shared_reads_match() {
    let header = Arc::new(native_header());
    let before = header.to_bytes().unwrap();
    let (imported, _) = load_header(header.clone());
    drop(header);
    std::thread::scope(|scope| {
        let readers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| imported.header().to_bytes().unwrap()))
            .collect();
        for reader in readers {
            assert_eq!(reader.join().unwrap(), before);
        }
    });
}
