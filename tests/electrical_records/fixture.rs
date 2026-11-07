use bessemer::electrical::{ElectricalArchive, ElectricalLimits, ElectricalProjectLimits};
use bessemer::sqlite_image::SqliteLimits;
use std::io::Read;

pub(super) struct Case {
    pub name: String,
    pub raw: Vec<u8>,
    pub expected: Vec<u8>,
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
        maximum_projects: 1000,
        maximum_database_links: 1000,
        maximum_file_links: 10000,
        maximum_checked_images: 100,
        maximum_accounted_bytes: 128 << 20,
    }
}

pub(super) fn archive(raw: &[u8]) -> ElectricalArchive {
    let limits = ElectricalLimits {
        maximum_bytes: 16 << 20,
        maximum_members: 1000,
        maximum_member_bytes: 16 << 20,
        maximum_expanded_bytes: 64 << 20,
        maximum_ini_records: 1000,
    };
    ElectricalArchive::from_bytes(raw, limits).unwrap()
}

pub(super) fn cases() -> Vec<Case> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/electrical-records/graphs.bin.gz")[..],
    )
    .take(2_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    assert!(raw.len() <= 2_000_000);
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"EPR1");
    let count = word(&mut cursor);
    let cases = (0..count)
        .map(|_| Case {
            name: String::from_utf8(blob(&mut cursor)).unwrap(),
            raw: blob(&mut cursor),
            expected: blob(&mut cursor),
        })
        .collect();
    assert!(cursor.is_empty());
    cases
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
    let length = word(cursor) as usize;
    take(cursor, length).to_vec()
}
