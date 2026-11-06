use super::*;
use FlowPrimaryEncoding::{Signed32, WideBits, WordBits};
use FlowPrimaryFieldKind as Kind;
use FlowPrimaryScope as Scope;
use decode::PrimaryReader;

#[derive(Clone, Copy)]
pub(super) enum PrimaryFactory {
    Word,
    Pair,
    Wide,
    Mixed,
}

impl PrimaryFactory {
    fn select(value: u8, cached: &mut Option<Self>) -> Result<Self> {
        if let Some(selected) = *cached {
            return Ok(selected);
        }
        let selected = Self::parse(value)?;
        *cached = Some(selected);
        Ok(selected)
    }

    fn parse(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Word),
            2 => Ok(Self::Pair),
            3 => Ok(Self::Wide),
            4 => Ok(Self::Mixed),
            _ => Err(table_failure(
                439,
                FailureClass::Unsupported,
                format!("populated element kind {value}; expected factory one through four"),
            )),
        }
    }
    fn width(self) -> usize {
        match self {
            Self::Word => 4,
            Self::Pair => 8,
            Self::Wide => 12,
            Self::Mixed => 16,
        }
    }
}

impl PrimaryReader<'_> {
    pub(super) fn group(
        &mut self,
        item: usize,
        group: usize,
        kind: u8,
        tag: u8,
        factory: &mut Option<PrimaryFactory>,
    ) -> Result<()> {
        let entity = Scope::Group(item, group);
        let scope = self.begin(entity)?;
        if self.version == 3 {
            self.byte(entity, Kind::Mode)?;
        }
        self.words(entity, Kind::Members, Signed32, 2)?;
        if self.byte(entity, Kind::Presence)? != 0 {
            self.body(item, group)?;
        }
        if matches!(kind, 2 | 3) {
            self.elements(item, group, tag, factory)?;
        }
        self.end(scope);
        Ok(())
    }

    fn body(&mut self, item: usize, group: usize) -> Result<()> {
        let entity = Scope::Body(item, group);
        let scope = self.begin(entity)?;
        self.vector(entity, Kind::FirstCount, Kind::FirstWords)?;
        self.vector(entity, Kind::SecondCount, Kind::SecondWords)?;
        self.words(entity, Kind::Triplet, Signed32, 3)?;
        self.words(entity, Kind::WideSix, WideBits, 6)?;
        self.words(entity, Kind::WideThree, WideBits, 3)?;
        if self.version == 0 {
            self.words(entity, Kind::LegacyWideNine, WideBits, 9)?;
        }
        if self.version >= 2 {
            self.byte(entity, Kind::StoredFlag)?;
        }
        self.end(scope);
        Ok(())
    }

    fn vector(&mut self, scope: Scope, count: Kind, words: Kind) -> Result<()> {
        let count = self.count(scope, count)?;
        self.words(scope, words, WordBits, count)?;
        Ok(())
    }

    fn elements(
        &mut self,
        item: usize,
        group: usize,
        tag: u8,
        factory: &mut Option<PrimaryFactory>,
    ) -> Result<()> {
        let entity = Scope::Group(item, group);
        let count = self.collection(
            entity,
            Kind::ElementCount,
            1,
            self.options.maximum_elements,
            8,
        )?;
        if count == 0 {
            return Ok(());
        }
        let selected = PrimaryFactory::select(tag, factory)?;
        for element in 0..count {
            self.element(item, group, element, selected)?;
        }
        Ok(())
    }

    fn element(
        &mut self,
        item: usize,
        group: usize,
        element: usize,
        factory: PrimaryFactory,
    ) -> Result<()> {
        let entity = Scope::Element(item, group, element);
        let scope = self.begin(entity)?;
        let count = self.collection(entity, Kind::RowCount, 2, self.options.maximum_rows, 4)?;
        for row in 0..count {
            self.row(Scope::Row(item, group, element, row), factory)?;
        }
        let blocks = self.collection(
            entity,
            Kind::WideBlockCount,
            3,
            self.options.maximum_wide_blocks,
            128,
        )?;
        for block in 0..blocks {
            self.block(Scope::WideBlock(item, group, element, block))?;
        }
        self.end(scope);
        Ok(())
    }

    fn block(&mut self, entity: Scope) -> Result<()> {
        let scope = self.begin(entity)?;
        self.words(entity, Kind::WideBlock, WideBits, 16)?;
        self.end(scope);
        Ok(())
    }

    fn row(&mut self, entity: Scope, factory: PrimaryFactory) -> Result<()> {
        let scope = self.begin(entity)?;
        let count = self.count(entity, Kind::EntryCount)?;
        let width = factory.width();
        self.minimum(count, width)?;
        let start = self.position();
        self.column(entity, Kind::RowFirst, WordBits, (start, count, width))?;
        self.row_tail(entity, factory, (start, count, width))?;
        self.cursor.take(count * width)?;
        self.end(scope);
        Ok(())
    }

    fn row_tail(
        &mut self,
        entity: Scope,
        factory: PrimaryFactory,
        layout: (usize, usize, usize),
    ) -> Result<()> {
        let (start, count, stride) = layout;
        let next = start + if count == 0 { 0 } else { 4 };
        match factory {
            PrimaryFactory::Word => Ok(()),
            PrimaryFactory::Pair => {
                self.column(entity, Kind::RowSecond, WordBits, (next, count, stride))
            }
            PrimaryFactory::Wide => {
                self.column(entity, Kind::RowWide, WideBits, (next, count, stride))
            }
            PrimaryFactory::Mixed => {
                self.column(entity, Kind::RowWide, WideBits, (next, count, stride))?;
                let last = start + if count == 0 { 0 } else { 12 };
                self.column(entity, Kind::RowLast, WordBits, (last, count, stride))
            }
        }
    }

    fn column(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowPrimaryEncoding,
        layout: (usize, usize, usize),
    ) -> Result<()> {
        self.field(
            scope,
            kind,
            encoding,
            FlowPrimaryFieldLayout {
                first_byte: layout.0,
                element_count: layout.1,
                stride_bytes: layout.2,
            },
        )
    }
}
