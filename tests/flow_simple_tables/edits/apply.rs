use super::*;

pub(super) struct SimpleEditView {
    pub source: SimpleView,
    pub result: SimpleView,
    pub carrier: FlowCarrierEdit,
}

pub(super) fn apply(
    source: &SimpleView,
    cell: SimpleCell,
    value: SimpleBits,
) -> bessemer::Result<SimpleEditView> {
    match source {
        SimpleView::Parallel(table) => parallel(table, cell, value),
        SimpleView::LeafPairs(table) => leaf(table, cell, value),
        SimpleView::Five(table) => five(table, cell, value),
        SimpleView::Bytes(table) => byte(table, cell, value),
    }
}

fn completed<T: Clone + Send + Sync>(
    edit: FlowSimpleTableEdit<T>,
    wrap: fn(T) -> SimpleView,
) -> SimpleEditView {
    SimpleEditView {
        source: wrap(edit.source().clone()),
        result: wrap(edit.result().clone()),
        carrier: edit.carrier_edit().clone(),
    }
}

fn parallel(
    table: &FlowParallelTable,
    cell: SimpleCell,
    value: SimpleBits,
) -> bessemer::Result<SimpleEditView> {
    let edit = match (cell, value) {
        (SimpleCell::Presence(group), SimpleBits::Signed(value)) => {
            table.replace_presence(group, value)
        }
        (
            SimpleCell::Parallel {
                group,
                array,
                index,
            },
            SimpleBits::Wide(bits),
        ) => table.replace_word(group, array, index, bits),
        _ => panic!("parallel fixture {cell:?} {value:?}; expected presence or wide cell"),
    }?;
    Ok(completed(edit, SimpleView::Parallel))
}

fn leaf(
    table: &FlowLeafPairs,
    cell: SimpleCell,
    value: SimpleBits,
) -> bessemer::Result<SimpleEditView> {
    let (SimpleCell::Leaf { row, component }, SimpleBits::Wide(bits)) = (cell, value) else {
        panic!("leaf fixture {cell:?} {value:?}; expected wide cell")
    };
    table
        .replace_word(row, component, bits)
        .map(|edit| completed(edit, SimpleView::LeafPairs))
}

fn five(
    table: &FlowFiveWordTable,
    cell: SimpleCell,
    value: SimpleBits,
) -> bessemer::Result<SimpleEditView> {
    let (SimpleCell::Five { row, component }, SimpleBits::Word(bits)) = (cell, value) else {
        panic!("five-word fixture {cell:?} {value:?}; expected raw word cell")
    };
    table
        .replace_word(row, component, bits)
        .map(|edit| completed(edit, SimpleView::Five))
}

fn byte(
    table: &FlowRawByteTable,
    cell: SimpleCell,
    value: SimpleBits,
) -> bessemer::Result<SimpleEditView> {
    let (SimpleCell::Byte(index), SimpleBits::Byte(value)) = (cell, value) else {
        panic!("byte fixture {cell:?} {value:?}; expected byte cell")
    };
    table
        .replace_byte(index, value)
        .map(|edit| completed(edit, SimpleView::Bytes))
}
