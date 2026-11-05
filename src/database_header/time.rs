/// Six raw native32 inputs to the database header time packer (9.2 / C-3597).
/// These fields are arithmetic inputs, including out-of-calendar and signed values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseTimeComponents {
    /// Raw year input; the packed representation wraps its year contribution.
    pub year: i32,
    /// Raw month input; the default Windows callback supplies month minus one.
    pub month: i32,
    /// Raw day input, without calendar validation or masking before packing.
    pub day: i32,
    /// Raw hour input, without a 24-hour restriction.
    pub hour: i32,
    /// Raw minute input, without a 60-minute restriction.
    pub minute: i32,
    /// Raw seconds input; packing uses an arithmetic right shift by one.
    pub seconds: i32,
}

impl DatabaseTimeComponents {
    /// Extracts the stored bit fields; e.g. `assert_eq!(DatabaseTimeComponents::from_packed_word(0).year, 1980)`.
    pub fn from_packed_word(word: u32) -> Self {
        Self {
            year: ((word >> 25) + 1980) as i32,
            month: ((word >> 21) & 15) as i32,
            day: ((word >> 16) & 31) as i32,
            hour: ((word >> 11) & 31) as i32,
            minute: ((word >> 5) & 63) as i32,
            seconds: (2 * (word & 31)) as i32,
        }
    }

    /// Packs with native wrapping arithmetic; e.g. `assert_eq!(DatabaseTimeComponents::from_packed_word(u32::MAX).to_packed_word(), u32::MAX)`.
    pub fn to_packed_word(self) -> u32 {
        let date = (self.year as u32)
            .wrapping_add(0x44)
            .wrapping_shl(4)
            .wrapping_add(self.month as u32)
            .wrapping_shl(21)
            .wrapping_add((self.day as u32).wrapping_shl(16));
        let clock = (self.hour as u32)
            .wrapping_shl(6)
            .wrapping_add(self.minute as u32)
            .wrapping_shl(5)
            .wrapping_add((self.seconds >> 1) as u32);
        date | clock
    }
}
