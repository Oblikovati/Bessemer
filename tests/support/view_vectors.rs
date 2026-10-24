use bessemer::display_lists::DisplayVersion;

pub fn read(bytes: &'static [u8], expected: usize) -> Vec<(DisplayVersion, u8, &'static [u8])> {
    let count = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
    let mut offset = 4;
    let mut records = Vec::new();
    for _ in 0..count {
        let version = DisplayVersion::new(u32::from_le_bytes(
            bytes[offset..offset + 4].try_into().unwrap(),
        ));
        let scenario = bytes[offset + 4];
        let length = u32::from_le_bytes(bytes[offset + 5..offset + 9].try_into().unwrap()) as usize;
        records.push((version, scenario, &bytes[offset + 9..offset + 9 + length]));
        offset += 9 + length;
    }
    assert_eq!(offset, bytes.len());
    assert_eq!(records.len(), expected);
    records
}
