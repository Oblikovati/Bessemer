use super::thermal::{SimulationThermalField, SimulationThermalLimits, SimulationThermalValue};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{SimulationEmbeddedArchive, SimulationEmbeddedProfile};
use crate::simulation::{SimulationPersistText, SimulationPersistVersion, invalid};
use std::sync::Arc;

pub(super) struct BinaryLoadReader<'a> {
    cursor: ArchiveCursor<'a>,
    pub bytes: Arc<[u8]>,
    pub version: i32,
    pub limits: SimulationThermalLimits,
    pub fields: Vec<SimulationThermalField>,
    pub offsets: Vec<(usize, usize)>,
    entries: usize,
}

impl<'a> BinaryLoadReader<'a> {
    pub fn new(
        input: &'a [u8],
        version: SimulationPersistVersion,
        limits: SimulationThermalLimits,
    ) -> Result<Self> {
        if input.len() > limits.maximum_bytes {
            return Err(invalid(
                "thermal input",
                format!(
                    "{} bytes; expected <= {}",
                    input.len(),
                    limits.maximum_bytes
                ),
            ));
        }
        Ok(Self {
            cursor: ArchiveCursor::new(input),
            bytes: input.into(),
            version: version.value(),
            limits,
            fields: Vec::new(),
            offsets: Vec::new(),
            entries: 0,
        })
    }

    pub fn position(&self) -> usize {
        self.bytes.len() - self.cursor.remaining()
    }

    fn push(&mut self, label: &str, start: usize, value: SimulationThermalValue) -> Result<()> {
        if self.fields.len() >= self.limits.maximum_fields {
            return Err(invalid(
                label,
                format!(
                    "field count {}; expected < {}",
                    self.fields.len(),
                    self.limits.maximum_fields
                ),
            ));
        }
        self.offsets.push((start, self.position()));
        self.fields.push(SimulationThermalField {
            path: label.into(),
            value,
        });
        Ok(())
    }

    pub fn signed(&mut self, label: &str) -> Result<i32> {
        let start = self.position();
        let value = self.cursor.i32()?;
        self.push(label, start, SimulationThermalValue::Signed(value))?;
        Ok(value)
    }

    pub fn unsigned(&mut self, label: &str) -> Result<()> {
        let start = self.position();
        let value = self.cursor.u32()?;
        self.push(label, start, SimulationThermalValue::Unsigned(value))
    }

    pub fn double(&mut self, label: &str) -> Result<()> {
        let start = self.position();
        let bits = self.cursor.u64()?;
        self.push(label, start, SimulationThermalValue::DoubleBits(bits))
    }

    pub fn text(&mut self, label: &str) -> Result<()> {
        let start = self.position();
        let text = SimulationPersistText::read_shared(
            self.bytes.clone(),
            start,
            self.limits.maximum_text_bytes,
        )?;
        self.cursor.take(text.source_offsets().2 - start)?;
        self.push(label, start, SimulationThermalValue::Text(text))
    }

    pub fn blob(&mut self, label: &str, count: usize) -> Result<()> {
        let start = self.position();
        let raw: Arc<[u8]> = self.cursor.take(count)?.into();
        self.push(label, start, SimulationThermalValue::Bytes(raw))
    }

    pub fn embedded(&mut self, label: &str) -> Result<()> {
        let start = self.position();
        let remaining = &self.bytes[start..];
        let version = ArchiveCursor::new(remaining).u32()?;
        let profile = if version == 4281 {
            SimulationEmbeddedProfile::Legacy4281
        } else {
            SimulationEmbeddedProfile::Modern
        };
        let (archive, consumed) =
            SimulationEmbeddedArchive::from_prefix(remaining, profile, self.limits.embedded)?;
        self.cursor.take(consumed)?;
        self.push(
            label,
            start,
            SimulationThermalValue::Embedded(Box::new(archive)),
        )
    }

    pub fn count(&mut self, label: &str, maximum: usize, minimum: usize) -> Result<usize> {
        let value = self.signed(label)?;
        self.bounded(value, label, maximum, minimum)
    }

    pub fn bounded(
        &mut self,
        value: i32,
        label: &str,
        maximum: usize,
        minimum: usize,
    ) -> Result<usize> {
        let count = self.available_count(value, label, maximum, minimum)?;
        self.charge_entries(label, count)?;
        Ok(count)
    }

    pub fn available_count(
        &self,
        value: i32,
        label: &str,
        maximum: usize,
        minimum: usize,
    ) -> Result<usize> {
        let capacity = self
            .cursor
            .remaining()
            .checked_div(minimum)
            .ok_or_else(|| {
                invalid(
                    label,
                    format!("minimum {minimum}; expected nonzero entry width"),
                )
            })?;
        usize::try_from(value)
            .ok()
            .filter(|count| *count <= maximum && *count <= capacity)
            .ok_or_else(|| {
                invalid(
                    label,
                    format!("count {value}; expected 0..={maximum} with {minimum} bytes per entry"),
                )
            })
    }

    fn charge_entries(&mut self, label: &str, count: usize) -> Result<()> {
        self.entries = self.entries.checked_add(count).ok_or_else(|| {
            invalid(
                label,
                format!("count {count}; expected representable aggregate entries"),
            )
        })?;
        if self.entries > self.limits.maximum_entries {
            return Err(invalid(
                label,
                format!(
                    "aggregate {}; expected <= {}",
                    self.entries, self.limits.maximum_entries
                ),
            ));
        }
        Ok(())
    }

    pub fn words(&mut self, label: &str, count: usize) -> Result<()> {
        for index in 0..count {
            self.signed(&format!("{label}[{index}]"))?;
        }
        Ok(())
    }

    pub fn doubles(&mut self, label: &str, count: usize) -> Result<()> {
        for index in 0..count {
            self.double(&format!("{label}[{index}]"))?;
        }
        Ok(())
    }

    pub fn texts(&mut self, label: &str, count: usize) -> Result<()> {
        for index in 0..count {
            self.text(&format!("{label}[{index}]"))?;
        }
        Ok(())
    }

    pub fn finish(&self) -> Result<()> {
        self.cursor.finish()
    }
}
