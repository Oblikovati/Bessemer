use super::packet::*;

struct BackendCase {
    name: Vec<u8>,
    raw: Vec<u8>,
    version: u8,
    tables: Vec<BackendTable>,
    end: u32,
    tail: Vec<u8>,
    complete: bool,
    full: bool,
    runtime: bool,
    stop: Vec<u8>,
}
struct BackendTable {
    ordinal: u32,
    start: u32,
    version: u8,
    count: i32,
    kind: Option<i32>,
    key: Vec<u8>,
}
fn cases() -> Vec<BackendCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-metadata/backend.bin.gz"),
        *b"FMB1",
    );
    let result = counted(&mut cursor, read_case);
    finish(&cursor);
    result
}
fn read_case(cursor: &mut PacketCursor) -> BackendCase {
    BackendCase {
        name: blob(cursor),
        raw: blob(cursor),
        version: byte(cursor),
        tables: counted(cursor, read_table),
        end: word(cursor),
        tail: blob(cursor),
        complete: boolean(cursor),
        full: boolean(cursor),
        runtime: boolean(cursor),
        stop: blob(cursor),
    }
}
fn read_table(cursor: &mut PacketCursor) -> BackendTable {
    BackendTable {
        ordinal: word(cursor),
        start: word(cursor),
        version: byte(cursor),
        count: signed(cursor),
        kind: optional(cursor, signed),
        key: blob(cursor),
    }
}
fn check_case(case: &BackendCase) {
    assert_eq!(case.version, 0);
    assert!(!case.full);
    assert!(!case.runtime);
    assert!((1..=2).contains(&case.tables.len()));
    let mut raw = vec![case.version];
    for table in &case.tables {
        append_table(&mut raw, table);
    }
    assert_eq!(raw.len(), case.end as usize);
    raw.extend_from_slice(&case.tail);
    assert_eq!(raw, case.raw, "{}", String::from_utf8_lossy(&case.name));
    check_stop(case);
}
fn append_table(raw: &mut Vec<u8>, table: &BackendTable) {
    assert_eq!(raw.len(), table.start as usize);
    assert_eq!(table.version, 0);
    assert!(table.count >= 0);
    assert_eq!(table.kind.is_some(), table.count > 0);
    raw.push(table.version);
    raw.extend_from_slice(&table.count.to_le_bytes());
    if let Some(kind) = table.kind {
        raw.extend_from_slice(&kind.to_le_bytes());
    }
    let keyed = table.ordinal == 0 && table.kind == Some(1);
    assert_eq!(table.key.len(), if keyed { 16 } else { 0 });
    raw.extend_from_slice(&table.key);
}
fn check_stop(case: &BackendCase) {
    let last = case.tables.last().unwrap();
    assert_eq!(case.complete, last.count == 0);
    let expected = match (last.count, last.ordinal) {
        (0, 1) => "backend_end",
        (count, 0) if count > 0 => "first_table_object_body",
        (count, 1) if count > 0 => "second_table_object_body",
        _ => panic!("invalid backend boundary"),
    };
    assert_eq!(case.stop, expected.as_bytes());
    for (ordinal, table) in case.tables.iter().enumerate() {
        assert_eq!(table.ordinal as usize, ordinal);
    }
}
#[test]
fn every_backend_prefix_rebuilds_all_known_bytes_and_retains_the_exact_opaque_tail() {
    let cases = cases();
    assert_eq!(cases.len(), 1039);
    for case in cases {
        check_case(&case);
    }
}
#[test]
fn native_backend_stops_at_the_first_kind_one_body_without_claiming_record_acceptance() {
    let native = cases().remove(0);
    assert_eq!(native.raw.len(), 13079);
    assert_eq!(native.end, 26);
    assert_eq!(native.tables.len(), 1);
    assert_eq!(native.tables[0].kind, Some(1));
    assert!(!native.complete);
    assert!(!native.full);
    assert!(!native.runtime);
    check_case(&native);
}
