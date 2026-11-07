use super::*;
use FlowBlockFieldValue as Value;
use FlowReferenceEntryField as Field;

#[derive(Clone, Copy, Debug)]
pub(super) enum ReferenceBits {
    Scalar(Value),
    Wide(u64),
}

impl ReferenceBits {
    pub(super) fn bytes(self) -> Vec<u8> {
        match self {
            Self::Scalar(Value::Byte(value)) => vec![value],
            Self::Scalar(Value::Word16(value)) => value.to_le_bytes().to_vec(),
            Self::Scalar(Value::Signed32(value)) => value.to_le_bytes().to_vec(),
            Self::Scalar(Value::Word32(value)) => value.to_le_bytes().to_vec(),
            Self::Wide(value) => value.to_le_bytes().to_vec(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ReferenceCell {
    Header(FlowReferenceHeaderField),
    Entry(usize, usize, Field),
    Level(usize),
    Mapping(usize, usize),
    Wide(usize, usize),
}

impl ReferenceCell {
    pub(super) fn apply(
        self,
        view: &FlowReferenceGroups,
        value: ReferenceBits,
    ) -> bessemer::Result<FlowReferenceGroupEdit> {
        match (self, value) {
            (Self::Header(field), ReferenceBits::Scalar(Value::Signed32(value))) => {
                view.replace_header(field, value)
            }
            (Self::Entry(group, entry, field), ReferenceBits::Scalar(value)) => {
                view.replace_entry(group, entry, field, value)
            }
            (Self::Level(group), ReferenceBits::Scalar(Value::Signed32(value))) => {
                view.replace_refinement_level(group, value)
            }
            (Self::Mapping(group, index), ReferenceBits::Scalar(Value::Signed32(value))) => {
                view.replace_mapping_word(group, index, value)
            }
            (Self::Wide(group, index), ReferenceBits::Wide(bits)) => {
                view.replace_wide_word(group, index, bits)
            }
            _ => panic!("cell {self:?} value {value:?}; expected matching fixture scalar kind"),
        }
    }
    pub(super) fn original(self, view: &FlowReferenceGroups) -> (usize, usize, ReferenceBits) {
        if let Self::Header(field) = self {
            return header_original(view, field);
        }
        if let Self::Entry(group, ordinal, field) = self {
            let entry = view.group(group).unwrap().entry(ordinal).unwrap();
            let (start, end) = entry.field_interval(field).unwrap();
            return (
                start,
                end - start,
                ReferenceBits::Scalar(entry.field(field).unwrap()),
            );
        }
        self.group_original(view)
    }
    fn group_original(self, view: &FlowReferenceGroups) -> (usize, usize, ReferenceBits) {
        match self {
            Self::Level(group) => level_original(view.group(group).unwrap()),
            Self::Mapping(group, index) => mapping_original(view.group(group).unwrap(), index),
            Self::Wide(group, index) => wide_original(view.group(group).unwrap(), index),
            _ => panic!("cell {self:?}; expected a group array or refinement level"),
        }
    }
    pub(super) fn replacement(
        self,
        view: &FlowReferenceGroups,
        old: ReferenceBits,
    ) -> ReferenceBits {
        if matches!(self, Self::Entry(_, _, Field::Descriptor(6))) {
            return old;
        }
        if matches!(self, Self::Entry(_, _, Field::SecondCount)) {
            return secondary_replacement(view.version(), old);
        }
        toggled(old)
    }
}

fn secondary_replacement(version: i32, old: ReferenceBits) -> ReferenceBits {
    if let ReferenceBits::Scalar(Value::Signed32(value)) = old {
        if value < 0 {
            return ReferenceBits::Scalar(Value::Signed32(value ^ 1));
        }
        if value == 0 && (version == 1 || version >= 10) {
            return ReferenceBits::Scalar(Value::Signed32(-1));
        }
    }
    old
}

pub(super) fn edit_cells(view: &FlowReferenceGroups, complete: bool) -> Vec<ReferenceCell> {
    let mut cells = Vec::new();
    for field in [
        FlowReferenceHeaderField::Discarded,
        FlowReferenceHeaderField::Stored,
    ] {
        if view.header(field).is_some() {
            cells.push(ReferenceCell::Header(field));
        }
    }
    for group in view.groups() {
        group_cells(&mut cells, group, complete);
        for entry in group.entries() {
            entry_cells(&mut cells, entry, complete);
        }
    }
    cells
}

fn group_cells(cells: &mut Vec<ReferenceCell>, group: FlowReferenceGroup<'_>, complete: bool) {
    if let Some(array) = group.mapping_words() {
        cells.push(ReferenceCell::Level(group.ordinal()));
        cells.extend(
            indices(array.word_count(), complete)
                .into_iter()
                .map(|index| ReferenceCell::Mapping(group.ordinal(), index)),
        );
    }
    if let Some(array) = group.wide_words() {
        cells.extend(
            indices(array.word_count(), complete)
                .into_iter()
                .map(|index| ReferenceCell::Wide(group.ordinal(), index)),
        );
    }
}

fn entry_cells(cells: &mut Vec<ReferenceCell>, entry: FlowReferenceEntry<'_>, complete: bool) {
    let (group, ordinal) = entry.path();
    scalar_cells(cells, entry);
    cells
        .extend((0..7).map(|index| ReferenceCell::Entry(group, ordinal, Field::Descriptor(index))));
    for (count, field) in [
        (
            entry.reference_count(),
            Field::Reference as fn(usize) -> Field,
        ),
        (entry.first_words().word_count(), Field::FirstWord),
        (entry.second_words().word_count(), Field::SecondWord),
    ] {
        cells.extend(
            indices(count, complete)
                .into_iter()
                .map(|index| ReferenceCell::Entry(group, ordinal, field(index))),
        );
    }
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

fn toggled(old: ReferenceBits) -> ReferenceBits {
    match old {
        ReferenceBits::Scalar(Value::Byte(value)) => {
            ReferenceBits::Scalar(Value::Byte(value ^ 128))
        }
        ReferenceBits::Scalar(Value::Word16(value)) => {
            ReferenceBits::Scalar(Value::Word16(value ^ 1))
        }
        ReferenceBits::Scalar(Value::Signed32(value)) => {
            ReferenceBits::Scalar(Value::Signed32(value ^ 1))
        }
        ReferenceBits::Scalar(Value::Word32(value)) => {
            ReferenceBits::Scalar(Value::Word32(value ^ 0x80000001))
        }
        ReferenceBits::Wide(value) => ReferenceBits::Wide(value ^ 0x8000000000000001),
    }
}

fn header_original(
    view: &FlowReferenceGroups,
    field: FlowReferenceHeaderField,
) -> (usize, usize, ReferenceBits) {
    let offset = if field == FlowReferenceHeaderField::Discarded || view.version() >= 6 {
        4
    } else {
        8
    };
    let bits = ReferenceBits::Scalar(Value::Signed32(view.header(field).unwrap()));
    (offset, 4, bits)
}
fn level_original(group: FlowReferenceGroup<'_>) -> (usize, usize, ReferenceBits) {
    let start = group.mapping_words().unwrap().payload_interval().0 - 4;
    let bits = ReferenceBits::Scalar(Value::Signed32(group.refinement_level().unwrap()));
    (start, 4, bits)
}
fn mapping_original(group: FlowReferenceGroup<'_>, index: usize) -> (usize, usize, ReferenceBits) {
    let array = group.mapping_words().unwrap();
    let bits = ReferenceBits::Scalar(Value::Signed32(array.raw_word(index).unwrap() as i32));
    (array.payload_interval().0 + index * 4, 4, bits)
}
fn wide_original(group: FlowReferenceGroup<'_>, index: usize) -> (usize, usize, ReferenceBits) {
    let array = group.wide_words().unwrap();
    let bits = ReferenceBits::Wide(array.raw_bits(index).unwrap());
    (array.payload_interval().0 + index * 8, 8, bits)
}

fn scalar_cells(cells: &mut Vec<ReferenceCell>, entry: FlowReferenceEntry<'_>) {
    let (group, ordinal) = entry.path();
    for field in [
        Field::Member20,
        Field::DiscardedPrefix,
        Field::State,
        Field::Flag2,
        Field::Flag4,
        Field::SecondCount,
        Field::DiscardedSuffix,
    ] {
        if entry.field(field).is_ok() {
            cells.push(ReferenceCell::Entry(group, ordinal, field));
        }
    }
}
