use super::layout::{
    ParameterGates, bounded_count, checked_extent, minimum_record_size, reserve_items,
};
use super::*;
use crate::archive_codec::ArchiveCursor;

pub(super) struct ParameterScan {
    header: SimulationParameterHeader,
    table: Vec<i32>,
    records: Vec<(usize, SimulationParameterRecordNumbers)>,
}

impl ParameterScan {
    pub fn materialize(
        self,
        bytes: Arc<[u8]>,
        limits: SimulationParameterLimits,
    ) -> Result<SimulationParameterParts> {
        let mut records = reserve_items(self.records.len(), "ParameterSets text materialization")?;
        for (offset, numbers) in self.records {
            let text = SimulationPersistText::read_shared(
                bytes.clone(),
                offset,
                limits.maximum_text_bytes,
            )?;
            records.push(SimulationParameterRecordParts { text, numbers });
        }
        Ok(SimulationParameterParts {
            header: self.header,
            table: self.table,
            records,
        })
    }
}

struct ParameterReader<'a> {
    cursor: ArchiveCursor<'a>,
    original: &'a [u8],
    gates: ParameterGates,
    limits: SimulationParameterLimits,
}

pub(super) fn scan(
    bytes: &[u8],
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<ParameterScan> {
    ParameterReader {
        cursor: ArchiveCursor::new(bytes),
        original: bytes,
        gates,
        limits,
    }
    .read_complete()
}

impl ParameterReader<'_> {
    fn read_complete(&mut self) -> Result<ParameterScan> {
        let (mut header, count, table_count) = self.read_prefix()?;
        self.require_capacity(count, table_count)?;
        let table = read_array(&mut self.cursor, table_count, 4, |cursor| cursor.i32())?;
        let records = self.read_records(count, table_count)?;
        header.tail = self.cursor.i32()?;
        header.gate820015 = self.optional_i32(self.gates.late_word)?;
        header.gate830009 = self.optional_i32(self.gates.second_late)?;
        self.cursor.finish()?;
        Ok(ParameterScan {
            header,
            table,
            records,
        })
    }

    fn read_prefix(&mut self) -> Result<(SimulationParameterHeader, usize, usize)> {
        let header = self.read_header()?;
        let count = bounded_count(
            self.cursor.i32()?,
            self.limits.maximum_records,
            "ParameterSets record count",
        )?;
        let table_count = self.table_count(&header)?;
        Ok((header, count, table_count))
    }

    fn table_count(&self, header: &SimulationParameterHeader) -> Result<usize> {
        let raw_count = if self.gates.modern_table {
            header.words[0]
        } else {
            100
        };
        bounded_count(
            raw_count,
            self.limits.maximum_table_items,
            "ParameterSets table count",
        )
    }

    fn read_header(&mut self) -> Result<SimulationParameterHeader> {
        let words = [
            self.cursor.i32()?,
            self.cursor.i32()?,
            self.cursor.i32()?,
            self.cursor.i32()?,
            self.cursor.i32()?,
        ];
        let gate700013 = self.optional_i32(self.gates.early_word)?;
        let gate700018 = self.optional_i32(self.gates.second_early)?;
        Ok(SimulationParameterHeader {
            words,
            gate700013,
            gate700018,
            tail: 0,
            gate820015: None,
            gate830009: None,
        })
    }

    fn read_records(
        &mut self,
        count: usize,
        table_count: usize,
    ) -> Result<Vec<(usize, SimulationParameterRecordNumbers)>> {
        let mut records = reserve_items(count, "ParameterSets record allocation")?;
        for _ in 0..count {
            let offset = self.skip_text()?;
            records.push((offset, self.read_numbers(table_count)?));
        }
        Ok(records)
    }

    fn require_capacity(&self, count: usize, table_count: usize) -> Result<()> {
        let table_bytes = checked_extent(table_count, 4, "ParameterSets table extent")?;
        let available = self.cursor.remaining().saturating_sub(table_bytes);
        let minimum = minimum_record_size(table_count, self.gates)?;
        require(
            count <= available / minimum,
            "ParameterSets record bodies",
            format!(
                "{count} records, {available} bytes after the table, minimum {minimum} bytes each; expected complete available minimum record bodies"
            ),
        )
    }

    fn skip_text(&mut self) -> Result<usize> {
        let offset = self.original.len() - self.cursor.remaining();
        let (_, end, _) =
            super::super::text::text_extent(self.original, offset, self.limits.maximum_text_bytes)?;
        self.cursor.take(end - offset)?;
        Ok(offset)
    }

    fn read_numbers(&mut self, count: usize) -> Result<SimulationParameterRecordNumbers> {
        let signed_word = self.cursor.i32()?;
        let unsigned_words = [self.cursor.u32()?, self.cursor.u32()?];
        let conditional_word = self.optional_i32(self.gates.record_word)?;
        let conditional_double_bits = self
            .gates
            .early_word
            .then(|| self.cursor.u64())
            .transpose()?;
        let values_bits = read_array(&mut self.cursor, count, 8, |cursor| cursor.u64())?;
        Ok(SimulationParameterRecordNumbers {
            signed_word,
            unsigned_words,
            conditional_word,
            conditional_double_bits,
            values_bits,
        })
    }

    fn optional_i32(&mut self, present: bool) -> Result<Option<i32>> {
        present.then(|| self.cursor.i32()).transpose()
    }
}

fn read_array<'a, T>(
    cursor: &mut ArchiveCursor<'a>,
    count: usize,
    width: usize,
    read: fn(&mut ArchiveCursor<'a>) -> Result<T>,
) -> Result<Vec<T>> {
    let bytes = cursor.take(checked_extent(count, width, "ParameterSets array extent")?)?;
    let mut bounded = ArchiveCursor::new(bytes);
    let mut values = reserve_items(count, "ParameterSets array allocation")?;
    for _ in 0..count {
        values.push(read(&mut bounded)?);
    }
    Ok(values)
}
