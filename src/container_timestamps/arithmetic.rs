use super::*;
impl ZipTimestampRequest {
    /// Models only explicit native metadata; e.g. `assert_eq!(ZipTimestampRequest { calendar_words: None, packed_override: 7 }.packed_word(), 7)`.
    pub fn packed_word(self) -> u32 {
        if self.packed_override != 0 {
            return self.packed_override;
        }
        self.calendar_words.map_or(0, pack_calendar)
    }
}
fn pack_calendar(words: [u32; 6]) -> u32 {
    let [second, minute, hour, day, month, year] = words;
    let adjusted = adjust_year(year);
    let time = hour
        .wrapping_mul(64)
        .wrapping_add(minute)
        .wrapping_mul(32)
        .wrapping_add(second >> 1);
    let date = adjusted
        .wrapping_mul(16)
        .wrapping_add(month)
        .wrapping_add(1)
        .wrapping_mul(32)
        .wrapping_add(day)
        .wrapping_mul(65536);
    time | date
}

fn adjust_year(year: u32) -> u32 {
    if year > 1980 {
        return year - 1980;
    }
    if year > 80 {
        return year - 80;
    }
    year
}
