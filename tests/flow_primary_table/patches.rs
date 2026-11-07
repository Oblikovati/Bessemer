use super::*;

#[test]
fn independently_rebuilt_primary_edits_change_exactly_one_existing_field_and_reverse() {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-primary-table/patches.bin.gz"),
        *b"F4EP",
    );
    let count = word(&mut cursor);
    assert!(count > 1000);
    for _ in 0..count {
        check_patch(&mut cursor);
    }
    finish(&cursor);
}

fn check_patch(cursor: &mut PacketCursor) {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let path = String::from_utf8(blob(cursor)).unwrap();
    let offset = word(cursor) as usize;
    let width = byte(cursor) as usize;
    let before = blob(cursor);
    let after = blob(cursor);
    assert!(matches!(width, 1 | 4 | 8), "{name}:{path}:{width}");
    assert_eq!(before.len(), after.len(), "{name}:{path}");
    assert_eq!(&before[..offset], &after[..offset], "{name}:{path}");
    assert_eq!(&before[offset + width..], &after[offset + width..]);
    assert_ne!(
        &before[offset..offset + width],
        &after[offset..offset + width]
    );
    check_reverse(&before, &after, offset, width);
}

fn check_reverse(before: &[u8], after: &[u8], offset: usize, width: usize) {
    let mut patched = before.to_vec();
    patched[offset..offset + width].copy_from_slice(&after[offset..offset + width]);
    assert_eq!(patched, after);
    patched[offset..offset + width].copy_from_slice(&before[offset..offset + width]);
    assert_eq!(patched, before);
}
