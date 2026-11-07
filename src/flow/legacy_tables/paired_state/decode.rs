use super::*;
struct PairedStateReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw_length: usize,
    options: FlowPairedStateOptions,
    totals: [usize; 2],
}
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowPairedStateOptions,
) -> Result<FlowPairedState> {
    let wire = read_paired_wire(source, id, options.maximum_payload_bytes)?;
    let (groups, totals) = parse_groups(&wire.decoded, options)?;
    Ok(FlowPairedState {
        wire,
        options,
        groups,
        totals,
    })
}
fn read_paired_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    let wire = wire::read(source, id, 417, FlowLegacyTableTransport::Raw, maximum)?;
    if wire.version != 1 {
        return Err(table_failure(
            417,
            FailureClass::Unsupported,
            format!(
                "version {}; expected paired-state version one",
                wire.version
            ),
        ));
    }
    Ok(wire)
}
fn parse_groups(
    raw: &[u8],
    options: FlowPairedStateOptions,
) -> Result<([PairedGroupSpan; 2], [usize; 2])> {
    let mut reader = PairedStateReader {
        cursor: ArchiveCursor::new(raw),
        raw_length: raw.len(),
        options,
        totals: [0; 2],
    };
    reader.cursor.take(4)?;
    let groups = [reader.group()?, reader.group()?];
    reader.cursor.finish()?;
    Ok((groups, reader.totals))
}
impl PairedStateReader<'_> {
    fn position(&self) -> usize {
        self.raw_length - self.cursor.remaining()
    }
    fn group(&mut self) -> Result<PairedGroupSpan> {
        let start = self.position();
        let indices = self.array(0, 4, self.options.maximum_index_words)?;
        let wides = if indices.0 != indices.1 {
            Some(self.array(1, 8, self.options.maximum_wide_words)?)
        } else {
            None
        };
        Ok(PairedGroupSpan {
            interval: (start, self.position()),
            indices,
            wides,
        })
    }
    fn array(&mut self, axis: usize, width: usize, maximum: usize) -> Result<(usize, usize)> {
        let count = self.array_count(axis, width, maximum)?;
        let start = self.position();
        self.cursor.take(count * width)?;
        Ok((start, self.position()))
    }
    fn array_count(&mut self, axis: usize, width: usize, maximum: usize) -> Result<usize> {
        let count = nonnegative_count(&mut self.cursor)?;
        bounded(self.totals[axis], count, maximum, width)?;
        physical_count(&self.cursor, count, width)?;
        self.totals[axis] += count;
        Ok(count)
    }
}
fn nonnegative_count(cursor: &mut ArchiveCursor<'_>) -> Result<usize> {
    let count = cursor.i32()?;
    if count >= 0 {
        return Ok(count as usize);
    }
    Err(paired_invalid(format!(
        "array count {count}; expected a nonnegative signed-i32 count"
    )))
}
fn bounded(used: usize, count: usize, maximum: usize, width: usize) -> Result<()> {
    if used <= maximum && count <= maximum - used {
        return Ok(());
    }
    Err(paired_invalid(format!(
        "{width}-byte array count {count}, previous total {used}; expected aggregate at most {maximum}"
    )))
}
fn physical_count(cursor: &ArchiveCursor<'_>, count: usize, width: usize) -> Result<()> {
    let available = cursor.remaining() / width;
    if count <= available {
        return Ok(());
    }
    Err(paired_invalid(format!(
        "array count {count}, width {width}; expected at most {available} complete values in remaining bytes"
    )))
}
