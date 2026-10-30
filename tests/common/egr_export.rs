use super::{Cursor, checks, groups, inflate, ranges};
use bessemer::simulation::*;
use std::sync::Arc;

pub fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

pub fn span(output: &mut Vec<u8>, bytes: &[u8]) {
    words(output, &[bytes.len() as u32]);
    output.extend_from_slice(bytes);
}

#[test]
#[ignore = "requires explicitly injected BESSEMER_EGR_PROJECTION_OUTPUT path"]
fn complete_owned_projection_export_matches_independent_actual_reader_packets() {
    let path = std::env::var_os("BESSEMER_EGR_PROJECTION_OUTPUT")
        .map(std::path::PathBuf::from)
        .expect("BESSEMER_EGR_PROJECTION_OUTPUT must name the explicitly injected export path");
    let mut output = b"EGRV".to_vec();
    export_tables(&mut output);
    export_fields(&mut output);
    export_names(&mut output);
    export_groups(&mut output);
    std::fs::write(&path, &output).unwrap();
    assert_eq!(std::fs::read(path).unwrap(), output);
}

fn export_tables(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/simulation-egr/tables.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    let count = cursor.word();
    words(output, &[count]);
    for _ in 0..count {
        let image = checks::table(cursor.span());
        cursor.span();
        cursor.span();
        span(output, &table_bytes(&image));
        span(
            output,
            &image
                .descriptor_writer_projection()
                .projected_descriptor_bytes,
        );
    }
    assert!(cursor.0.is_empty());
}

pub fn table_bytes(image: &SimulationEgrArchive) -> Vec<u8> {
    let mut output = Vec::new();
    words(&mut output, &image.header_words().map(|word| word as u32));
    for bytes in [
        image.header_padding(),
        image.field_alignment(),
        image.directory_padding(),
    ] {
        span(&mut output, bytes);
    }
    append_metadata(image, &mut output);
    append_stored_rows(image, &mut output);
    output
}

fn append_metadata(image: &SimulationEgrArchive, output: &mut Vec<u8>) {
    words(output, &[image.fields().len() as u32]);
    for field in image.fields() {
        words(output, &[field.tag as u32, field.width as u32]);
    }
    words(output, &[image.directory().len() as u32]);
    for number in image.directory() {
        words(output, &[*number as u32]);
    }
    words(
        output,
        &[
            image.coverage().allocated_records as u32,
            image.coverage().unallocated_capacity as u32,
        ],
    );
}

fn append_live_row(image: &SimulationEgrArchive, id: &SimulationEgrRowId, output: &mut Vec<u8>) {
    append_live_header(image, id, output);
    let row = image.live_row(id).unwrap();
    append_live_fields(image, row, output);
    append_conditional(image, id, output);
}

fn append_deleted(image: &SimulationEgrArchive, output: &mut Vec<u8>) {
    words(output, &[image.free_records().len() as u32]);
    for link in image.free_records() {
        words(
            output,
            &[link.record_number, link.next_record_number as u32],
        );
        span(
            output,
            image
                .physical_record(&image.physical_id(link.record_number).unwrap())
                .unwrap(),
        );
    }
}

fn export_fields(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/simulation-egr/fields.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    let count = cursor.word();
    words(output, &[count]);
    for _ in 0..count {
        let image = checks::table(cursor.span());
        let ranges = cursor.word();
        words(output, &[ranges]);
        for _ in 0..ranges {
            span(output, &field_bytes(&image, &mut cursor));
        }
    }
    assert!(cursor.0.is_empty());
}

fn field_bytes(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) -> Vec<u8> {
    let input = ranges::range_input(cursor);
    let (logical, first, last, _, _, _) = input.fields;
    let replacement = input.replacement;
    let id = image.field_range_id(logical, first, last).unwrap();
    let view = image.field_range(&id).unwrap();
    let edit = image.edit_field_range(&id, replacement).unwrap();
    let mut output = Vec::new();
    words(&mut output, &range_words(&view));
    span(&mut output, &view.bytes);
    span(&mut output, &edit.replacement);
    span(&mut output, &edit.archive.to_bytes());
    output
}

fn export_names(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/simulation-egr/names.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    let count = cursor.word();
    words(output, &[count]);
    for _ in 0..count {
        let code = SimulationSolverElementCode::from_signed(cursor.word() as i32);
        let destination: Arc<[u8]> = cursor.span().into();
        cursor.span();
        span(output, &name_bytes(code.project_destination(destination)));
    }
    assert!(cursor.0.is_empty());
}

pub fn name_bytes(result: bessemer::Result<SimulationSolverNameProjection>) -> Vec<u8> {
    let Ok(result) = result else {
        return vec![0; 4];
    };
    let kind = name_kind(result.lookup);
    let mut output = Vec::new();
    words(
        &mut output,
        &[
            1,
            kind,
            u32::from(result.output.is_some()),
            u32::from(result.native_return.is_some()),
            result.native_return.unwrap_or(0) as u32,
        ],
    );
    span(&mut output, result.output.as_deref().unwrap_or(&[]));
    output
}

fn export_groups(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/simulation-egr/groups.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    let count = cursor.word();
    words(output, &[count]);
    let profile = SimulationEgrRowProfile::from_loaded_widths(&[4; 32]).unwrap();
    for _ in 0..count {
        let row = SimulationEgrRowImage::from_bytes(cursor.span(), profile).unwrap();
        let ordinal = SimulationEgrLogicalOrdinal::new(cursor.word()).unwrap();
        let offset = cursor.word() as i32;
        cursor.span();
        span(output, &group_bytes(&row.project_egroup(ordinal, offset)));
    }
    assert!(cursor.0.is_empty());
}

pub fn group_bytes(result: &SimulationEgroupProjection) -> Vec<u8> {
    let kind = match result.completion {
        SimulationEgroupCompletion::SkippedIdentifierMismatch => return vec![0; 4],
        SimulationEgroupCompletion::ProjectedKnownNameLine => 1,
        SimulationEgroupCompletion::IncompleteNameLookupState => 2,
    };
    let mut output = Vec::new();
    words(&mut output, &[kind, result.group_number.unwrap() as u32]);
    words(
        &mut output,
        &result.ordered_parameters.unwrap().map(|value| value as u32),
    );
    words(&mut output, &[u32::from(result.output.is_some())]);
    span(&mut output, result.output.as_deref().unwrap_or(&[]));
    output
}

fn append_conditional(image: &SimulationEgrArchive, id: &SimulationEgrRowId, output: &mut Vec<u8>) {
    let applicable =
        image.fields().len() == 32 && image.fields().iter().all(|field| field.width == 4);
    words(output, &[u32::from(applicable)]);
    if applicable {
        span(
            output,
            &groups::row_view_bytes(&image.conditional_row(id).unwrap().view()),
        );
        return;
    }
    assert_eq!(
        image.conditional_row(id).unwrap_err().class,
        bessemer::FailureClass::Unsupported
    );
}

fn append_stored_rows(image: &SimulationEgrArchive, output: &mut Vec<u8>) {
    let records = image.allocated_record_bytes().as_chunks::<128>().0;
    words(output, &[records.len() as u32]);
    for record in records {
        span(output, record);
    }
    words(output, &[image.coverage().live_rows as u32]);
    for ordinal in 1..=image.directory().len() as u32 {
        if let Some(id) = image.live_row_id(ordinal).unwrap() {
            append_live_row(image, &id, output);
        }
    }
    append_deleted(image, output);
}

fn append_live_header(image: &SimulationEgrArchive, id: &SimulationEgrRowId, output: &mut Vec<u8>) {
    let parts = image.parts();
    let body = 128
        + parts.field_directory.len()
        + parts.row_directory.len()
        + (id.record_number() as usize - 1) * 128;
    words(
        output,
        &[
            id.logical_ordinal(),
            id.record_number(),
            body as u32,
            image.fields().len() as u32,
        ],
    );
}

fn append_live_fields(image: &SimulationEgrArchive, row: &[u8; 128], output: &mut Vec<u8>) {
    let mut end = 0;
    for field in image.fields() {
        words(
            output,
            &[
                field.tag as u32,
                field.row_offset as u32,
                field.row_end as u32,
            ],
        );
        span(output, &row[field.row_offset..field.row_end]);
        end = field.row_end;
    }
    span(output, &row[end..]);
}

fn name_kind(lookup: SimulationSolverNameState) -> u32 {
    match lookup {
        SimulationSolverNameState::Known(_) => 1,
        SimulationSolverNameState::MatchedNullSource => 2,
        SimulationSolverNameState::UnmatchedNoWrite => 3,
    }
}

fn range_words(view: &SimulationEgrFieldRange) -> [u32; 6] {
    [
        view.logical_ordinal,
        view.first_field,
        view.last_field,
        view.record_number,
        view.row_offset as u32,
        view.row_end as u32,
    ]
}
