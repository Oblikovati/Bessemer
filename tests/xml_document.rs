//! Complete document edits and deterministic reconstruction through the shipped example.
#[path = "support/xml.rs"]
mod support;
use bessemer::{Document, metadata::XmlMetadata};
use support::*;
const PART: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");

#[test]
fn property_edits_reopen_without_changing_other_streams_and_revert_the_entire_document() {
    let mut document = Document::from_bytes(PART).unwrap();
    let name = "docProps/ISolidWorksInformation.xml";
    let original = document.read_stream(name).unwrap();
    let metadata = parse(name, &original);
    let leaf = metadata
        .elements()
        .find(|element| element.children().len() == 0 && !element.direct_text().is_empty())
        .unwrap();
    let edited = metadata
        .with_leaf_text(&leaf.id(), "Bessemer <MOD-DIAM> λ", LIMITS)
        .unwrap();
    document
        .replace_stream(name, edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let decoded = parse(name, &reopened.read_stream(name).unwrap());
    assert_eq!(
        decoded.element_at(leaf.ordinal()).unwrap().direct_text(),
        "Bessemer <MOD-DIAM> λ"
    );
    for entry in document.streams().filter(|entry| entry.name != name) {
        assert_eq!(
            reopened.read_stream(entry.name).unwrap(),
            document.read_stream(entry.name).unwrap()
        );
    }
    document.replace_stream(name, original.clone()).unwrap();
    assert_eq!(document.to_bytes().unwrap(), PART);
    assert_eq!(metadata.to_bytes().unwrap(), original);
}

#[test]
fn all_owned_native_xml_streams_force_rebuild_to_the_identical_document() {
    let mut document = Document::from_bytes(PART).unwrap();
    let names: Vec<String> = document
        .streams()
        .filter(|entry| bessemer::metadata::MetadataStream::parse(entry.name).is_ok())
        .map(|entry| entry.name.to_owned())
        .collect();
    assert!(!names.is_empty());
    for name in names {
        let metadata = parse(&name, &document.read_stream(&name).unwrap());
        let rebuilt =
            XmlMetadata::new(metadata.stream().clone(), metadata.source().clone(), LIMITS).unwrap();
        document
            .replace_stream(&name, rebuilt.to_bytes().unwrap())
            .unwrap();
    }
    assert_eq!(document.to_bytes().unwrap(), PART);
}
