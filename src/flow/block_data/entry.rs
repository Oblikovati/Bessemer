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
        let descriptors = self.position();
        let first_count = self.descriptors()?;
        let first = self.word_array(first_count, 0)?;
        scalars[8] = Some(self.position());
        let second_count = self.second_count()?;
        let second = self.word_array(second_count.max(0) as usize, 1)?;
        scalars[9] = self.optional_offset(matches!(self.version, 16 | 17), 2)?;
        Ok(EntrySpan {
            start,
            end: self.position(),
            scalars,
            descriptors,
            first,
            second,
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
        fields[4] = self.optional_offset((11..=13).contains(&self.version), 1)?;
        fields[5] = self.optional_offset(true, 1)?;
        fields[6] = self.optional_offset(self.version >= 5, 1)?;
        fields[7] = self.optional_offset(self.version >= 17, 1)?;
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

    fn descriptors(&mut self) -> Result<usize> {
        let mut last = 0;
        for _ in 0..7 {
            last = if self.version == 1 {
                usize::from(self.cursor.u8()?)
            } else {
                usize::from(self.cursor.u16()?)
            };
        }
        Ok(last)
    }

    fn second_count(&mut self) -> Result<i32> {
        match self.version {
            1 => Ok(i32::from(self.cursor.u8()? as i8)),
            2..=9 => Ok(i32::from(self.cursor.u16()?)),
            _ => self.cursor.i32(),
        }
    }

    fn word_array(&mut self, count: usize, which: usize) -> Result<(usize, usize)> {
        let used = self.word_counts[0] + self.word_counts[1];
        let available = self.options.maximum_integer_words - used;
        if count > available || count > self.cursor.remaining() / 4 {
            return Err(invalid_block(format!(
                "array {which} count {count}, used {used}; expected <= {available} additional words and <= {} words in remaining payload",
                self.cursor.remaining() / 4
            )));
        }
        let start = self.position();
        self.cursor.take(count * 4)?;
        self.word_counts[which] += count;
        Ok((start, self.position()))
    }
}
