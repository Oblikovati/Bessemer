use bessemer::composer::geometry::{
    ComposerGeometryArchive, ComposerSceneContext, ComposerSceneLimits,
};
use bessemer::composer::links::*;
use bessemer::metadata::{ExpandedName, XmlAttribute, XmlLimits};
use std::io::{Cursor, Read};

pub const LIMITS: ComposerLinkLimits = ComposerLinkLimits {
    xml: XmlLimits {
        maximum_bytes: 100000000,
        maximum_nodes: 1000000,
        maximum_attributes: 2000000,
        maximum_depth: 1000,
    },
    maximum_geodes: 10000,
    maximum_references: 100000,
    maximum_path_units: 1000000,
};

pub struct LinkCase<'a> {
    pub scene: &'a [u8],
    pub xml: &'a [u8],
    pub oracle: &'a [u8],
}

pub fn link_cases(raw: &[u8]) -> Vec<LinkCase<'_>> {
    let mut reader = Cursor::new(raw);
    assert_eq!(span(&mut reader, 4), b"XLNK");
    let cases = (0..word(&mut reader))
        .map(|_| read_case(&mut reader))
        .collect();
    assert_eq!(reader.position() as usize, raw.len());
    cases
}

fn read_case<'a>(reader: &mut Cursor<&'a [u8]>) -> LinkCase<'a> {
    let (scene, xml, oracle) = (word(reader), word(reader), word(reader));
    LinkCase {
        scene: span(reader, scene as usize),
        xml: span(reader, xml as usize),
        oracle: span(reader, oracle as usize),
    }
}

fn word(reader: &mut Cursor<&[u8]>) -> u32 {
    let mut raw = [0; 4];
    reader.read_exact(&mut raw).unwrap();
    u32::from_le_bytes(raw)
}

fn span<'a>(reader: &mut Cursor<&'a [u8]>, count: usize) -> &'a [u8] {
    let input = *reader.get_ref();
    let start = reader.position() as usize;
    reader.set_position((start + count) as u64);
    &input[start..start + count]
}

pub fn scene(case: &LinkCase<'_>) -> ComposerGeometryArchive {
    let limits = ComposerSceneLimits {
        maximum_encoded_bytes: 100000000,
        maximum_decoded_bytes: 100000000,
        maximum_fields: 100000,
        maximum_records: 10000,
        maximum_text_units: 1000000,
    };
    ComposerGeometryArchive::from_decoded(
        case.scene,
        ComposerSceneContext {
            texture_import_enabled: true,
        },
        limits,
    )
    .unwrap()
}

pub fn check_case(case: &LinkCase<'_>) -> ComposerXmlLinks {
    let scene = scene(case);
    let index = ComposerXmlLinks::from_bytes(case.xml, &scene, LIMITS).unwrap();
    assert_eq!(index.to_bytes(), case.xml);
    assert_eq!(index.scene_revision(), scene.revision());
    assert_eq!(oracle(&index), case.oracle);
    index
}

fn put_word(raw: &mut Vec<u8>, value: usize) {
    raw.extend_from_slice(&u32::try_from(value).unwrap().to_le_bytes());
}

fn put_text(raw: &mut Vec<u8>, value: &str) {
    put_word(raw, value.len());
    raw.extend_from_slice(value.as_bytes());
}

fn put_numbers(raw: &mut Vec<u8>, values: &[usize]) {
    put_word(raw, values.len());
    for value in values {
        put_word(raw, *value);
    }
}

fn name(name: &ExpandedName) -> String {
    match &name.namespace {
        Some(namespace) => format!("{{{namespace}}}{}", name.local),
        None => name.local.to_string(),
    }
}

fn put_attributes<'a>(raw: &mut Vec<u8>, attributes: impl Iterator<Item = &'a XmlAttribute>) {
    let mut pairs = attributes
        .map(|attribute| (name(&attribute.name), &attribute.value))
        .collect::<Vec<_>>();
    pairs.sort();
    put_word(raw, pairs.len());
    for (key, value) in pairs {
        put_text(raw, &key);
        put_text(raw, value);
    }
}

fn put_geodes(raw: &mut Vec<u8>, geodes: &[ComposerGeodeId]) {
    put_numbers(
        raw,
        &geodes.iter().map(|id| id.entry_index()).collect::<Vec<_>>(),
    );
}

pub fn oracle(index: &ComposerXmlLinks) -> Vec<u8> {
    let mut raw = Vec::new();
    put_attributes(&mut raw, index.root_attributes());
    put_word(&mut raw, index.guid_groups().len());
    for group in index.guid_groups() {
        put_word(&mut raw, group.identifier.len());
        for unit in group.identifier.iter() {
            raw.extend_from_slice(&unit.to_le_bytes());
        }
        put_geodes(&mut raw, &group.candidates);
    }
    put_geodes(&mut raw, index.unkeyed_geodes());
    put_geodes(&mut raw, index.unreferenced_geodes());
    put_word(&mut raw, index.references().len());
    for reference in index.references() {
        put_reference(&mut raw, reference);
    }
    raw
}

fn put_reference(raw: &mut Vec<u8>, reference: &ComposerXmlReference) {
    put_numbers(raw, &reference.element_path);
    put_text(raw, &name(&reference.name));
    put_attributes(raw, reference.attributes.iter());
    put_text(raw, &reference.identifier);
    let status = match reference.status {
        ComposerLinkStatus::Empty => "empty",
        ComposerLinkStatus::Unmatched => "unmatched",
        ComposerLinkStatus::Unique => "unique",
        ComposerLinkStatus::Ambiguous => "ambiguous",
    };
    put_text(raw, status);
    put_geodes(raw, &reference.candidates);
}
