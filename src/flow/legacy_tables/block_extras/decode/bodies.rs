use super::*;

impl ExtraReader<'_> {
    pub(super) fn body(&mut self, block: usize) -> Result<()> {
        let entity = Entity::Body(block);
        let scope = self.begin(entity)?;
        self.first_list(block)?;
        let count = usize::from(self.byte(entity, Kind::SecondCount)?);
        self.minimum(count, 4)?;
        for index in 0..count {
            self.second(block, index)?;
        }
        let count = usize::from(self.byte(entity, Kind::ThirdCount)?);
        self.minimum(count, 36)?;
        for index in 0..count {
            self.third(block, index)?;
        }
        let fourth = self.fourth(block)?;
        self.body_tail(block, fourth)?;
        self.end(scope);
        Ok(())
    }

    fn first_list(&mut self, block: usize) -> Result<()> {
        let entity = Entity::Body(block);
        let count = usize::from(self.byte(entity, Kind::FirstCount)?);
        self.minimum(count, if self.version < 5 { 35 } else { 43 })?;
        for index in 0..count {
            self.first(block, index)?;
        }
        Ok(())
    }

    fn first(&mut self, block: usize, index: usize) -> Result<()> {
        let entity = Entity::First { block, index };
        let scope = self.begin(entity)?;
        self.words(entity, Kind::PairBytes, Byte, 2)?;
        self.words(entity, Kind::FirstWords, Signed32, 5)?;
        self.mixed(Entity::FirstMixed { block, index })?;
        let encoding = if self.version < 5 { Signed32 } else { WideBits };
        self.words(entity, Kind::ReferenceWords, encoding, 2)?;
        let count = self.bytes(entity)?;
        if self.version >= 7 && count > 0 {
            self.additional(entity, count)?;
        }
        self.end(scope);
        Ok(())
    }

    fn additional(&mut self, entity: Entity, count: usize) -> Result<()> {
        let flag = self.byte(entity, Kind::AdditionalFlag)?;
        if flag != 0 {
            self.words(entity, Kind::AdditionalWords, Signed32, 3 * count)?;
        }
        Ok(())
    }

    fn second(&mut self, block: usize, index: usize) -> Result<()> {
        let entity = Entity::Second { block, index };
        let scope = self.begin(entity)?;
        self.words(entity, Kind::PairBytes, Byte, 2)?;
        self.byte(entity, Kind::TagByte)?;
        self.bytes(entity)?;
        self.end(scope);
        Ok(())
    }

    fn third(&mut self, block: usize, index: usize) -> Result<()> {
        let entity = Entity::Third { block, index };
        let scope = self.begin(entity)?;
        self.signed(entity, Kind::ThirdWord)?;
        self.words(entity, Kind::ThirdWide, WideBits, 4)?;
        self.end(scope);
        Ok(())
    }

    fn fourth(&mut self, block: usize) -> Result<usize> {
        let entity = Entity::Fourth(block);
        let scope = self.begin(entity)?;
        let count = usize::from(self.byte(entity, Kind::FourthCount)?);
        self.words(entity, Kind::FourthWide, WideBits, 3 * count)?;
        self.words(entity, Kind::FourthBytes, Byte, count)?;
        self.end(scope);
        Ok(count)
    }

    fn body_tail(&mut self, block: usize, fourth: usize) -> Result<()> {
        let entity = Entity::Body(block);
        let outer = self.byte(entity, Kind::OuterFlag)?;
        if outer == 1 {
            if self.byte(entity, Kind::NestedFlag)? == 1 {
                self.nested_rows(block, fourth)?;
            }
            if self.byte(entity, Kind::FinalFlag)? == 1 {
                self.final_tail(block)?;
            }
        }
        if self.version >= 3 && self.byte(entity, Kind::ExtraFlag)? != 0 {
            self.extra_tail(block)?;
        }
        if self.version >= 4 {
            self.byte(entity, Kind::LastByte)?;
        }
        Ok(())
    }

    fn final_tail(&mut self, block: usize) -> Result<()> {
        let entity = Entity::Final(block);
        let scope = self.begin(entity)?;
        self.signed(entity, Kind::FinalWord)?;
        let count = self.count(entity, Kind::FinalCount)?;
        self.words(entity, Kind::FinalWords, Signed32, count)?;
        self.end(scope);
        Ok(())
    }

    fn extra_tail(&mut self, block: usize) -> Result<()> {
        let entity = Entity::Extra(block);
        let scope = self.begin(entity)?;
        self.signed(entity, Kind::ExtraWord)?;
        self.mixed(Entity::ExtraMixed(block))?;
        self.end(scope);
        Ok(())
    }
}
