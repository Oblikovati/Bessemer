use super::*;
use FlowPrimaryEncoding::{Byte, Signed32};
use FlowPrimaryFieldKind as Kind;
use FlowPrimaryScope as Scope;

pub(super) struct PrimaryReader<'a> {
    pub(super) cursor: ArchiveCursor<'a>,
    pub(super) raw: &'a [u8],
    pub(super) version: i32,
    pub(super) options: FlowPrimaryTableOptions,
    pub(super) scopes: Vec<PrimaryScopeSpan>,
    pub(super) fields: Vec<PrimaryFieldSpan>,
    pub(super) totals: [usize; 5],
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowPrimaryTableOptions,
) -> Result<FlowPrimaryTable> {
    validate_options(options)?;
    let wire = wire::read(
        source,
        id,
        439,
        FlowLegacyTableTransport::Raw,
        options.maximum_decoded_bytes,
    )?;
    let mut reader = PrimaryReader::new(&wire, options)?;
    reader.record()?;
    Ok(FlowPrimaryTable {
        scopes: reader.scopes.into(),
        fields: reader.fields.into(),
        totals: reader.totals,
        wire,
        options,
    })
}

fn validate_options(options: FlowPrimaryTableOptions) -> Result<()> {
    if options.maximum_decoded_bytes >= 8 {
        return Ok(());
    }
    Err(primary_invalid(format!(
        "decoded byte limit {}; expected at least eight raw bytes",
        options.maximum_decoded_bytes
    )))
}

impl<'a> PrimaryReader<'a> {
    fn new(wire: &'a TableWire, options: FlowPrimaryTableOptions) -> Result<Self> {
        if !(0..=3).contains(&wire.version) {
            return Err(table_failure(
                439,
                FailureClass::Unsupported,
                format!(
                    "version {}; expected primary-class version zero through three",
                    wire.version
                ),
            ));
        }
        Ok(Self {
            cursor: ArchiveCursor::new(&wire.decoded),
            raw: &wire.decoded,
            version: wire.version,
            options,
            scopes: Vec::new(),
            fields: Vec::new(),
            totals: [0; 5],
        })
    }

    pub(super) fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }

    fn record(&mut self) -> Result<()> {
        let scope = self.begin(Scope::Record)?;
        self.words(Scope::Record, Kind::Version, Signed32, 1)?;
        let count = self.collection(
            Scope::Record,
            Kind::ItemCount,
            0,
            self.options.maximum_items,
            20,
        )?;
        for item in 0..count {
            self.item(item)?;
        }
        self.cursor.finish()?;
        self.end(scope);
        Ok(())
    }

    fn item(&mut self, item: usize) -> Result<()> {
        let entity = Scope::Item(item);
        let scope = self.begin(entity)?;
        let kind = self.byte(entity, Kind::Kind)?;
        let tag = self.byte(entity, Kind::ElementKind)?;
        let mut factory = None;
        for group in 0..2 {
            self.group(item, group, kind, tag, &mut factory)?;
        }
        self.end(scope);
        Ok(())
    }

    pub(super) fn begin(&mut self, scope: Scope) -> Result<usize> {
        bounded(
            self.scopes.len(),
            1,
            self.options.maximum_scopes,
            "scope descriptors",
        )?;
        let ordinal = self.scopes.len();
        self.scopes.push(PrimaryScopeSpan {
            scope,
            interval: (self.position(), self.position()),
        });
        Ok(ordinal)
    }

    pub(super) fn end(&mut self, ordinal: usize) {
        self.scopes[ordinal].interval.1 = self.position();
    }

    pub(super) fn minimum(&self, count: usize, width: usize) -> Result<()> {
        bounded(
            0,
            count,
            self.cursor.remaining() / width,
            "complete physical entries",
        )
    }

    pub(super) fn field(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowPrimaryEncoding,
        layout: FlowPrimaryFieldLayout,
    ) -> Result<()> {
        bounded(
            self.fields.len(),
            1,
            self.options.maximum_fields,
            "field descriptors",
        )?;
        bounded(
            self.totals[4],
            layout.element_count,
            self.options.maximum_values,
            "stored scalar values",
        )?;
        self.fields.push(PrimaryFieldSpan {
            scope,
            kind,
            encoding,
            layout,
        });
        self.totals[4] += layout.element_count;
        Ok(())
    }

    pub(super) fn words(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowPrimaryEncoding,
        count: usize,
    ) -> Result<usize> {
        let width = encoding.width();
        self.minimum(count, width)?;
        let start = self.position();
        self.field(
            scope,
            kind,
            encoding,
            FlowPrimaryFieldLayout {
                first_byte: start,
                element_count: count,
                stride_bytes: width,
            },
        )?;
        self.cursor.take(count * width)?;
        Ok(start)
    }

    pub(super) fn byte(&mut self, scope: Scope, kind: Kind) -> Result<u8> {
        let start = self.words(scope, kind, Byte, 1)?;
        Ok(self.raw[start])
    }

    pub(super) fn count(&mut self, scope: Scope, kind: Kind) -> Result<usize> {
        let start = self.words(scope, kind, Signed32, 1)?;
        let count = wire::signed_at(self.raw, start);
        if count >= 0 {
            return Ok(count as usize);
        }
        Err(primary_invalid(format!(
            "{scope:?} {kind:?} count {count}; expected nonnegative signed-i32 count"
        )))
    }

    pub(super) fn collection(
        &mut self,
        scope: Scope,
        kind: Kind,
        axis: usize,
        limit: usize,
        width: usize,
    ) -> Result<usize> {
        let count = self.count(scope, kind)?;
        self.minimum(count, width)?;
        bounded(self.totals[axis], count, limit, &format!("{kind:?}"))?;
        self.totals[axis] += count;
        Ok(count)
    }
}

fn bounded(used: usize, additional: usize, limit: usize, label: &str) -> Result<()> {
    if additional <= limit.saturating_sub(used) && used <= limit {
        return Ok(());
    }
    Err(primary_invalid(format!(
        "{label} additional {additional}, used {used}; expected total at most {limit}"
    )))
}
