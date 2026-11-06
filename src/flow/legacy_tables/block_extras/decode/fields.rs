use super::*;

impl ExtraReader<'_> {
    pub(super) fn words(
        &mut self,
        entity: Entity,
        kind: Kind,
        encoding: FlowBlockExtraEncoding,
        count: usize,
    ) -> Result<usize> {
        let width = encoding.element_bytes();
        self.minimum(count, width)?;
        self.field_room(count)?;
        let start = self.position();
        self.cursor.take(count * width)?;
        self.field(entity, kind, encoding, (start, count, width))?;
        Ok(start)
    }

    fn field(
        &mut self,
        entity: Entity,
        kind: Kind,
        encoding: FlowBlockExtraEncoding,
        layout: (usize, usize, usize),
    ) -> Result<()> {
        let (start, count, stride) = layout;
        self.field_room(count)?;
        self.fields.push(ExtraFieldSpan {
            entity,
            kind,
            encoding,
            start,
            count,
            stride,
        });
        self.values += count;
        Ok(())
    }

    fn field_room(&self, count: usize) -> Result<()> {
        let available = self.options.maximum_values - self.values;
        if self.fields.len() < self.options.maximum_fields && count <= available {
            return Ok(());
        }
        Err(extra_invalid(format!(
            "field count {}, additional values {count}, used {}; expected fields below {} and <= {available} additional values",
            self.fields.len(),
            self.values,
            self.options.maximum_fields
        )))
    }

    pub(super) fn signed(&mut self, entity: Entity, kind: Kind) -> Result<i32> {
        let start = self.words(entity, kind, Signed32, 1)?;
        Ok(wire::signed_at(self.raw, start))
    }

    pub(super) fn byte(&mut self, entity: Entity, kind: Kind) -> Result<u8> {
        let start = self.words(entity, kind, Byte, 1)?;
        Ok(self.raw[start])
    }

    pub(super) fn count(&mut self, entity: Entity, kind: Kind) -> Result<usize> {
        let count = self.signed(entity, kind)?;
        if count >= 0 {
            return Ok(count as usize);
        }
        Err(extra_invalid(format!(
            "{entity:?} {kind:?} count {count}; expected nonnegative signed-i32 count"
        )))
    }

    pub(super) fn bytes(&mut self, entity: Entity) -> Result<usize> {
        let count = usize::from(self.byte(entity, Kind::ByteArrayCount)?);
        self.words(entity, Kind::ByteArray, Byte, count)?;
        Ok(count)
    }

    pub(super) fn mixed(&mut self, entity: Entity) -> Result<()> {
        let scope = self.begin(entity)?;
        let count = self.count(entity, Kind::MixedCount)?;
        if self.version < 8 {
            self.words(entity, Kind::MixedWords, Signed32, count)?;
            self.words(entity, Kind::MixedWide, WideBits, count)?;
        } else {
            self.interleaved(entity, count)?;
        }
        self.end(scope);
        Ok(())
    }

    fn interleaved(&mut self, entity: Entity, count: usize) -> Result<()> {
        self.minimum(count, 12)?;
        let start = self.position();
        self.cursor.take(count * 12)?;
        self.field(entity, Kind::MixedWords, Signed32, (start, count, 12))?;
        let wide_start = start + if count == 0 { 0 } else { 4 };
        self.field(entity, Kind::MixedWide, WideBits, (wide_start, count, 12))
    }
}
