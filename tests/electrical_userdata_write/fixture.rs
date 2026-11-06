use bessemer::{electrical::*, sqlite_image::SqliteLimits};
use std::io::Read;

pub(super) struct OwnedCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub replacement: String,
    pub success: bool,
    pub expected: Vec<u8>,
}
pub(super) struct NativeCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub keys: Vec<ElectricalUserdataKey>,
}

pub(super) fn owned_cases() -> Vec<OwnedCase> {
    let raw = inflate(
        include_bytes!("../fixtures/electrical-userdata-write/owned.bin.gz"),
        100_000,
    );
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"EUW1");
    let count = word(&mut cursor);
    let cases = (0..count).map(|_| owned_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}

fn owned_case(cursor: &mut &[u8]) -> OwnedCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let replacement = String::from_utf8(blob(cursor)).unwrap();
    let success = take(cursor, 1) == [1];
    let expected = blob(cursor);
    OwnedCase {
        name,
        raw,
        replacement,
        success,
        expected,
    }
}

pub(super) fn native_cases() -> Vec<NativeCase> {
    let raw = inflate(
        include_bytes!("../fixtures/electrical-userdata/queries.bin.gz"),
        2_000_000,
    );
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"EUQ1");
    let count = word(&mut cursor);
    (0..count)
        .filter_map(|_| native_case(&mut cursor))
        .collect()
}

fn native_case(cursor: &mut &[u8]) -> Option<NativeCase> {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let payload = blob(cursor);
    if !name.ends_with(".tewzip") {
        return None;
    }
    let mut fields = payload.as_slice();
    let count = word(&mut fields);
    let keys = (0..count).filter_map(|_| native_key(&mut fields)).collect();
    Some(NativeCase { name, raw, keys })
}

fn native_key(cursor: &mut &[u8]) -> Option<ElectricalUserdataKey> {
    let object_id = i32::from_le_bytes(take(cursor, 4).try_into().unwrap());
    let object_no = i32::from_le_bytes(take(cursor, 4).try_into().unwrap());
    let expected = blob(cursor);
    let mut rows = expected.as_slice();
    let columns = word(&mut rows);
    for _ in 0..columns {
        blob(&mut rows);
    }
    (word(&mut rows) == 1).then_some(ElectricalUserdataKey {
        object_id,
        object_no,
    })
}

fn inflate(compressed: &[u8], limit: u64) -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed)
        .take(limit + 1)
        .read_to_end(&mut raw)
        .unwrap();
    assert!(raw.len() as u64 <= limit);
    raw
}

pub(super) fn limits() -> SqliteLimits {
    SqliteLimits {
        maximum_image_bytes: 16 << 20,
        maximum_schema_objects: 4000,
        maximum_rows: 100_000,
        maximum_columns: 2000,
        maximum_value_bytes: 8 << 20,
        maximum_output_bytes: 64 << 20,
        maximum_vm_steps: 100_000_000,
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

pub(super) fn owned_member(archive: &ElectricalArchive) -> ElectricalMemberId {
    archive
        .member_named("Database/tew_project_data.sqlite")
        .unwrap()
}
pub(super) fn native_member(archive: &ElectricalArchive) -> ElectricalMemberId {
    archive
        .member_named("Projects/2/Database/tew_project_data.sqlite")
        .unwrap()
}
pub(super) fn key() -> ElectricalUserdataKey {
    ElectricalUserdataKey {
        object_id: 7,
        object_no: 0,
    }
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
