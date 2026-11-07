use super::header::{END, FREE};
use super::{Result, require};
use std::ops::Range;

// #110: MS-CFB 2.8 reserves this entire version-4 sector for byte-range locking.
const LOCK_BASE: usize = 0x7fff_f000;
const TWO_GIB: usize = 0x8000_0000;

pub(super) fn lock_sector(size: usize) -> Option<usize> {
    (size == 4096).then_some(0x7_fffe)
}

pub(super) fn physical_sector(logical: usize, size: usize) -> usize {
    if lock_sector(size).is_some_and(|reserved| logical >= reserved) {
        return logical + 1;
    }
    logical
}

pub(super) fn physical_count(logical: usize, size: usize) -> usize {
    if lock_sector(size).is_some_and(|reserved| logical > reserved) {
        return logical + 1;
    }
    logical
}

pub(super) fn lock_state(marker: u32, file_bytes: usize) -> Result<bool> {
    let allocated = marker == END;
    require(
        allocated || (file_bytes <= TWO_GIB && marker == FREE),
        format!(
            "range-lock FAT marker {marker:#010x} in {file_bytes}-byte carrier; expected ENDOFCHAIN above 2 GiB or FREESECT at the boundary"
        ),
    )?;
    Ok(allocated)
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CopySegment {
    pub output: usize,
    pub source: Range<usize>,
}

pub(super) fn copy_segments(offset: usize, count: usize, size: usize) -> [CopySegment; 2] {
    let prefix = if size == 4096 {
        LOCK_BASE.saturating_sub(offset).min(count)
    } else {
        count
    };
    let suffix_offset = offset + prefix + if size == 4096 { size } else { 0 };
    [
        CopySegment {
            output: offset,
            source: 0..prefix,
        },
        CopySegment {
            output: suffix_offset,
            source: prefix..count,
        },
    ]
}

#[cfg(test)]
mod tests;
