use super::*;
use FlowConditionalEncoding::{Byte, WideBits, WordBits};
use FlowConditionalFieldKind as Kind;
use FlowConditionalScopeKind as Scope;
use decode::ConditionalReader;

impl ConditionalReader<'_> {
    pub(super) fn header(&mut self, root: usize, ordinal: usize) -> Result<()> {
        let scope = self.begin(Scope::HeaderGroup, Some(root), ordinal)?;
        let version = self.signed(scope, Kind::Version)?;
        self.require(
            version == 0,
            format!("group version {version}; expected zero"),
        )?;
        self.words(scope, Kind::MemberA0Pattern, WordBits, 1)?;
        let mode = self.mode(scope)?;
        let protocol = self.signed(scope, Kind::Protocol)?;
        self.require(protocol == 1, format!("protocol {protocol}; expected one"))?;
        let gate = self.unsigned_gate(scope, mode)?;
        if gate {
            self.entries(scope)?;
        }
        self.end(scope);
        Ok(())
    }

    fn unsigned_gate(&mut self, scope: usize, mode: i32) -> Result<bool> {
        if mode != 0 {
            return Ok(true);
        }
        let start = self.words(scope, Kind::GatePattern, WordBits, 1)?;
        Ok(wire::unsigned_at(self.raw, start) != 0)
    }

    pub(super) fn entries(&mut self, parent: usize) -> Result<()> {
        let count = self.count(parent, Kind::Count)?;
        self.minimum(count, 1)?;
        self.spend(0, count, self.options.maximum_entries, "presence entries")?;
        for ordinal in 0..count {
            let scope = self.begin(Scope::Entry, Some(parent), ordinal)?;
            if self.byte(scope, Kind::Presence)? != 0 {
                self.item(scope)?;
            }
            self.end(scope);
        }
        Ok(())
    }

    fn item(&mut self, parent: usize) -> Result<()> {
        self.spend(1, 1, self.options.maximum_items, "present items")?;
        let scope = self.begin(Scope::Item, Some(parent), 0)?;
        let version = self.signed(scope, Kind::Version)?;
        self.require(
            matches!(version, 0..=5),
            format!("item version {version}; expected zero through five"),
        )?;
        if self.byte(scope, Kind::Active)? != 0 {
            self.active_item(scope, version as u8)?;
        }
        self.end(scope);
        Ok(())
    }

    fn active_item(&mut self, scope: usize, version: u8) -> Result<()> {
        if version < 4 {
            self.item_groups(scope, 2, false)?;
        }
        self.dimension(scope, self.expected_dimension)?;
        if version < 3 {
            self.signed(scope, Kind::LegacyWord)?;
        }
        self.signed(scope, Kind::Member84)?;
        if version != 0 {
            self.item_mode(scope, version)?;
        }
        if version >= 4 {
            let count = self.count(scope, Kind::GroupCount)?;
            self.minimum(count, 24)?;
            self.item_groups(scope, count, true)?;
        }
        Ok(())
    }

    fn item_mode(&mut self, scope: usize, version: u8) -> Result<()> {
        let mode = self.byte(scope, Kind::ItemMode)?;
        if version == 1 {
            self.legacy_vectors(scope, mode)?;
        }
        if matches!(version, 2 | 3) {
            self.spend(3, 2, self.options.maximum_subrecords, "subrecords")?;
            for index in 0..2 {
                self.subrecord(scope, index)?;
            }
        }
        Ok(())
    }

    fn legacy_vectors(&mut self, scope: usize, mode: u8) -> Result<()> {
        self.require(
            mode == 0,
            format!("version-one item mode {mode}; expected zero"),
        )?;
        for index in 0..2 {
            self.vector(
                scope,
                Kind::LegacyVectorCount(index),
                Kind::LegacyVectorValues(index),
                WordBits,
            )?;
        }
        Ok(())
    }

    fn item_groups(&mut self, parent: usize, count: usize, modern: bool) -> Result<()> {
        self.spend(2, count, self.options.maximum_groups, "groups")?;
        if modern {
            self.spend(3, count, self.options.maximum_subrecords, "subrecords")?;
        }
        for ordinal in 0..count {
            let scope = self.begin(Scope::ItemGroup, Some(parent), ordinal)?;
            self.vector(scope, Kind::WideCount, Kind::WideValues, WideBits)?;
            self.vector(scope, Kind::BytesCount, Kind::Bytes, Byte)?;
            if modern {
                self.subrecord(scope, 0)?;
            }
            self.end(scope);
        }
        Ok(())
    }

    fn subrecord(&mut self, parent: usize, ordinal: usize) -> Result<()> {
        let scope = self.begin(Scope::Subrecord, Some(parent), ordinal)?;
        let kind = self.signed(scope, Kind::SubKind)?;
        self.signed(scope, Kind::MemberC)?;
        self.signed(scope, Kind::Member10)?;
        self.vector(scope, Kind::FirstCount, Kind::FirstValues, WordBits)?;
        if kind == 2 {
            self.vector(scope, Kind::SubExtraCount, Kind::SubExtraValues, WordBits)?;
        }
        self.end(scope);
        Ok(())
    }
}
