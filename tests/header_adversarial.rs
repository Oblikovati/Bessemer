//! Header framing, allocation budgets, and fail-closed graph validation.
use bessemer::FailureClass;
use bessemer::header::*;

const SOURCE: &[u8] = include_bytes!("fixtures/headers/version-3f13.bin");

fn read(bytes: &[u8]) -> bessemer::Result<DocumentHeader> {
    DocumentHeader::from_bytes(bytes, ArchiveVersion::new(0x3f13), 100)
}

#[test]
fn every_truncation_and_trailing_bytes_fail_closed() {
    for end in 0..SOURCE.len() {
        assert!(read(&SOURCE[..end]).is_err(), "prefix {end}");
    }
    let mut trailing = SOURCE.to_vec();
    trailing.push(0);
    assert!(read(&trailing).is_err());
    assert!(read(&[0, 0]).is_err());
}

#[test]
fn object_budget_is_enforced_at_the_exact_boundary() {
    let count = read(SOURCE).unwrap().contents().objects.len();
    let version = ArchiveVersion::new(0x3f13);
    assert!(DocumentHeader::from_bytes(SOURCE, version, count).is_ok());
    assert!(
        DocumentHeader::from_bytes(SOURCE, version, count - 1)
            .unwrap_err()
            .cause
            .contains("object count")
    );
    assert!(DocumentHeader::from_bytes(SOURCE, version, 0).is_err());
}

#[test]
fn undocumented_legacy_external_bodies_report_unsupported() {
    let bytes = include_bytes!("fixtures/headers/version-0f18.bin");
    let error = DocumentHeader::from_bytes(bytes, ArchiveVersion::new(0xf17), 100).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("0xF18"));
}

#[test]
fn impossible_collection_counts_and_bad_references_are_rejected() {
    let start = SOURCE
        .windows(b"su_CStringArray".len())
        .position(|bytes| bytes == b"su_CStringArray")
        .unwrap()
        + b"su_CStringArray".len();
    let mut oversized = SOURCE.to_vec();
    oversized.splice(start..start + 2, [255, 255, 255, 255, 255, 255]);
    assert!(
        read(&oversized)
            .unwrap_err()
            .cause
            .contains("collection count")
    );
    let mut missing = SOURCE.to_vec();
    missing[20..22].copy_from_slice(&0x7ffe_u16.to_le_bytes());
    assert!(read(&missing).is_err());
}

#[test]
fn missing_orphan_and_wrong_class_objects_are_rejected() {
    let source = read(SOURCE).unwrap();
    let mut missing = source.contents().clone();
    missing.objects.remove(&missing.root);
    assert!(source.with_contents(missing).is_err());
    let mut orphan = source.contents().clone();
    orphan.objects.insert(
        HeaderObjectId::new(100).unwrap(),
        HeaderNode {
            schema: 0,
            object: HeaderObject::List(ObjectList { items: Vec::new() }),
        },
    );
    assert!(
        source
            .with_contents(orphan)
            .unwrap_err()
            .cause
            .contains("orphan")
    );
    let mut wrong = source.contents().clone();
    let HeaderObject::Header(root) = &mut wrong.objects.get_mut(&wrong.root).unwrap().object else {
        panic!("expected header");
    };
    root.users = HeaderReference::Object(wrong.root);
    assert!(
        source
            .with_contents(wrong)
            .unwrap_err()
            .cause
            .contains("su_CStringArray")
    );
}

#[test]
fn version_absence_null_and_box_presence_have_distinct_contracts() {
    let source = read(SOURCE).unwrap();
    let mut contents = source.contents().clone();
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    root.linked_attachments = None;
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("linked_attachments")
    );
    let mut contents = source.contents().clone();
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    root.bounding_box.as_mut().unwrap().has_box = 0;
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("box flag")
    );
}

#[test]
fn nonminimal_counts_keep_original_bytes_for_equivalent_edits_and_reverts() {
    let start = SOURCE
        .windows(b"su_CStringArray".len())
        .position(|bytes| bytes == b"su_CStringArray")
        .unwrap()
        + b"su_CStringArray".len();
    let mut bytes = SOURCE.to_vec();
    bytes.splice(start..start + 2, [255, 255, 1, 0, 0, 0]);
    let source = read(&bytes).unwrap();
    assert_eq!(
        source
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    let mut contents = source.contents().clone();
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    root.next_id = 999;
    let edited = source.with_contents(contents).unwrap();
    assert_eq!(
        edited
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    let rebuilt = DocumentHeader::new(source.archive_version(), source.contents().clone()).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), SOURCE);
}

#[test]
fn external_name_variants_and_list_element_classes_are_checked() {
    let source = read(SOURCE).unwrap();
    let mut contents = source.contents().clone();
    let external = contents
        .objects
        .values_mut()
        .find_map(|node| match &mut node.object {
            HeaderObject::External(external) => Some(external),
            _ => None,
        })
        .unwrap();
    external.path =
        SharedFileName::Text(ArchiveString::new("wrong", TextEncoding::Latin1).unwrap());
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("expected handle")
    );
    let mut contents = source.contents().clone();
    let list_id = HeaderObjectId::new(100).unwrap();
    contents.objects.insert(
        list_id,
        HeaderNode {
            schema: 0,
            object: HeaderObject::ExternalList(ObjectList {
                items: vec![HeaderReference::Object(contents.root)],
            }),
        },
    );
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    root.external_references = HeaderReference::Object(list_id);
    assert!(
        source
            .with_contents(contents)
            .unwrap_err()
            .cause
            .contains("expected External")
    );
}

#[test]
fn empty_bounds_and_extended_empty_string_arrays_are_valid() {
    let source = read(SOURCE).unwrap();
    let mut contents = source.contents().clone();
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    root.bounding_box = Some(HeaderBounds {
        has_box: 0,
        bounds: None,
    });
    let edited = source.with_contents(contents).unwrap();
    let encoded = edited.to_bytes().unwrap();
    assert_eq!(read(&encoded).unwrap().contents(), edited.contents());
    let start = encoded
        .windows(b"su_CStringArray".len())
        .position(|bytes| bytes == b"su_CStringArray")
        .unwrap()
        + b"su_CStringArray".len();
    let mut empty = encoded;
    empty.splice(start..start + 10, [255, 255, 0, 0, 0, 0]);
    assert!(read(&empty).is_ok());
}
