//! Shared identities, cyclic graphs, large collections, and concurrency.
use bessemer::header::*;

const SOURCE: &[u8] = include_bytes!("fixtures/headers/version-3f13.bin");

fn source() -> DocumentHeader {
    DocumentHeader::from_bytes(SOURCE, ArchiveVersion::new(0x3f13), 100).unwrap()
}

fn install_list(contents: &mut HeaderContents, list: HeaderObjectId) {
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected root");
    };
    root.replaced_components = Some(HeaderReference::Object(list));
}

fn insert_list(
    contents: &mut HeaderContents,
    identity: HeaderObjectId,
    items: Vec<HeaderReference>,
) {
    contents.objects.insert(
        identity,
        HeaderNode {
            schema: 0,
            object: HeaderObject::List(ObjectList { items }),
        },
    );
}

#[test]
fn cyclic_lists_and_root_references_roundtrip_without_recursion() {
    let source = source();
    let mut contents = source.contents().clone();
    let list = HeaderObjectId::new(100).unwrap();
    let root = contents.root;
    install_list(&mut contents, list);
    insert_list(
        &mut contents,
        list,
        vec![
            HeaderReference::Object(list),
            HeaderReference::Null,
            HeaderReference::Object(root),
        ],
    );
    let encoded = source.with_contents(contents).unwrap().to_bytes().unwrap();
    let reopened = DocumentHeader::from_bytes(&encoded, source.archive_version(), 100).unwrap();
    let HeaderObject::Header(root) = &reopened.contents().objects[&reopened.contents().root].object
    else {
        panic!("expected root");
    };
    let HeaderReference::Object(id) = root.replaced_components.unwrap() else {
        panic!("expected list");
    };
    let HeaderObject::List(list) = &reopened.contents().objects[&id].object else {
        panic!("expected list");
    };
    assert_eq!(
        list.items,
        [
            HeaderReference::Object(id),
            HeaderReference::Null,
            HeaderReference::Object(reopened.contents().root)
        ]
    );
}

#[test]
fn deeply_nested_objects_work_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(128 * 1024)
        .spawn(|| {
            let source = source();
            let mut contents = source.contents().clone();
            install_list(&mut contents, HeaderObjectId::new(100).unwrap());
            for ordinal in 100..30_100 {
                let next = if ordinal == 30_099 {
                    HeaderReference::Null
                } else {
                    HeaderReference::Object(HeaderObjectId::new(ordinal + 1).unwrap())
                };
                insert_list(
                    &mut contents,
                    HeaderObjectId::new(ordinal).unwrap(),
                    vec![next],
                );
            }
            let header = source.with_contents(contents).unwrap();
            let encoded = header.to_bytes().unwrap();
            let reopened =
                DocumentHeader::from_bytes(&encoded, source.archive_version(), 30_100).unwrap();
            assert_eq!(
                reopened.contents().objects.len(),
                header.contents().objects.len()
            );
            let rebuilt =
                DocumentHeader::new(source.archive_version(), reopened.contents().clone()).unwrap();
            assert_eq!(rebuilt.to_bytes().unwrap(), encoded);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn extended_collection_counts_preserve_shared_and_distinct_equal_objects() {
    let source = source();
    let mut contents = source.contents().clone();
    let list = HeaderObjectId::new(100).unwrap();
    let first = HeaderObjectId::new(101).unwrap();
    let second = HeaderObjectId::new(102).unwrap();
    install_list(&mut contents, list);
    let text = ArchiveString::new("equal", TextEncoding::Utf16).unwrap();
    for id in [first, second] {
        contents.objects.insert(
            id,
            HeaderNode {
                schema: 7,
                object: HeaderObject::StringHandle(text.clone()),
            },
        );
    }
    let mut items = vec![HeaderReference::Object(first); 65_535];
    items.push(HeaderReference::Object(second));
    insert_list(&mut contents, list, items);
    let encoded = source.with_contents(contents).unwrap().to_bytes().unwrap();
    assert!(
        encoded
            .windows(6)
            .any(|bytes| bytes == [255, 255, 0, 0, 1, 0])
    );
    let reopened = DocumentHeader::from_bytes(&encoded, source.archive_version(), 100).unwrap();
    let lists: Vec<&ObjectList> = reopened
        .contents()
        .objects
        .values()
        .filter_map(|node| match &node.object {
            HeaderObject::List(list) if list.items.len() == 65_536 => Some(list),
            _ => None,
        })
        .collect();
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].items[0], lists[0].items[65_534]);
    assert_ne!(lists[0].items[0], lists[0].items[65_535]);
}

#[test]
fn every_public_header_type_is_send_and_sync() {
    fn assert_thread_safe<T: Send + Sync>() {}
    assert_thread_safe::<DocumentHeader>();
    assert_thread_safe::<ArchiveVersion>();
    assert_thread_safe::<HeaderContents>();
    assert_thread_safe::<HeaderNode>();
    assert_thread_safe::<HeaderObject>();
    assert_thread_safe::<HeaderObjectId>();
    assert_thread_safe::<HeaderReference>();
    assert_thread_safe::<HeaderRecord>();
    assert_thread_safe::<ExternalFileReference>();
    assert_thread_safe::<SharedFileName>();
    assert_thread_safe::<StringArray>();
    assert_thread_safe::<ObjectList>();
    assert_thread_safe::<LogRecord>();
    assert_thread_safe::<HeaderBounds>();
    assert_thread_safe::<HeaderBox>();
    assert_thread_safe::<StoredDouble>();
}

#[test]
fn concurrent_reads_and_independent_edits_match_serial_bytes() {
    let source = source();
    let expected = source.to_bytes().unwrap();
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let source = source.clone();
            let expected = expected.clone();
            std::thread::spawn(move || {
                for _ in 0..30 {
                    assert_eq!(source.to_bytes().unwrap(), expected);
                    let mut contents = source.contents().clone();
                    let HeaderObject::Header(root) =
                        &mut contents.objects.get_mut(&contents.root).unwrap().object
                    else {
                        panic!("expected root");
                    };
                    root.next_id = 1234;
                    let edited = source.with_contents(contents).unwrap();
                    let rebuilt =
                        DocumentHeader::new(source.archive_version(), edited.contents().clone())
                            .unwrap();
                    assert_eq!(edited.to_bytes().unwrap(), rebuilt.to_bytes().unwrap());
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(source.to_bytes().unwrap(), expected);
}

#[test]
fn external_handle_and_list_sharing_survive_edits() {
    let source = source();
    let mut contents = source.contents().clone();
    let external_id = contents
        .objects
        .iter()
        .find_map(|(id, node)| matches!(node.object, HeaderObject::External(_)).then_some(*id))
        .unwrap();
    let HeaderObject::External(external) =
        &mut contents.objects.get_mut(&external_id).unwrap().object
    else {
        panic!("expected external");
    };
    let SharedFileName::Handle(HeaderReference::Object(old_name)) = external.name else {
        panic!("expected handle");
    };
    external.name = external.path.clone();
    external.flags = 0x81;
    contents.objects.remove(&old_name);
    let list_id = HeaderObjectId::new(100).unwrap();
    contents.objects.insert(
        list_id,
        HeaderNode {
            schema: 42,
            object: HeaderObject::ExternalList(ObjectList {
                items: vec![HeaderReference::Object(external_id); 2],
            }),
        },
    );
    let HeaderObject::Header(root) = &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected root");
    };
    root.external_references = HeaderReference::Object(list_id);
    let encoded = source.with_contents(contents).unwrap().to_bytes().unwrap();
    let reopened = DocumentHeader::from_bytes(&encoded, source.archive_version(), 100).unwrap();
    let externals: Vec<_> = reopened
        .contents()
        .objects
        .values()
        .filter_map(|node| match &node.object {
            HeaderObject::External(external) => Some(external),
            _ => None,
        })
        .collect();
    assert_eq!(externals.len(), 1);
    assert_eq!(externals[0].path, externals[0].name);
    assert_eq!(externals[0].flags, 0x81);
    let lists: Vec<_> = reopened
        .contents()
        .objects
        .values()
        .filter_map(|node| match &node.object {
            HeaderObject::ExternalList(list) => Some((node.schema, list)),
            _ => None,
        })
        .collect();
    assert_eq!(lists[0].0, 42);
    assert_eq!(lists[0].1.items[0], lists[0].1.items[1]);
}
