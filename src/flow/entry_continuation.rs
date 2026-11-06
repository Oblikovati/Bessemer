//! Shared serialized entry suffix; the caller owns preamble identity and error context.
use crate::archive_codec::ArchiveCursor;
use crate::{BessemerError, Result};

pub(super) struct EntryContinuationContext {
    pub version: i32,
    pub length: usize,
    pub maximum_words: usize,
    pub word_counts: [usize; 2],
    pub invalid: fn(String) -> BessemerError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EntryContinuationSpan {
    pub scalars: [Option<usize>; 6],
    pub descriptors: usize,
    pub first: (usize, usize),
    pub second: (usize, usize),
    pub word_counts: [usize; 2],
}

struct ContinuationReader<'a, 'b> {
    cursor: &'a mut ArchiveCursor<'b>,
    context: EntryContinuationContext,
}

pub(super) fn read(
    cursor: &mut ArchiveCursor<'_>,
    context: EntryContinuationContext,
) -> Result<EntryContinuationSpan> {
    ContinuationReader { cursor, context }.continuation()
}

impl ContinuationReader<'_, '_> {
    fn position(&self) -> usize {
        self.context.length - self.cursor.remaining()
    }

    fn continuation(&mut self) -> Result<EntryContinuationSpan> {
        let mut scalars = self.selectors()?;
        let descriptors = self.position();
        let first_count = self.descriptor_count()?;
        let first = self.word_array(first_count, 0)?;
        scalars[4] = Some(self.position());
        let second_count = self.secondary_count()?;
        let second = self.word_array(second_count.max(0) as usize, 1)?;
        scalars[5] = self.optional_offset(matches!(self.context.version, 16 | 17), 2)?;
        Ok(EntryContinuationSpan {
            scalars,
            descriptors,
            first,
            second,
            word_counts: self.context.word_counts,
        })
    }

    fn selectors(&mut self) -> Result<[Option<usize>; 6]> {
        let version = self.context.version;
        let mut fields = [None; 6];
        fields[0] = self.optional_offset((11..=13).contains(&version), 1)?;
        fields[1] = self.optional_offset(true, 1)?;
        fields[2] = self.optional_offset(version >= 5, 1)?;
        fields[3] = self.optional_offset(version >= 17, 1)?;
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

    fn descriptor_count(&mut self) -> Result<usize> {
        let mut last = 0;
        for _ in 0..7 {
            last = if self.context.version == 1 {
                usize::from(self.cursor.u8()?)
            } else {
                usize::from(self.cursor.u16()?)
            };
        }
        Ok(last)
    }

    fn secondary_count(&mut self) -> Result<i32> {
        match self.context.version {
            1 => Ok(i32::from(self.cursor.u8()? as i8)),
            0 | 2..=9 => Ok(i32::from(self.cursor.u16()?)),
            _ => self.cursor.i32(),
        }
    }

    fn word_array(&mut self, count: usize, which: usize) -> Result<(usize, usize)> {
        let used = self.context.word_counts[0] + self.context.word_counts[1];
        let available = self.context.maximum_words - used;
        if count > available || count > self.cursor.remaining() / 4 {
            return Err((self.context.invalid)(format!(
                "array {which} count {count}, used {used}; expected <= {available} additional words and <= {} words in remaining payload",
                self.cursor.remaining() / 4
            )));
        }
        let start = self.position();
        self.cursor.take(count * 4)?;
        self.context.word_counts[which] += count;
        Ok((start, self.position()))
    }
}
