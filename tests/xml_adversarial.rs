//! XML malformed input, physical framing, namespace, resource and immutable edit boundaries.
#[path = "support/xml.rs"]
mod support;
use bessemer::metadata::*;
use support::*;

#[test]
fn truncations_fail_until_the_complete_document_boundary_and_final_whitespace_is_valid() {
    let closed = PROPERTIES
        .windows(b"</c:Properties>".len())
        .rposition(|span| span == b"</c:Properties>")
        .unwrap()
        + b"</c:Properties>".len();
    for end in 0..closed {
        assert!(
            XmlMetadata::from_bytes(stream("docProps/custom.xml"), &PROPERTIES[..end], LIMITS)
                .is_err(),
            "{end}"
        );
    }
    for end in closed..=PROPERTIES.len() {
        assert!(
            XmlMetadata::from_bytes(stream("docProps/custom.xml"), &PROPERTIES[..end], LIMITS)
                .is_ok(),
            "{end}"
        );
    }
    for bytes in [
        b"\xff\xfe\x00\xd8".as_slice(),
        b"\xfe\xff\xdc\x00",
        b"\xff\xfe\0",
        b"\xff",
        b"\xef\xbb\xbf",
    ] {
        assert!(
            XmlMetadata::from_bytes(stream("swXmlContents/Tables"), bytes, LIMITS).is_err(),
            "{bytes:?}"
        );
    }
}

#[test]
fn malformed_namespaces_entities_roots_and_multiple_documents_are_typed_failures() {
    for text in [
        "<Tables><x></Tables>",
        "<Tables/><Tables/>",
        "<Tables>&missing;</Tables>",
        "<Tables><p:x/></Tables>",
        "<Tables a='1' a='2'/>",
        "<Tables xmlns:a='urn:same' xmlns:b='urn:same' a:x='1' b:x='2'/>",
        "<Tables>&#0;</Tables>",
        "<Tables>\0</Tables>",
        "<Tables>\u{1}</Tables>",
        "<Tables xmlns:xml='urn:wrong'/>",
        "<Tables xmlns:xmlns='urn:wrong'/>",
    ] {
        assert!(
            XmlMetadata::new(stream("swXmlContents/Tables"), source(text), LIMITS).is_err(),
            "{text:?}"
        );
    }
    let error = XmlMetadata::new(
        stream("swXmlContents/Tables"),
        source("<NotTables/>"),
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    let external = "<!DOCTYPE Tables [<!ENTITY external SYSTEM 'file:///this-file-must-never-be-read'>]><Tables>&external;</Tables>";
    assert_eq!(
        XmlMetadata::new(stream("swXmlContents/Tables"), source(external), LIMITS)
            .unwrap_err()
            .class,
        bessemer::FailureClass::Unsupported
    );
}

#[test]
fn source_context_and_encoding_declarations_cannot_be_silently_reinterpreted() {
    for name in [
        "PreviewPNG",
        "docProps/Config-N-Properties.xml",
        "docProps/Config--1-Properties.xml",
        "docProps/Config-4294967296-Properties.xml",
        "swXmlContents/xmlstream-",
        "docProps/Config-1x-Cutlist-Properties.xml",
    ] {
        assert!(MetadataStream::parse(name).is_err(), "{name}");
    }
    for text in [
        "<?xml version='1.0' encoding='UTF-16'?><Tables/>",
        "<?xml version='1.0' encoding='shift_jis'?><Tables/>",
    ] {
        assert!(XmlMetadata::new(stream("swXmlContents/Tables"), source(text), LIMITS).is_err());
    }
    let mut framed = source("<Tables/>");
    framed.format.keyword_prefix = Some(0x85);
    assert!(XmlMetadata::new(stream("swXmlContents/Tables"), framed.clone(), LIMITS).is_err());
    assert!(XmlMetadata::new(stream("swXmlContents/KeyWords"), framed, LIMITS).is_err());
    let mut empty = source("<Tables/>");
    empty.document = None;
    empty.format.bom = true;
    assert!(XmlMetadata::new(stream("swXmlContents/Tables"), empty, LIMITS).is_err());
    assert!(XmlMetadata::from_bytes(stream("docProps/custom.xml"), &[], LIMITS).is_err());
    let stylesheet = tables("<?xml-stylesheet encoding='shift_jis'?><Tables/>");
    assert!(stylesheet.root().is_some());
}

#[test]
fn every_budget_rejects_insufficient_limits_and_equivalent_edits_recheck_them() {
    let metadata = properties();
    for limits in [
        XmlLimits {
            maximum_bytes: 10,
            ..LIMITS
        },
        XmlLimits {
            maximum_nodes: 1,
            ..LIMITS
        },
        XmlLimits {
            maximum_attributes: 0,
            ..LIMITS
        },
        XmlLimits {
            maximum_depth: 1,
            ..LIMITS
        },
    ] {
        assert!(
            XmlMetadata::from_bytes(stream("docProps/custom.xml"), PROPERTIES, limits).is_err()
        );
        assert!(
            metadata
                .with_source(metadata.source().clone(), limits)
                .is_err()
        );
    }
    let mut utf16 = source("<Tables>é</Tables>");
    utf16.format.encoding = XmlEncoding::Utf16Le;
    utf16.format.bom = true;
    let maximum = utf16.document.as_ref().unwrap().len();
    assert!(
        XmlMetadata::new(
            stream("swXmlContents/Tables"),
            utf16,
            XmlLimits {
                maximum_bytes: maximum,
                ..LIMITS
            }
        )
        .is_err()
    );
}

#[test]
fn edits_reject_child_elements_invalid_characters_missing_attributes_and_stale_identities() {
    let metadata = properties();
    let root = metadata.root().unwrap();
    assert!(
        metadata
            .with_leaf_text(&root.id(), "discard", LIMITS)
            .is_err()
    );
    assert!(metadata.element_at(usize::MAX).is_err());
    assert!(
        metadata
            .with_attribute_value(&root.id(), None, "missing", "x", LIMITS)
            .is_err()
    );
    let leaf = metadata
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
    for invalid in ['\0', '\u{1}', '\u{fffe}', '\u{ffff}'] {
        assert!(
            metadata
                .with_leaf_text(&leaf.id(), &invalid.to_string(), LIMITS)
                .is_err()
        );
    }
    let changed = metadata
        .with_leaf_text(&leaf.id(), "changed", LIMITS)
        .unwrap();
    assert!(changed.element(&leaf.id()).is_err());
    let other = properties();
    assert!(metadata.element(&other.root().unwrap().id()).is_err());
    assert_eq!(metadata.to_bytes().unwrap(), PROPERTIES);
}

#[test]
fn deep_element_graphs_parse_edit_and_drop_without_recursive_owned_trees() {
    let text = format!(
        "<Tables>{}text{}</Tables>",
        "<x>".repeat(5000),
        "</x>".repeat(5000)
    );
    let metadata = tables(&text);
    assert_eq!(metadata.elements().len(), 5001);
    let leaf = metadata.element_at(5000).unwrap();
    assert_eq!(leaf.direct_text(), "text");
    let changed = metadata
        .with_leaf_text(&leaf.id(), "changed", LIMITS)
        .unwrap();
    assert_eq!(changed.element_at(5000).unwrap().direct_text(), "changed");
}

#[test]
fn normalized_namespace_bindings_keep_ordinary_xmlns_attributes_and_reserved_rules() {
    let metadata = tables(
        "<Tables xmlns:p='urn:test' p:xmlns='ordinary'><p:child xmlns:q='urn:te&#115;t' q:value='x'/></Tables>",
    );
    assert_eq!(
        metadata
            .root()
            .unwrap()
            .attribute(Some("urn:test"), "xmlns"),
        Some("ordinary")
    );
    let child = metadata.element_at(1).unwrap();
    assert_eq!(child.name().namespace.as_deref(), Some("urn:test"));
    assert_eq!(child.attribute(Some("urn:test"), "value"), Some("x"));
    for text in [
        "<Tables xmlns:xmlns='urn:wrong'/>",
        "<Tables xmlns:p='http://www.w3.org/XML/1998/namespac&#101;'/>",
        "<Tables xmlns:p='urn:a' xmlns:q='urn:&#97;' p:x='a' q:x='b'/>",
        "<Tables><1bad/></Tables>",
        "<Tables><a:b:c/></Tables>",
        "<?xml encoding='UTF-8' version='1.0'?><Tables/>",
        "<Tables>]]></Tables>",
        "<Tables>&#0;</Tables>",
        "<Tables/><?XML bad?>",
        "<Tables/><!--bad--->",
        "<Tables/><![CDATA[]]>",
        "<Tables/>&#32;",
    ] {
        assert!(
            XmlMetadata::new(stream("swXmlContents/Tables"), source(text), LIMITS).is_err(),
            "{text}"
        );
    }
    let unicode = tables("<Tables><é:鋼 xmlns:é='urn:unicode' é:名='x'/></Tables>");
    assert_eq!(unicode.element_at(1).unwrap().name().local.as_ref(), "鋼");
}

#[test]
fn declaration_grammar_and_xml_normalization_preserve_character_reference_distinctions() {
    for declaration in [
        "<?xml version='1.0'?>",
        "<?xml version='1.0' standalone='yes'?>",
        "<?xml version='1.0' encoding='UTF-8' standalone='no'?>",
    ] {
        assert!(
            XmlMetadata::new(
                stream("swXmlContents/Tables"),
                source(&format!("{declaration}<Tables/>")),
                LIMITS
            )
            .is_ok()
        );
    }
    for text in [
        "<?xml version='1.0' standalone='maybe'?><Tables/>",
        "<?xml version='1.1'?><Tables/>",
        "<?xml version='1.0' encoding='1bad'?><Tables/>",
        "<?xml version='1.0' future='x'?><Tables/>",
        "<Tables xmlns:p=''/>",
        "<Tables a='literal<value'/>",
        "\u{feff}<Tables/>",
    ] {
        assert!(
            XmlMetadata::new(stream("swXmlContents/Tables"), source(text), LIMITS).is_err(),
            "{text}"
        );
    }
    let parsed = tables(
        "<Tables a='A\r\nB\tC&#13;&#10;&#9;'>A\r\nB\rC&#13;&#10;&#9;<![CDATA[&lt;\r\nD]]></Tables>",
    );
    let root = parsed.root().unwrap();
    assert_eq!(root.attribute(None, "a"), Some("A B C\r\n\t"));
    assert_eq!(root.direct_text(), "A\nB\nC\r\n\t&lt;\nD");
}
