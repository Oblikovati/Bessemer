use super::*;
use FlowConditionalEncoding::{Byte, Signed32, WideBits};
use FlowConditionalFieldKind as Kind;
use FlowConditionalScopeKind as Scope;

pub(super) struct ConditionalReader<'a> {
    pub(super) cursor: ArchiveCursor<'a>,
    pub(super) raw: &'a [u8],
    pub(super) identifier: u32,
    pub(super) expected_dimension: i32,
    pub(super) options: FlowConditionalItemOptions,
    pub(super) fields: Vec<ConditionalFieldSpan>,
    pub(super) scopes: Vec<ConditionalScopeSpan>,
    pub(super) totals: [usize; 6],
    pub(super) scale: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    format: FlowConditionalFormat,
    options: FlowConditionalItemOptions,
) -> Result<FlowConditionalItems> {
    let wire = transport::read(source, id, format, options)?;
    let mut reader = ConditionalReader::new(&wire, format, options);
    reader.record(format, wire.version)?;
    Ok(FlowConditionalItems {
        fields: reader.fields.into(),
        scopes: reader.scopes.into(),
        totals: reader.totals,
        scale: reader.scale,
        wire,
        format,
        options,
    })
}

impl<'a> ConditionalReader<'a> {
    fn new(
        wire: &'a TableWire,
        format: FlowConditionalFormat,
        options: FlowConditionalItemOptions,
    ) -> Self {
        let scale = if format == FlowConditionalFormat::Record128 {
            if wire.version == 6 { 9 } else { 6 }
        } else {
            1
        };
        Self {
            cursor: ArchiveCursor::new(&wire.decoded),
            raw: &wire.decoded,
            identifier: format.identifier(),
            expected_dimension: format.dimension(),
            options,
            fields: Vec::new(),
            scopes: Vec::new(),
            totals: [0; 6],
            scale,
        }
    }

    pub(super) fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }

    fn record(&mut self, format: FlowConditionalFormat, version: i32) -> Result<()> {
        let root = self.begin(Scope::Record, None, 0)?;
        match format {
            FlowConditionalFormat::Record126 => self.record126(root, version)?,
            FlowConditionalFormat::Record315 => self.record315(root)?,
            FlowConditionalFormat::Record128 => {
                self.spend(2, 1, self.options.maximum_groups, "groups")?;
                self.header(root, 0)?;
            }
        }
        self.cursor.finish()?;
        self.end(root);
        Ok(())
    }

    fn record126(&mut self, root: usize, version: i32) -> Result<()> {
        self.signed(root, Kind::Version)?;
        if version == 1 {
            return self.parallel(root);
        }
        let inner = self.signed(root, Kind::InnerWord)?;
        self.require(inner < 1, format!("inner word {inner}; expected below one"))?;
        self.signed(root, Kind::MemberA0Signed)?;
        let mode = self.mode(root)?;
        self.dimension(root, 3)?;
        let gate = if mode == 0 {
            self.signed(root, Kind::GateSigned)? != 0
        } else {
            true
        };
        if gate {
            self.entries(root)?;
        }
        Ok(())
    }

    fn parallel(&mut self, root: usize) -> Result<()> {
        let count = self.count(root, Kind::Count)?;
        self.minimum(count, 24)?;
        for index in 0..3 {
            self.array(root, Kind::LegacyArrayValues(index), WideBits, count)?;
        }
        Ok(())
    }

    fn record315(&mut self, root: usize) -> Result<()> {
        self.signed(root, Kind::Version)?;
        for index in 0..2 {
            self.vector(
                root,
                Kind::LegacyArrayCount(index),
                Kind::LegacyArrayValues(index),
                WideBits,
            )?;
        }
        self.vector(root, Kind::CommonCount, Kind::CommonValues, WideBits)?;
        let count = self.count(root, Kind::ExtraCount)?;
        if count != 0 {
            self.signed(root, Kind::ExtraMode)?;
        }
        self.array(root, Kind::ExtraValues, WideBits, count)?;
        self.spend(2, 2, self.options.maximum_groups, "groups")?;
        for index in 0..2 {
            self.header(root, index)?;
        }
        Ok(())
    }

    pub(super) fn begin(
        &mut self,
        kind: Scope,
        parent: Option<usize>,
        ordinal: usize,
    ) -> Result<usize> {
        self.bound(
            self.scopes.len(),
            1,
            self.options.maximum_scopes,
            "scope descriptors",
        )?;
        let index = self.scopes.len();
        self.scopes.push(ConditionalScopeSpan {
            kind,
            parent,
            ordinal,
            interval: (self.position(), self.position()),
        });
        Ok(index)
    }
    pub(super) fn end(&mut self, scope: usize) {
        self.scopes[scope].interval.1 = self.position();
    }

    pub(super) fn minimum(&self, count: usize, width: usize) -> Result<()> {
        self.bound(
            0,
            count,
            self.cursor.remaining() / width,
            "complete physical entries",
        )
    }

    pub(super) fn bound(
        &self,
        used: usize,
        additional: usize,
        limit: usize,
        role: &str,
    ) -> Result<()> {
        if used <= limit && additional <= limit - used {
            return Ok(());
        }
        Err(table_failure(
            self.identifier,
            FailureClass::Invalid,
            format!("{role} additional {additional}, used {used}; expected total at most {limit}"),
        ))
    }

    pub(super) fn spend(
        &mut self,
        axis: usize,
        count: usize,
        limit: usize,
        role: &str,
    ) -> Result<()> {
        self.bound(self.totals[axis], count, limit, role)?;
        self.totals[axis] += count;
        Ok(())
    }

    pub(super) fn require(&self, accepted: bool, cause: String) -> Result<()> {
        if accepted {
            return Ok(());
        }
        Err(table_failure(self.identifier, FailureClass::Invalid, cause))
    }

    pub(super) fn dimension(&mut self, scope: usize, expected: i32) -> Result<()> {
        let dimension = self.signed(scope, Kind::Dimension)?;
        self.require(
            dimension == expected,
            format!("dimension {dimension}; expected {expected}"),
        )
    }

    pub(super) fn words(
        &mut self,
        scope: usize,
        kind: Kind,
        encoding: FlowConditionalEncoding,
        count: usize,
    ) -> Result<usize> {
        let width = encoding.width();
        self.minimum(count, width)?;
        let start = self.position();
        self.field(scope, kind, encoding, start, count)?;
        self.cursor.take(count * width)?;
        Ok(start)
    }

    fn field(
        &mut self,
        scope: usize,
        kind: Kind,
        encoding: FlowConditionalEncoding,
        start: usize,
        count: usize,
    ) -> Result<()> {
        self.bound(
            self.fields.len(),
            1,
            self.options.maximum_fields,
            "field descriptors",
        )?;
        self.spend(5, count, self.options.maximum_values, "stored values")?;
        self.fields.push(ConditionalFieldSpan {
            scope,
            kind,
            encoding,
            layout: FlowConditionalFieldLayout {
                first_byte: start,
                element_count: count,
                stride_bytes: encoding.width(),
            },
        });
        Ok(())
    }

    pub(super) fn array(
        &mut self,
        scope: usize,
        kind: Kind,
        encoding: FlowConditionalEncoding,
        count: usize,
    ) -> Result<()> {
        self.spend(
            4,
            count,
            self.options.maximum_vector_values,
            "vector values",
        )?;
        self.words(scope, kind, encoding, count)?;
        Ok(())
    }

    pub(super) fn vector(
        &mut self,
        scope: usize,
        count_kind: Kind,
        values: Kind,
        encoding: FlowConditionalEncoding,
    ) -> Result<()> {
        let count = self.count(scope, count_kind)?;
        let scale = if encoding == WideBits { self.scale } else { 1 };
        self.minimum(count, scale * encoding.width())?;
        self.array(scope, values, encoding, count * scale)
    }

    pub(super) fn signed(&mut self, scope: usize, kind: Kind) -> Result<i32> {
        let start = self.words(scope, kind, Signed32, 1)?;
        Ok(wire::signed_at(self.raw, start))
    }
    pub(super) fn byte(&mut self, scope: usize, kind: Kind) -> Result<u8> {
        let start = self.words(scope, kind, Byte, 1)?;
        Ok(self.raw[start])
    }
    pub(super) fn count(&mut self, scope: usize, kind: Kind) -> Result<usize> {
        let count = self.signed(scope, kind)?;
        self.require(
            count >= 0,
            format!("{kind:?} count {count}; expected nonnegative signed-i32 count"),
        )?;
        Ok(count as usize)
    }
    pub(super) fn mode(&mut self, scope: usize) -> Result<i32> {
        let mode = self.signed(scope, Kind::Mode)?;
        self.require(
            matches!(mode, 0..=2),
            format!("mode {mode}; expected zero through two"),
        )?;
        Ok(mode)
    }
}
