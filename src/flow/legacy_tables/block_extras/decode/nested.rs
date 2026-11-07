use super::*;

impl ExtraReader<'_> {
    pub(super) fn nested_rows(&mut self, block: usize, count: usize) -> Result<()> {
        self.minimum(count, if self.version < 5 { 1 } else { 4 })?;
        for row in 0..count {
            self.nested_row(block, row)?;
        }
        Ok(())
    }

    fn nested_row(&mut self, block: usize, row: usize) -> Result<()> {
        let entity = Entity::NestedRow { block, row };
        let scope = self.begin(entity)?;
        let count = if self.version < 5 {
            usize::from(self.byte(entity, Kind::NestedCount)?)
        } else {
            self.count(entity, Kind::NestedCount)?
        };
        self.minimum(count, if self.version >= 6 { 5 } else { 1 })?;
        self.entity_room(count)?;
        for index in 0..count {
            self.nested_entry(block, row, index)?;
        }
        self.end(scope);
        Ok(())
    }

    fn nested_entry(&mut self, block: usize, row: usize, index: usize) -> Result<()> {
        let entity = Entity::NestedEntry { block, row, index };
        let scope = self.begin(entity)?;
        let count = usize::from(self.byte(entity, Kind::NestedElementCount)?);
        self.entities[scope].index = Some(self.nested_index(entity, index)?);
        self.words(entity, Kind::NestedWords, Signed32, count)?;
        self.words(entity, Kind::NestedWide, WideBits, 3 * count)?;
        self.end(scope);
        Ok(())
    }

    fn nested_index(&mut self, entity: Entity, ordinal: usize) -> Result<ExtraIndexSource> {
        if self.version < 6 {
            return Ok(ExtraIndexSource::Derived(ordinal as i32));
        }
        let offset = self.position();
        self.signed(entity, Kind::NestedIndex)?;
        Ok(ExtraIndexSource::Stored(offset))
    }
}
