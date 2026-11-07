use bessemer::{electrical::*, sqlite_image::SqliteLimits};
use std::io::Read;

pub(super) struct Case {
    pub name: String,
    pub raw: Vec<u8>,
    pub missing: Option<String>,
    pub expected: Vec<u8>,
}

pub(super) fn cases() -> Vec<Case> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/electrical-file-fields/maps.bin.gz")[..],
    )
    .take(4_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    assert!(raw.len() <= 4_000_000);
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"EFF1");
    let count = word(&mut cursor);
    let cases = (0..count).map(|_| case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}

fn case(cursor: &mut &[u8]) -> Case {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let missing = String::from_utf8(blob(cursor)).unwrap();
    let expected = blob(cursor);
    Case {
        name,
        raw,
        missing: (!missing.is_empty()).then_some(missing),
        expected,
    }
}

pub(super) fn project_limits() -> ElectricalProjectLimits {
    let sqlite = SqliteLimits {
        maximum_image_bytes: 16 << 20,
        maximum_schema_objects: 4000,
        maximum_rows: 100000,
        maximum_columns: 2000,
        maximum_value_bytes: 8 << 20,
        maximum_output_bytes: 64 << 20,
        maximum_vm_steps: 100_000_000,
    };
    ElectricalProjectLimits {
        sqlite,
        maximum_projects: 100,
        maximum_database_links: 100,
        maximum_file_links: 100,
        maximum_checked_images: 100,
        maximum_accounted_bytes: 128 << 20,
    }
}

pub(super) fn archive(raw: &[u8]) -> ElectricalArchive {
    ElectricalArchive::from_bytes(
        raw,
        ElectricalLimits {
            maximum_bytes: 16 << 20,
            maximum_members: 1000,
            maximum_member_bytes: 16 << 20,
            maximum_expanded_bytes: 64 << 20,
            maximum_ini_records: 1000,
        },
    )
    .unwrap()
}

pub(super) fn maps(view: &ElectricalProjectView) -> bessemer::Result<Vec<ElectricalFileReadMap>> {
    view.projects()
        .iter()
        .flat_map(|project| project.databases.iter())
        .flat_map(|database| database.files.iter())
        .map(ElectricalFileLink::file_read_map)
        .collect()
}

fn take<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (head, tail) = cursor.split_at(count);
    *cursor = tail;
    head
}
fn word(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(take(cursor, 4).try_into().unwrap())
}
fn blob(cursor: &mut &[u8]) -> Vec<u8> {
    let count = word(cursor) as usize;
    take(cursor, count).to_vec()
}
