use crate::sqlite_wire::{
    write_sqlite_blob as blob, write_sqlite_count as word, write_sqlite_value as value,
};
use bessemer::electrical::*;

pub(super) fn render(maps: &[ElectricalFileReadMap]) -> Vec<u8> {
    let mut encoded = Vec::new();
    word(&mut encoded, maps.len());
    for mapped in maps {
        blob(&mut encoded, &field_map(mapped));
    }
    encoded
}

fn field_map(mapped: &ElectricalFileReadMap) -> Vec<u8> {
    let mut encoded = Vec::new();
    stored_record(&mut encoded, mapped.record());
    word(&mut encoded, mapped.reads().len());
    for read in mapped.reads() {
        field_read(&mut encoded, read);
    }
    word(&mut encoded, mapped.unmapped_columns().count());
    for (name, cell) in mapped.unmapped_columns() {
        blob(&mut encoded, name.as_bytes());
        value(&mut encoded, cell);
    }
    encoded
}

fn stored_record(encoded: &mut Vec<u8>, record: &ElectricalRecord) {
    word(encoded, record.columns().len());
    for (name, cell) in record.columns().iter().zip(record.values()) {
        blob(encoded, name.as_bytes());
        value(encoded, cell);
    }
}

fn field_read(encoded: &mut Vec<u8>, read: &ElectricalFileRead) {
    blob(encoded, read.column.as_bytes());
    encoded.push(match read.reader {
        ElectricalFileReaderKind::Long => 1,
        ElectricalFileReaderKind::Bool => 2,
        ElectricalFileReaderKind::Double => 3,
        ElectricalFileReaderKind::Str => 4,
        ElectricalFileReaderKind::Date => 5,
    });
    destination(encoded, read.destination);
    value(encoded, &read.value);
}

fn destination(encoded: &mut Vec<u8>, destination: ElectricalFileReadDestination) {
    match destination {
        ElectricalFileReadDestination::ObjectByteOffset(offset) => {
            encoded.push(0);
            encoded.extend(offset.to_le_bytes());
        }
        ElectricalFileReadDestination::IdentityTemporary => encoded.push(1),
        ElectricalFileReadDestination::SharedLongTemporary => encoded.push(2),
        ElectricalFileReadDestination::SharedBoolTemporary => encoded.push(3),
        ElectricalFileReadDestination::SharedDoubleTemporary => encoded.push(4),
    }
}
