use super::invalid;
use std::io::{self, Write};

pub(super) struct GzipOutput {
    pub bytes: Vec<u8>,
    maximum: usize,
}

impl GzipOutput {
    pub fn new(maximum: usize) -> Self {
        Self {
            bytes: Vec::new(),
            maximum,
        }
    }
}

impl Write for GzipOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let next = self.bytes.len().checked_add(bytes.len());
        if !matches!(next, Some(size) if size <= self.maximum) {
            return Err(io::Error::other(invalid(format!(
                "compressed bytes {} + {}; expected at most {}",
                self.bytes.len(),
                bytes.len(),
                self.maximum
            ))));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_compression_output_rejects_growth_without_modifying_existing_bytes() {
        let mut output = GzipOutput::new(3);
        output.write_all(&[1, 2]).unwrap();
        output.flush().unwrap();
        assert!(output.write_all(&[3, 4]).is_err());
        assert_eq!(output.bytes, [1, 2]);
        output.write_all(&[3]).unwrap();
        assert_eq!(output.bytes, [1, 2, 3]);
    }
}
