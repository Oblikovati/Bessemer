//! Snapshot-scoped identities, ownership, shared immutable names and concurrent XML editing.
#[path = "support/xml.rs"]
mod support;
use bessemer::metadata::*;
use support::*;

#[test]
fn every_public_metadata_type_is_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<XmlMetadata>();
    shared::<XmlSource>();
    shared::<XmlFormat>();
    shared::<XmlEncoding>();
    shared::<XmlLimits>();
    shared::<MetadataStream>();
    shared::<MetadataFamily>();
    shared::<XmlElementId>();
    shared::<ExpandedName>();
    shared::<XmlAttribute>();
    shared::<XmlElementRef<'static>>();
    shared::<PropertySection<'static>>();
    shared::<MetadataProperty<'static>>();
    shared::<PropertyValue<'static>>();
    shared::<PropertyValueKind>();
}

#[test]
fn source_bytes_owned_edits_failures_equivalence_and_authentic_reverts_remain_isolated() {
    let mut bytes = PROPERTIES.to_vec();
    let metadata = parse("docProps/custom.xml", &bytes);
    bytes.fill(0);
    let original = metadata.source().clone();
    let id = metadata.root().unwrap().id();
    let mut source = original.clone();
    source.document.as_mut().unwrap().push_str("<!--edited-->");
    let changed = metadata.with_source(source.clone(), LIMITS).unwrap();
    source.document.as_mut().unwrap().clear();
    assert!(
        changed
            .source()
            .document
            .as_ref()
            .unwrap()
            .ends_with("<!--edited-->")
    );
    assert!(metadata.element(&id).is_ok());
    assert!(changed.element(&id).is_err());
    let equivalent = metadata.with_source(original.clone(), LIMITS).unwrap();
    assert_eq!(equivalent.root().unwrap().id(), id);
    assert!(changed.with_source(source, LIMITS).is_err());
    assert_eq!(
        changed
            .with_source(original, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        PROPERTIES
    );
    assert_eq!(metadata.to_bytes().unwrap(), PROPERTIES);
}

#[test]
fn inherited_namespace_names_share_owned_storage_within_one_operation_only() {
    let metadata = properties();
    let values: Vec<_> = metadata
        .property_sections()
        .next()
        .unwrap()
        .properties()
        .next()
        .unwrap()
        .values()
        .collect();
    let first = values[0].element().name().namespace.as_ref().unwrap();
    let another = values[2].element().name().namespace.as_ref().unwrap();
    assert!(std::sync::Arc::ptr_eq(first, another));
    let independent = properties();
    let independent_value = independent
        .property_sections()
        .next()
        .unwrap()
        .properties()
        .next()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert!(!std::sync::Arc::ptr_eq(
        first,
        independent_value
            .element()
            .name()
            .namespace
            .as_ref()
            .unwrap()
    ));
}

#[test]
fn concurrent_property_edits_equal_serial_bytes_and_leave_source_unchanged() {
    let metadata = properties();
    let expected: Vec<Vec<u8>> = (0..8).map(|index| edit(&metadata, index)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|index| {
                    let metadata = &metadata;
                    scope.spawn(move || edit(metadata, index))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(metadata.to_bytes().unwrap(), PROPERTIES);
}

fn edit(metadata: &XmlMetadata, index: usize) -> Vec<u8> {
    let value = metadata
        .property_sections()
        .next()
        .unwrap()
        .properties()
        .next()
        .unwrap()
        .values()
        .next()
        .unwrap()
        .element();
    metadata
        .with_leaf_text(&value.id(), &format!("value {index} < & λ"), LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap()
}

#[test]
fn independent_processes_publish_identical_edited_bytes() {
    let first = process_oracle();
    assert_eq!(first, process_oracle());
    assert!(first.starts_with("BESSEMER_XML:"));
}

fn process_oracle() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "process_output_child", "--nocapture"])
        .env("BESSEMER_XML_PROCESS_ORACLE", "1")
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .find(|line| line.starts_with("BESSEMER_XML:"))
        .unwrap()
        .to_owned()
}

#[test]
fn process_output_child() {
    if std::env::var_os("BESSEMER_XML_PROCESS_ORACLE").is_none() {
        return;
    }
    let metadata = properties();
    let leaf = metadata
        .elements()
        .find(|element| element.children().len() == 0 && !element.direct_text().is_empty())
        .unwrap();
    let edited = metadata
        .with_leaf_text(&leaf.id(), "deterministic λ & <>", LIMITS)
        .unwrap();
    let bytes = edited.to_bytes().unwrap();
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    println!("BESSEMER_XML:{hex}");
}
