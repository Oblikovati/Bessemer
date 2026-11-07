use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
mod packet;
use packet::Cursor;

struct FinalCase<'a> {
    profile: u32,
    maximum: usize,
    text_maximum: usize,
    ordinal: i32,
    accepted: bool,
    raw: &'a [u8],
    replacement: &'a [u8],
}

fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

fn span(output: &mut Vec<u8>, raw: &[u8]) {
    words(output, &[raw.len().try_into().unwrap()]);
    output.extend_from_slice(raw);
}

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-lbc-final-streams/owned.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

#[test]
fn complete_owned_final_lbc_frames_edits_and_rejections_match_independent_oracles() {
    let expected = owned();
    assert_eq!(project_packet(&expected), expected);
}

#[test]
#[ignore = "requires injected native input and complete final LBC projection output paths"]
fn export_complete_native_final_lbc_frames_and_edits() {
    let input = std::env::var_os("BESSEMER_LBC_FINAL_NATIVE_INPUT")
        .expect("expected injected native input path");
    let output = std::env::var_os("BESSEMER_LBC_FINAL_PROJECTION_OUTPUT")
        .expect("expected injected projection output path");
    let native = std::fs::read(input).unwrap();
    let mut result = Vec::new();
    span(&mut result, &project_packet(&owned()));
    span(&mut result, &project_packet(&native));
    std::fs::write(output, result).unwrap();
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"LBCF");
    let count = cursor.word();
    let mut output = b"LBCF".to_vec();
    words(&mut output, &[count]);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected = cursor.span();
        let actual = project(&case);
        assert_eq!(
            actual, expected,
            "profile {}, ordinal {}, accepted {}",
            case.profile, case.ordinal, case.accepted
        );
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> FinalCase<'a> {
    FinalCase {
        profile: cursor.word(),
        maximum: cursor.word() as usize,
        text_maximum: cursor.word() as usize,
        ordinal: cursor.word() as i32,
        accepted: cursor.word() == 1,
        raw: cursor.span(),
        replacement: cursor.span(),
    }
}

fn append_case(output: &mut Vec<u8>, case: &FinalCase<'_>, actual: &[u8]) {
    words(
        output,
        &[
            case.profile,
            case.maximum as u32,
            case.text_maximum as u32,
            case.ordinal as u32,
            u32::from(case.accepted),
        ],
    );
    span(output, case.raw);
    span(output, case.replacement);
    span(output, actual);
}

fn project(case: &FinalCase<'_>) -> Vec<u8> {
    match case.profile {
        0 => project_names(case),
        1 => project_maps(case),
        other => panic!("{other}; expected profile 0 or 1"),
    }
}

fn rejected(error: bessemer::BessemerError) -> Vec<u8> {
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    vec![0; 4]
}

fn project_names(case: &FinalCase<'_>) -> Vec<u8> {
    let parsed = SimulationLbcNameOrder::from_bytes(case.raw, case.maximum, case.text_maximum);
    if !case.accepted {
        return rejected(parsed.unwrap_err());
    }
    let source = parsed.unwrap();
    let rebuilt =
        SimulationLbcNameOrder::from_items(source.items(), case.maximum, case.text_maximum)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), case.raw);
    let mut output = Vec::new();
    if case.ordinal < 0 {
        words(&mut output, &[1]);
        name_image(&mut output, &source);
        return output;
    }
    name_edit(&mut output, &source, case);
    assert_eq!(source.to_bytes(), case.raw);
    output
}

fn name_image(output: &mut Vec<u8>, source: &SimulationLbcNameOrder) {
    span(output, &source.to_bytes());
    words(output, &[source.items().len() as u32]);
    for item in source.items() {
        words(output, &[item.source_offsets().0 as u32]);
        span(output, &text_frame(item));
        assert_eq!(item.source_parent(), source.to_bytes());
    }
}

fn name_edit(output: &mut Vec<u8>, source: &SimulationLbcNameOrder, case: &FinalCase<'_>) {
    let replacement =
        SimulationPersistText::read_from(case.replacement, 0, case.text_maximum).unwrap();
    let id = source.item_id(case.ordinal as usize).unwrap();
    let edit = source.edit_item(&id, &replacement).unwrap();
    let changed = edit.completion == SimulationLbcNameEditCompletion::Changed;
    words(output, &[2]);
    span(output, &edit.source.to_bytes());
    name_image(output, &edit.archive);
    span(output, &text_frame(&edit.replacement));
    words(output, &[u32::from(changed), edit.history.len() as u32]);
    assert_eq!(changed, source.revision() != edit.archive.revision());
    name_history(output, &edit, changed);
}

fn name_history(output: &mut Vec<u8>, edit: &SimulationLbcNameEdit, changed: bool) {
    let source = &edit.source;
    for (index, entry) in edit.history.iter().enumerate() {
        assert_eq!(entry.parent, source.item_id(index).unwrap());
        assert_eq!(entry.result, edit.archive.item_id(index).unwrap());
        if !changed {
            assert_eq!(entry.parent, entry.result);
        }
        let before = source.item(&entry.parent).unwrap().source_offsets().0;
        let after = edit.archive.item(&entry.result).unwrap().source_offsets().0;
        words(
            output,
            &[
                before as u32,
                after as u32,
                u32::from(entry.relation == SimulationLbcNameRelation::Derived),
            ],
        );
    }
}

fn text_frame(text: &SimulationPersistText) -> Vec<u8> {
    [text.header(), text.payload()].concat()
}

fn project_maps(case: &FinalCase<'_>) -> Vec<u8> {
    let parsed = SimulationLegacyMidsurfMap::from_bytes(case.raw, case.maximum);
    if !case.accepted {
        return rejected(parsed.unwrap_err());
    }
    let source = parsed.unwrap();
    verify_map_construction(&source, case.maximum);
    let mut output = Vec::new();
    if case.ordinal < 0 {
        words(&mut output, &[1]);
        map_image(&mut output, &source);
        return output;
    }
    map_edit(&mut output, &source, case);
    assert_eq!(source.to_bytes(), case.raw);
    output
}

fn verify_map_construction(source: &SimulationLegacyMidsurfMap, maximum: usize) {
    let groups = source.groups().each_ref().map(|group| group.to_vec());
    let rebuilt = SimulationLegacyMidsurfMap::from_groups(&groups, maximum).unwrap();
    assert_eq!(rebuilt.to_bytes(), source.to_bytes());
}

fn map_ids(source: &SimulationLegacyMidsurfMap) -> Vec<SimulationLegacyMapCellId> {
    let mut ids = Vec::new();
    for (group_index, group) in source.groups().iter().enumerate() {
        for record in 0..group.len() {
            ids.extend(
                (0..4).map(|component| source.cell_id(group_index, record, component).unwrap()),
            );
        }
    }
    ids
}

fn map_image(output: &mut Vec<u8>, source: &SimulationLegacyMidsurfMap) {
    span(output, &source.to_bytes());
    let ids = map_ids(source);
    words(output, &[ids.len() as u32]);
    for id in ids {
        words(output, &[id.stream_offset() as u32]);
        span(output, &source.cell(&id).unwrap().to_le_bytes());
    }
}

fn map_edit(output: &mut Vec<u8>, source: &SimulationLegacyMidsurfMap, case: &FinalCase<'_>) {
    let id = map_ids(source)[case.ordinal as usize].clone();
    let replacement = i32::from_le_bytes(case.replacement.try_into().unwrap());
    let edit = source.edit_cell(&id, replacement).unwrap();
    super::maps::verify_map_history(&edit);
    words(output, &[2]);
    span(output, &edit.source.to_bytes());
    map_image(output, &edit.archive);
    span(output, &edit.replacement.to_le_bytes());
    words(
        output,
        &[
            u32::from(edit.completion == SimulationLegacyMapEditCompletion::Changed),
            edit.cell_history.len() as u32,
        ],
    );
    map_history(output, &edit);
}

fn map_history(output: &mut Vec<u8>, edit: &SimulationLegacyMapCellEdit) {
    for entry in edit.cell_history.iter() {
        words(
            output,
            &[
                entry.parent.stream_offset() as u32,
                entry.result.stream_offset() as u32,
                u32::from(entry.relation == SimulationLegacyMapRelation::Derived),
            ],
        );
    }
}
