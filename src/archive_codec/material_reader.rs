use crate::{BessemerError, Result};

/// Named primitives used by shared model/display appearance serializers.
/// Implementors keep field storage, budgets, scopes and error context local to their archive.
pub(crate) trait MaterialRead: Sized + Send {
    fn version(&self) -> u32;
    fn remaining(&self) -> usize;
    fn invalid(&self, cause: String) -> BessemerError;
    fn entries(&mut self, count: usize, label: &str) -> Result<()>;
    fn u8(&mut self, label: &str) -> Result<u8>;
    fn u16(&mut self, label: &str) -> Result<u16>;
    fn i16(&mut self, label: &str) -> Result<i16>;
    fn i32(&mut self, label: &str) -> Result<i32>;
    fn u32(&mut self, label: &str) -> Result<u32>;
    fn f32(&mut self, label: &str) -> Result<()>;
    fn f64(&mut self, label: &str) -> Result<()>;
    fn text(&mut self, label: &str) -> Result<()>;
    fn i32s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.i32(label)?;
        }
        Ok(())
    }
    fn f32s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.f32(label)?;
        }
        Ok(())
    }
    fn f64s(&mut self, labels: &[&str]) -> Result<()> {
        for label in labels {
            self.f64(label)?;
        }
        Ok(())
    }
    fn scope<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> Result<T>) -> Result<T>;
}
