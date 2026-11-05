//! Complete Source-bound configuration visual records and isolated operations (#181).
use bessemer::entity_visual::{EntityVisualStoredEdit, EntityVisualStoredWords};
use bessemer::model_archives::*;
use bessemer::{Document, header::ArchiveVersion, tessellation::StoredFloat};
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const PART_OFFSETS: [usize; 5] = [20714, 21017, 21278, 23534, 23907];
const ASSEMBLY_OFFSETS: [usize; 5] = [20099, 20402, 20663, 22895, 23213];

fn configuration_document(part: bool) -> Document {
    let bytes: &[u8] = if part {
        include_bytes!("fixtures/blank-part.SLDPRT")
    } else {
        include_bytes!("fixtures/blank-assembly.SLDASM")
    };
    Document::from_bytes(bytes).unwrap()
}

fn part_visual_configuration(document: &Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn assembly_visual_configuration(document: &Document) -> AssemblyConfigurationArchive {
    AssemblyConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        AssemblyRootKind::Assembly,
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

fn literal_configuration_words(raw: &[u8]) -> EntityVisualStoredWords {
    EntityVisualStoredWords {
        layer_number: u16::from_le_bytes(raw[..2].try_into().unwrap()),
        by_layer_flags: u16::from_le_bytes(raw[2..4].try_into().unwrap()),
        layer_attribute_cache: raw[4],
        color_word: u32::from_le_bytes(raw[5..9].try_into().unwrap()),
        line_style: u16::from_le_bytes(raw[9..11].try_into().unwrap()),
        standard_width: i16::from_le_bytes(raw[11..13].try_into().unwrap()),
        custom_width: Some(StoredFloat::from_bits(u32::from_le_bytes(
            raw[13..17].try_into().unwrap(),
        ))),
    }
}

fn expected_configuration_rgb(raw: &[u8], offset: usize) -> (EntityVisualStoredEdit, Vec<u8>) {
    let mut expected = raw.to_vec();
    let color = u32::from_le_bytes(raw[offset + 5..offset + 9].try_into().unwrap()) ^ 0x00a53cf0;
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    let request =
        EntityVisualStoredEdit::ColorRgb([color as u8, (color >> 8) as u8, (color >> 16) as u8]);
    (request, expected)
}

fn reopened_configuration_document(original: &Document, changed: &[u8]) -> Document {
    let mut output = original.clone();
    output
        .replace_stream("Contents/Config-0", changed.to_vec())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/Config-0")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            original.read_stream(stream.name).unwrap()
        );
    }
    let raw = original.read_stream("Contents/Config-0").unwrap();
    // Authentic compressed-byte reversal belongs to the retained original checkpoint.
    output.replace_stream("Contents/Config-0", raw).unwrap();
    assert!(output.to_bytes().unwrap() == original.to_bytes().unwrap());
    reopened
}

macro_rules! configuration_visual_test {
    ($name:ident, $loader:ident, $document:expr, $offsets:expr) => {
        #[test]
        fn $name() {
            let document = $document;
            let source = $loader(&document);
            let raw = document.read_stream("Contents/Config-0").unwrap();
            assert_eq!(source.to_bytes().unwrap(), raw);
            let states = source.entity_visual_states().unwrap();
            assert_eq!(states.len(), 5);
            for (state, offset) in states.iter().zip($offsets) {
                assert_eq!(
                    state.words(),
                    literal_configuration_words(&raw[offset..offset + 17])
                );
                let (request, expected) = expected_configuration_rgb(&raw, offset);
                let changed = source.edit_entity_visual_state(state, request).unwrap();
                verify_configuration_result(&source, &changed, state, &expected);
                let reopened = reopened_configuration_document(&document, &expected);
                assert_eq!($loader(&reopened).to_bytes().unwrap(), expected);
                let noop = source
                    .edit_entity_visual_state(
                        state,
                        EntityVisualStoredEdit::ColorWord(state.words().color_word),
                    )
                    .unwrap();
                assert_eq!(noop.completion(), ModelEditCompletion::Unchanged);
                assert!(noop.changes().is_empty());
            }
        }
    };
}

macro_rules! configuration_result_check {
    ($name:ident, $archive:ty, $outcome:ty) => {
        fn $name(
            source: &$archive,
            changed: &$outcome,
            state: &ModelEntityVisualStoredState,
            expected: &[u8],
        ) {
            assert_eq!(changed.archive().to_bytes().unwrap(), expected);
            assert_eq!(changed.completion(), ModelEditCompletion::Changed);
            assert_eq!(changed.changes().len(), 1);
            let change = &changed.changes()[0];
            assert_eq!(change.parents().len(), 1);
            assert!(source.resolve_field(&change.parents()[0]).is_ok());
            assert!(
                changed
                    .archive()
                    .resolve_field(change.result().unwrap())
                    .is_ok()
            );
            assert!(
                changed
                    .archive()
                    .entity_visual_state(state.source_field())
                    .is_err()
            );
            let reverted = changed
                .archive()
                .edit_fields(source.fields().to_vec())
                .unwrap();
            assert_eq!(
                reverted.archive().to_bytes().unwrap(),
                source.to_bytes().unwrap()
            );
        }
    };
}

mod part {
    use super::*;
    configuration_result_check!(
        verify_configuration_result,
        PartConfigurationArchive,
        PartConfigurationEdit
    );
    configuration_visual_test!(
        complete_part_visuals_match_source_spans_and_document_reversals,
        part_visual_configuration,
        configuration_document(true),
        PART_OFFSETS
    );
}
mod assembly {
    use super::*;
    configuration_result_check!(
        verify_configuration_result,
        AssemblyConfigurationArchive,
        AssemblyConfigurationEdit
    );
    configuration_visual_test!(
        complete_assembly_visuals_match_source_spans_and_document_reversals,
        assembly_visual_configuration,
        configuration_document(false),
        ASSEMBLY_OFFSETS
    );
}

#[test]
fn configuration_visual_states_cannot_cross_archive_families_or_decodes() {
    let part = part_visual_configuration(&configuration_document(true));
    let assembly = assembly_visual_configuration(&configuration_document(false));
    let first = part.entity_visual_states().unwrap();
    assert!(
        assembly
            .entity_visual_state(first[0].source_field())
            .is_err()
    );
    assert!(
        assembly
            .edit_entity_visual_state(&first[0], EntityVisualStoredEdit::ColorWord(0))
            .is_err()
    );
    let other = part_visual_configuration(&configuration_document(true));
    assert!(other.entity_visual_state(first[0].source_field()).is_err());
    assert!(
        other
            .edit_entity_visual_state(&first[0], EntityVisualStoredEdit::ColorWord(0))
            .is_err()
    );
}

fn routing_visual_document() -> Document {
    let mut document = configuration_document(false);
    let raw = include_bytes!("fixtures/assembly-configurations/owned-routing.bin");
    document
        .replace_stream("Contents/Config-0", raw.to_vec())
        .unwrap();
    document
}

fn routing_visual_configuration(document: &Document) -> AssemblyConfigurationArchive {
    AssemblyConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        AssemblyRootKind::RoutingAssembly,
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

mod routing {
    use super::*;
    const OFFSETS: [usize; 5] = [20104, 20407, 20668, 22900, 23218];
    configuration_result_check!(
        verify_configuration_result,
        AssemblyConfigurationArchive,
        AssemblyConfigurationEdit
    );
    configuration_visual_test!(
        complete_routing_visuals_match_source_spans_and_document_reversals,
        routing_visual_configuration,
        routing_visual_document(),
        OFFSETS
    );
}
