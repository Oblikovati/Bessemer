mod entries;
use super::*;
use FlowPointerBodyKind as Kind;

struct PointerBodyReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    version: i32,
    options: &'a FlowPointerBodyOptions,
    pools: Vec<BodyPoolSpan>,
    entries: Vec<BodyEntrySpan>,
    vectors: Vec<BodySpan>,
    groups: [BodyGroupSpan; 3],
    wide_words: usize,
    raw_bytes: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowPointerBodyOptions,
) -> Result<FlowPointerBodies> {
    validate_options(&options)?;
    let wire = body_wire(source, id, &options)?;
    let mut reader = PointerBodyReader::new(&wire, &options);
    reader.record()?;
    Ok(FlowPointerBodies {
        pools: reader.pools.into(),
        entries: reader.entries.into(),
        vectors: reader.vectors.into(),
        groups: reader.groups,
        wide_words: reader.wide_words,
        raw_bytes: reader.raw_bytes,
        wire,
        options,
    })
}

fn body_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: &FlowPointerBodyOptions,
) -> Result<TableWire> {
    let wire = wire::read(
        source,
        id,
        421,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    validate_context(wire.version, options)?;
    Ok(wire)
}

fn validate_options(options: &FlowPointerBodyOptions) -> Result<()> {
    let limits = [
        ("pools", options.maximum_pools),
        ("entries", options.maximum_entries),
        ("vectors", options.maximum_vectors),
        ("wide words", options.maximum_wide_words),
        ("raw bytes", options.maximum_raw_bytes),
    ];
    for (name, limit) in limits {
        if limit == 0 {
            return Err(body_invalid(format!(
                "{name} limit {limit}; expected a positive bound"
            )));
        }
    }
    Ok(())
}

fn validate_context(version: i32, options: &FlowPointerBodyOptions) -> Result<()> {
    if !(0..=5).contains(&version) {
        return Err(table_failure(
            421,
            FailureClass::Unsupported,
            format!("version {version}; expected legacy pool/body versions zero through five"),
        ));
    }
    let codes = options.entry_codes.as_deref();
    if version >= 3 && codes.is_some() {
        return Err(body_invalid(format!(
            "version {version}, legacy code count {:?}; expected absent runtime code context",
            codes.map(<[u8]>::len)
        )));
    }
    validate_legacy_codes(version, options)
}

fn validate_legacy_codes(version: i32, options: &FlowPointerBodyOptions) -> Result<()> {
    let codes = options.entry_codes.as_deref();
    let count = options.group_counts.values()[2] as usize;
    if version < 3 && count > 0 && codes.map(<[u8]>::len) != Some(count) {
        return Err(body_invalid(format!(
            "runtime code count {:?}; expected exactly {count} codes for the legacy third collection",
            codes.map(<[u8]>::len)
        )));
    }
    Ok(())
}

impl<'a> PointerBodyReader<'a> {
    fn new(wire: &'a TableWire, options: &'a FlowPointerBodyOptions) -> Self {
        let empty = BodySpan { start: 0, end: 0 };
        Self {
            cursor: ArchiveCursor::new(&wire.decoded),
            length: wire.decoded.len(),
            version: wire.version,
            options,
            pools: Vec::new(),
            entries: Vec::new(),
            vectors: Vec::new(),
            groups: [BodyGroupSpan {
                full: empty,
                entries: empty,
            }; 3],
            wide_words: 0,
            raw_bytes: 0,
        }
    }

    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    fn interval(&self, start: usize) -> BodySpan {
        BodySpan {
            start,
            end: self.position(),
        }
    }

    fn record(&mut self) -> Result<()> {
        self.cursor.i32()?;
        let pools = self.options.leaf_count.value() as usize;
        self.minimum(pools as u64, self.options.maximum_pools, "pools")?;
        for _ in 0..pools {
            self.pool()?;
        }
        self.minimum(
            self.options.group_counts.total(),
            self.options.maximum_entries,
            "entries",
        )?;
        for group in 0..3 {
            self.group(group)?;
        }
        self.cursor.finish()
    }

    fn minimum(&self, count: u64, maximum: usize, name: &str) -> Result<()> {
        let limit = maximum.min(self.cursor.remaining());
        if count <= limit as u64 {
            return Ok(());
        }
        Err(body_invalid(format!(
            "{name} count {count}; expected at most {limit} from remaining bytes and configured bound {maximum}"
        )))
    }

    fn pool(&mut self) -> Result<()> {
        let start = self.position();
        let count = self.cursor.u8()? as usize;
        let wide = self.wide(count * 3)?;
        self.pools.push(BodyPoolSpan {
            full: BodySpan {
                start,
                end: self.position(),
            },
            wide,
        });
        Ok(())
    }

    fn group(&mut self, group: usize) -> Result<()> {
        let start = self.position();
        let first = self.entries.len();
        for ordinal in 0..self.options.group_counts.values()[group] as usize {
            self.entry(group, ordinal)?;
        }
        self.groups[group] = BodyGroupSpan {
            full: BodySpan {
                start,
                end: self.position(),
            },
            entries: BodySpan {
                start: first,
                end: self.entries.len(),
            },
        };
        Ok(())
    }

    fn take(&mut self, count: usize, width: usize) -> Result<BodySpan> {
        if count > self.cursor.remaining() / width {
            return Err(body_invalid(format!(
                "{count} elements of width {width}; expected at most {} in remaining bytes",
                self.cursor.remaining() / width
            )));
        }
        let start = self.position();
        self.cursor.take(count * width)?;
        Ok(BodySpan {
            start,
            end: self.position(),
        })
    }

    fn wide(&mut self, count: usize) -> Result<BodySpan> {
        let total = bounded_total(
            self.wide_words,
            count,
            self.options.maximum_wide_words,
            "wide words",
        )?;
        let span = self.take(count, 8)?;
        self.wide_words = total;
        Ok(span)
    }

    fn raw(&mut self, count: usize) -> Result<BodySpan> {
        let total = bounded_total(
            self.raw_bytes,
            count,
            self.options.maximum_raw_bytes,
            "raw bytes",
        )?;
        let span = self.take(count, 1)?;
        self.raw_bytes = total;
        Ok(span)
    }

    fn vector(&mut self) -> Result<BodySpan> {
        bounded_total(
            self.vectors.len(),
            1,
            self.options.maximum_vectors,
            "vectors",
        )?;
        let count = self.cursor.u8()? as usize;
        let span = self.raw(count)?;
        self.vectors.push(span);
        Ok(span)
    }
}

fn bounded_total(current: usize, count: usize, maximum: usize, name: &str) -> Result<usize> {
    current
        .checked_add(count)
        .filter(|total| *total <= maximum)
        .ok_or_else(|| {
            body_invalid(format!(
                "{name} count {current} plus {count}; expected total at most {maximum}"
            ))
        })
}
