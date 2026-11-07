use crate::sqlite_row_wire::write_sqlite_rows as rows;
use crate::sqlite_wire::{
    write_sqlite_blob as blob, write_sqlite_count as word, write_sqlite_value as value,
};
use bessemer::sqlite_image::{SqliteImage, SqliteSchemaObject, SqliteValue};

pub(super) fn render(image: &SqliteImage) -> Vec<u8> {
    let mut encoded = Vec::new();
    word(&mut encoded, image.objects().len());
    for object in image.objects() {
        encode_object(&mut encoded, object, image);
    }
    encoded
}

fn encode_object(encoded: &mut Vec<u8>, object: &SqliteSchemaObject, image: &SqliteImage) {
    for spelling in [&object.kind, &object.name, &object.table] {
        blob(encoded, spelling.as_bytes());
    }
    encoded.extend(object.root_page.to_le_bytes());
    let sql = object
        .sql
        .as_ref()
        .map(|sql| SqliteValue::Text(sql.as_bytes().into()))
        .unwrap_or(SqliteValue::Null);
    value(encoded, &sql);
    encoded.push(u8::from(object.kind == "table"));
    if object.kind != "table" {
        return;
    }
    rows(encoded, object.columns.as_ref().unwrap());
    rows(encoded, object.foreign_keys.as_ref().unwrap());
    rows(encoded, &image.table_rows(&object.name).unwrap());
}
