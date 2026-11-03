//! Complete modern component owners preserve literal visual words and aliases (#181).
use bessemer::display_lists::*;
use bessemer::tessellation::StoredFloat;
#[allow(
    dead_code,
    reason = "Shared helpers serve other display archive tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
#[path = "component_visual_carriers/bit_edits.rs"]
mod bit_edits;
#[allow(
    dead_code,
    reason = "The visual checks use only complete case bytes and context."
)]
#[path = "support/old_display_cases.rs"]
mod complete_cases;
#[path = "component_visual_carriers/documents.rs"]
mod documents;

fn retained_component_cases() -> Vec<complete_cases::OldCase> {
    [
        include_bytes!("fixtures/display-lists/component-properties.bin").as_slice(),
        include_bytes!("fixtures/display-lists/component-property-holders-part.bin").as_slice(),
        include_bytes!("fixtures/display-lists/component-property-holders-assembly.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(complete_cases::decode)
    .collect()
}

fn component_baseline_values() -> [DisplayFieldValue; 7] {
    [
        DisplayFieldValue::U16(65535),
        DisplayFieldValue::U16(32768),
        DisplayFieldValue::U8(254),
        DisplayFieldValue::U32(0xfedcba98),
        DisplayFieldValue::U16(65535),
        DisplayFieldValue::I16(-32768),
        DisplayFieldValue::F32(StoredFloat::from_bits(0x7f800042)),
    ]
}

fn component_visual_offset(raw: &[u8]) -> usize {
    let literal = [
        255, 255, 0, 128, 254, 152, 186, 220, 254, 255, 255, 0, 128, 66, 0, 128, 127,
    ];
    let matches: Vec<_> = raw
        .windows(17)
        .enumerate()
        .filter(|(_, bytes)| *bytes == literal)
        .collect();
    assert_eq!(matches.len(), 1);
    matches[0].0
}

fn component_rgb_expected(raw: &[u8]) -> Vec<u8> {
    let mut expected = raw.to_vec();
    let offset = component_visual_offset(raw) + 5;
    expected[offset..offset + 4].copy_from_slice(&0xfe798668_u32.to_le_bytes());
    expected
}

fn verify_component_rgb_edit(source: &DisplayArchive, raw: &[u8], index: usize) {
    let mut contents = source.contents().clone();
    contents.fields[index + 3].value = DisplayFieldValue::U32(0xfe798668);
    let result = source
        .edit_with_history(contents, archive_support::LIMITS)
        .unwrap();
    assert_eq!(
        result.archive().to_bytes().unwrap(),
        component_rgb_expected(raw)
    );
    assert_eq!(result.completion(), DisplayEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(result.changes()[0].parents().len(), 1);
    assert_eq!(source.to_bytes().unwrap(), raw);
    verify_component_field_history(source, &result, index + 3);
    verify_component_rgb_reopen(source, result.archive(), raw);
    if source.context().version == DisplayVersion::new(19000) {
        documents::verify_component_document(source, result.archive());
    }
}

fn verify_component_field_history(
    source: &DisplayArchive,
    result: &DisplayArchiveEdit,
    index: usize,
) {
    let change = &result.changes()[0];
    assert_eq!(change.relation(), DisplayFieldRelation::Derived);
    let parent = &change.parents()[0];
    let target = change.result().unwrap();
    assert_eq!(
        source.resolve_field(parent).unwrap(),
        &source.contents().fields[index]
    );
    assert_eq!(
        result.archive().resolve_field(target).unwrap(),
        &result.archive().contents().fields[index]
    );
    assert!(source.resolve_field(target).is_err());
    assert!(result.archive().resolve_field(parent).is_err());
    verify_component_unselected(source, result.archive(), index);
}

fn verify_component_unselected(source: &DisplayArchive, result: &DisplayArchive, index: usize) {
    for (ordinal, (before, after)) in source
        .contents()
        .fields
        .iter()
        .zip(&result.contents().fields)
        .enumerate()
    {
        if ordinal != index {
            assert_eq!(before, after);
        }
    }
}

fn verify_component_rgb_reopen(source: &DisplayArchive, result: &DisplayArchive, raw: &[u8]) {
    let reopened = DisplayArchive::from_bytes(
        &result.to_bytes().unwrap(),
        source.context(),
        archive_support::LIMITS,
    )
    .unwrap();
    assert_eq!(reopened.contents(), result.contents());
    let reversed = reopened
        .edit_with_history(source.contents().clone(), archive_support::LIMITS)
        .unwrap();
    assert_eq!(reversed.completion(), DisplayEditCompletion::Changed);
    assert_eq!(reversed.archive().to_bytes().unwrap(), raw);
    assert_eq!(reversed.archive().contents(), source.contents());
    assert_eq!(reversed.changes().len(), 1);
}

fn verify_component_unchanged(source: &DisplayArchive, raw: &[u8]) {
    let unchanged = source
        .edit_with_history(source.contents().clone(), archive_support::LIMITS)
        .unwrap();
    assert_eq!(unchanged.completion(), DisplayEditCompletion::Unchanged);
    assert_eq!(unchanged.archive().to_bytes().unwrap(), raw);
    assert!(unchanged.changes().is_empty());
}

fn verify_component_baseline(case: complete_cases::OldCase) -> usize {
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..archive_support::context()
    };
    let source = DisplayArchive::from_bytes(case.bytes, context, archive_support::LIMITS).unwrap();
    verify_component_unchanged(&source, case.bytes);
    verify_component_visual_groups(&source, case.bytes)
}

fn verify_component_visual_groups(source: &DisplayArchive, raw: &[u8]) -> usize {
    let positions: Vec<_> = source
        .contents()
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".ent_vis_prop.unknown_0"))
        .map(|(index, _)| index)
        .collect();
    for &index in &positions {
        let actual: Vec<_> = source.contents().fields[index..index + 7]
            .iter()
            .map(|field| field.value.clone())
            .collect();
        assert_eq!(actual, component_baseline_values());
        verify_component_rgb_edit(source, raw, index);
        bit_edits::verify_component_bit_writes(source, raw, index);
    }
    positions.len()
}

#[test]
fn retained_component_visuals_roundtrip_and_preserve_every_edited_bit() {
    let cases = retained_component_cases();
    assert_eq!(cases.len(), 88);
    assert_eq!(
        cases
            .into_iter()
            .map(verify_component_baseline)
            .sum::<usize>(),
        86
    );
}
