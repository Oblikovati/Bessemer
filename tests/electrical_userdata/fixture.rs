use bessemer::{electrical::*, sqlite_image::SqliteLimits};
use std::io::Read;

pub(super) struct Case {
    pub name: String,
    pub raw: Vec<u8>,
    pub queries: Vec<(ElectricalUserdataKey, Vec<u8>)>,
    pub lexical: Vec<(i32, Vec<u8>)>,
}

pub(super) type IdentityCase = (i32, i32, ElectricalUserdataTransition);

pub(super) fn cases() -> (Vec<Case>, Vec<IdentityCase>) {
    let raw = fixture_bytes();
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"EUQ1");
    let count = word(&mut cursor);
    let cases = (0..count).map(|_| case(&mut cursor)).collect();
    let count = word(&mut cursor);
    let identities = (0..count).map(|_| identity(&mut cursor)).collect();
    assert!(cursor.is_empty());
    (cases, identities)
}

fn identity(cursor: &mut &[u8]) -> IdentityCase {
    let current = signed(cursor);
    let loaded = signed(cursor);
    let expected = ElectricalUserdataTransition {
        stored_id: signed(cursor),
        delegated_success: take(cursor, 1) == [1],
    };
    (current, loaded, expected)
}

fn fixture_bytes() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/electrical-userdata/queries.bin.gz")[..],
    )
    .take(2_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    assert!(raw.len() < 2_000_001);
    raw
}

fn case(cursor: &mut &[u8]) -> Case {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let expected = blob(cursor);
    let mut fields = expected.as_slice();
    let queries = queries(&mut fields);
    let count = word(&mut fields);
    let lexical = (0..count)
        .map(|_| (signed(&mut fields), blob(&mut fields)))
        .collect();
    assert!(fields.is_empty());
    Case {
        name,
        raw,
        queries,
        lexical,
    }
}

fn queries(cursor: &mut &[u8]) -> Vec<(ElectricalUserdataKey, Vec<u8>)> {
    let count = word(cursor);
    (0..count)
        .map(|_| {
            let key = ElectricalUserdataKey {
                object_id: signed(cursor),
                object_no: signed(cursor),
            };
            (key, blob(cursor))
        })
        .collect()
}

pub(super) fn sqlite_limits() -> SqliteLimits {
    SqliteLimits {
        maximum_image_bytes: 16 << 20,
        maximum_schema_objects: 4000,
        maximum_rows: 100000,
        maximum_columns: 2000,
        maximum_value_bytes: 8 << 20,
        maximum_output_bytes: 64 << 20,
        maximum_vm_steps: 100_000_000,
    }
}

pub(super) fn project_limits() -> ElectricalProjectLimits {
    ElectricalProjectLimits {
        sqlite: sqlite_limits(),
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

pub(super) fn query(
    archive: &ElectricalArchive,
    key: ElectricalUserdataKey,
) -> ElectricalUserdataQuery {
    let member = archive
        .member_named("Projects/2/Database/tew_project_data.sqlite")
        .unwrap();
    archive
        .userdata_by_key(&member, key, sqlite_limits())
        .unwrap()
}

fn take<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (head, tail) = cursor.split_at(count);
    *cursor = tail;
    head
}
fn word(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(take(cursor, 4).try_into().unwrap())
}
fn signed(cursor: &mut &[u8]) -> i32 {
    i32::from_le_bytes(take(cursor, 4).try_into().unwrap())
}
fn blob(cursor: &mut &[u8]) -> Vec<u8> {
    let count = word(cursor) as usize;
    take(cursor, count).to_vec()
}
