use super::*;
use crate::packet::{PacketCursor, blob, byte, cursor, finish, word};

#[test]
fn all_independent_primary_patches_match_complete_expected_carriers_history_and_fresh_reversals() {
    let mut cursor = cursor(
        include_bytes!("../../fixtures/flow-primary-table/patches.bin.gz"),
        *b"F4EP",
    );
    assert_eq!(word(&mut cursor), 1402);
    for _ in 0..1402 {
        check_request(&mut cursor);
    }
    finish(&cursor);
}

fn check_request(cursor: &mut PacketCursor) {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let path = String::from_utf8(blob(cursor)).unwrap();
    let offset = word(cursor) as usize;
    let width = byte(cursor) as usize;
    let before = blob(cursor);
    let after = blob(cursor);
    let source = open(&before, options()).unwrap();
    let (field, index) = located(&source, offset);
    assert_eq!(field.encoding().width(), width, "{name}:{path}");
    let next = typed(field.encoding(), &after[offset..offset + width]);
    check_noop(&source, field, index);
    let edit = source.replace_field(&field.id(), index, next).unwrap();
    assert_eq!(edit.result().decoded_bytes(), after, "{name}:{path}");
    assert_eq!(
        edit.result().source().as_bytes(),
        carrier(&after).as_bytes()
    );
    assert_eq!(source.decoded_bytes(), before);
    check_reverse(&edit, field.value(index).unwrap(), offset);
    check_history(edit.carrier_edit(), true);
}

fn located(table: &FlowPrimaryTable, offset: usize) -> (FlowPrimaryField<'_>, usize) {
    table
        .fields()
        .find_map(|field| {
            (0..field.layout().element_count)
                .find(|&index| field.element_offset(index) == Some(offset))
                .map(|index| (field, index))
        })
        .unwrap()
}

fn typed(encoding: FlowPrimaryEncoding, raw: &[u8]) -> FlowPrimaryValue {
    match encoding {
        FlowPrimaryEncoding::Byte => FlowPrimaryValue::Byte(raw[0]),
        FlowPrimaryEncoding::Signed32 => {
            FlowPrimaryValue::Signed32(i32::from_le_bytes(raw.try_into().unwrap()))
        }
        FlowPrimaryEncoding::WordBits => {
            FlowPrimaryValue::WordBits(u32::from_le_bytes(raw.try_into().unwrap()))
        }
        FlowPrimaryEncoding::WideBits => {
            FlowPrimaryValue::WideBits(u64::from_le_bytes(raw.try_into().unwrap()))
        }
    }
}

fn check_noop(table: &FlowPrimaryTable, field: FlowPrimaryField<'_>, index: usize) {
    let edit = table
        .replace_field(&field.id(), index, field.value(index).unwrap())
        .unwrap();
    assert_eq!(edit.result().source().as_bytes(), table.source().as_bytes());
    assert_eq!(edit.result().record(), table.record());
    assert_eq!(
        edit.result().field(&field.id()).unwrap().layout(),
        field.layout()
    );
    check_history(edit.carrier_edit(), false);
}

fn check_reverse(edit: &FlowPrimaryTableEdit, original: FlowPrimaryValue, offset: usize) {
    let fresh = open(edit.result().decoded_bytes(), options()).unwrap();
    let (field, index) = located(&fresh, offset);
    let reverse = fresh.replace_field(&field.id(), index, original).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        edit.source().source().as_bytes()
    );
    assert!(
        edit.result()
            .field(&located(edit.source(), offset).0.id())
            .is_err()
    );
}

fn check_history(edit: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edit.history().len(), 3);
    for ((before, after), entry) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .zip(edit.history())
    {
        assert_eq!(entry.source, before.id);
        assert_eq!(entry.result, after.id);
        let relation = if before.id.ordinal() == 1 && changed {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(entry.relation, relation);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}
