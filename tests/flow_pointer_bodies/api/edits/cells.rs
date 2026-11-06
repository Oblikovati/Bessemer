use super::*;

#[derive(Clone, Copy)]
pub(super) enum BodyCell {
    Pool(usize, usize),
    Wide(usize, usize, usize),
    Vector(usize, usize, usize, usize),
    Raw(usize, usize, usize),
    Tag(usize, usize),
}

impl BodyCell {
    pub(super) fn is_wide(self) -> bool {
        matches!(self, Self::Pool(..) | Self::Wide(..))
    }
    pub(super) fn apply(
        self,
        bodies: &FlowPointerBodies,
        value: u64,
    ) -> bessemer::Result<FlowPointerBodyEdit> {
        match self {
            Self::Pool(pool, index) => bodies.replace_pool_word(pool, index, value),
            Self::Wide(group, entry, index) => {
                bodies.replace_entry_word(group, entry, index, value)
            }
            Self::Vector(group, entry, vector, index) => {
                bodies.replace_vector_byte(group, entry, vector, index, value as u8)
            }
            Self::Raw(group, entry, index) => {
                bodies.replace_raw_byte(group, entry, index, value as u8)
            }
            Self::Tag(group, entry) => bodies.replace_tag(group, entry, value as u8),
        }
    }
    pub(super) fn original(self, bodies: &FlowPointerBodies) -> (usize, u64) {
        let offset = self.offset(bodies);
        let bytes = bodies.decoded_bytes();
        let bits = if self.is_wide() {
            u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
        } else {
            u64::from(bytes[offset])
        };
        (offset, bits)
    }
    fn offset(self, bodies: &FlowPointerBodies) -> usize {
        if let Self::Pool(pool, index) = self {
            return bodies.pool(pool).unwrap().wide_words().payload_interval().0 + index * 8;
        }
        let (group, ordinal) = self.path();
        let entry = bodies.entry(group, ordinal).unwrap();
        match self {
            Self::Wide(_, _, index) => entry.wide_words().unwrap().payload_interval().0 + index * 8,
            Self::Vector(_, _, vector, index) => entry.vector_interval(vector).unwrap().0 + index,
            Self::Raw(_, _, index) => entry.raw_interval().unwrap().0 + index,
            Self::Tag(..) => entry.payload_interval().0,
            Self::Pool(..) => panic!("pool fixture; expected early pool offset return"),
        }
    }
    fn path(self) -> (usize, usize) {
        match self {
            Self::Wide(group, entry, _)
            | Self::Vector(group, entry, _, _)
            | Self::Raw(group, entry, _)
            | Self::Tag(group, entry) => (group, entry),
            Self::Pool(..) => panic!("pool fixture; expected collection entry"),
        }
    }
    pub(super) fn replacement(self, old: u64) -> u64 {
        if let Self::Tag(..) = self {
            return if old == 1 || old == 2 {
                old
            } else if old == 255 {
                0
            } else {
                255
            };
        }
        old ^ if self.is_wide() {
            0x8000000000000001
        } else {
            255
        }
    }
}

pub(super) fn edit_cells(bodies: &FlowPointerBodies, complete: bool) -> Vec<BodyCell> {
    let mut cells = Vec::new();
    for (ordinal, pool) in bodies.pools().enumerate() {
        cells.extend(
            indices(pool.wide_words().word_count(), complete)
                .into_iter()
                .map(|index| BodyCell::Pool(ordinal, index)),
        );
    }
    for group in 0..3 {
        for ordinal in 0..bodies.entry_count(group).unwrap() {
            entry_cells(&mut cells, bodies.entry(group, ordinal).unwrap(), complete);
        }
    }
    cells
}

fn entry_cells(cells: &mut Vec<BodyCell>, entry: FlowPointerBodyEntry<'_>, complete: bool) {
    let (group, ordinal) = entry.path();
    if let Some(words) = entry.wide_words() {
        cells.extend(
            indices(words.word_count(), complete)
                .into_iter()
                .map(|index| BodyCell::Wide(group, ordinal, index)),
        );
    }
    if let Some(raw) = entry.raw_payload() {
        cells.extend(
            indices(raw.len(), complete)
                .into_iter()
                .map(|index| BodyCell::Raw(group, ordinal, index)),
        );
    }
    vector_cells(cells, entry, complete);
    if entry.tag_byte().is_some() {
        cells.push(BodyCell::Tag(group, ordinal));
    }
}

fn vector_cells(cells: &mut Vec<BodyCell>, entry: FlowPointerBodyEntry<'_>, complete: bool) {
    let (group, ordinal) = entry.path();
    for vector in 0..entry.vector_count() {
        cells.extend(
            indices(entry.vector(vector).unwrap().len(), complete)
                .into_iter()
                .map(|index| BodyCell::Vector(group, ordinal, vector, index)),
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
