use super::*;

const CAPTURE_LIMITS: XmlLimits = XmlLimits {
    maximum_bytes: 1024,
    maximum_nodes: 100,
    maximum_attributes: 10,
    maximum_depth: 10,
};
const CAPTURE_XML: &[u8] = b"<r xmlns:x='urn:x'>a<x:s>b<![CDATA[c]]></x:s>d<!--c--><?p i?></r>";

#[test]
fn default_facade_and_validation_skip_ordered_capture() {
    let xml = Utf8XmlDocument::from_bytes_for(CAPTURE_XML, CAPTURE_LIMITS, "test XML").unwrap();
    assert!(xml.ordered_view().is_none());
    let model =
        reader::parse_document(std::str::from_utf8(CAPTURE_XML).unwrap(), CAPTURE_LIMITS).unwrap();
    assert!(model.ordered.is_none());
    assert_eq!(model.elements.len(), 2);
}

#[test]
fn opt_in_capture_preserves_default_element_projection_and_qualified_text_order() {
    let ordinary =
        Utf8XmlDocument::from_bytes_for(CAPTURE_XML, CAPTURE_LIMITS, "test XML").unwrap();
    let captured =
        Utf8XmlDocument::from_bytes_ordered(CAPTURE_XML, CAPTURE_LIMITS, "test XML").unwrap();
    assert_eq!(ordinary.bytes(), captured.bytes());
    assert_eq!(ordinary.element_count(), captured.element_count());
    for ordinal in 0..ordinary.element_count() {
        assert_eq!(ordinary.name(ordinal), captured.name(ordinal));
        assert_eq!(ordinary.parent(ordinal), captured.parent(ordinal));
        assert_eq!(ordinary.children(ordinal), captured.children(ordinal));
        assert_eq!(
            ordinary.attributes(ordinal).collect::<Vec<_>>(),
            captured.attributes(ordinal).collect::<Vec<_>>()
        );
    }
    let view = captured.ordered_view().unwrap();
    assert_eq!(view.qualified_name(1), "x:s");
    assert_eq!(view.descendant_text(0), "abcd");
    assert_eq!(view.content(0).len(), 5);
}
