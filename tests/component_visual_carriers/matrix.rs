use super::*;
use complete_cases::framing;

struct ComponentMatrixTemplate {
    kind: DisplayDocumentKind,
    bytes: &'static [u8],
    offset: usize,
}

fn component_matrix_template(cursor: &mut &'static [u8]) -> ComponentMatrixTemplate {
    assert_eq!(framing::take_u32(cursor).unwrap(), 19000);
    let kind = match framing::take(cursor, 1).unwrap()[0] {
        0 => DisplayDocumentKind::Part,
        1 => DisplayDocumentKind::Assembly,
        other => panic!("{other}; expected part0 or assembly1"),
    };
    assert_eq!(framing::take(cursor, 1).unwrap(), [21]);
    let size = framing::take_u32(cursor).unwrap() as usize;
    let offset = framing::take_u32(cursor).unwrap() as usize;
    let bytes = framing::take(cursor, size).unwrap();
    ComponentMatrixTemplate {
        kind,
        bytes,
        offset,
    }
}

fn component_matrix_templates() -> Vec<ComponentMatrixTemplate> {
    let mut cursor =
        include_bytes!("../fixtures/entity-visual-state/component-templates.bin").as_slice();
    assert_eq!(framing::take_u32(&mut cursor).unwrap(), 2);
    let templates = (0..2)
        .map(|_| component_matrix_template(&mut cursor))
        .collect();
    framing::require_finished(cursor).unwrap();
    templates
}

fn component_matrix_word_bytes(columns: &[&str]) -> Vec<u8> {
    assert_eq!(columns.len(), 13);
    let mut raw = Vec::new();
    for index in [2, 3] {
        raw.extend_from_slice(&columns[index].parse::<u16>().unwrap().to_le_bytes());
    }
    raw.push(columns[4].parse::<u8>().unwrap());
    raw.extend_from_slice(&columns[5].parse::<u32>().unwrap().to_le_bytes());
    raw.extend_from_slice(&columns[6].parse::<u16>().unwrap().to_le_bytes());
    raw.extend_from_slice(&columns[7].parse::<i16>().unwrap().to_le_bytes());
    raw.extend_from_slice(&columns[8].parse::<u32>().unwrap().to_le_bytes());
    assert_eq!(raw.len(), 17);
    raw
}

fn matrix_template_for<'a>(
    templates: &'a [ComponentMatrixTemplate],
    name: &str,
) -> &'a ComponentMatrixTemplate {
    templates
        .iter()
        .find(|template| match template.kind {
            DisplayDocumentKind::Part => name == "part",
            DisplayDocumentKind::Assembly => name == "assembly",
        })
        .unwrap()
}

fn matrix_archive(raw: &[u8], kind: DisplayDocumentKind) -> DisplayArchive {
    let context = DisplayArchiveContext {
        kind,
        ..archive_support::context()
    };
    DisplayArchive::from_bytes(raw, context, archive_support::LIMITS).unwrap()
}

fn matrix_visual_position(source: &DisplayArchive) -> usize {
    let positions: Vec<_> = source
        .contents()
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".ent_vis_prop.unknown_0"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(positions.len(), 1);
    positions[0]
}

fn verify_matrix_fields(
    template: &DisplayArchive,
    source: &DisplayArchive,
    index: usize,
    words: &[u8],
) {
    let expected = bit_edits::component_literal_words(words);
    assert_eq!(
        template.contents().fields.len(),
        source.contents().fields.len()
    );
    for (position, (before, after)) in template
        .contents()
        .fields
        .iter()
        .zip(&source.contents().fields)
        .enumerate()
    {
        assert_eq!(before.path, after.path);
        if (index..index + 7).contains(&position) {
            assert_eq!(after.value, expected[position - index]);
        } else {
            assert_eq!(before, after);
        }
    }
}

fn component_matrix_rgb_result(
    source: &DisplayArchive,
    raw: &[u8],
    index: usize,
    offset: usize,
) -> Vec<u8> {
    let original = u32::from_le_bytes(raw[offset + 5..offset + 9].try_into().unwrap());
    let color = original ^ 0x00a53cf0;
    let mut contents = source.contents().clone();
    contents.fields[index + 3].value = DisplayFieldValue::U32(color);
    let result = source
        .edit_with_history(contents, archive_support::LIMITS)
        .unwrap();
    let mut expected = raw.to_vec();
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    let actual = result.archive().to_bytes().unwrap();
    assert_eq!(actual, expected);
    verify_matrix_rgb_result(source, &result, raw, index);
    assert_eq!(color >> 24, original >> 24);
    actual
}

fn verify_matrix_rgb_result(
    source: &DisplayArchive,
    result: &DisplayArchiveEdit,
    raw: &[u8],
    index: usize,
) {
    assert_eq!(result.completion(), DisplayEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    verify_component_field_history(source, result, index + 3);
    verify_component_rgb_reopen(source, result.archive(), raw);
    documents::verify_component_document(source, result.archive());
}

fn append_matrix_outputs(output: &mut Option<Vec<u8>>, raw: &[u8], edited: &[u8]) {
    let Some(output) = output else {
        return;
    };
    for bytes in [raw, edited] {
        output.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        output.extend_from_slice(bytes);
    }
}

fn component_matrix_forced_bytes(source: &DisplayArchive) -> Vec<u8> {
    let rebuilt = DisplayArchive::new(
        source.contents().clone(),
        source.context(),
        archive_support::LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.contents(), source.contents());
    rebuilt.to_bytes().unwrap()
}

fn verify_component_matrix_case(
    template: &ComponentMatrixTemplate,
    columns: &[&str],
    output: &mut Option<Vec<u8>>,
) {
    let words = component_matrix_word_bytes(columns);
    assert_eq!(template.offset, columns[9].parse::<usize>().unwrap());
    assert_eq!(template.bytes.len(), columns[10].parse::<usize>().unwrap());
    let mut raw = template.bytes.to_vec();
    raw[template.offset..template.offset + 17].copy_from_slice(&words);
    let baseline = matrix_archive(template.bytes, template.kind);
    let source = matrix_archive(&raw, template.kind);
    let index = matrix_visual_position(&source);
    verify_matrix_fields(&baseline, &source, index, &words);
    verify_component_unchanged(&source, &raw);
    typed_state::verify_typed_matrix_rgb(&source, &raw, index, template.offset);
    let edited = component_matrix_rgb_result(&source, &raw, index, template.offset);
    let reconstructed = component_matrix_forced_bytes(&source);
    assert_eq!(reconstructed, raw);
    append_matrix_outputs(output, &reconstructed, &edited);
}

#[test]
fn complete_component_word_matrix_reconstructs_edits_and_reverses_every_document() {
    let templates = component_matrix_templates();
    let path = std::env::var_os("BESSEMER_COMPONENT_VISUAL_MATRIX_OUTPUT");
    let mut output = path.as_ref().map(|_| 7662_u32.to_le_bytes().to_vec());
    let mut counts = [0, 0];
    for row in include_str!("../fixtures/entity-visual-state/component-words.tsv").lines() {
        let columns: Vec<_> = row.split('\t').collect();
        let template = matrix_template_for(&templates, columns[0]);
        verify_component_matrix_case(template, &columns, &mut output);
        counts[usize::from(template.kind == DisplayDocumentKind::Assembly)] += 1;
    }
    assert_eq!(counts, [3831, 3831]);
    if let (Some(path), Some(output)) = (path, output) {
        std::fs::write(path, output).unwrap();
    }
}
