use super::{Result, SimulationPersistScalarProfile, invalid, require};
use crate::records::{dword, region};

#[derive(Debug, Clone, Copy)]
pub(super) struct ScalarLayout {
    pub used: usize,
    pub capacity: Option<i32>,
    pub header: usize,
    pub width: usize,
}

pub(super) fn decode(
    bytes: &[u8],
    profile: SimulationPersistScalarProfile,
    maximum: usize,
) -> Result<ScalarLayout> {
    let used = used_count(bytes, maximum)?;
    let layout = match profile {
        SimulationPersistScalarProfile::CountedI32 => ScalarLayout {
            used,
            capacity: None,
            header: 4,
            width: 4,
        },
        SimulationPersistScalarProfile::UniqueIdsI64 => unique_layout(bytes, used)?,
    };
    let expected = frame_extent(layout.header, layout.used, layout.width)?;
    require(
        bytes.len() == expected,
        "scalar frame extent",
        format!(
            "{} bytes for {profile:?} with {used} entries; expected exactly {expected}",
            bytes.len()
        ),
    )?;
    Ok(layout)
}

fn unique_layout(bytes: &[u8], used: usize) -> Result<ScalarLayout> {
    let capacity = dword(bytes, 4)? as i32;
    require(
        capacity >= 0 && capacity as usize >= used,
        "UniqueIds capacity",
        format!(
            "{capacity} for {used} used entries; expected a nonnegative signed-i32 capacity at least {used}"
        ),
    )?;
    Ok(ScalarLayout {
        used,
        capacity: Some(capacity),
        header: 8,
        width: 8,
    })
}

pub(super) fn frame_extent(header: usize, count: usize, width: usize) -> Result<usize> {
    count.checked_mul(width).and_then(|size| header.checked_add(size)).ok_or_else(|| invalid("scalar frame extent", format!("header {header}, {count} entries of width {width}; expected representable complete byte extent")))
}

pub(super) fn i64_word(bytes: &[u8], offset: usize) -> Result<i64> {
    let raw = region(bytes, offset, 8)?;
    Ok(i64::from_le_bytes([
        raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
    ]))
}

fn used_count(bytes: &[u8], maximum: usize) -> Result<usize> {
    let signed_used = dword(bytes, 0)? as i32;
    require(
        signed_used >= 0 && signed_used as usize <= maximum,
        "scalar used count",
        format!("{signed_used}; expected a nonnegative signed-i32 count at most {maximum}"),
    )?;
    Ok(signed_used as usize)
}

#[cfg(test)]
mod tests {
    use super::frame_extent;

    #[test]
    fn complete_scalar_extent_guards_multiplication_and_header_overflow() {
        assert_eq!(frame_extent(8, 0, 8).unwrap(), 8);
        assert_eq!(frame_extent(4, 3, 4).unwrap(), 16);
        assert!(frame_extent(8, usize::MAX, 8).is_err());
        assert!(frame_extent(usize::MAX, 1, 1).is_err());
    }
}
