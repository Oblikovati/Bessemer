mod tails;
use super::*;

struct LinkReader<'a> {
    cursor: ArchiveCursor<'a>,
    length: usize,
    version: i32,
    options: FlowLinkRecordOptions,
    elements: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLinkRecordOptions,
) -> Result<FlowLinkRecords> {
    validate_options(options)?;
    let wire = link_wire(source, id, options)?;
    let mut reader = LinkReader::new(&wire, options)?;
    let entries = reader.entries()?;
    let variable_integers = reader.elements;
    let tail = reader.tail()?;
    reader.cursor.finish()?;
    let array_elements = reader.elements;
    Ok(FlowLinkRecords {
        wire,
        options,
        entries: entries.into(),
        tail,
        array_elements,
        variable_integers,
    })
}

impl<'a> LinkReader<'a> {
    fn new(wire: &'a TableWire, options: FlowLinkRecordOptions) -> Result<Self> {
        if !(0..=9).contains(&wire.version) {
            return Err(table_failure(
                402,
                FailureClass::Unsupported,
                format!(
                    "version {}; expected legacy link versions zero through nine",
                    wire.version
                ),
            ));
        }
        let mut cursor = ArchiveCursor::new(&wire.decoded);
        cursor.take(4)?;
        Ok(Self {
            cursor,
            length: wire.decoded.len(),
            version: wire.version,
            options,
            elements: 0,
        })
    }

    fn position(&self) -> usize {
        self.length - self.cursor.remaining()
    }

    fn count(&mut self, field: &str) -> Result<usize> {
        let value = self.cursor.i32()?;
        if value >= 0 {
            return Ok(value as usize);
        }
        Err(link_invalid(format!(
            "{field} count {value}; expected nonnegative signed-i32 count"
        )))
    }

    fn entries(&mut self) -> Result<Vec<EntrySpan>> {
        let count = self.count("entry")?;
        if count > self.options.maximum_entries
            || count > self.cursor.remaining().saturating_sub(44) / 66
        {
            return Err(link_invalid(format!(
                "entry count {count}; expected <= {} entries and complete 66-byte entries plus 44-byte tail within {} bytes",
                self.options.maximum_entries,
                self.cursor.remaining()
            )));
        }
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push(self.entry()?);
        }
        Ok(entries)
    }

    fn entry(&mut self) -> Result<EntrySpan> {
        let start = self.position();
        let references = [self.cursor.i32()?, self.cursor.i32()?];
        check_references(references, self.options.global_pointer_count)?;
        self.cursor.take(57)?;
        let variable_count = usize::from(self.cursor.u8()?);
        self.array(variable_count, 4)?;
        Ok(EntrySpan {
            start,
            end: self.position(),
            variable_count,
        })
    }

    fn array(&mut self, count: usize, width: usize) -> Result<(usize, usize)> {
        let available = self.options.maximum_array_elements - self.elements;
        if count > available || count > self.cursor.remaining() / width {
            return Err(link_invalid(format!(
                "array count {count}, width {width}; expected <= {available} additional elements and <= {} complete elements in remaining bytes",
                self.cursor.remaining() / width
            )));
        }
        let start = self.position();
        self.cursor.take(count * width)?;
        self.elements += count;
        Ok((start, self.position()))
    }
}

fn check_references(references: [i32; 2], context: Option<FlowGlobalPointerCount>) -> Result<()> {
    let Some(context) = context else {
        return Ok(());
    };
    for index in references {
        if index != -1 && (index < 0 || index as u32 >= context.value()) {
            return Err(link_invalid(format!(
                "reference index {index}; expected -1 sentinel or zero through {} exclusive",
                context.value()
            )));
        }
    }
    Ok(())
}

fn validate_options(options: FlowLinkRecordOptions) -> Result<()> {
    if options.maximum_decoded_bytes < 52
        || options.maximum_entries == 0
        || options.maximum_array_elements == 0
    {
        return Err(link_invalid(format!(
            "limits {options:?}; expected decoded bytes >= 52, positive entry and array-element limits"
        )));
    }
    Ok(())
}

fn link_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLinkRecordOptions,
) -> Result<TableWire> {
    wire::read(
        source,
        id,
        402,
        options.compression,
        options.maximum_decoded_bytes,
    )
}
