use super::{ModelDecoder, ModelFieldValue as Value};
use crate::archive_codec::{ArchiveString, MaterialRead, StoredDouble, StoredFloat};
use crate::{BessemerError, Result};

macro_rules! model_integer {
    ($method:ident, $result:ty, $variant:ident, $read:ident) => {
        fn $method(&mut self, label: &str) -> Result<$result> {
            let value = self.scalar(|cursor| cursor.$read())? as $result;
            self.field(label, Value::$variant(value))?;
            Ok(value)
        }
    };
}

impl MaterialRead for ModelDecoder<'_> {
    fn version(&self) -> u32 {
        self.archive_version().value()
    }

    fn remaining(&self) -> usize {
        ModelDecoder::remaining(self)
    }

    fn invalid(&self, cause: String) -> BessemerError {
        super::invalid(cause)
    }

    fn entries(&mut self, count: usize, label: &str) -> Result<()> {
        ModelDecoder::entries(self, count, label)
    }

    model_integer!(u8, u8, U8, u8);

    fn u16(&mut self, label: &str) -> Result<u16> {
        ModelDecoder::u16(self, label)
    }
    model_integer!(i16, i16, I16, u16);
    model_integer!(i32, i32, I32, i32);
    model_integer!(u32, u32, U32, u32);

    fn f32(&mut self, label: &str) -> Result<()> {
        let value = StoredFloat::from_bits(self.scalar(|cursor| cursor.u32())?);
        self.field(label, Value::F32(value))
    }

    fn f64(&mut self, label: &str) -> Result<()> {
        let value = StoredDouble::from_bits(self.scalar(|cursor| cursor.u64())?);
        self.field(label, Value::F64(value))
    }

    fn text(&mut self, label: &str) -> Result<()> {
        let value = self.scalar(ArchiveString::decode)?;
        self.field(label, Value::Text(value))
    }

    fn scope<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        ModelDecoder::scope(self, name, read)
    }
}
