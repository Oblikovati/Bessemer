mod bodies;
mod fields;
mod nested;
use super::*;
use FlowBlockExtraEncoding::{Byte, Signed32, WideBits};
use FlowBlockExtraEntity as Entity;
use FlowBlockExtraFieldKind as Kind;

struct ExtraReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw: &'a [u8],
    version: i32,
    options: FlowBlockExtraOptions,
    entities: Vec<ExtraEntitySpan>,
    fields: Vec<ExtraFieldSpan>,
    values: usize,
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowBlockExtraOptions,
) -> Result<FlowBlockExtras> {
    validate_options(options)?;
    let wire = wire::read(
        source,
        id,
        408,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    let mut reader = ExtraReader::new(&wire, options)?;
    reader.record()?;
    Ok(FlowBlockExtras {
        entities: reader.entities.into(),
        fields: reader.fields.into(),
        values: reader.values,
        wire,
        options,
    })
}

impl<'a> ExtraReader<'a> {
    fn new(wire: &'a TableWire, options: FlowBlockExtraOptions) -> Result<Self> {
        if !(0..=8).contains(&wire.version) {
            return Err(table_failure(
                408,
                FailureClass::Unsupported,
                format!(
                    "version {}; expected legacy block-extra versions zero through eight",
                    wire.version
                ),
            ));
        }
        Ok(Self {
            cursor: ArchiveCursor::new(&wire.decoded),
            raw: &wire.decoded,
            version: wire.version,
            options,
            entities: Vec::new(),
            fields: Vec::new(),
            values: 0,
        })
    }

    fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }

    fn record(&mut self) -> Result<()> {
        let root = self.begin(Entity::Record)?;
        self.signed(Entity::Record, Kind::Version)?;
        let count = self.options.leaf_count.value() as usize;
        self.minimum(count, if self.version >= 3 { 5 } else { 4 })?;
        for ordinal in 0..count {
            self.block(ordinal)?;
        }
        self.cursor.finish()?;
        self.end(root);
        Ok(())
    }

    fn block(&mut self, ordinal: usize) -> Result<()> {
        let entity = Entity::Block(ordinal);
        let scope = self.begin(entity)?;
        let token = self.signed(entity, Kind::Token)?;
        let flag = if self.version >= 3 {
            self.byte(entity, Kind::BodyFlag)?
        } else {
            0
        };
        if token == -1 || flag != 0 {
            self.body(ordinal)?;
        }
        self.end(scope);
        Ok(())
    }

    fn begin(&mut self, entity: Entity) -> Result<usize> {
        self.entity_room(1)?;
        let index = self.entities.len();
        self.entities.push(ExtraEntitySpan {
            entity,
            start: self.position(),
            end: self.position(),
            index: None,
        });
        Ok(index)
    }

    fn end(&mut self, index: usize) {
        self.entities[index].end = self.position();
    }

    fn entity_room(&self, count: usize) -> Result<()> {
        let available = self.options.maximum_entities - self.entities.len();
        if count <= available {
            return Ok(());
        }
        Err(extra_invalid(format!(
            "additional entity count {count}, used {}; expected <= {available} additional scopes",
            self.entities.len()
        )))
    }

    fn minimum(&self, count: usize, width: usize) -> Result<()> {
        if count <= self.cursor.remaining() / width {
            return Ok(());
        }
        Err(extra_invalid(format!(
            "item count {count}, minimum width {width}; expected <= {} complete items in remaining {} bytes",
            self.cursor.remaining() / width,
            self.cursor.remaining()
        )))
    }
}

fn validate_options(options: FlowBlockExtraOptions) -> Result<()> {
    let blocks = options.leaf_count.value() as usize;
    if options.maximum_decoded_bytes < 4
        || options.maximum_blocks == 0
        || options.maximum_entities == 0
        || options.maximum_fields == 0
        || options.maximum_values == 0
        || blocks > options.maximum_blocks
    {
        return Err(extra_invalid(format!(
            "limits/context {options:?}; expected decoded bytes >= 4, positive resource limits and leaf count {blocks} <= maximum blocks"
        )));
    }
    Ok(())
}
