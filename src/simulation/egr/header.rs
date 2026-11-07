use super::{EgrHeader, Result, SimulationEgrLimits, invalid, require, wire};

pub(super) fn decode_header(bytes: &[u8], limits: SimulationEgrLimits) -> Result<EgrHeader> {
    require(
        bytes.len() >= 128,
        "EGR header",
        format!(
            "{} bytes; expected at least one complete 128-byte header",
            bytes.len()
        ),
    )?;
    let words = wire::patterns::<9>(&bytes[..36], "EGR header words")?.map(|word| word as i32);
    validate_counts(words, limits)?;
    header_extents(words)
}

fn validate_counts(words: [i32; 9], limits: SimulationEgrLimits) -> Result<()> {
    require(
        words[0] == 128,
        "EGR record stride",
        format!("{}; expected 128", words[0]),
    )?;
    require(
        (0..=32).contains(&words[2]),
        "EGR field count",
        format!("{}; expected 0..=32", words[2]),
    )?;
    require(
        words[1] >= 0 && words[1] as usize <= limits.maximum_rows,
        "EGR capacity",
        format!("{}; expected 0..={}", words[1], limits.maximum_rows),
    )?;
    require(
        words[4] >= 0
            && words[5] >= 0
            && i64::from(words[4]) + i64::from(words[5]) <= i64::from(words[1]),
        "EGR allocated counts",
        format!(
            "live {}, deleted {}, capacity {}; expected nonnegative counts whose sum fits capacity",
            words[4], words[5], words[1]
        ),
    )
}

fn header_extents(words: [i32; 9]) -> Result<EgrHeader> {
    let capacity = words[1] as usize;
    let field_count = words[2] as usize;
    let allocated = words[4] as usize + words[5] as usize;
    let directory_start = 128 + aligned_extent(field_count, 8)?;
    let rows_start = checked_extent(directory_start, aligned_extent(capacity, 4)?, 1)?;
    let expected_bytes = checked_extent(rows_start, allocated, 128)?;
    Ok(EgrHeader {
        words,
        capacity,
        field_count,
        live_count: words[4] as usize,
        free_count: words[5] as usize,
        allocated,
        directory_start,
        rows_start,
        expected_bytes,
    })
}

fn aligned_extent(count: usize, width: usize) -> Result<usize> {
    count
        .checked_mul(width)
        .and_then(|size| size.checked_add(127))
        .map(|size| size / 128 * 128)
        .ok_or_else(|| {
            invalid(
                "EGR aligned extent",
                format!(
                    "count {count}, width {width}; expected representable 128-byte-aligned extent"
                ),
            )
        })
}

fn checked_extent(base: usize, count: usize, width: usize) -> Result<usize> {
    count.checked_mul(width).and_then(|size| base.checked_add(size))
        .ok_or_else(|| invalid("EGR region endpoint", format!("base {base}, count {count}, width {width}; expected representable complete endpoint")))
}
