use super::archive_decode::DisplayDecoder;
use crate::{BessemerError, Result, archive_codec::MaterialRead};

impl MaterialRead for DisplayDecoder<'_> {
    fn version(&self) -> u32 {
        self.context.version.value()
    }

    fn remaining(&self) -> usize {
        self.cursor.remaining()
    }

    fn invalid(&self, cause: String) -> BessemerError {
        super::archive_error::invalid(cause)
    }

    fn entries(&mut self, count: usize, label: &str) -> Result<()> {
        DisplayDecoder::entries(self, count, label)
    }

    fn u8(&mut self, label: &str) -> Result<u8> {
        DisplayDecoder::u8(self, label)
    }

    fn i16(&mut self, label: &str) -> Result<i16> {
        DisplayDecoder::i16(self, label)
    }

    fn u16(&mut self, label: &str) -> Result<u16> {
        DisplayDecoder::u16(self, label)
    }

    fn i32(&mut self, label: &str) -> Result<i32> {
        DisplayDecoder::i32(self, label)
    }

    fn u32(&mut self, label: &str) -> Result<u32> {
        DisplayDecoder::u32(self, label)
    }

    fn f32(&mut self, label: &str) -> Result<()> {
        DisplayDecoder::f32(self, label)
    }

    fn f64(&mut self, label: &str) -> Result<()> {
        DisplayDecoder::f64(self, label)
    }

    fn text(&mut self, label: &str) -> Result<()> {
        DisplayDecoder::text(self, label)
    }

    fn scope<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        DisplayDecoder::scope(self, name, read)
    }
}
