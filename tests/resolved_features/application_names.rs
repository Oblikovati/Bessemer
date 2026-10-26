//! Document naming is composed by the application; raw field edits keep their scope (#56).
use super::{ModelArchiveLimits, ModelField, ModelFieldValue, resolved_features};
use super::{context_snapshot_tests, manager_context_tests, owned_resolved_feature_tests};
use crate::Document;
use crate::header::{ArchiveString, DocumentHeader, HeaderObject, TextEncoding};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NamingNodeIdentity(i32);

fn owned_plane_identity(fields: &[ModelField]) -> NamingNodeIdentity {
    let field = fields
        .iter()
        .find(|field| field.path.as_ref() == "root.features[13].model_feature.feature.node.id")
        .unwrap();
    let ModelFieldValue::I32(value) = field.value else {
        panic!("expected persisted signed node identity")
    };
    NamingNodeIdentity(value)
}

fn renamed_header_graph(header: &DocumentHeader, identity: NamingNodeIdentity) -> DocumentHeader {
    let mut contents = header.contents().clone();
    let mut matches = 0;
    for node in contents.objects.values_mut() {
        let HeaderObject::Log(log) = &mut node.object else {
            continue;
        };
        if NamingNodeIdentity(log.id) != identity {
            continue;
        }
        log.unknown_50 = ArchiveString::new("Bessemer Plane Ω", TextEncoding::Utf16).unwrap();
        matches += 1;
    }
    assert_eq!(
        matches, 1,
        "logical node {identity:?}; expected one header log"
    );
    header.with_contents(contents).unwrap()
}

fn document_with_native_plane_name(document: &Document) -> Document {
    let source = context_snapshot_tests::configuration_from(document);
    let manager =
        manager_context_tests::decode(&document.read_stream("Contents/CMgr").unwrap()).unwrap();
    let stream = "Contents/Config-0-ResolvedFeatures";
    let parsed = resolved_features::parse(
        &document.read_stream(stream).unwrap(),
        &source,
        &manager,
        LIMITS,
    )
    .unwrap();
    let identity = owned_plane_identity(&parsed.fields);
    let edited = resolved_features::construct(
        owned_resolved_feature_tests::renamed_plane(&parsed.fields),
        &source,
        &manager,
        LIMITS,
    )
    .unwrap();
    let global = DocumentHeader::from_bytes(
        &document.read_stream("Header2").unwrap(),
        source.context.version,
        LIMITS.maximum_objects,
    )
    .unwrap();
    let selected = source.context.configuration.header_name();
    let mut output = document.clone();
    for (name, header) in [
        ("Header2", &global),
        (selected.as_str(), source.context.header.as_ref()),
    ] {
        output
            .replace_stream(
                name,
                renamed_header_graph(header, identity).to_bytes().unwrap(),
            )
            .unwrap();
    }
    output
        .replace_stream(
            stream,
            super::encode_fields(&edited.fields, LIMITS.maximum_bytes).unwrap(),
        )
        .unwrap();
    output
}

#[test]
fn application_plane_name_policy_matches_native_accepted_document_and_reverts_exactly() {
    let original_bytes = include_bytes!("../fixtures/blank-part.SLDPRT");
    let original = Document::from_bytes(original_bytes).unwrap();
    let changed = document_with_native_plane_name(&original);
    let expected =
        include_bytes!("../fixtures/resolved-features/owned-plane-renamed-document.SLDPRT");
    assert!(
        changed.to_bytes().unwrap() == expected,
        "expected the complete native-accepted plane-name fixture"
    );
    assert_eq!(original.to_bytes().unwrap(), original_bytes);
    let reopened = Document::from_bytes(expected).unwrap();
    for entry in original.streams().filter(|entry| {
        !matches!(
            entry.name,
            "Header2" | "Contents/Config-0-ModelHeader" | "Contents/Config-0-ResolvedFeatures"
        )
    }) {
        assert_eq!(
            original.read_stream(entry.name).unwrap(),
            reopened.read_stream(entry.name).unwrap()
        );
    }
    // Exact revert needs the operation's original compressed spans, which a fresh read cannot recover.
    let mut reverted = changed;
    for name in [
        "Header2",
        "Contents/Config-0-ModelHeader",
        "Contents/Config-0-ResolvedFeatures",
    ] {
        reverted
            .replace_stream(name, original.read_stream(name).unwrap())
            .unwrap();
    }
    assert!(
        reverted.to_bytes().unwrap() == original_bytes,
        "expected the original container bytes after reverting the operation"
    );
}

#[test]
fn naming_identity_is_the_node_id_and_never_the_archive_object_map_index() {
    let source = context_snapshot_tests::owned_configuration();
    let manager = manager_context_tests::decode(&manager_context_tests::manager_bytes()).unwrap();
    let parsed = resolved_features::parse(
        include_bytes!("../fixtures/resolved-features/owned-features.bin"),
        &source,
        &manager,
        LIMITS,
    )
    .unwrap();
    assert_eq!(owned_plane_identity(&parsed.fields), NamingNodeIdentity(2));
    let field = parsed
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.features[13]")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("feature pointer")
    };
    let super::ModelObjectRecord::New { index, .. } = &tag.record else {
        panic!("new feature")
    };
    assert_ne!(
        i64::from(index.value()),
        i64::from(owned_plane_identity(&parsed.fields).0)
    );
    let edited = renamed_header_graph(source.context.header.as_ref(), NamingNodeIdentity(2));
    assert_ne!(
        edited.to_bytes().unwrap(),
        source.context.header.to_bytes().unwrap()
    );
}
