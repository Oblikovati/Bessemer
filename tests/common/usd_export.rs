use super::{Cursor, checks, inflate, limits};
use bessemer::plastics::*;

#[test]
#[ignore = "requires an explicitly injected BESSEMER_USD_PROJECTION_OUTPUT path"]
fn complete_owned_projection_export_matches_independent_reader_packets() {
    let path = std::env::var_os("BESSEMER_USD_PROJECTION_OUTPUT")
        .map(std::path::PathBuf::from)
        .expect("BESSEMER_USD_PROJECTION_OUTPUT must name the explicitly injected export path");
    let mut output = b"USDV".to_vec();
    export_records(&mut output);
    export_layouts(&mut output);
    export_edits(&mut output);
    std::fs::write(&path, &output).unwrap();
    assert_eq!(std::fs::read(path).unwrap(), output);
}

fn export_records(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/plastics-usd/records.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USD1");
    let count = cursor.word();
    checks::append_words(output, &[count]);
    for _ in 0..count {
        let record =
            UsdStringRecord::from_bytes(cursor.span(), limits().maximum_payload_bytes).unwrap();
        cursor.span();
        checks::append_span(output, &checks::record_bytes(&record));
    }
    assert!(cursor.0.is_empty());
}

fn export_layouts(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/plastics-usd/layouts.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USDL");
    let count = cursor.word();
    checks::append_words(output, &[count]);
    for _ in 0..count {
        cursor.word();
        let archive = UsdArchive::from_bytes(cursor.span(), limits()).unwrap();
        cursor.span();
        checks::append_span(
            output,
            &checks::layout_bytes(&archive.writer_layout().unwrap()),
        );
    }
    assert!(cursor.0.is_empty());
}

fn export_edits(output: &mut Vec<u8>) {
    let raw = inflate(include_bytes!("../fixtures/plastics-usd/edits.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USDE");
    let count = cursor.word();
    checks::append_words(output, &[count]);
    for _ in 0..count {
        export_edit(&mut cursor, output);
    }
    assert!(cursor.0.is_empty());
}

fn export_edit(cursor: &mut Cursor<'_>, output: &mut Vec<u8>) {
    let archive = UsdArchive::from_bytes(cursor.span(), limits()).unwrap();
    let id = archive.record_id(cursor.word() as usize).unwrap();
    let replacement =
        UsdStringRecord::from_bytes(cursor.span(), limits().maximum_payload_bytes).unwrap();
    cursor.span();
    cursor.span();
    let edit = archive.edit_record(&id, &replacement).unwrap();
    checks::append_span(output, &edit.archive.to_bytes());
    checks::append_span(output, &checks::archive_bytes(&edit.archive));
}
