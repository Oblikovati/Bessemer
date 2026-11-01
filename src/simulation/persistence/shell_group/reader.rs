use super::{SimulationShellGroupField, SimulationShellGroupLimits, SimulationShellGroupValue};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ArchiveString},
    simulation::invalid,
};

pub(super) struct ShellGroupReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    limits: SimulationShellGroupLimits,
    pub fields: Vec<SimulationShellGroupField>,
    pub offsets: Vec<(usize, usize)>,
    pub entries: usize,
    pub text_bytes: usize,
}

impl<'a> ShellGroupReader<'a> {
    pub fn new(input: &'a [u8], limits: SimulationShellGroupLimits) -> Result<Self> {
        if input.len() > limits.maximum_bytes {
            return Err(invalid(
                "ShellManagerGroup input",
                format!(
                    "{} bytes; expected <= {}",
                    input.len(),
                    limits.maximum_bytes
                ),
            ));
        }
        Ok(Self {
            cursor: ArchiveCursor::new(input),
            length: input.len(),
            limits,
            fields: Vec::new(),
            offsets: Vec::new(),
            entries: 0,
            text_bytes: 0,
        })
    }

    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    fn require_field(&self, label: &str) -> Result<()> {
        if self.fields.len() < self.limits.maximum_fields {
            return Ok(());
        }
        Err(invalid(
            label,
            format!(
                "field count {}; expected < {}",
                self.fields.len(),
                self.limits.maximum_fields
            ),
        ))
    }

    fn push(&mut self, label: &str, start: usize, value: SimulationShellGroupValue) -> Result<()> {
        self.require_field(label)?;
        self.offsets.push((start, self.position()));
        self.fields.push(SimulationShellGroupField {
            path: label.into(),
            value,
        });
        Ok(())
    }

    pub fn signed(&mut self, label: &str) -> Result<i32> {
        let start = self.position();
        let value = self
            .cursor
            .i32()
            .map_err(|error| shell_group_field_error(label, error))?;
        self.push(label, start, SimulationShellGroupValue::Signed(value))?;
        Ok(value)
    }

    pub fn double(&mut self, label: &str) -> Result<()> {
        let start = self.position();
        let value = self
            .cursor
            .u64()
            .map_err(|error| shell_group_field_error(label, error))?;
        self.push(label, start, SimulationShellGroupValue::DoubleBits(value))
    }

    pub fn text(&mut self, label: &str) -> Result<()> {
        self.require_field(label)?;
        let start = self.position();
        let remaining = self.limits.maximum_text_bytes - self.text_bytes;
        let (value, size) = ArchiveString::decode_bounded(&mut self.cursor, remaining)
            .map_err(|error| shell_group_field_error(label, error))?;
        self.text_bytes += size;
        self.push(label, start, SimulationShellGroupValue::Text(value))
    }

    pub fn record_count(&mut self) -> Result<usize> {
        let signed = self.signed("count")?;
        let available = self.cursor.remaining() / 32;
        let maximum = 100000usize.min(self.limits.maximum_entries).min(available);
        if signed < 0 || signed as usize > maximum {
            return Err(invalid(
                "ShellManagerGroup count",
                format!("{signed}; expected 0..={maximum} records with at least 32 bytes each"),
            ));
        }
        self.entries = signed as usize;
        Ok(self.entries)
    }

    pub fn finish(&self) -> Result<()> {
        self.cursor.finish()
    }
}

fn shell_group_field_error(label: &str, mut error: crate::BessemerError) -> crate::BessemerError {
    error.operation = "decode ShellManagerGroup";
    error.entity = label.into();
    error
}
