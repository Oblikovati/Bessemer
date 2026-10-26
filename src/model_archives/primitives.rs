use super::{ModelDecoder, ModelFieldValue, invalid};
use crate::Result;
use crate::archive_codec::{ObjectReader, ObjectRecord};

impl ModelDecoder<'_> {
    pub fn escaped_index(&mut self, label: &str, escape: u16) -> Result<i32> {
        let index = self.scalar(|cursor| cursor.u16())?;
        if index != escape {
            self.field(label, ModelFieldValue::U16(index))?;
            return Ok(i32::from(index));
        }
        self.field(&format!("{label}_escape"), ModelFieldValue::U16(index))?;
        <Self as crate::archive_codec::MaterialRead>::i32(self, label)
    }

    pub fn transform(&mut self, label: &str) -> Result<()> {
        let value = self.scalar(crate::archive_codec::transform_codec::read_compact)?;
        self.field(label, ModelFieldValue::Transform(Box::new(value)))
    }

    pub fn u64(&mut self, label: &str) -> Result<u64> {
        let value = self.scalar(|cursor| cursor.u64())?;
        self.field(label, ModelFieldValue::U64(value))?;
        Ok(value)
    }

    pub fn signed_count(&mut self, label: &str, minimum_bytes: usize) -> Result<usize> {
        let count = <Self as crate::archive_codec::MaterialRead>::i32(self, label)?;
        self.signed_count_value(label, count, minimum_bytes)
    }

    pub fn signed_count_value(
        &mut self,
        label: &str,
        count: i32,
        minimum_bytes: usize,
    ) -> Result<usize> {
        self.counted_entries(label, i64::from(count), minimum_bytes)
    }

    pub fn unsigned_count(&mut self, label: &str, minimum: usize) -> Result<usize> {
        let count = <Self as crate::archive_codec::MaterialRead>::u32(self, label)?;
        self.counted_entries(label, i64::from(count), minimum)
    }

    pub fn counted_entries(&mut self, label: &str, count: i64, minimum: usize) -> Result<usize> {
        let available = self.count_capacity(label, minimum)?;
        let shape = usize::try_from(count).ok().filter(|value| *value <= available).ok_or_else(|| {
            invalid(format!("{label} count {count}; expected 0..={available} entries of at least {minimum} bytes"))
        });
        let count = self.checked(shape)?;
        self.entries(count, label)?;
        Ok(count)
    }

    fn count_capacity(&mut self, label: &str, minimum_bytes: usize) -> Result<usize> {
        let available = self.remaining().checked_div(minimum_bytes).ok_or_else(|| {
            invalid(format!(
                "{label} minimum entry bytes {minimum_bytes}; expected nonzero"
            ))
        });
        self.checked(available)
    }

    pub fn fixed_octets(&mut self, label: &str, size: usize) -> Result<()> {
        self.entries(size, label)?;
        for index in 0..size {
            <Self as crate::archive_codec::MaterialRead>::u8(self, &format!("{label}[{index}]"))?;
        }
        Ok(())
    }

    pub fn u16(&mut self, label: &str) -> Result<u16> {
        let value = self.scalar(|cursor| cursor.u16())?;
        self.field(label, ModelFieldValue::U16(value))?;
        Ok(value)
    }

    pub fn object(
        &mut self,
        label: &str,
        map: &mut ObjectReader,
        role: super::ModelObjectRole<'_>,
    ) -> Result<()> {
        self.object_record(label, map, role).map(|_| ())
    }

    pub fn required_object(
        &mut self,
        label: &str,
        map: &mut ObjectReader,
        role: super::ModelObjectRole<'_>,
    ) -> Result<()> {
        let record = self.object_record(label, map, role)?;
        let shape = if matches!(record, ObjectRecord::Null) {
            Err(invalid(format!(
                "{label} null object; expected a nonnull declared object"
            )))
        } else {
            Ok(())
        };
        self.checked(shape)
    }

    pub(super) fn object_record(
        &mut self,
        label: &str,
        map: &mut ObjectReader,
        role: super::ModelObjectRole<'_>,
    ) -> Result<ObjectRecord> {
        let record = self.object_tag(label, map, role)?;
        if let ObjectRecord::New { class, .. } = &record {
            self.scope(label, |reader| super::object_body::read(reader, map, class))?;
        }
        Ok(record)
    }

    pub fn object_count(&mut self, label: &str) -> Result<usize> {
        self.count_with_minimum(label, 2)
    }

    pub fn count_with_minimum(&mut self, label: &str, minimum: usize) -> Result<usize> {
        let count = self.stored_object_count(label)?;
        let available = self.remaining().checked_div(minimum).ok_or_else(|| {
            invalid(format!(
                "{label} minimum entry bytes {minimum}; expected nonzero"
            ))
        });
        let available = self.checked(available)?;
        let shape = if count <= available {
            Ok(())
        } else {
            Err(invalid(format!(
                "{label} count {count}; expected <= {available} entries of at least {minimum} remaining bytes"
            )))
        };
        self.checked(shape)?;
        self.entries(count, label)?;
        Ok(count)
    }

    fn stored_object_count(&mut self, label: &str) -> Result<usize> {
        let short = self.u16(&format!("{label}.count_word"))?;
        if short != u16::MAX {
            return Ok(short as usize);
        }
        let count = <Self as crate::archive_codec::MaterialRead>::u32(
            self,
            &format!("{label}.count_dword"),
        )?;
        Ok(count as usize)
    }
}
