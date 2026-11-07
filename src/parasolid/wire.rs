use super::{NodeReference, TransmitFormat, invalid};
use crate::Result;

pub(super) struct TransmitCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
    pub format: TransmitFormat,
}

impl<'a> TransmitCursor<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self> {
        let (format, offset) = if bytes.starts_with(b"PS") {
            (TransmitFormat::Neutral, 2)
        } else if bytes.starts_with(b"B") {
            (TransmitFormat::Bare, 1)
        } else {
            return Err(invalid(format!(
                "transmit prefix {:?}; expected PS or B",
                bytes.get(..bytes.len().min(2))
            )));
        };
        Ok(Self {
            bytes,
            offset,
            format,
        })
    }

    pub fn take(&mut self, count: usize) -> Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(count)
            .ok_or_else(|| invalid(format!("offset {} + {count}; expected usize", self.offset)))?;
        let result = self.bytes.get(self.offset..end).ok_or_else(|| {
            invalid(format!(
                "range {}..{end}; expected within {} transmit bytes",
                self.offset,
                self.bytes.len()
            ))
        })?;
        self.offset = end;
        Ok(result)
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.offset
    }
    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    pub fn u16(&mut self) -> Result<u16> {
        let span = self.take(2)?;
        let bytes = [span[0], span[1]];
        Ok(match self.format {
            TransmitFormat::Neutral => u16::from_be_bytes(bytes),
            TransmitFormat::Bare => u16::from_le_bytes(bytes),
        })
    }
    pub fn u32(&mut self) -> Result<u32> {
        let span = self.take(4)?;
        let bytes = [span[0], span[1], span[2], span[3]];
        Ok(match self.format {
            TransmitFormat::Neutral => u32::from_be_bytes(bytes),
            TransmitFormat::Bare => u32::from_le_bytes(bytes),
        })
    }
    pub fn u64(&mut self) -> Result<u64> {
        let first = u64::from(self.u32()?);
        let second = u64::from(self.u32()?);
        Ok(match self.format {
            TransmitFormat::Neutral => first << 32 | second,
            TransmitFormat::Bare => first | second << 32,
        })
    }
    pub fn index(&mut self) -> Result<NodeReference> {
        let first = self.u16()? as i16;
        let value = if first >= 0 {
            first as u32
        } else {
            u32::from((-i32::from(first) - 1) as u16) | (u32::from(self.u16()?) << 15)
        };
        NodeReference::from_stored(value)
    }
    pub fn string8(&mut self) -> Result<String> {
        let count = usize::from(self.u8()?);
        self.latin1(count)
    }
    pub fn string32(&mut self) -> Result<String> {
        let count = self.u32()? as usize;
        self.latin1(count)
    }
    fn latin1(&mut self, count: usize) -> Result<String> {
        Ok(self
            .take(count)?
            .iter()
            .map(|byte| char::from(*byte))
            .collect())
    }
}

pub(super) struct TransmitWriter {
    pub output: Vec<u8>,
    format: TransmitFormat,
}

impl TransmitWriter {
    pub fn new(format: TransmitFormat) -> Self {
        let marker = match format {
            TransmitFormat::Neutral => b"PS".as_slice(),
            TransmitFormat::Bare => b"B",
        };
        Self {
            output: marker.to_vec(),
            format,
        }
    }
    pub fn u8(&mut self, value: u8) {
        self.output.push(value);
    }
    pub fn u16(&mut self, value: u16) {
        self.output.extend_from_slice(&match self.format {
            TransmitFormat::Neutral => value.to_be_bytes(),
            TransmitFormat::Bare => value.to_le_bytes(),
        });
    }
    pub fn u32(&mut self, value: u32) {
        self.output.extend_from_slice(&match self.format {
            TransmitFormat::Neutral => value.to_be_bytes(),
            TransmitFormat::Bare => value.to_le_bytes(),
        });
    }
    pub fn u64(&mut self, value: u64) {
        self.output.extend_from_slice(&match self.format {
            TransmitFormat::Neutral => value.to_be_bytes(),
            TransmitFormat::Bare => value.to_le_bytes(),
        });
    }
    pub fn index(&mut self, reference: NodeReference) {
        let value = reference.stored();
        if value <= 0x7fff {
            self.u16(value as u16);
            return;
        }
        self.u16((-((value & 0x7fff) as i32) - 1) as i16 as u16);
        self.u16((value >> 15) as u16);
    }
    pub fn string8(&mut self, text: &str) -> Result<()> {
        let count = super::definitions::latin1_length(text, 255)?;
        self.u8(count as u8);
        self.latin1(text);
        Ok(())
    }
    pub fn string32(&mut self, text: &str) -> Result<()> {
        let count = super::definitions::latin1_length(text, u32::MAX as usize)?;
        self.u32(count as u32);
        self.latin1(text);
        Ok(())
    }
    fn latin1(&mut self, text: &str) {
        self.output
            .extend(text.chars().map(|character| character as u8));
    }
}
