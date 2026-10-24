use bessemer::display_lists::*;
pub const LIMITS: DisplayViewLimits = DisplayViewLimits {
    maximum_bytes: 1_000_000,
    maximum_views: 100,
    maximum_nested_entries: 1000,
};
const VECTORS: &[u8] = include_bytes!("../fixtures/display-lists/named-views.bin");

pub fn vectors() -> Vec<(DisplayVersion, u8, &'static [u8])> {
    let mut offset = 4;
    let count = u32::from_le_bytes(VECTORS[..4].try_into().unwrap());
    let mut records = Vec::new();
    for _ in 0..count {
        let version = DisplayVersion::new(u32::from_le_bytes(
            VECTORS[offset..offset + 4].try_into().unwrap(),
        ));
        let scenario = VECTORS[offset + 4];
        let length =
            u32::from_le_bytes(VECTORS[offset + 5..offset + 9].try_into().unwrap()) as usize;
        records.push((version, scenario, &VECTORS[offset + 9..offset + 9 + length]));
        offset += 9 + length;
    }
    assert_eq!(offset, VECTORS.len());
    assert_eq!(records.len(), 60);
    records
}
