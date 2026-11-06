use super::*;

struct PointerReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    version: i32,
    options: FlowPointerTableOptions,
    pairs: usize,
    wide: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowPointerTableOptions,
) -> Result<FlowPointerTables> {
    validate_options(options)?;
    let wire = wire::read(
        source,
        id,
        401,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    let (groups, pair_count, wide_count) = read_groups(&wire, options)?;
    Ok(FlowPointerTables {
        wire,
        groups,
        options,
        pair_count,
        wide_count,
    })
}

impl<'a> PointerReader<'a> {
    fn new(wire: &'a TableWire, options: FlowPointerTableOptions) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(&wire.decoded);
        cursor.take(4)?;
        Ok(Self {
            cursor,
            length: wire.decoded.len(),
            version: wire.version,
            options,
            pairs: 0,
            wide: 0,
        })
    }

    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    fn group(&mut self, ordinal: usize) -> Result<GroupSpan> {
        let start = self.position();
        let count = self.nonnegative("group pair count")?;
        let discarded_count = self.optional_word(self.version == 1)?;
        self.check_pair_count(count, ordinal)?;
        let body = if count == 0 {
            None
        } else {
            Some(self.group_body(ordinal, count)?)
        };
        self.pairs += count;
        Ok(GroupSpan {
            start,
            end: self.position(),
            count,
            discarded_count,
            body,
        })
    }

    fn check_pair_count(&self, count: usize, ordinal: usize) -> Result<()> {
        if count > self.options.maximum_pairs - self.pairs || count > self.cursor.remaining() / 8 {
            return Err(table_failure(
                401,
                FailureClass::Invalid,
                format!(
                    "group {ordinal} pair count {count}, used {}; expected <= {} total pairs and <= {} complete pairs in remaining bytes",
                    self.pairs,
                    self.options.maximum_pairs,
                    self.cursor.remaining() / 8
                ),
            ));
        }
        Ok(())
    }

    fn group_body(&mut self, ordinal: usize, count: usize) -> Result<GroupBodySpan> {
        let members = self.position();
        self.cursor.take(12)?;
        let lengths = self.array_lengths()?;
        let discarded = self.discarded_triplets(ordinal)?;
        let arrays = [
            self.wide_array(lengths[0])?,
            self.wide_array(lengths[1])?,
            self.wide_array(lengths[2])?,
        ];
        let pre_pairs = self.optional_word(self.version == 1)?;
        let pairs = self.pair_array(count)?;
        Ok(GroupBodySpan {
            members,
            lengths,
            discarded,
            arrays,
            pre_pairs,
            pairs,
        })
    }

    fn array_lengths(&mut self) -> Result<[usize; 3]> {
        Ok([
            self.nonnegative("array 0 length")?,
            self.nonnegative("array 1 length")?,
            self.nonnegative("array 2 length")?,
        ])
    }

    fn discarded_triplets(&mut self, ordinal: usize) -> Result<(usize, usize)> {
        let discarded_count =
            usize::from(ordinal == 1 && self.version < 2) + if self.version == 1 { 2 } else { 0 };
        let discarded_start = self.position();
        self.cursor.take(discarded_count * 12)?;
        Ok((discarded_start, self.position()))
    }

    fn nonnegative(&mut self, field: &str) -> Result<usize> {
        let count = self.cursor.i32()?;
        if count >= 0 {
            return Ok(count as usize);
        }
        Err(table_failure(
            401,
            FailureClass::Invalid,
            format!("{field} {count}; expected nonnegative signed-i32 count"),
        ))
    }

    fn optional_word(&mut self, present: bool) -> Result<Option<usize>> {
        if !present {
            return Ok(None);
        }
        let offset = self.position();
        self.cursor.take(4)?;
        Ok(Some(offset))
    }

    fn wide_array(&mut self, count: usize) -> Result<(usize, usize)> {
        let available = self.options.maximum_wide_words - self.wide;
        if count > available || count > self.cursor.remaining() / 8 {
            return Err(table_failure(
                401,
                FailureClass::Invalid,
                format!(
                    "wide array count {count}, used {}; expected <= {available} additional words and <= {} words in remaining bytes",
                    self.wide,
                    self.cursor.remaining() / 8
                ),
            ));
        }
        let start = self.position();
        self.cursor.take(count * 8)?;
        self.wide += count;
        Ok((start, self.position()))
    }

    fn pair_array(&mut self, count: usize) -> Result<(usize, usize)> {
        if count > self.cursor.remaining() / 8 {
            return Err(table_failure(
                401,
                FailureClass::Invalid,
                format!(
                    "pair count {count}; expected <= {} complete pairs in remaining bytes",
                    self.cursor.remaining() / 8
                ),
            ));
        }
        let start = self.position();
        let raw = self.cursor.take(count * 8)?;
        if let Some(context) = self.options.global_pointer_count {
            check_pair_indices(raw, context)?;
        }
        Ok((start, self.position()))
    }
}

fn check_pair_indices(raw: &[u8], context: FlowGlobalPointerCount) -> Result<()> {
    for (ordinal, bytes) in raw.as_chunks::<4>().0.iter().enumerate() {
        let index = i32::from_le_bytes(*bytes);
        if index < 0 || index as u32 >= context.value() {
            return Err(table_failure(
                401,
                FailureClass::Invalid,
                format!(
                    "pair word {ordinal} index {index}; expected nonnegative index below caller pointer count {}",
                    context.value()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_options(options: FlowPointerTableOptions) -> Result<()> {
    if options.maximum_pairs == 0 || options.maximum_wide_words == 0 {
        return Err(table_failure(
            401,
            FailureClass::Invalid,
            format!(
                "pair/wide limits {}/{}; expected positive total limits",
                options.maximum_pairs, options.maximum_wide_words
            ),
        ));
    }
    Ok(())
}

fn read_groups(
    wire: &TableWire,
    options: FlowPointerTableOptions,
) -> Result<([GroupSpan; 2], usize, usize)> {
    if !(0..=3).contains(&wire.version) {
        return Err(table_failure(
            401,
            FailureClass::Unsupported,
            format!(
                "version {}; expected legacy pointer-table versions zero through three",
                wire.version
            ),
        ));
    }
    let mut reader = PointerReader::new(wire, options)?;
    let groups = [reader.group(0)?, reader.group(1)?];
    reader.cursor.finish()?;
    Ok((groups, reader.pairs, reader.wide))
}
