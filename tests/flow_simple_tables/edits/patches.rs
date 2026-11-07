use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) enum SimpleCell {
    Presence(usize),
    Parallel {
        group: usize,
        array: usize,
        index: usize,
    },
    Leaf {
        row: usize,
        component: usize,
    },
    Five {
        row: usize,
        component: usize,
    },
    Byte(usize),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SimpleBits {
    Signed(i32),
    Wide(u64),
    Word(u32),
    Byte(u8),
}

pub(super) struct SimplePatch {
    pub cell: SimpleCell,
    pub offset: usize,
    pub before: SimpleBits,
    pub after: SimpleBits,
}

pub(super) fn patches(case: &SimpleCase) -> Vec<SimplePatch> {
    match &case.expected {
        ExpectedSimpleTable::Parallel(groups) => parallel_patches(groups),
        ExpectedSimpleTable::LeafPairs(rows) => leaf_patches(rows),
        ExpectedSimpleTable::FiveWords(table) => five_patches(table),
        ExpectedSimpleTable::Bytes(table) => byte_patches(table),
    }
}

fn parallel_patches(groups: &[ParallelGroup; 2]) -> Vec<SimplePatch> {
    let mut result = Vec::new();
    for (group, frozen) in groups.iter().enumerate() {
        if let Some(value) = frozen.presence {
            result.push(presence_patch(group, frozen.start as usize, value));
        }
        for (array, words) in frozen.arrays.iter().enumerate() {
            for (index, &bits) in words.bits.iter().enumerate() {
                result.push(wide_patch(
                    SimpleCell::Parallel {
                        group,
                        array,
                        index,
                    },
                    words.start as usize + index * 8,
                    bits,
                ));
            }
        }
    }
    result
}

fn presence_patch(group: usize, offset: usize, value: i32) -> SimplePatch {
    let after = if value == 0 {
        0
    } else if value == 2 {
        3
    } else {
        2
    };
    SimplePatch {
        cell: SimpleCell::Presence(group),
        offset,
        before: SimpleBits::Signed(value),
        after: SimpleBits::Signed(after),
    }
}

fn wide_patch(cell: SimpleCell, offset: usize, value: u64) -> SimplePatch {
    SimplePatch {
        cell,
        offset,
        before: SimpleBits::Wide(value),
        after: SimpleBits::Wide(value ^ 0x8000000000000001),
    }
}

fn leaf_patches(rows: &[LeafPair]) -> Vec<SimplePatch> {
    let mut result = Vec::new();
    for (row, frozen) in rows.iter().enumerate() {
        for (component, &bits) in frozen.bits.iter().enumerate() {
            result.push(wide_patch(
                SimpleCell::Leaf { row, component },
                frozen.start as usize + component * 8,
                bits,
            ));
        }
    }
    result
}

fn five_patches(table: &FiveWords) -> Vec<SimplePatch> {
    let mut result = Vec::new();
    for (row, words) in table.rows.iter().enumerate() {
        for (component, &value) in words.iter().enumerate() {
            result.push(SimplePatch {
                cell: SimpleCell::Five { row, component },
                offset: table.start as usize + row * 20 + component * 4,
                before: SimpleBits::Word(value),
                after: SimpleBits::Word(value ^ 0x80000001),
            });
        }
    }
    result
}

fn byte_patches(table: &ByteTable) -> Vec<SimplePatch> {
    table
        .bytes
        .iter()
        .enumerate()
        .map(|(index, &value)| SimplePatch {
            cell: SimpleCell::Byte(index),
            offset: table.start as usize + index,
            before: SimpleBits::Byte(value),
            after: SimpleBits::Byte(value ^ 0x80),
        })
        .collect()
}

pub(super) fn bytes(value: SimpleBits) -> Vec<u8> {
    match value {
        SimpleBits::Signed(value) => value.to_le_bytes().to_vec(),
        SimpleBits::Wide(value) => value.to_le_bytes().to_vec(),
        SimpleBits::Word(value) => value.to_le_bytes().to_vec(),
        SimpleBits::Byte(value) => vec![value],
    }
}
