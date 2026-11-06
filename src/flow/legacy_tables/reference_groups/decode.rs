use super::*;

struct ReferenceReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    version: i32,
    options: FlowReferenceGroupOptions,
    headers: [Option<usize>; 2],
    entries: Vec<ReferenceEntrySpan>,
    word_counts: [usize; 2],
    reference_words: usize,
    mapping_words: usize,
    wide_words: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowReferenceGroupOptions,
) -> Result<FlowReferenceGroups> {
    validate_options(options)?;
    let wire = reference_wire(source, id, options)?;
    let mut reader = ReferenceReader::new(&wire, options);
    let groups = reader.record()?;
    Ok(FlowReferenceGroups {
        headers: reader.headers,
        entries: reader.entries.into(),
        groups,
        word_counts: reader.word_counts,
        reference_words: reader.reference_words,
        mapping_words: reader.mapping_words,
        wide_words: reader.wide_words,
        wire,
        options,
    })
}

fn reference_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowReferenceGroupOptions,
) -> Result<TableWire> {
    let wire = wire::read(
        source,
        id,
        416,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    if !(0..=17).contains(&wire.version) {
        return Err(table_failure(
            416,
            FailureClass::Unsupported,
            format!(
                "version {}; expected legacy reference groups zero through seventeen",
                wire.version
            ),
        ));
    }
    Ok(wire)
}

fn validate_options(options: FlowReferenceGroupOptions) -> Result<()> {
    for (name, limit) in [
        ("entries", options.maximum_entries),
        ("references", options.maximum_reference_words),
        ("integer words", options.maximum_integer_words),
        ("mapping words", options.maximum_mapping_words),
        ("wide words", options.maximum_wide_words),
    ] {
        if limit == 0 {
            return Err(reference_invalid(format!(
                "{name} bound {limit}; expected a positive limit"
            )));
        }
    }
    Ok(())
}

impl<'a> ReferenceReader<'a> {
    fn new(wire: &'a TableWire, options: FlowReferenceGroupOptions) -> Self {
        Self {
            cursor: ArchiveCursor::new(&wire.decoded),
            length: wire.decoded.len(),
            version: wire.version,
            options,
            headers: [None; 2],
            entries: Vec::new(),
            word_counts: [0; 2],
            reference_words: 0,
            mapping_words: 0,
            wide_words: 0,
        }
    }
    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }
    fn record(&mut self) -> Result<[ReferenceGroupSpan; 2]> {
        self.cursor.i32()?;
        self.headers[0] = self.optional_word(self.version < 6)?;
        self.headers[1] = self.optional_word(self.version >= 3)?;
        let groups = [self.group(0)?, self.group(1)?];
        self.cursor.finish()?;
        Ok(groups)
    }
    fn optional_word(&mut self, present: bool) -> Result<Option<usize>> {
        if !present {
            return Ok(None);
        }
        let offset = self.position();
        self.cursor.i32()?;
        Ok(Some(offset))
    }
    fn group(&mut self, ordinal: usize) -> Result<ReferenceGroupSpan> {
        let start = self.position();
        let count = self
            .cursor
            .nonnegative_count(17, "reference-group entries")?;
        let entries = self.group_entries(count)?;
        let mapped = ordinal == 0 && self.options.first_group_mapping_enabled && count > 0;
        let level = self.optional_word(mapped)?;
        let mapping = self.mapping(count, mapped)?;
        let wide = self.appended_wide(count, ordinal == 1 || self.version >= 15)?;
        Ok(ReferenceGroupSpan {
            full: (start, self.position()),
            entries,
            level,
            mapping,
            wide,
        })
    }
    fn group_entries(&mut self, count: usize) -> Result<(usize, usize)> {
        budget(
            self.entries.len(),
            count,
            self.options.maximum_entries,
            "entries",
        )?;
        let first = self.entries.len();
        for _ in 0..count {
            self.entry()?;
        }
        Ok((first, self.entries.len()))
    }
    fn entry(&mut self) -> Result<()> {
        let start = self.position();
        let (member20, references) = self.reference_preamble()?;
        let continuation = self.continuation()?;
        self.entries.push(ReferenceEntrySpan {
            full: (start, self.position()),
            references,
            member20,
            continuation,
        });
        Ok(())
    }
    fn reference_preamble(&mut self) -> Result<(Option<usize>, (usize, usize))> {
        let count = self.cursor.nonnegative_count(4, "reference-list words")?;
        let total = budget(
            self.reference_words,
            count,
            self.options.maximum_reference_words,
            "reference words",
        )?;
        let member20 = self.optional_word(self.version < 12)?;
        let references = self.take(count, 4)?;
        self.reference_words = total;
        Ok((member20, references))
    }
    fn continuation(&mut self) -> Result<EntryContinuationSpan> {
        let continuation = entry_continuation::read(
            &mut self.cursor,
            EntryContinuationContext {
                version: self.version,
                length: self.length,
                maximum_words: self.options.maximum_integer_words,
                word_counts: self.word_counts,
                invalid: reference_invalid,
            },
        )?;
        self.word_counts = continuation.word_counts;
        Ok(continuation)
    }
    fn take(&mut self, count: usize, width: usize) -> Result<(usize, usize)> {
        if count > self.cursor.remaining() / width {
            return Err(reference_invalid(format!(
                "{count} elements of width {width}; expected at most {} in remaining bytes",
                self.cursor.remaining() / width
            )));
        }
        let start = self.position();
        self.cursor.take(count * width)?;
        Ok((start, self.position()))
    }
    fn mapping(&mut self, count: usize, present: bool) -> Result<(usize, usize)> {
        let words = multiplied(count, if present { 2 } else { 0 })?;
        let total = budget(
            self.mapping_words,
            words,
            self.options.maximum_mapping_words,
            "mapping words",
        )?;
        let span = self.take(words, 4)?;
        self.mapping_words = total;
        Ok(span)
    }
    fn appended_wide(&mut self, count: usize, present: bool) -> Result<(usize, usize)> {
        let words = multiplied(count, if present { 3 } else { 0 })?;
        let total = budget(
            self.wide_words,
            words,
            self.options.maximum_wide_words,
            "wide words",
        )?;
        let span = self.take(words, 8)?;
        self.wide_words = total;
        Ok(span)
    }
}

fn budget(used: usize, count: usize, maximum: usize, label: &str) -> Result<usize> {
    used.checked_add(count)
        .filter(|total| *total <= maximum)
        .ok_or_else(|| {
            reference_invalid(format!(
                "{label} {used} plus {count}; expected total at most {maximum}"
            ))
        })
}

fn multiplied(count: usize, factor: usize) -> Result<usize> {
    count.checked_mul(factor).ok_or_else(|| {
        reference_invalid(format!(
            "count {count} times {factor}; expected product to fit usize"
        ))
    })
}
