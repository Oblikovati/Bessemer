use super::*;
struct TaggedStateReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw_length: usize,
    maximum_wides: usize,
    wide_words: usize,
}
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowTaggedStateOptions,
) -> Result<FlowTaggedState> {
    let wire = read_tagged_wire(source, id, options.maximum_payload_bytes)?;
    let (rows, wide_words) = parse_rows(&wire.decoded, options)?;
    Ok(FlowTaggedState {
        wire,
        options,
        rows: rows.into(),
        wide_words,
    })
}
fn read_tagged_wire(source: &FlowCarrier, id: &FlowRecordId, maximum: usize) -> Result<TableWire> {
    let wire = wire::read(source, id, 129, FlowLegacyTableTransport::Raw, maximum)?;
    if wire.version != 9 {
        return Err(table_failure(
            129,
            FailureClass::Unsupported,
            format!(
                "version {}; expected tagged-state version nine",
                wire.version
            ),
        ));
    }
    Ok(wire)
}
fn parse_rows(raw: &[u8], options: FlowTaggedStateOptions) -> Result<(Vec<TaggedRowSpan>, usize)> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.take(4)?;
    let count = nonnegative_count(&mut cursor)?;
    row_bound(count, cursor.remaining(), options.maximum_rows)?;
    let tags = cursor.take(count)?;
    let mut reader = TaggedStateReader {
        cursor,
        raw_length: raw.len(),
        maximum_wides: options.maximum_wide_words,
        wide_words: 0,
    };
    let mut rows = Vec::with_capacity(count);
    for &tag in tags {
        rows.push(reader.row(tag)?);
    }
    if count > 0 {
        reader.cursor.i32()?;
    }
    reader.cursor.finish()?;
    Ok((rows, reader.wide_words))
}
fn row_bound(count: usize, available: usize, maximum: usize) -> Result<()> {
    if count <= maximum && count <= available {
        return Ok(());
    }
    Err(tagged_invalid(format!(
        "row count {count}; expected at most {maximum} descriptors and at most {available} tag bytes in remaining payload"
    )))
}
fn nonnegative_count(cursor: &mut ArchiveCursor<'_>) -> Result<usize> {
    let count = cursor.i32()?;
    if count >= 0 {
        return Ok(count as usize);
    }
    Err(tagged_invalid(format!(
        "stored count {count}; expected a nonnegative signed-i32 count"
    )))
}
impl TaggedStateReader<'_> {
    fn position(&self) -> usize {
        self.raw_length - self.cursor.remaining()
    }
    fn row(&mut self, tag: u8) -> Result<TaggedRowSpan> {
        let start = self.position();
        if tag == 0 {
            return Ok(TaggedRowSpan {
                interval: (start, start),
                values: None,
            });
        }
        let values = self.wides()?;
        Ok(TaggedRowSpan {
            interval: (start, self.position()),
            values: Some(values),
        })
    }
    fn wides(&mut self) -> Result<(usize, usize)> {
        let width = nonnegative_count(&mut self.cursor)?;
        wide_bound(
            self.wide_words,
            width,
            self.maximum_wides,
            self.cursor.remaining(),
        )?;
        let start = self.position();
        self.cursor.take(width * 8)?;
        self.wide_words += width;
        Ok((start, self.position()))
    }
}
fn wide_bound(used: usize, count: usize, maximum: usize, remaining: usize) -> Result<()> {
    if used <= maximum && count <= maximum - used && count <= remaining / 8 {
        return Ok(());
    }
    Err(tagged_invalid(format!(
        "wide count {count}, previous total {used}; expected aggregate at most {maximum} and at most {} complete values in remaining bytes",
        remaining / 8
    )))
}
