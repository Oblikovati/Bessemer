use bessemer::sqlite_image::SqliteLimits;
use std::io::{Cursor, Read};

pub(super) struct Case {
    pub name: String,
    pub image: Vec<u8>,
    pub expected: Vec<u8>,
}

pub(super) fn limits() -> SqliteLimits {
    SqliteLimits {
        maximum_image_bytes: 32 << 20,
        maximum_schema_objects: 4000,
        maximum_rows: 100_000,
        maximum_columns: 2000,
        maximum_value_bytes: 16 << 20,
        maximum_output_bytes: 128 << 20,
        maximum_vm_steps: 100_000_000,
    }
}

pub(super) fn cases() -> Vec<Case> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/sqlite-images/images.bin.gz")[..])
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(take(&mut cursor, 4), b"SQI1");
    let count = word(&mut cursor);
    let cases = (0..count)
        .map(|_| Case {
            name: String::from_utf8(blob(&mut cursor)).unwrap(),
            image: blob(&mut cursor),
            expected: blob(&mut cursor),
        })
        .collect();
    assert_eq!(cursor.position() as usize, raw.len());
    cases
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let count = word(cursor) as usize;
    take(cursor, count)
}

fn word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(take(cursor, 4).try_into().unwrap())
}

fn take(cursor: &mut Cursor<&[u8]>, count: usize) -> Vec<u8> {
    let mut bytes = vec![0; count];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}
