use super::*;

impl LinkReader<'_> {
    pub(super) fn tail(&mut self) -> Result<LinkTailSpan> {
        let start = self.position();
        let members = self.cursor.take(44)?;
        let conditional_count = wire::signed_at(members, 24).max(0) as usize;
        let discarded = self.discarded()?;
        let control = self.control()?;
        let conditional = self.conditional(control, conditional_count)?;
        let numeric = self.numeric()?;
        let final_tail = self.final_tail()?;
        Ok(LinkTailSpan {
            start,
            discarded,
            control: control.map(|(offset, _)| offset),
            conditional,
            numeric,
            final_tail,
        })
    }

    fn discarded(&mut self) -> Result<(usize, usize)> {
        let start = self.position();
        self.cursor.take(match self.version {
            2 => 4,
            3 => 12,
            _ => 0,
        })?;
        Ok((start, self.position()))
    }

    fn control(&mut self) -> Result<Option<(usize, i32)>> {
        if self.version < 5 {
            return Ok(None);
        }
        let offset = self.position();
        Ok(Some((offset, self.cursor.i32()?)))
    }

    fn conditional(
        &mut self,
        control: Option<(usize, i32)>,
        count: usize,
    ) -> Result<(usize, usize)> {
        if control.is_some_and(|(_, value)| value > 0) {
            return self.array(count, 4);
        }
        Ok((self.position(), self.position()))
    }

    fn numeric(&mut self) -> Result<Option<NumericSpan>> {
        if self.version < 6 {
            return Ok(None);
        }
        let start = self.position();
        let count = self.count("numeric tail")?;
        self.array(count, 8)?;
        let indices = if self.version >= 8 {
            Some(self.array(count, 4)?.0)
        } else {
            None
        };
        Ok(Some(NumericSpan {
            start,
            end: self.position(),
            count,
            indices,
        }))
    }

    fn final_tail(&mut self) -> Result<Option<FinalSpan>> {
        if self.version < 7 {
            return Ok(None);
        }
        let start = self.position();
        let count = self.count("final tail")?;
        let width = if self.version == 9 { 8 } else { 4 };
        self.array(count, width)?;
        Ok(Some(FinalSpan {
            start,
            end: self.position(),
            count,
            width,
        }))
    }
}
