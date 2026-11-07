use super::{ArchiveCursor, StoredDouble};
use crate::Result;

/// The ten doubles of an `mgBBox_c` archive body; lengths are stored in metres.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderBox {
    /// Bounding-sphere centre.
    pub centre: [StoredDouble; 3],
    /// High corner.
    pub high: [StoredDouble; 3],
    /// Low corner.
    pub low: [StoredDouble; 3],
    /// Stored sphere radius, without recomputation.
    pub radius: StoredDouble,
}

pub(crate) fn read_point(cursor: &mut ArchiveCursor<'_>) -> Result<[StoredDouble; 3]> {
    let mut point = [StoredDouble::from_bits(0); 3];
    for coordinate in &mut point {
        *coordinate = StoredDouble::from_bits(cursor.u64()?);
    }
    Ok(point)
}

pub(crate) fn read_box(cursor: &mut ArchiveCursor<'_>) -> Result<HeaderBox> {
    Ok(HeaderBox {
        centre: read_point(cursor)?,
        high: read_point(cursor)?,
        low: read_point(cursor)?,
        radius: StoredDouble::from_bits(cursor.u64()?),
    })
}

pub(crate) fn write_box(bounds: &HeaderBox, output: &mut Vec<u8>) {
    for point in [&bounds.centre, &bounds.high, &bounds.low] {
        for coordinate in point {
            output.extend_from_slice(&coordinate.bits().to_le_bytes());
        }
    }
    output.extend_from_slice(&bounds.radius.bits().to_le_bytes());
}
