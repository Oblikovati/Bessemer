use super::{fixture, slot};
use bessemer::{electrical::*, sqlite_image::SqliteValue};

pub(super) fn run_case(case: &fixture::NativeCase, complete: bool) -> usize {
    let archive = fixture::archive(&case.raw);
    let member = fixture::native_member(&archive);
    let keys = if complete {
        case.keys.as_slice()
    } else {
        &case.keys[..1]
    };
    let indices = slot_indices(complete);
    let mut count = 0;
    for key in keys {
        for index in &indices {
            run_slot(&archive, &case.raw, &member, *key, *index);
            count += 1;
        }
    }
    assert!(archive.to_bytes() == case.raw, "{}", case.name);
    count
}

fn slot_indices(complete: bool) -> Vec<usize> {
    if complete {
        return (0..20).collect();
    }
    vec![0, 19]
}

fn run_slot(
    archive: &ElectricalArchive,
    raw: &[u8],
    member: &ElectricalMemberId,
    key: ElectricalUserdataKey,
    index: usize,
) {
    let text = format!("Bessemer #{}/{}/{}\0µ", key.object_id, key.object_no, index);
    let edit = archive
        .edit_userdata_text(member, key, slot(index), &text, fixture::limits())
        .unwrap();
    assert_selected(&edit, key, index, &text);
    assert_eq!(edit.coverage().checked_tables, 48);
    assert_eq!(edit.source().to_bytes(), raw);
    compare_unselected_payloads(&edit, raw);
    let noop = archive
        .edit_userdata_text(member, key, slot(index), "", fixture::limits())
        .unwrap();
    assert_eq!(noop.result().to_bytes(), raw);
    assert_eq!(noop.completion(), ElectricalEditCompletion::Unchanged);
}

fn assert_selected(
    edit: &ElectricalUserdataWrite,
    key: ElectricalUserdataKey,
    index: usize,
    text: &str,
) {
    let read = edit
        .result()
        .userdata_by_key(&edit.history().result_member, key, fixture::limits())
        .unwrap();
    assert_eq!(
        read.rows().rows()[0][index + 2],
        SqliteValue::Text(text.as_bytes().into())
    );
    assert_eq!(edit.completion(), ElectricalEditCompletion::Changed);
}

fn compare_unselected_payloads(edit: &ElectricalUserdataWrite, source_raw: &[u8]) {
    let result_raw = edit.result().to_bytes();
    let sources: Vec<_> = edit.source().members().collect();
    let results: Vec<_> = edit.result().members().collect();
    assert_eq!(sources.len(), results.len());
    for (source, result) in sources.iter().zip(results.iter()) {
        if source.id == edit.history().source_member {
            continue;
        }
        assert_eq!(source.name, result.name);
        assert_eq!(source.crc32, result.crc32);
        let before = edit.source().member_metadata(&source.id).unwrap();
        let after = edit.result().member_metadata(&result.id).unwrap();
        assert_eq!(
            payload(source_raw, before.header_offset, source.stored_bytes),
            payload(&result_raw, after.header_offset, result.stored_bytes)
        );
    }
}

fn payload(raw: &[u8], offset: usize, stored: u32) -> &[u8] {
    assert_eq!(&raw[offset..offset + 4], b"PK\x03\x04");
    let name = u16::from_le_bytes(raw[offset + 26..offset + 28].try_into().unwrap()) as usize;
    let extra = u16::from_le_bytes(raw[offset + 28..offset + 30].try_into().unwrap()) as usize;
    let start = offset + 30 + name + extra;
    &raw[start..start + stored as usize]
}
