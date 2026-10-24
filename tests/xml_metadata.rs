//! Independent XML framing, namespace, lexical preservation and complete section/value expectations.
#[path = "support/xml.rs"]
mod support;
use bessemer::metadata::*;
use support::*;

#[test]
fn every_documented_stream_family_has_a_complete_independent_root_vector() {
    for row in include_str!("fixtures/xml/families.tsv").lines() {
        let (name, file) = row.split_once('\t').unwrap();
        let bytes = fixture(file);
        let metadata = parse(name, &bytes);
        assert_eq!(metadata.stream().name(), name);
        assert_eq!(metadata.elements().len(), 1);
        assert_eq!(
            metadata.root().unwrap().raw_source(),
            std::str::from_utf8(&bytes).unwrap()
        );
        assert_eq!(
            XmlMetadata::new(stream(name), metadata.source().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
    assert_eq!(
        stream("docProps/Config-001-Cutlist-Properties.xml").family(),
        MetadataFamily::CutlistProperties(1)
    );
    assert_eq!(
        stream("docProps/Config-0002-Properties.xml").family(),
        MetadataFamily::ConfigurationProperties(2)
    );
}

#[test]
fn all_unicode_bom_and_keyword_frames_rebuild_without_the_original_buffer() {
    for file in [
        "properties-utf8.bin",
        "properties-utf8-bom.bin",
        "properties-utf-16le-bom.bin",
        "properties-utf-16be-bom.bin",
        "properties-utf-16le.bin",
        "properties-utf-16be.bin",
    ] {
        let bytes = fixture(file);
        let metadata = parse("docProps/custom.xml", &bytes);
        assert_eq!(metadata.property_sections().count(), 2, "{file}");
        assert_eq!(
            XmlMetadata::new(metadata.stream().clone(), metadata.source().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes,
            "{file}"
        );
        assert_eq!(
            metadata
                .with_source(metadata.source().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
    for prefix in [0x85, 0x86] {
        let bytes = fixture(&format!("keywords-{prefix:02x}.bin"));
        let metadata = parse("swXmlContents/KeyWords", &bytes);
        assert_eq!(metadata.source().format.keyword_prefix, Some(prefix));
        assert_eq!(
            metadata.element_at(1).unwrap().direct_text(),
            "<MOD-DIAM> λ"
        );
        assert_eq!(
            XmlMetadata::new(metadata.stream().clone(), metadata.source().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
    let bytes = fixture("material-utf16.bin");
    let metadata = parse("swXmlContents/MATERIALTREE", &bytes);
    assert_eq!(metadata.element_at(1).unwrap().direct_text(), "鋼");
    assert_eq!(
        XmlMetadata::new(metadata.stream().clone(), metadata.source().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
}

#[test]
fn explicit_namespace_resets_are_unqualified_and_properties_remain_section_scoped() {
    let metadata = properties();
    let sections: Vec<_> = metadata.property_sections().collect();
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].element().name().namespace, None);
    assert_eq!(
        sections[0].element().attribute(None, "fmtid"),
        Some("section-one")
    );
    let first = sections[0].properties().next().unwrap();
    let second = sections[1].properties().next().unwrap();
    assert_eq!(first.element().attribute(None, "pid"), Some("20"));
    assert_eq!(second.element().attribute(None, "pid"), Some("20"));
    assert_eq!(first.element().attribute(None, "name"), Some(""));
    assert_eq!(first.element().attribute(None, "IsEquation"), Some("No"));
    let aliases: Vec<_> = sections[0]
        .dictionary_entries()
        .map(|entry| entry.attribute(None, "name").unwrap())
        .collect();
    assert_eq!(aliases, ["", "SW-Author", "SW-Author(Author)"]);
    assert_eq!(
        second.values().next().unwrap().element().direct_text(),
        "second section"
    );
    let kinds: Vec<_> = first.values().map(|value| value.kind()).collect();
    assert_eq!(
        kinds,
        [
            PropertyValueKind::LpStr,
            PropertyValueKind::FpVals,
            PropertyValueKind::Blob,
            PropertyValueKind::Bool,
            PropertyValueKind::Empty,
            PropertyValueKind::I2,
            PropertyValueKind::R8,
            PropertyValueKind::Other
        ]
    );
    let nested: Vec<_> = first
        .values()
        .nth(1)
        .unwrap()
        .children()
        .map(|value| (value.kind(), value.element().direct_text()))
        .collect();
    assert_eq!(
        nested,
        [
            (PropertyValueKind::I4, "-0002"),
            (PropertyValueKind::LpStr, "fallback")
        ]
    );
    assert_eq!(first.values().nth(3).unwrap().element().direct_text(), "No");
    assert_eq!(
        first.values().nth(6).unwrap().element().direct_text(),
        "1,5"
    );
}

#[test]
fn leaf_text_edits_preserve_comments_pi_siblings_and_literal_display_markup() {
    let metadata = parse("swXmlContents/Tables", &fixture("leaf.xml"));
    let root = metadata.root().unwrap();
    assert_eq!(root.direct_text(), "AB & <CD");
    let changed = metadata
        .with_leaf_text(&root.id(), "x<y & z\rλ", LIMITS)
        .unwrap();
    assert_eq!(changed.root().unwrap().direct_text(), "x<y & z\rλ");
    assert_eq!(
        changed.source().document.as_deref(),
        Some("<Tables q='a&gt;b'>x&lt;y &amp; z&#13;λ<!--note--><?keep x='y'?></Tables>")
    );
    assert_eq!(metadata.to_bytes().unwrap(), fixture("leaf.xml"));
    let metadata = properties();
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
    let changed = metadata
        .with_leaf_text(&value.id(), "Reviewed", LIMITS)
        .unwrap();
    let property = changed
        .property_sections()
        .next()
        .unwrap()
        .properties()
        .next()
        .unwrap();
    assert_eq!(property.values().count(), 8);
    assert_eq!(property.values().nth(1).unwrap().children().count(), 2);
    assert_eq!(
        changed
            .property_sections()
            .next()
            .unwrap()
            .dictionary_entries()
            .count(),
        3
    );
}

#[test]
fn attribute_edits_keep_quotes_whitespace_and_namespaced_identity_even_beyond_packed_ranges() {
    let bytes = fixture("long-attribute.xml");
    let metadata = parse("swXmlContents/Tables", &bytes);
    let root = metadata.root().unwrap();
    let local = "X".repeat(65536);
    assert_eq!(root.attribute(Some("urn:attribute"), &local), Some("A & B"));
    let changed = metadata
        .with_attribute_value(
            &root.id(),
            Some("urn:attribute"),
            &local,
            "'\"\t\n\r<&",
            LIMITS,
        )
        .unwrap();
    assert_eq!(
        changed
            .root()
            .unwrap()
            .attribute(Some("urn:attribute"), &local),
        Some("'\"\t\n\r<&")
    );
    let expected = std::str::from_utf8(&bytes)
        .unwrap()
        .replace("'A &amp; B'", "'&apos;\"&#9;&#10;&#13;&lt;&amp;'");
    assert_eq!(
        changed.source().document.as_deref(),
        Some(expected.as_str())
    );
    assert_eq!(metadata.to_bytes().unwrap(), bytes);
}

#[test]
fn self_closing_leaf_changes_retain_prefix_and_original_tag_spacing() {
    let metadata=XmlMetadata::new(stream("docProps/custom.xml"),source("<p:Properties xmlns:p='http://schemas.openxmlformats.org/officeDocument/2006/custom-properties'  />"),LIMITS).unwrap();
    let changed = metadata
        .with_leaf_text(&metadata.root().unwrap().id(), "text", LIMITS)
        .unwrap();
    assert!(
        changed
            .source()
            .document
            .as_deref()
            .unwrap()
            .ends_with("  >text</p:Properties>")
    );
    let metadata = tables("<Tables><!--note--><?p x?></Tables>");
    let changed = metadata
        .with_leaf_text(&metadata.root().unwrap().id(), "new", LIMITS)
        .unwrap();
    assert_eq!(
        changed.source().document.as_deref(),
        Some("<Tables><!--note--><?p x?>new</Tables>")
    );
}

#[test]
fn completed_empty_streams_are_distinct_from_an_empty_root() {
    for name in [
        "swXmlContents/MATERIALTREE",
        "swXmlContents/CustomProperties",
        "swXmlContents/Tables",
    ] {
        let metadata = parse(name, &[]);
        assert!(metadata.root().is_none());
        assert!(metadata.source().document.is_none());
        assert_eq!(metadata.elements().len(), 0);
        assert!(metadata.to_bytes().unwrap().is_empty());
    }
    let root = tables("<Tables/>");
    assert!(root.root().is_some());
    assert!(!root.to_bytes().unwrap().is_empty());
    let mut empty = root.source().clone();
    empty.document = None;
    assert!(root.with_source(empty, LIMITS).unwrap().root().is_none());
}
