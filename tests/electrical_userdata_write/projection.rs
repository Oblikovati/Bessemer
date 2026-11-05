use crate::sqlite_row_wire::write_sqlite_rows;
use crate::sqlite_wire::{write_sqlite_blob, write_sqlite_count};
use bessemer::sqlite_image::SqliteImage;

pub(super) fn logical_snapshot(image: &SqliteImage) -> Vec<u8> {
    let tables: Vec<_> = image
        .objects()
        .iter()
        .filter(|object| object.kind == "table")
        .collect();
    let mut encoded = Vec::new();
    write_sqlite_count(&mut encoded, tables.len());
    for object in tables {
        write_sqlite_blob(&mut encoded, object.name.as_bytes());
        write_sqlite_rows(&mut encoded, &image.table_rows(&object.name).unwrap());
    }
    encoded
}
