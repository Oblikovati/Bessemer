use super::*;
use crate::archive_codec::{ArchiveCursor, StoredDouble};
use crate::composer::arrays::{self, ComposerVectorDimensions, ComposerVectorLayout};

pub(super) struct SceneReader<'a, 'b> {
    input: &'a [u8],
    cursor: ArchiveCursor<'a>,
    pub ledger: &'b mut SceneLedger,
    pub context: ComposerSceneContext,
    pub limits: ComposerSceneLimits,
}

impl<'a, 'b> SceneReader<'a, 'b> {
    pub fn new(
        input: &'a [u8],
        ledger: &'b mut SceneLedger,
        context: ComposerSceneContext,
        limits: ComposerSceneLimits,
    ) -> Self {
        Self {
            input,
            cursor: ArchiveCursor::new(input),
            ledger,
            context,
            limits,
        }
    }

    pub fn remaining(&self) -> usize {
        self.cursor.remaining()
    }

    pub fn push(&mut self, path: &str, value: ComposerSceneValue) -> Result<()> {
        budget(
            "stored fields",
            self.ledger.fields.len() + 1,
            self.limits.maximum_fields,
        )?;
        self.ledger.fields.push(ComposerSceneField {
            path: path.into(),
            value,
        });
        Ok(())
    }

    pub fn byte(&mut self, path: &str) -> Result<u8> {
        let value = self.cursor.u8()?;
        self.push(path, ComposerSceneValue::Byte(value))?;
        Ok(value)
    }

    pub fn word(&mut self, path: &str) -> Result<u32> {
        let value = self.cursor.u32()?;
        self.push(path, ComposerSceneValue::Word(value))?;
        Ok(value)
    }

    pub fn wide(&mut self, path: &str) -> Result<()> {
        let value = self.cursor.u64()?;
        self.push(path, ComposerSceneValue::Wide(value))
    }

    pub fn version(&mut self) -> Result<()> {
        let bits = self.cursor.u64()?;
        if bits != 7.9_f64.to_bits() {
            return Err(unsupported(format!(
                "version bits 0x{bits:016x}; expected version 7.9"
            )));
        }
        self.push(
            "information.version",
            ComposerSceneValue::Double(StoredDouble::from_bits(bits)),
        )
    }

    pub fn patterns(&mut self, path: &str, count: usize) -> Result<()> {
        let words = (0..count)
            .map(|_| self.cursor.u32())
            .collect::<Result<Vec<_>>>()?;
        self.push(path, ComposerSceneValue::Words(words.into()))
    }

    pub fn text(&mut self, path: &str) -> Result<usize> {
        let value = self.read_text_units()?;
        let count = value.units.len();
        self.push(path, ComposerSceneValue::Text(value))?;
        Ok(count)
    }

    fn read_text_units(&mut self) -> Result<ComposerSceneText> {
        let mut units = Vec::new();
        let terminator = loop {
            let word = self.cursor.u16()?;
            if word == 0 {
                break ComposerTextTerminator::Nul;
            }
            if word == 10 {
                break ComposerTextTerminator::Newline;
            }
            self.push_text_unit(&mut units, word)?;
        };
        Ok(ComposerSceneText {
            units: units.into(),
            terminator,
        })
    }

    fn push_text_unit(&mut self, units: &mut Vec<u16>, word: u16) -> Result<()> {
        budget(
            "UTF-16 units",
            self.ledger.text_units + 1,
            self.limits.maximum_text_units,
        )?;
        self.ledger.text_units += 1;
        units.push(word);
        Ok(())
    }

    pub fn extension(&mut self, path: &str, count: usize) -> Result<()> {
        let bytes = self.cursor.take(count)?;
        self.ledger.extension_bytes += count;
        self.push(path, ComposerSceneValue::SizedExtension(bytes.into()))
    }

    pub fn vector(
        &mut self,
        path: &str,
        count: u32,
        dimensions: ComposerVectorDimensions,
    ) -> Result<()> {
        let bytes = &self.input[self.input.len() - self.remaining()..];
        let layout = ComposerVectorLayout { count, dimensions };
        let (block, size) =
            arrays::read_vector_prefix(bytes, layout, self.limits.maximum_decoded_bytes)?;
        self.cursor.take(size)?;
        self.push(path, ComposerSceneValue::Vector(block))
    }

    pub fn indices(&mut self, path: &str, count: u32, domain: u32) -> Result<()> {
        let bytes = &self.input[self.input.len() - self.remaining()..];
        let (block, size) =
            arrays::read_index_prefix(bytes, count, domain, self.limits.maximum_decoded_bytes)?;
        self.cursor.take(size)?;
        self.push(path, ComposerSceneValue::Indices(block))
    }

    pub fn bounded<T>(
        &mut self,
        size: u32,
        read: impl FnOnce(&mut SceneReader<'_, '_>) -> Result<T>,
    ) -> Result<T> {
        let bytes = self.cursor.take(size as usize)?;
        let mut child = SceneReader::new(bytes, self.ledger, self.context, self.limits);
        let result = read(&mut child)?;
        child.cursor.finish()?;
        Ok(result)
    }

    pub fn sized<T>(
        &mut self,
        path: &str,
        omit_text_terminators: bool,
        read: impl FnOnce(&mut SceneReader<'_, '_>) -> Result<T>,
    ) -> Result<T> {
        let size_field = self.ledger.fields.len();
        let size = self.word(path)?;
        let start_field = self.ledger.fields.len();
        let result = self.bounded(size, read)?;
        let end_field = self.ledger.fields.len();
        self.ledger.rules.push(SizeRule {
            size_field,
            start_field,
            end_field,
            omit_text_terminators,
        });
        Ok(result)
    }

    pub fn record(&mut self, geode: bool) -> Result<()> {
        budget(
            "records",
            self.ledger.geodes + self.ledger.drawables + 1,
            self.limits.maximum_records,
        )?;
        if geode {
            self.ledger.geodes += 1;
        } else {
            self.ledger.drawables += 1;
        }
        Ok(())
    }

    pub fn count_fits(&self, count: u32, width: usize, label: &str) -> Result<()> {
        budget(label, count as usize, self.remaining() / width)
    }
}
