use super::{wire::*, *};

struct ValueReader<'a> {
    bytes: &'a [u8],
    position: usize,
    profile: DeferredValueProfile,
    budget: ValueBudget,
}

pub(super) fn document(
    bytes: &[u8],
    profile: DeferredValueProfile,
    limits: DeferredValueLimits,
) -> Result<(DeferredValue, DeferredValueCoverage)> {
    check_limits(bytes.len(), limits)?;
    let mut reader = ValueReader::new(bytes, profile, limits);
    let value = reader.value(0)?;
    Ok((value, reader.coverage()?))
}

impl<'a> ValueReader<'a> {
    fn new(bytes: &'a [u8], profile: DeferredValueProfile, limits: DeferredValueLimits) -> Self {
        Self {
            bytes,
            position: 0,
            profile,
            budget: ValueBudget {
                limits,
                used: 0,
                deepest: 0,
            },
        }
    }

    fn coverage(&self) -> Result<DeferredValueCoverage> {
        if self.position != self.bytes.len() {
            return Err(invalid(format!(
                "trailing bytes at {}; expected EOF at {}",
                self.position,
                self.bytes.len()
            )));
        }
        Ok(DeferredValueCoverage {
            encoded_bytes: self.bytes.len(),
            values: self.budget.used,
            deepest_depth: self.budget.deepest,
        })
    }
    fn take(&mut self, length: usize) -> Result<&[u8]> {
        let end = self.position.checked_add(length).ok_or_else(|| {
            invalid(format!(
                "range {}+{length}; expected no usize overflow",
                self.position
            ))
        })?;
        let bytes = self.bytes.get(self.position..end).ok_or_else(|| {
            invalid(format!(
                "range {}..{end}; expected within {} bytes",
                self.position,
                self.bytes.len()
            ))
        })?;
        self.position = end;
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8> {
        let bytes = self.take(1)?;
        Ok(bytes[0])
    }

    fn word(&mut self, width: usize) -> Result<u64> {
        let profile = self.profile;
        Ok(unpack(self.take(width)?, profile))
    }

    fn value(&mut self, depth: usize) -> Result<DeferredValue> {
        self.budget.charge(depth)?;
        let tag = self.byte()?;
        match tag {
            0x00..=0x7f => Ok(DeferredValue::UnsignedInteger(u64::from(tag))),
            0xe0..=0xff => Ok(DeferredValue::NegativeInteger(i64::from(tag as i8))),
            0x80..=0x8f => self.object(usize::from(tag & 15), depth),
            0x90..=0x9f => self.array(usize::from(tag & 15), depth),
            0xa0..=0xbf => Ok(DeferredValue::StringBytes(
                self.take(usize::from(tag & 31))?.to_vec(),
            )),
            _ => self.extended(tag, depth),
        }
    }

    fn extended(&mut self, tag: u8, depth: usize) -> Result<DeferredValue> {
        match tag {
            0xc0 | 0xc2 | 0xc3 | 0xca..=0xd3 => self.scalar(tag),
            0xc4..=0xc9 | 0xd4..=0xd8 => self.blob(tag),
            0xd9..=0xdf => self.container(tag, depth),
            _ => Err(unsupported(format!(
                "tag {tag:#04x}; expected one of the installed writer forms"
            ))),
        }
    }

    fn scalar(&mut self, tag: u8) -> Result<DeferredValue> {
        match tag {
            0xc0 => Ok(DeferredValue::Null),
            0xc2 | 0xc3 => Ok(DeferredValue::Boolean(tag == 0xc3)),
            0xca => Ok(DeferredValue::Float32Bits(self.word(4)? as u32)),
            0xcb => Ok(DeferredValue::Float64Bits(self.word(8)?)),
            0xcc..=0xcf => Ok(DeferredValue::UnsignedInteger(
                self.word(1 << (tag - 0xcc))?,
            )),
            0xd0..=0xd3 => self.negative(1 << (tag - 0xd0)),
            _ => Err(invalid(format!(
                "scalar tag {tag:#04x}; expected null/bool/float/integer"
            ))),
        }
    }

    fn negative(&mut self, width: usize) -> Result<DeferredValue> {
        let bits = self.word(width)?;
        let shift = 64 - width * 8;
        let value = ((bits << shift) as i64) >> shift;
        Ok(if value < 0 {
            DeferredValue::NegativeInteger(value)
        } else {
            DeferredValue::UnsignedInteger(value as u64)
        })
    }

    fn blob(&mut self, tag: u8) -> Result<DeferredValue> {
        let length = match tag {
            0xc4..=0xc6 => self.word(1 << (tag - 0xc4))? as usize,
            0xc7..=0xc9 => self.word(1 << (tag - 0xc7))? as usize,
            0xd4..=0xd8 => 1 << (tag - 0xd4),
            _ => {
                return Err(invalid(format!(
                    "blob tag {tag:#04x}; expected binary or extension"
                )));
            }
        };
        if (0xc4..=0xc6).contains(&tag) {
            return Ok(DeferredValue::BinaryBytes(self.take(length)?.to_vec()));
        }
        let type_byte = self.byte()?;
        Ok(DeferredValue::Extension {
            type_byte,
            bytes: self.take(length)?.to_vec(),
        })
    }

    fn container(&mut self, tag: u8, depth: usize) -> Result<DeferredValue> {
        let width = if tag == 0xd9 {
            1
        } else if matches!(tag, 0xda | 0xdc | 0xde) {
            2
        } else {
            4
        };
        let count = self.word(width)? as usize;
        match tag {
            0xd9..=0xdb => Ok(DeferredValue::StringBytes(self.take(count)?.to_vec())),
            0xdc | 0xdd => self.array(count, depth),
            0xde | 0xdf => self.object(count, depth),
            _ => Err(invalid(format!(
                "container tag {tag:#04x}; expected string/array/object"
            ))),
        }
    }

    fn array(&mut self, count: usize, depth: usize) -> Result<DeferredValue> {
        self.require_payload_values(count)?;
        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(self.value(depth + 1)?);
        }
        Ok(DeferredValue::Array(elements))
    }

    fn object(&mut self, count: usize, depth: usize) -> Result<DeferredValue> {
        let children = count
            .checked_mul(2)
            .ok_or_else(|| invalid(format!("pair count {count}; expected 2*count within usize")))?;
        self.require_payload_values(children)?;
        let mut pairs = Vec::with_capacity(count);
        for _ in 0..count {
            pairs.push((self.value(depth + 1)?, self.value(depth + 1)?));
        }
        Ok(DeferredValue::ObjectPairs(pairs))
    }

    fn require_payload_values(&self, count: usize) -> Result<()> {
        self.budget.require_children(count)?;
        let remaining = self.bytes.len() - self.position;
        if count > remaining {
            return Err(invalid(format!(
                "child count {count}; expected at most {remaining} remaining minimum-width bytes"
            )));
        }
        Ok(())
    }
}
