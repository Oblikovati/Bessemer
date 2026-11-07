use bessemer::sqlite_image::SqliteValue;

pub(crate) fn write_sqlite_value(encoded: &mut Vec<u8>, cell: &SqliteValue) {
    match cell {
        SqliteValue::Null => encoded.push(0),
        SqliteValue::Integer(number) => {
            encoded.push(1);
            encoded.extend(number.to_le_bytes());
        }
        SqliteValue::RealBits(bits) => {
            encoded.push(2);
            encoded.extend(bits.to_le_bytes());
        }
        SqliteValue::Text(bytes) => {
            encoded.push(3);
            write_sqlite_blob(encoded, bytes);
        }
        SqliteValue::Blob(bytes) => {
            encoded.push(4);
            write_sqlite_blob(encoded, bytes);
        }
    }
}

pub(crate) fn write_sqlite_count(encoded: &mut Vec<u8>, count: usize) {
    encoded.extend(u32::try_from(count).unwrap().to_le_bytes());
}

pub(crate) fn write_sqlite_blob(encoded: &mut Vec<u8>, bytes: &[u8]) {
    write_sqlite_count(encoded, bytes.len());
    encoded.extend(bytes);
}
