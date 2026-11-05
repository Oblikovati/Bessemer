use super::{wire::*, *};

struct ValueWriter {
    output: Vec<u8>,
    profile: DeferredValueProfile,
    budget: ValueBudget,
}

pub(super) fn document(
    value: &DeferredValue,
    profile: DeferredValueProfile,
    limits: DeferredValueLimits,
) -> Result<Vec<u8>> {
    check_limits(0, limits)?;
    let mut writer = ValueWriter {
        output: Vec::new(),
        profile,
        budget: ValueBudget {
            limits,
            used: 0,
            deepest: 0,
        },
    };
    writer.value(value, 0)?;
    Ok(writer.output)
}

impl ValueWriter {
    fn append(&mut self, bytes: &[u8]) -> Result<()> {
        let length = self.output.len().checked_add(bytes.len()).ok_or_else(|| {
            invalid(format!(
                "output {}+{}; expected no usize overflow",
                self.output.len(),
                bytes.len()
            ))
        })?;
        check_limits(length, self.budget.limits)?;
        self.output.extend_from_slice(bytes);
        Ok(())
    }

    fn word(&mut self, tag: u8, word: u64, width: usize) -> Result<()> {
        self.append(&[tag])?;
        self.append(&packed(word, width, self.profile))
    }

    fn value(&mut self, value: &DeferredValue, depth: usize) -> Result<()> {
        self.budget.charge(depth)?;
        match value {
            DeferredValue::Array(elements) => self.array(elements, depth),
            DeferredValue::ObjectPairs(pairs) => self.object(pairs, depth),
            DeferredValue::StringBytes(bytes) => self.blob(bytes, true),
            DeferredValue::BinaryBytes(bytes) => self.blob(bytes, false),
            DeferredValue::Extension { type_byte, bytes } => self.extension(*type_byte, bytes),
            _ => self.scalar(value),
        }
    }

    fn scalar(&mut self, value: &DeferredValue) -> Result<()> {
        match value {
            DeferredValue::Null => self.append(&[0xc0]),
            DeferredValue::Boolean(value) => self.append(&[if *value { 0xc3 } else { 0xc2 }]),
            DeferredValue::NegativeInteger(value) => self.negative(*value),
            DeferredValue::UnsignedInteger(value) => self.unsigned(*value),
            DeferredValue::Float32Bits(bits) => self.word(0xca, u64::from(*bits), 4),
            DeferredValue::Float64Bits(bits) => self.word(0xcb, *bits, 8),
            _ => Err(invalid(format!("value {value:?}; expected scalar"))),
        }
    }

    fn negative(&mut self, value: i64) -> Result<()> {
        if value >= 0 {
            return Err(invalid(format!(
                "negative integer {value}; expected < 0; use UnsignedInteger for nonnegative persisted values"
            )));
        }
        if value >= -32 {
            return self.append(&[value as u8]);
        }
        let width: usize = if value >= -128 {
            1
        } else if value >= -32768 {
            2
        } else if value >= -2147483648 {
            4
        } else {
            8
        };
        self.word(0xd0 + width.trailing_zeros() as u8, value as u64, width)
    }

    fn unsigned(&mut self, value: u64) -> Result<()> {
        if value < 128 {
            return self.append(&[value as u8]);
        }
        let width: usize = if value <= 255 {
            1
        } else if value <= 65535 {
            2
        } else if value <= u64::from(u32::MAX) {
            4
        } else {
            8
        };
        self.word(0xcc + width.trailing_zeros() as u8, value, width)
    }

    fn count(
        &mut self,
        count: usize,
        fix: Option<(u8, usize)>,
        byte: Option<u8>,
        short: u8,
        long: u8,
    ) -> Result<()> {
        let length = checked_count(count)?;
        if let Some((tag, _)) = fix.filter(|(_, maximum)| count <= *maximum) {
            return self.append(&[tag | length as u8]);
        }
        if let Some(tag) = byte.filter(|_| count <= 255) {
            return self.word(tag, u64::from(length), 1);
        }
        if count <= 65535 {
            return self.word(short, u64::from(length), 2);
        }
        self.word(long, u64::from(length), 4)
    }

    fn blob(&mut self, bytes: &[u8], string: bool) -> Result<()> {
        if string {
            self.count(bytes.len(), Some((0xa0, 31)), Some(0xd9), 0xda, 0xdb)?;
        } else {
            self.count(bytes.len(), None, Some(0xc4), 0xc5, 0xc6)?;
        }
        self.append(bytes)
    }

    fn extension(&mut self, type_byte: u8, bytes: &[u8]) -> Result<()> {
        match bytes.len() {
            1 | 2 | 4 | 8 | 16 => self.append(&[0xd4 + bytes.len().trailing_zeros() as u8])?,
            _ => self.count(bytes.len(), None, Some(0xc7), 0xc8, 0xc9)?,
        }
        self.append(&[type_byte])?;
        self.append(bytes)
    }

    fn array(&mut self, elements: &[DeferredValue], depth: usize) -> Result<()> {
        self.budget.require_children(elements.len())?;
        self.count(elements.len(), Some((0x90, 15)), None, 0xdc, 0xdd)?;
        elements
            .iter()
            .try_for_each(|value| self.value(value, depth + 1))
    }

    fn object(&mut self, pairs: &[(DeferredValue, DeferredValue)], depth: usize) -> Result<()> {
        let children = pairs.len().checked_mul(2).ok_or_else(|| {
            invalid(format!(
                "pair count {}; expected 2*count within usize",
                pairs.len()
            ))
        })?;
        self.budget.require_children(children)?;
        self.count(pairs.len(), Some((0x80, 15)), None, 0xde, 0xdf)?;
        for (key, value) in pairs {
            self.value(key, depth + 1)?;
            self.value(value, depth + 1)?;
        }
        Ok(())
    }
}

fn checked_count(count: usize) -> Result<u32> {
    u32::try_from(count)
        .map_err(|_| unsupported(format!("length/count {count}; expected <= u32::MAX")))
}

#[cfg(test)]
#[path = "../../tests/deferred_values/lengths.rs"]
mod length_tests;
