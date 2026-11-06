use super::*;
use FlowLoadFieldKind as Kind;
use FlowLoadScope as Scope;
use FlowPrimaryEncoding::{Signed32, WideBits, WordBits};

struct LoadReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw: &'a [u8],
    options: FlowLoadStateOptions,
    fields: Vec<LoadFieldSpan>,
    scopes: Vec<LoadScopeSpan>,
    texts: Vec<LoadTextSpan>,
    totals: [usize; 4],
    width: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLoadStateOptions,
) -> Result<FlowLoadState> {
    let wire = load_wire(source, id, options)?;
    let mut reader = LoadReader::new(&wire.decoded, options);
    reader.record()?;
    Ok(FlowLoadState {
        fields: reader.fields.into(),
        scopes: reader.scopes.into(),
        texts: reader.texts.into(),
        totals: reader.totals,
        width: reader.width,
        wire,
        options,
    })
}

fn load_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLoadStateOptions,
) -> Result<TableWire> {
    if options.maximum_payload_bytes < 16 {
        return Err(load_invalid(format!(
            "payload byte bound {}; expected at least sixteen",
            options.maximum_payload_bytes
        )));
    }
    let wire = wire::read(
        source,
        id,
        11,
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    supported_version(wire.version)?;
    Ok(wire)
}

fn supported_version(version: i32) -> Result<()> {
    if version != 18 {
        return Err(table_failure(
            11,
            FailureClass::Unsupported,
            format!("version {}; expected eighteen", version),
        ));
    }
    Ok(())
}

impl<'a> LoadReader<'a> {
    fn new(raw: &'a [u8], options: FlowLoadStateOptions) -> Self {
        Self {
            cursor: ArchiveCursor::new(raw),
            raw,
            options,
            fields: Vec::new(),
            scopes: Vec::new(),
            texts: Vec::new(),
            totals: [0; 4],
            width: 0,
        }
    }
    fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }

    fn record(&mut self) -> Result<()> {
        let scope = self.begin(Scope::Record)?;
        self.scalar(Scope::Record, Kind::Version, Signed32, 1)?;
        let count = self.count(Scope::Record, Kind::ItemCount)?;
        self.scalar(Scope::Record, Kind::Member130, Signed32, 1)?;
        self.width = self.count(Scope::Record, Kind::ArrayWidth)?;
        self.minimum(count, 88)?;
        self.spend(0, count, self.options.maximum_items, "items")?;
        for item in 0..count {
            self.item(item)?;
        }
        if count != 0 {
            self.tail()?;
        }
        self.cursor.finish()?;
        self.end(scope);
        Ok(())
    }

    fn item(&mut self, item: usize) -> Result<()> {
        let scope = Scope::Item(item);
        let index = self.begin(scope)?;
        for name in 0..2 {
            self.text(Scope::Name(item, name))?;
        }
        self.scalar(scope, Kind::MemberWords, Signed32, 9)?;
        self.scalar(scope, Kind::States, Signed32, 2)?;
        self.scalar(scope, Kind::WideF8, WideBits, 1)?;
        self.scalar(scope, Kind::Member1F8, Signed32, 1)?;
        self.scalar(scope, Kind::WideA0B0, WideBits, 2)?;
        for array in 0..4 {
            self.array(scope, Kind::ItemWideArray(array), WideBits, self.width)?;
        }
        let count = self.count(scope, Kind::ExtraCount)?;
        self.array(scope, Kind::ExtraWords, WordBits, count)?;
        self.text(Scope::Description(item))?;
        self.end(index);
        Ok(())
    }

    fn tail(&mut self) -> Result<()> {
        for array in 0..4 {
            let encoding = if array < 2 { WordBits } else { WideBits };
            self.array(
                Scope::Record,
                Kind::SharedArray(array),
                encoding,
                self.width,
            )?;
        }
        self.scalar(Scope::Record, Kind::FinalMembers, Signed32, 2)?;
        if self.options.mode.value() {
            return Ok(());
        }
        let index = self.begin(Scope::ModeTail)?;
        self.array(Scope::ModeTail, Kind::ModeWords, WordBits, 16)?;
        self.array(Scope::ModeTail, Kind::ModeWide, WideBits, 16)?;
        self.end(index);
        Ok(())
    }

    fn text(&mut self, scope: Scope) -> Result<()> {
        let index = self.begin(scope)?;
        let start = self.position();
        let count = self.count(scope, Kind::TextUnitCount)?;
        self.spend(1, count, self.options.maximum_text_units, "UTF16 units")?;
        let units_field = self.fields.len();
        let units_start = self.field(scope, Kind::TextUnits, FlowLoadEncoding::Utf16Unit, count)?;
        let prefix_units = (0..count)
            .position(|unit| fields::unit_at(self.raw, units_start + unit * 2) == 0)
            .unwrap_or(count);
        self.texts.push(LoadTextSpan {
            scope,
            interval: (start, self.position()),
            units_field,
            prefix_units,
        });
        self.end(index);
        Ok(())
    }

    fn begin(&mut self, scope: Scope) -> Result<usize> {
        self.bound(
            self.scopes.len(),
            1,
            self.options.maximum_scopes,
            "scope descriptors",
        )?;
        let index = self.scopes.len();
        self.scopes.push(LoadScopeSpan {
            scope,
            interval: (self.position(), self.position()),
        });
        Ok(index)
    }
    fn end(&mut self, index: usize) {
        self.scopes[index].interval.1 = self.position();
    }

    fn minimum(&self, count: usize, width: usize) -> Result<()> {
        self.bound(
            0,
            count,
            self.cursor.remaining() / width,
            "complete serialized elements",
        )
    }
    fn bound(&self, used: usize, additional: usize, limit: usize, role: &str) -> Result<()> {
        if used <= limit && additional <= limit - used {
            return Ok(());
        }
        Err(load_invalid(format!(
            "{role} additional {additional}, used {used}; expected total at most {limit}"
        )))
    }
    fn spend(&mut self, axis: usize, count: usize, limit: usize, role: &str) -> Result<()> {
        self.bound(self.totals[axis], count, limit, role)?;
        self.totals[axis] += count;
        Ok(())
    }

    fn field(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowLoadEncoding,
        count: usize,
    ) -> Result<usize> {
        self.minimum(count, encoding.width())?;
        self.bound(
            self.fields.len(),
            1,
            self.options.maximum_fields,
            "field descriptors",
        )?;
        self.spend(3, count, self.options.maximum_values, "stored values")?;
        let start = self.position();
        self.push_field(scope, kind, encoding, start, count);
        self.cursor.take(count * encoding.width())?;
        Ok(start)
    }

    fn push_field(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowLoadEncoding,
        start: usize,
        count: usize,
    ) {
        let layout = FlowLoadFieldLayout {
            first_byte: start,
            element_count: count,
            stride_bytes: encoding.width(),
        };
        self.fields.push(LoadFieldSpan {
            scope,
            kind,
            encoding,
            layout,
        });
    }

    fn scalar(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowPrimaryEncoding,
        count: usize,
    ) -> Result<usize> {
        self.field(scope, kind, FlowLoadEncoding::Scalar(encoding), count)
    }
    fn array(
        &mut self,
        scope: Scope,
        kind: Kind,
        encoding: FlowPrimaryEncoding,
        count: usize,
    ) -> Result<()> {
        self.spend(2, count, self.options.maximum_array_values, "array values")?;
        self.scalar(scope, kind, encoding, count)?;
        Ok(())
    }
    fn count(&mut self, scope: Scope, kind: Kind) -> Result<usize> {
        let start = self.scalar(scope, kind, Signed32, 1)?;
        let count = wire::signed_at(self.raw, start);
        if count >= 0 {
            return Ok(count as usize);
        }
        Err(load_invalid(format!(
            "{kind:?} count {count}; expected nonnegative signed-i32 count"
        )))
    }
}
