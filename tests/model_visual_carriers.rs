//! Complete visual callers against owned original-reader byte and word oracles (#181).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use bessemer::tessellation::StoredFloat;

#[path = "model_visual_carriers/edits.rs"]
mod edits;
#[path = "model_visual_carriers/typed_state.rs"]
mod typed_state;
#[path = "model_visual_carriers/word_edits.rs"]
mod word_edits;

const VISUAL_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

struct VisualCarrierCorpus {
    packet: &'static [u8],
    words: &'static str,
    suffix: &'static str,
    final_slots: usize,
}

fn visual_model_context() -> (PartConfigurationArchive, Vec<u8>) {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        VISUAL_LIMITS,
    )
    .unwrap();
    (source, document.read_stream("Contents/CMgr").unwrap())
}

fn visual_case_bytes(cursor: &mut &'static [u8], columns: &[&str]) -> &'static [u8] {
    assert_eq!(columns.len(), 11);
    let length = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
    let raw = &cursor[4..4 + length];
    *cursor = &cursor[4 + length..];
    assert_eq!(length, columns[10].parse::<usize>().unwrap());
    raw
}

fn visual_literal_values(raw: &[u8]) -> [ModelFieldValue; 7] {
    [
        ModelFieldValue::U16(u16::from_le_bytes(raw[..2].try_into().unwrap())),
        ModelFieldValue::U16(u16::from_le_bytes(raw[2..4].try_into().unwrap())),
        ModelFieldValue::U8(raw[4]),
        ModelFieldValue::U32(u32::from_le_bytes(raw[5..9].try_into().unwrap())),
        ModelFieldValue::U16(u16::from_le_bytes(raw[9..11].try_into().unwrap())),
        ModelFieldValue::I16(i16::from_le_bytes(raw[11..13].try_into().unwrap())),
        ModelFieldValue::F32(StoredFloat::from_bits(u32::from_le_bytes(
            raw[13..17].try_into().unwrap(),
        ))),
    ]
}

fn visual_oracle_values(columns: &[&str]) -> [ModelFieldValue; 7] {
    [
        ModelFieldValue::U16(columns[1].parse().unwrap()),
        ModelFieldValue::U16(columns[2].parse().unwrap()),
        ModelFieldValue::U8(columns[3].parse().unwrap()),
        ModelFieldValue::U32(columns[4].parse().unwrap()),
        ModelFieldValue::U16(columns[5].parse().unwrap()),
        ModelFieldValue::I16(columns[6].parse().unwrap()),
        ModelFieldValue::F32(StoredFloat::from_bits(columns[7].parse().unwrap())),
    ]
}

fn verify_visual_groups(
    archive: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) {
    for (ordinal, index) in visual_group_positions(archive.fields(), suffix)
        .into_iter()
        .enumerate()
    {
        let start = columns[8 + ordinal].parse::<usize>().unwrap();
        let expected = visual_literal_values(&raw[start..start + 17]);
        if ordinal == 0 {
            assert_eq!(expected, visual_oracle_values(columns));
        }
        for (actual, value) in archive.fields()[index..index + 7].iter().zip(expected) {
            assert_eq!(actual.value, value, "{}", actual.path);
        }
    }
}

fn visual_group_positions(fields: &[ModelField], suffix: &str) -> Vec<usize> {
    let groups: Vec<_> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(suffix))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(groups.len(), 2);
    groups
}

fn verify_visual_reconstruction(archive: &PartResolvedFeaturesArchive, raw: &[u8]) {
    let reconstructed = archive.edit_fields(archive.fields().to_vec()).unwrap();
    assert_eq!(reconstructed.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(reconstructed.archive().to_bytes().unwrap(), raw);
    assert!(reconstructed.changes().is_empty());
}

fn verify_visual_carrier_corpus(corpus: VisualCarrierCorpus) {
    let (source, manager) = visual_model_context();
    let mut cursor = corpus.packet;
    let mut count = 0;
    for line in corpus.words.lines() {
        let columns: Vec<_> = line.split('\t').collect();
        let raw = visual_case_bytes(&mut cursor, &columns);
        let archive =
            PartResolvedFeaturesArchive::from_bytes(raw, source.clone(), &manager, VISUAL_LIMITS)
                .unwrap();
        assert_eq!(archive.object_slots(), (97, corpus.final_slots));
        assert_eq!(archive.root_count(), 3);
        verify_visual_groups(&archive, raw, &columns, corpus.suffix);
        verify_visual_reconstruction(&archive, raw);
        edits::verify_visual_rgb_writes(&archive, raw, &columns, corpus.suffix);
        word_edits::verify_visual_word_writes(&archive, raw, &columns, corpus.suffix);
        typed_state::verify_typed_visual_states(&archive, raw, &columns, corpus.suffix);
        count += 1;
    }
    assert_eq!(count, 3831);
    assert!(cursor.is_empty());
}

fn folder_visual_corpus() -> VisualCarrierCorpus {
    VisualCarrierCorpus {
        packet: include_bytes!("fixtures/entity-visual-state/model.bin"),
        words: include_str!("fixtures/entity-visual-state/model-words.tsv"),
        suffix: ".entity_visual_properties.unknown_0",
        final_slots: 100,
    }
}

fn sketch_visual_corpus() -> VisualCarrierCorpus {
    VisualCarrierCorpus {
        packet: include_bytes!("fixtures/entity-visual-state/sketch.bin"),
        words: include_str!("fixtures/entity-visual-state/sketch-words.tsv"),
        suffix: ".visual_properties.unknown_0",
        final_slots: 100,
    }
}

fn nested_display_visual_corpus() -> VisualCarrierCorpus {
    VisualCarrierCorpus {
        packet: include_bytes!("fixtures/entity-visual-state/display.bin"),
        words: include_str!("fixtures/entity-visual-state/display-words.tsv"),
        suffix: ".visual_properties.unknown_0",
        final_slots: 108,
    }
}

#[test]
fn complete_folder_visuals_match_literal_fields_and_forced_reconstruction() {
    verify_visual_carrier_corpus(folder_visual_corpus());
}

#[test]
fn complete_sketch_visuals_match_literal_fields_and_forced_reconstruction() {
    verify_visual_carrier_corpus(sketch_visual_corpus());
}

#[test]
fn complete_nested_display_visuals_match_literal_fields_and_forced_reconstruction() {
    verify_visual_carrier_corpus(nested_display_visual_corpus());
}
