use super::{
    DisplayFieldValue as Value, StoredDouble, archive_decode::DisplayDecoder,
    archive_error::invalid,
};
use crate::{
    Result,
    archive_codec::{ArchiveString, StoredFloat},
};

macro_rules! scalar {
    ($name:ident,$type:ty,$variant:ident,$read:ident) => {
        pub fn $name(&mut self, label: &str) -> Result<$type> {
            let value = self.cursor.$read()?;
            self.field(label, Value::$variant(value))?;
            Ok(value)
        }
    };
}
impl DisplayDecoder<'_> {
    scalar!(u8, u8, U8, u8);
    scalar!(u16, u16, U16, u16);
    scalar!(i32, i32, I32, i32);
    scalar!(u32, u32, U32, u32);
    pub fn i16(&mut self, label: &str) -> Result<i16> {
        let value = self.cursor.u16()? as i16;
        self.field(label, Value::I16(value))?;
        Ok(value)
    }
    pub fn f32(&mut self, label: &str) -> Result<()> {
        let value = StoredFloat::from_bits(self.cursor.u32()?);
        self.field(label, Value::F32(value))
    }
    pub fn f64(&mut self, label: &str) -> Result<()> {
        let value = StoredDouble::from_bits(self.cursor.u64()?);
        self.field(label, Value::F64(value))
    }
    pub fn text(&mut self, label: &str) -> Result<()> {
        let value = ArchiveString::decode(&mut self.cursor)?;
        self.field(label, Value::Text(value))
    }
    pub fn transform(&mut self, label: &str) -> Result<()> {
        let value = super::transform_codec::read_compact(&mut self.cursor)?;
        self.field(label, Value::Transform(value))
    }
    pub fn fixed_transform(&mut self, label: &str) -> Result<()> {
        let value =
            super::transform_codec::read(&mut self.cursor, super::TransformEncoding::Fixed)?;
        let super::TransformContents::Fixed(value) = value else {
            return Err(invalid(
                "fixed transform reader returned a compact form; expected fixed overload",
            ));
        };
        self.field(label, Value::FixedTransform(value))
    }
    pub fn bounds(&mut self, label: &str) -> Result<()> {
        let value = crate::archive_codec::read_box(&mut self.cursor)?;
        self.field(label, Value::Box(value))
    }
    pub fn i32s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.i32(label)?;
        }
        Ok(())
    }
    pub fn f32s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.f32(label)?;
        }
        Ok(())
    }
    pub fn f64s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.f64(label)?;
        }
        Ok(())
    }
    pub fn count(&mut self, label: &str, minimum: usize) -> Result<usize> {
        let count = self.i32(label)?;
        if count < 0 || count as usize > self.cursor.remaining() / minimum {
            return Err(invalid(format!(
                "{label} count {count}; expected nonnegative count fitting {} bytes",
                self.cursor.remaining()
            )));
        }
        self.entries(count as usize, label)?;
        Ok(count as usize)
    }
}
