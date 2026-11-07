//! Native headers, independent version boundaries, and immutable edit behavior.
#[path = "support/header_cases.rs"]
mod header_cases;

use bessemer::header::*;

const NATIVE: &[u8] = include_bytes!("fixtures/headers/blank-part.bin");

fn root(header: &DocumentHeader) -> &HeaderRecord {
    let HeaderObject::Header(record) = &header.contents().objects[&header.contents().root].object
    else {
        panic!("expected header");
    };
    record
}

fn decode(bytes: &[u8]) -> DocumentHeader {
    DocumentHeader::from_bytes(bytes, ArchiveVersion::new(19000), 100_000).unwrap()
}

#[test]
fn independent_threshold_vectors_rebuild_every_stored_byte() {
    for (name, version, bytes) in header_cases::CASES {
        let version = ArchiveVersion::new(*version);
        let header = DocumentHeader::from_bytes(bytes, version, 1000).unwrap();
        assert_eq!(root(&header).next_id, -24, "{name}");
        assert_eq!(header.to_bytes().unwrap(), *bytes, "{name}");
        let rebuilt = DocumentHeader::new(version, header.contents().clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), *bytes, "{name}");
    }
}

#[test]
fn native_kinds_rebuild_without_retaining_original_bytes() {
    for bytes in [
        NATIVE,
        include_bytes!("fixtures/headers/blank-assembly.bin"),
        include_bytes!("fixtures/headers/blank-drawing.bin"),
    ] {
        let parsed = decode(bytes);
        let rebuilt =
            DocumentHeader::new(parsed.archive_version(), parsed.contents().clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
        assert!(
            parsed
                .contents()
                .objects
                .values()
                .any(|node| matches!(node.object, HeaderObject::Log(_)))
        );
    }
    let header = decode(NATIVE);
    assert_eq!(
        (
            root(&header).next_id,
            root(&header).body_stamp,
            root(&header).update_stamp
        ),
        (24, Some(101), Some(102))
    );
    assert_eq!(root(&header).saved_while_model_exists, Some(1));
    let HeaderReference::Object(logs) = root(&header).revision_logs else {
        panic!("expected logs");
    };
    let HeaderObject::List(logs) = &header.contents().objects[&logs].object else {
        panic!("expected list");
    };
    assert_eq!(logs.items.len(), 22);
}

#[test]
fn embedded_override_applies_only_to_the_header_body() {
    let old = decode(include_bytes!("fixtures/headers/override-old.bin"));
    assert_eq!(root(&old).update_stamp, None);
    assert_eq!(root(&old).unknown_50, None);
    assert_eq!(root(&old).has_graphics_feature, Some(-14));
    let modern = DocumentHeader::from_bytes(
        include_bytes!("fixtures/headers/override-modern.bin"),
        ArchiveVersion::new(9000),
        100,
    )
    .unwrap();
    assert_eq!(root(&modern).unknown_50, Some(0xaabbccdd));
    assert_eq!(root(&modern).has_graphics_feature, None);
    assert_eq!(root(&modern).saved_while_model_exists, None);
    assert_eq!(modern.archive_version().value(), 9000);
}

#[test]
fn edits_reverts_and_failed_edits_isolate_the_source() {
    let source = decode(NATIVE);
    let mut contents = source.contents().clone();
    let HeaderObject::Header(record) =
        &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    record.next_id = -123;
    let edited = source.with_contents(contents.clone()).unwrap();
    let reopened = decode(&edited.to_bytes().unwrap());
    assert_eq!(root(&reopened).next_id, -123);
    assert_eq!(root(&source).next_id, 24);
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
    contents.objects.clear();
    assert!(source.with_contents(contents).is_err());
    assert_eq!(root(&edited).next_id, -123);
    let reverted = edited.with_contents(source.contents().clone()).unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), NATIVE);
    assert_eq!(
        source
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        NATIVE
    );
}

#[test]
fn bounding_box_bits_survive_nan_payloads_signed_zero_and_raw_flags() {
    let source = decode(include_bytes!("fixtures/headers/override-signed.bin"));
    let mut contents = source.contents().clone();
    let HeaderObject::Header(record) =
        &mut contents.objects.get_mut(&contents.root).unwrap().object
    else {
        panic!("expected header");
    };
    let bounds = record.bounding_box.as_mut().unwrap();
    bounds.has_box = i32::MIN;
    let box_values = bounds.bounds.as_mut().unwrap();
    box_values.centre[0] = StoredDouble::from_bits(0x7ff8_1234_5678_9abc);
    box_values.high[1] = StoredDouble::from_bits((-0.0_f64).to_bits());
    box_values.radius = StoredDouble::from_bits(f64::INFINITY.to_bits());
    let expected = bounds.clone();
    let edited = source.with_contents(contents).unwrap();
    let reopened = decode(&edited.to_bytes().unwrap());
    assert_eq!(root(&reopened).bounding_box.as_ref().unwrap(), &expected);
    assert!(expected.bounds.unwrap().centre[0].value().is_nan());
}

#[test]
fn versions_and_identities_reject_out_of_shape_values() {
    assert_eq!(
        ArchiveVersion::from_stream_name("_MO_VERSION_-1/Contents/Body")
            .unwrap()
            .value(),
        u32::MAX
    );
    assert_eq!(
        ArchiveVersion::from_stream_name("_MO_VERSION_4294967295/History")
            .unwrap()
            .value(),
        u32::MAX
    );
    for name in [
        "Header2",
        "_MO_VERSION_/History",
        "_MO_VERSION_4294967296/History",
        "_MO_VERSION_-2147483649/History",
        "_MO_VERSION_x/History",
    ] {
        assert!(ArchiveVersion::from_stream_name(name).is_err(), "{name}");
    }
    for identity in [0, 0x80000000, u32::MAX] {
        assert!(HeaderObjectId::new(identity).is_err());
    }
    assert_eq!(HeaderObjectId::new(0x7fffffff).unwrap().value(), 0x7fffffff);
}

#[test]
fn whole_document_edits_preserve_every_other_stream_and_header_field() {
    for bytes in [
        include_bytes!("fixtures/blank-part.SLDPRT").as_slice(),
        include_bytes!("fixtures/blank-assembly.SLDASM"),
        include_bytes!("fixtures/blank-drawing.SLDDRW"),
    ] {
        let original = bessemer::Document::from_bytes(bytes).unwrap();
        let mut document = original.clone();
        let header = decode(&document.read_stream("Header2").unwrap());
        let mut contents = header.contents().clone();
        let HeaderObject::Header(record) =
            &mut contents.objects.get_mut(&contents.root).unwrap().object
        else {
            panic!("expected header");
        };
        record.next_id = -123;
        let edited = header.with_contents(contents).unwrap();
        document
            .replace_stream("Header2", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        let reopened_header = decode(&reopened.read_stream("Header2").unwrap());
        assert_eq!(reopened_header.contents(), edited.contents());
        for stream in original.streams() {
            if stream.name == "Header2" {
                continue;
            }
            assert_eq!(
                original.read_stream(stream.name).unwrap(),
                reopened.read_stream(stream.name).unwrap(),
                "{}",
                stream.name
            );
        }
        assert_eq!(original.to_bytes().unwrap(), bytes);
    }
}
