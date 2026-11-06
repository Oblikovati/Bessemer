use super::super::entry_continuation::{self, EntryContinuationContext, EntryContinuationSpan};
use super::*;

pub(super) struct BodyReader<'a> {
    pub(super) cursor: ArchiveCursor<'a>,
    pub(super) length: usize,
    pub(super) version: i32,
    pub(super) options: FlowBlockDataOptions,
    pub(super) entries: Vec<EntrySpan>,
    pub(super) word_counts: [usize; 2],
}

impl<'a> BodyReader<'a> {
    pub(super) fn new(
        cursor: ArchiveCursor<'a>,
        length: usize,
        version: i32,
        options: FlowBlockDataOptions,
    ) -> Self {
        Self {
            cursor,
            length,
            version,
            options,
            entries: Vec::new(),
            word_counts: [0; 2],
        }
    }

    pub(super) fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    pub(super) fn read_entry(&mut self) -> Result<EntrySpan> {
        let start = self.position();
        let mut scalars = self.header_scalars()?;
        let continuation = self.continuation()?;
        scalars[4..].copy_from_slice(&continuation.scalars);
        Ok(EntrySpan {
            start,
            end: self.position(),
            scalars,
            descriptors: continuation.descriptors,
            first: continuation.first,
            second: continuation.second,
        })
    }

    fn header_scalars(&mut self) -> Result<[Option<usize>; 10]> {
        let mut fields = [None; 10];
        fields[0] = self.optional_offset(true, 4)?;
        fields[1] = self.optional_offset(self.version < 12, 4)?;
        fields[2] = if self.version == 1 {
            fields[1]
        } else {
            self.optional_offset(self.version < 12, 4)?
        };
        fields[3] = self.optional_offset(true, 4)?;
        Ok(fields)
    }

    fn optional_offset(&mut self, present: bool, width: usize) -> Result<Option<usize>> {
        if !present {
            return Ok(None);
        }
        let offset = self.position();
        self.cursor.take(width)?;
        Ok(Some(offset))
    }

    fn continuation(&mut self) -> Result<EntryContinuationSpan> {
        let parsed = entry_continuation::read(
            &mut self.cursor,
            EntryContinuationContext {
                version: self.version,
                length: self.length,
                maximum_words: self.options.maximum_integer_words,
                word_counts: self.word_counts,
                invalid: invalid_block,
            },
        )?;
        self.word_counts = parsed.word_counts;
        Ok(parsed)
    }
}
