use super::*;

struct ParallelReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    maximum: usize,
    wide_words: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowParallelTableOptions,
) -> Result<FlowParallelTable> {
    positive_limit(407, "wide words", options.maximum_wide_words)?;
    let maximum = options.maximum_decoded_bytes;
    let wire = load(source, id, 407, options.compression, maximum)?;
    let mut reader = ParallelReader::new(&wire, options.maximum_wide_words)?;
    let groups = if wire.version == 0 {
        reader.legacy_groups()?
    } else {
        reader.inactive_groups(wire.version)?
    };
    reader.cursor.finish()?;
    let wide_words = reader.wide_words;
    Ok(FlowParallelTable {
        wire,
        options,
        groups,
        wide_words,
    })
}

impl<'a> ParallelReader<'a> {
    fn new(wire: &'a TableWire, maximum: usize) -> Result<Self> {
        Ok(Self {
            cursor: payload_cursor(wire)?,
            length: wire.decoded.len(),
            maximum,
            wide_words: 0,
        })
    }

    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    fn legacy_groups(&mut self) -> Result<[ParallelSpan; 2]> {
        let counts = [
            nonnegative_count(&mut self.cursor, 407)?,
            nonnegative_count(&mut self.cursor, 407)?,
        ];
        Ok([self.group(counts[0])?, self.group(counts[1])?])
    }

    fn group(&mut self, count: FlowSimpleLegacyCount) -> Result<ParallelSpan> {
        let start = self.position();
        let present = self.cursor.i32()? != 0;
        let arrays = if present {
            Some(self.arrays(count.value() as usize)?)
        } else {
            None
        };
        Ok(ParallelSpan {
            start,
            end: self.position(),
            count,
            presence: Some(start),
            arrays,
        })
    }

    fn arrays(&mut self, count: usize) -> Result<[(usize, usize); 4]> {
        let available = self.maximum - self.wide_words;
        if count > available / 4 || count > self.cursor.remaining() / 32 {
            return Err(simple_invalid(
                407,
                format!(
                    "four-array count {count}, used {} words; expected <= {} additional rows within budget and <= {} rows in remaining bytes",
                    self.wide_words,
                    available / 4,
                    self.cursor.remaining() / 32
                ),
            ));
        }
        let arrays = [
            self.array(count)?,
            self.array(count)?,
            self.array(count)?,
            self.array(count)?,
        ];
        self.wide_words += 4 * count;
        Ok(arrays)
    }

    fn array(&mut self, count: usize) -> Result<(usize, usize)> {
        let start = self.position();
        self.cursor.take(count * 8)?;
        Ok((start, self.position()))
    }

    fn inactive_groups(&mut self, version: i32) -> Result<[ParallelSpan; 2]> {
        Ok([self.inactive_group(version)?, self.inactive_group(version)?])
    }

    fn inactive_group(&mut self, version: i32) -> Result<ParallelSpan> {
        let start = self.position();
        let count = nonnegative_count(&mut self.cursor, 407)?;
        if count.value() != 0 {
            return Err(restored(407, version, "populated parallel group"));
        }
        Ok(ParallelSpan {
            start,
            end: self.position(),
            count,
            presence: None,
            arrays: None,
        })
    }
}
