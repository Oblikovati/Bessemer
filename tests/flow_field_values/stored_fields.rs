use std::collections::BTreeSet;

pub struct StoredField {
    pub path: String,
    pub offset: u32,
    pub encoding: u8,
    pub count: u32,
    pub raw: Vec<u8>,
}

impl StoredField {
    pub fn width(&self) -> usize {
        match self.encoding {
            0 => 1,
            1 | 2 => 4,
            3 => 8,
            4 => 2,
            other => panic!("Fixture encoding {other}; expected0..4"),
        }
    }
}

pub fn patterns<'a>(fields: impl Iterator<Item = &'a StoredField>, encoding: u8) -> BTreeSet<u64> {
    fields
        .filter(|field| field.encoding == encoding)
        .flat_map(|field| field.raw.chunks_exact(field.width()))
        .map(pattern_value)
        .collect()
}

fn pattern_value(raw: &[u8]) -> u64 {
    match raw.len() {
        4 => u64::from(u32::from_le_bytes(raw.try_into().unwrap())),
        8 => u64::from_le_bytes(raw.try_into().unwrap()),
        other => panic!("Fixture numeric pattern width {other}; expected4/8"),
    }
}
