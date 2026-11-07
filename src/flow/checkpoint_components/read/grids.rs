use super::*;
impl Reader<'_> {
    pub(super) fn grid(&mut self, with_slots: bool) -> Result<GridLayout> {
        let dimensions = self.dimensions()?;
        self.cursor.take(12)?;
        let groups = (0..3)
            .map(|axis| self.group(dimensions[(axis + 1) % 3] as usize, with_slots))
            .collect::<Result<Vec<_>>>()?;
        Ok(GridLayout { dimensions, groups })
    }
    fn dimensions(&mut self) -> Result<[i32; 3]> {
        let dimensions = [self.cursor.i32()?, self.cursor.i32()?, self.cursor.i32()?];
        for dimension in dimensions {
            if dimension < 0 {
                return Err(invalid(
                    "grid dimension",
                    format!("dimension {dimension}; expected a nonnegative signed dimension"),
                ));
            }
            self.admit(Counter::Rows, dimension as usize)?;
        }
        Ok(dimensions)
    }
    fn group(&mut self, count: usize, with_slots: bool) -> Result<GroupLayout> {
        let start = self.position();
        let flags = self.cursor.take(count)?;
        let presence = PatternSpan {
            start,
            count,
            shape: ValueShape::Byte,
        };
        let rows = flags
            .iter()
            .map(|flag| self.present_row(*flag, with_slots))
            .collect::<Result<Vec<_>>>()?;
        Ok(GroupLayout { presence, rows })
    }
    fn present_row(&mut self, flag: u8, with_slots: bool) -> Result<Option<RowLayout>> {
        if flag == 0 {
            return Ok(None);
        }
        self.counts.present_rows += 1;
        self.row(with_slots).map(Some)
    }
    fn row(&mut self, with_slots: bool) -> Result<RowLayout> {
        let start = self.position();
        let count = self
            .cursor
            .nonnegative_count(if with_slots { 5 } else { 4 }, "checkpoint row")?;
        let indices = self.patterns(count, ValueShape::Pattern32)?;
        let slots = if with_slots {
            Some(self.slots(count)?)
        } else {
            None
        };
        Ok(RowLayout {
            start,
            end: self.position(),
            indices,
            slots,
        })
    }
    fn slots(&mut self, count: usize) -> Result<Vec<SlotLayout>> {
        self.admit(Counter::Slots, count)?;
        (0..count).map(|_| self.slot()).collect()
    }
    fn slot(&mut self) -> Result<SlotLayout> {
        let start = self.position();
        let tag = self.cursor.u8()?;
        self.counts.noncanonical_tags += usize::from(tag > 2);
        let container = if tag > 1 {
            Some(self.container()?)
        } else {
            None
        };
        Ok(SlotLayout { start, container })
    }
}
