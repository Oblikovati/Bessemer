use super::*;

struct FieldValueReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw: &'a [u8],
    maximum_wide: usize,
    wide_words: usize,
    spans: Vec<FieldValueSpan>,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowFieldValueOptions,
) -> Result<FlowFieldValues> {
    validate_options(&options)?;
    let wire = versioned_wire(source, id, &options)?;
    let mut reader = FieldValueReader::new(&wire.decoded, options.maximum_wide_words);
    let table_words = reader.layout(wire.version, &options)?;
    reader.cursor.finish()?;
    Ok(FlowFieldValues {
        spans: reader.spans.into(),
        wide_words: reader.wide_words,
        table_words,
        wire,
        options,
    })
}

fn validate_options(options: &FlowFieldValueOptions) -> Result<()> {
    for (name, limit) in [
        ("items", options.maximum_items),
        ("table words", options.maximum_table_words),
        ("wide words", options.maximum_wide_words),
    ] {
        if limit == 0 {
            return Err(field_invalid(format!(
                "{name} limit {limit}; expected a positive bound"
            )));
        }
    }
    Ok(())
}

fn versioned_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: &FlowFieldValueOptions,
) -> Result<TableWire> {
    wire::versioned(
        source,
        id,
        108,
        options.body_compression,
        "Flow field values",
        |body, version| {
            version_gate(version, options.compatibility_restriction)?;
            expanded_body(body, options)
        },
    )
}

fn version_gate(version: i32, restriction: bool) -> Result<()> {
    if version < 5 || (restriction && version > 7) {
        return Err(table_failure(
            108,
            FailureClass::Unsupported,
            format!(
                "version {version}, compatibility restriction {restriction}; expected version at least five and at most seven when restriction is enabled"
            ),
        ));
    }
    Ok(())
}

fn expanded_body(stored: &[u8], options: &FlowFieldValueOptions) -> Result<Vec<u8>> {
    if options.body_compression == FlowLegacyTableTransport::Zlib {
        return super::super::super::payload_codec::inflate_zlib(
            stored,
            options.maximum_decoded_body_bytes,
            "Flow field values",
        );
    }
    if stored.len() <= options.maximum_decoded_body_bytes {
        return Ok(stored.to_vec());
    }
    Err(field_invalid(format!(
        "raw body {} bytes; expected at most {} decoded body bytes excluding the four-byte version",
        stored.len(),
        options.maximum_decoded_body_bytes
    )))
}

impl<'a> FieldValueReader<'a> {
    fn new(raw: &'a [u8], maximum_wide: usize) -> Self {
        Self {
            cursor: ArchiveCursor::new(raw),
            raw,
            maximum_wide,
            wide_words: 0,
            spans: Vec::new(),
        }
    }
    fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }
    fn layout(&mut self, version: i32, options: &FlowFieldValueOptions) -> Result<usize> {
        match (&options.layout, version) {
            (FlowFieldLayout::Legacy(layout), 5 | 6) => self.legacy(layout, options),
            (FlowFieldLayout::Regions(layout), 7..) => {
                self.regions(layout, options.maximum_items)?;
                Ok(0)
            }
            _ => Err(field_invalid(format!(
                "layout {} for version {version}; expected legacy layout in five/six or region layout from seven",
                layout_name(&options.layout)
            ))),
        }
    }
    fn legacy(
        &mut self,
        layout: &FlowLegacyFieldLayout,
        options: &FlowFieldValueOptions,
    ) -> Result<usize> {
        maximum(
            layout.object_widths().len(),
            options.maximum_items,
            "global-object positions",
        )?;
        let table_words = self.legacy_table(layout, options.maximum_table_words)?;
        for width in layout.object_widths() {
            let start = self.position();
            let wide = self.wide(u64::from(width.value()))?;
            self.spans.push(FieldValueSpan {
                full: (start, self.position()),
                wide,
            });
        }
        Ok(table_words)
    }
    fn legacy_table(&mut self, layout: &FlowLegacyFieldLayout, limit: usize) -> Result<usize> {
        let table_words = layout.width_table_count().value() as usize;
        maximum(
            table_words,
            limit.min(self.cursor.remaining() / 4),
            "width-table words",
        )?;
        self.cursor.take(table_words * 4)?;
        check_table(self.raw, layout)?;
        Ok(table_words)
    }
    fn regions(&mut self, layout: &FlowRegionFieldLayout, limit: usize) -> Result<()> {
        let regions = layout.regions();
        maximum(
            regions.len(),
            limit.min(self.cursor.remaining().saturating_sub(4) / 4),
            "region positions",
        )?;
        let count = self.cursor.i32()?;
        if count != regions.len() as i32 {
            return Err(field_invalid(format!(
                "stored region count {count}; expected supplied count {}",
                regions.len()
            )));
        }
        for region in regions {
            self.region(*region)?;
        }
        Ok(())
    }
    fn region(&mut self, region: FlowFieldRegionContext) -> Result<()> {
        let start = self.position();
        let stored = self.cursor.i32()?;
        if stored != region.width.value() as i32 {
            return Err(field_invalid(format!(
                "stored region width {stored}; expected supplied width {}",
                region.width.value()
            )));
        }
        let count = if region.active {
            u64::from(region.row_count.value()) * u64::from(region.width.value())
        } else {
            0
        };
        let wide = self.wide(count)?;
        self.spans.push(FieldValueSpan {
            full: (start, self.position()),
            wide,
        });
        Ok(())
    }
    fn wide(&mut self, count: u64) -> Result<(usize, usize)> {
        let available = (self.maximum_wide - self.wide_words).min(self.cursor.remaining() / 8);
        if count > available as u64 {
            return Err(field_invalid(format!(
                "wide pattern count {count}, used {}; expected at most {available} additional patterns from body bytes and configured bound",
                self.wide_words
            )));
        }
        let start = self.position();
        self.cursor.take(count as usize * 8)?;
        self.wide_words += count as usize;
        Ok((start, self.position()))
    }
}

fn layout_name(layout: &FlowFieldLayout) -> &'static str {
    match layout {
        FlowFieldLayout::Legacy(_) => "legacy",
        FlowFieldLayout::Regions(_) => "regions",
    }
}

fn maximum(count: usize, limit: usize, label: &str) -> Result<()> {
    if count <= limit {
        return Ok(());
    }
    Err(field_invalid(format!(
        "{label} count {count}; expected at most {limit}"
    )))
}

fn check_table(raw: &[u8], layout: &FlowLegacyFieldLayout) -> Result<()> {
    let Some(expected) = layout.expected_width_table() else {
        return Ok(());
    };
    for (index, expected) in expected.iter().enumerate() {
        if let Some(expected) = expected {
            let stored = wire::signed_at(raw, index * 4);
            if stored != expected.value() as i32 {
                return Err(field_invalid(format!(
                    "width-table word {index} value {stored}; expected supplied width {}",
                    expected.value()
                )));
            }
        }
    }
    Ok(())
}
