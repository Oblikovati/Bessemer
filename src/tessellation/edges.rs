use super::codec::FacetCodec;
use super::context::{CollectionClass, FacetState};
use super::output::FacetOutput;
use super::record::archive_record;
use super::{StoredDouble, invalid};
use crate::Result;
use crate::archive_codec::ArchiveCursor;

/// Raw vertex-pair count and low/high u32 words in their stored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeVertexPairs {
    /// The native branch treats every value below one as no pairs; it is retained.
    pub count: i32,
    /// Pairs are present only when count is positive.
    pub pairs: Vec<[u32; 2]>,
}

impl FacetCodec for EdgeVertexPairs {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let count = cursor.i32()?;
        let size = count.max(0) as usize;
        require_elements(cursor, context, size, 8)?;
        let pairs = (0..size)
            .map(|_| <[u32; 2]>::read(cursor, context))
            .collect::<Result<Vec<_>>>()?;
        context.skip_edge_first = context.version == 0x3f0f && count < 1;
        Ok(Self { count, pairs })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        let expected = self.count.max(0) as usize;
        if self.pairs.len() != expected {
            return Err(invalid(format!(
                "edge vertex-pair length {}; expected {expected} for count {}",
                self.pairs.len(),
                self.count
            )));
        }
        output.i32(self.count);
        for pair in &self.pairs {
            pair.write(output, context)?;
        }
        context.skip_edge_first = context.version == 0x3f0f && self.count < 1;
        Ok(())
    }
}

archive_record! {
    /// Complete `gcEdgeFullInfo_c` record, retaining native ignored and unknown fields.
    FacetEdgeInfo(context, 24, Element) {
        /// Omitted when the preceding edge returned one at version 0x3F0F.
        first: optional i32 => !context.skip_edge_first;
        /// Edge-map tag.
        tag: required i32 => true;
        /// Raw packed curve/edge type word, without sign or enum normalization.
        types: required i32 => true;
        /// Every stored double, including coordinates the native runtime discards.
        point: required Vec<StoredDouble> => true;
        /// Original word normally written zero.
        zero: required i32 => true;
        /// Complete uninterpreted direct integer array.
        list_38: required Vec<i32> => true;
        /// Raw count and low-word-first vertex pairs; updates the omission state.
        vertices: required EdgeVertexPairs => true;
    }
}

archive_record! {
    /// Edge ID and one stored point.
    FacetEdgePoint(context, 28, Element) {
        /// Stored edge identifier.
        edge_id: required i32 => true;
        /// Stored point, in metres.
        point: required [StoredDouble; 3] => true;
    }
}

/// Edge points framed by an i64 count rather than the ordinary i32 direct-array count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetEdgePoints {
    /// Every stored edge/point record, in order.
    pub entries: Vec<FacetEdgePoint>,
}

impl FacetCodec for FacetEdgePoints {
    const MINIMUM: usize = 8;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let count = cursor.u64()? as i64;
        if count < 0 {
            return Err(invalid(format!(
                "edge-point count {count}; expected nonnegative i64"
            )));
        }
        let size = usize::try_from(count)
            .map_err(|_| invalid(format!("edge-point count {count}; expected usize")))?;
        require_elements(cursor, context, size, FacetEdgePoint::MINIMUM)?;
        let entries = (0..size)
            .map(|_| FacetEdgePoint::read(cursor, context))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { entries })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        let count = i64::try_from(self.entries.len()).map_err(|_| {
            invalid(format!(
                "edge-point length {}; expected i64",
                self.entries.len()
            ))
        })?;
        output.u64(count as u64);
        for entry in &self.entries {
            entry.write(output, context)?;
        }
        Ok(())
    }
}

fn require_elements(
    cursor: &ArchiveCursor<'_>,
    context: &mut FacetState,
    count: usize,
    minimum: usize,
) -> Result<()> {
    context.require_count(count, CollectionClass::Element)?;
    if count <= cursor.remaining() / minimum {
        return Ok(());
    }
    Err(invalid(format!(
        "edge array count {count}; expected 0..={} in remaining bytes",
        cursor.remaining() / minimum
    )))
}
