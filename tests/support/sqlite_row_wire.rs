use crate::sqlite_wire::{write_sqlite_blob, write_sqlite_count, write_sqlite_value};
use bessemer::sqlite_image::SqliteRows;

pub(crate) fn write_sqlite_rows(encoded: &mut Vec<u8>, rows: &SqliteRows) {
    write_sqlite_count(encoded, rows.columns().len());
    for name in rows.columns() {
        write_sqlite_blob(encoded, name.as_bytes());
    }
    write_sqlite_count(encoded, rows.rows().len());
    for row in rows.rows() {
        for value in row {
            write_sqlite_value(encoded, value);
        }
    }
}
