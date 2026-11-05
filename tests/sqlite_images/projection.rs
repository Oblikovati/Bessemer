use bessemer::sqlite_image::{SqliteImage, SqliteRows, SqliteSchemaObject, SqliteValue};

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

fn rows(encoded: &mut Vec<u8>, rows: &SqliteRows) {
    word(encoded, rows.columns().len());
    for name in rows.columns() {
        blob(encoded, name.as_bytes());
    }
    word(encoded, rows.rows().len());
    for row in rows.rows() {
        for cell in row {
            value(encoded, cell);
        }
    }
}

fn value(encoded: &mut Vec<u8>, cell: &SqliteValue) {
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
            blob(encoded, bytes);
        }
        SqliteValue::Blob(bytes) => {
            encoded.push(4);
            blob(encoded, bytes);
        }
    }
}

fn word(encoded: &mut Vec<u8>, count: usize) {
    encoded.extend(u32::try_from(count).unwrap().to_le_bytes());
}

fn blob(encoded: &mut Vec<u8>, bytes: &[u8]) {
    word(encoded, bytes.len());
    encoded.extend(bytes);
}
