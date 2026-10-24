// Integration crates compile this shared module separately and use different subsets.
#![allow(dead_code)]
use bessemer::parasolid::*;

pub const BASE_TEXT: &[u8] = include_bytes!("../fixtures/parasolid/base-schema.s_t");
pub const NEUTRAL: &[u8] = include_bytes!("../fixtures/parasolid/neutral-partition.bin");
pub const DELTAS: &[u8] = include_bytes!("../fixtures/parasolid/neutral-deltas.bin");
pub const LIMITS: TransmitLimits = TransmitLimits {
    maximum_records: 100_000,
    maximum_field_elements: 1_000_000,
};
pub const CASES: &[&[u8]] = &[
    NEUTRAL,
    DELTAS,
    include_bytes!("../fixtures/parasolid/bare-partition.bin"),
    include_bytes!("../fixtures/parasolid/bare-deltas.bin"),
    include_bytes!("../fixtures/parasolid/blank-partition.bin"),
    include_bytes!("../fixtures/parasolid/blank-deltas.bin"),
    include_bytes!("../fixtures/parasolid/neutral-initial-deltas.bin"),
    include_bytes!("../fixtures/parasolid/bare-initial-deltas.bin"),
];

pub fn base() -> BaseSchema {
    BaseSchema::from_bytes(BASE_TEXT).unwrap()
}
pub fn parse(bytes: &[u8]) -> TransmitFile {
    TransmitFile::from_bytes(bytes, &base(), LIMITS).unwrap()
}
pub fn list(contents: &TransmitContents) -> &RecordList {
    match &contents.body {
        TransmitBody::Partition(list) => list,
        TransmitBody::Deltas(sections) => &sections[0].records,
    }
}
pub fn list_mut(contents: &mut TransmitContents) -> &mut RecordList {
    match &mut contents.body {
        TransmitBody::Partition(list) => list,
        TransmitBody::Deltas(sections) => &mut sections[0].records,
    }
}
