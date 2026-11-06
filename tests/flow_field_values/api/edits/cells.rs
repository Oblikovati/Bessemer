use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) enum FieldBits {
    Table(i32),
    Wide(u64),
}

impl FieldBits {
    pub(super) fn bytes(self) -> Vec<u8> {
        match self {
            Self::Table(value) => value.to_le_bytes().to_vec(),
            Self::Wide(bits) => bits.to_le_bytes().to_vec(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum FieldCell {
    Table(usize),
    Object(usize, usize),
    Region(usize, usize),
}

impl FieldCell {
    pub(super) fn apply(
        self,
        view: &FlowFieldValues,
        value: FieldBits,
    ) -> bessemer::Result<FlowFieldValueEdit> {
        match (self, value) {
            (Self::Table(index), FieldBits::Table(value)) => {
                view.replace_width_table_word(index, value)
            }
            (Self::Object(object, index), FieldBits::Wide(bits)) => {
                view.replace_object_word(object, index, bits)
            }
            (Self::Region(region, index), FieldBits::Wide(bits)) => {
                view.replace_region_word(region, index, bits)
            }
            _ => panic!(
                "cell {self:?} value {value:?}; expected a matching table word or wide pattern"
            ),
        }
    }
    pub(super) fn original(self, view: &FlowFieldValues) -> (usize, FieldBits) {
        match self {
            Self::Table(index) => (
                index * 4,
                FieldBits::Table(view.width_table_word(index).unwrap()),
            ),
            Self::Object(object, index) => {
                original_wide(view.object(object).unwrap().wide_words(), index)
            }
            Self::Region(region, index) => {
                original_wide(view.region(region).unwrap().wide_words(), index)
            }
        }
    }
    pub(super) fn replacement(self, view: &FlowFieldValues, old: FieldBits) -> FieldBits {
        match (self, old) {
            (Self::Table(index), FieldBits::Table(value)) => {
                let FlowFieldLayout::Legacy(layout) = &view.options().layout else {
                    panic!("table; expected legacy layout");
                };
                if layout
                    .expected_width_table()
                    .is_some_and(|expected| expected[index].is_some())
                {
                    return old;
                }
                FieldBits::Table(value ^ 1)
            }
            (_, FieldBits::Wide(bits)) => FieldBits::Wide(bits ^ 0x8000000000000001),
            _ => panic!("cell {self:?}; expected matching fixture bits"),
        }
    }
}

fn original_wide(array: FlowTableWideArray<'_>, index: usize) -> (usize, FieldBits) {
    (
        array.payload_interval().0 + index * 8,
        FieldBits::Wide(array.raw_bits(index).unwrap()),
    )
}

pub(super) fn edit_cells(view: &FlowFieldValues, complete: bool) -> Vec<FieldCell> {
    let mut cells = Vec::new();
    if let Some(table) = view.width_table() {
        cells.extend((0..table.word_count()).map(FieldCell::Table));
    }
    for object in view.objects() {
        cells.extend(
            indices(object.wide_words().word_count(), complete)
                .into_iter()
                .map(|index| FieldCell::Object(object.ordinal(), index)),
        );
    }
    for region in view.regions() {
        cells.extend(
            indices(region.wide_words().word_count(), complete)
                .into_iter()
                .map(|index| FieldCell::Region(region.ordinal(), index)),
        );
    }
    cells
}

fn indices(count: usize, complete: bool) -> Vec<usize> {
    if complete {
        return (0..count).collect();
    }
    match count {
        0 => vec![],
        1 => vec![0],
        _ => vec![0, count - 1],
    }
}
