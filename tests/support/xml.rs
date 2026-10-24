// Each integration crate uses a different subset of these independently authored documents.
#![allow(dead_code)]
use bessemer::metadata::*;
pub const PROPERTIES: &[u8] = include_bytes!("../fixtures/xml/properties-utf8.bin");
pub const LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 2 * 1024 * 1024,
    maximum_nodes: 100_000,
    maximum_attributes: 100_000,
    maximum_depth: 10_000,
};

pub fn stream(name: &str) -> MetadataStream {
    MetadataStream::parse(name).unwrap()
}
pub fn parse(name: &str, bytes: &[u8]) -> XmlMetadata {
    XmlMetadata::from_bytes(stream(name), bytes, LIMITS).unwrap()
}
pub fn properties() -> XmlMetadata {
    parse("docProps/custom.xml", PROPERTIES)
}
pub fn plain() -> XmlFormat {
    XmlFormat {
        encoding: XmlEncoding::Utf8,
        bom: false,
        keyword_prefix: None,
    }
}
pub fn source(text: &str) -> XmlSource {
    XmlSource {
        format: plain(),
        document: Some(text.into()),
    }
}
pub fn tables(text: &str) -> XmlMetadata {
    XmlMetadata::new(stream("swXmlContents/Tables"), source(text), LIMITS).unwrap()
}
pub fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/xml")
            .join(name),
    )
    .unwrap()
}
