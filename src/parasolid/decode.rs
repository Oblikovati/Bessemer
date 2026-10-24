use super::wire::TransmitCursor;
use super::{
    BaseSchema, DeltaChanges, DeltaSection, FieldDefinition, LayoutDefinition, NewNode, NodeRecord,
    NodeReference, RecordList, SchemaKey, TransmitBody, TransmitContents, TransmitLimits, invalid,
    unsupported,
};
use crate::Result;
use std::collections::BTreeMap;

type TransmitHeader = (String, SchemaKey, u16);
struct Decoder<'a, 's> {
    cursor: TransmitCursor<'a>,
    base: &'s BaseSchema,
    limits: TransmitLimits,
    count: usize,
    layouts: BTreeMap<u16, LayoutDefinition>,
    effective: BTreeMap<u16, Vec<FieldDefinition>>,
}

pub(super) fn file(
    bytes: &[u8],
    base: &BaseSchema,
    limits: TransmitLimits,
) -> Result<TransmitContents> {
    let mut decoder = Decoder::new(TransmitCursor::new(bytes)?, base, limits);
    let (title, schema_key, n_types) = read_header(&mut decoder.cursor, base)?;
    let body = if super::validate::is_delta_title(&title)? {
        decoder.deltas()?
    } else {
        TransmitBody::Partition(decoder.record_list(false, false)?)
    };
    decoder.finish(title, schema_key, n_types, body)
}

fn read_header(cursor: &mut TransmitCursor<'_>, base: &BaseSchema) -> Result<TransmitHeader> {
    let title = cursor.string32()?;
    let schema_key = SchemaKey::parse(&cursor.string32()?)?;
    require_schema(&schema_key, base)?;
    let n_types = cursor.u16()?;
    let user_field_size = cursor.u32()?;
    if user_field_size != 0 {
        return Err(unsupported(format!(
            "user_field_size {user_field_size}; expected zero for documented transmit layout"
        )));
    }
    Ok((title, schema_key, n_types))
}

pub(super) fn require_schema(key: &SchemaKey, base: &BaseSchema) -> Result<()> {
    if key.base == base.number() {
        return Ok(());
    }
    Err(invalid(format!(
        "transmit base schema {}; expected supplied schema {}",
        key.base.value(),
        base.number().value()
    )))
}

impl<'a, 's> Decoder<'a, 's> {
    fn new(cursor: TransmitCursor<'a>, base: &'s BaseSchema, limits: TransmitLimits) -> Self {
        Self {
            cursor,
            base,
            limits,
            count: 0,
            layouts: BTreeMap::new(),
            effective: BTreeMap::new(),
        }
    }

    fn finish(
        self,
        title: String,
        schema_key: SchemaKey,
        n_types: u16,
        body: TransmitBody,
    ) -> Result<TransmitContents> {
        if self.cursor.remaining() != 0 {
            return Err(invalid(format!(
                "transmit trailing bytes {}; expected complete file consumption",
                self.cursor.remaining()
            )));
        }
        Ok(TransmitContents {
            format: self.cursor.format,
            title,
            schema_key,
            n_types,
            layouts: self.layouts,
            body,
        })
    }

    fn record_list(&mut self, deltas: bool, modified: bool) -> Result<RecordList> {
        let mut records = Vec::new();
        loop {
            let code = self.cursor.u16()?;
            if code == 1 {
                return Ok(RecordList {
                    records,
                    terminator: self.cursor.index()?,
                });
            }
            records.push(self.record(code, modified || (deltas && !matches!(code, 3 | 4)))?);
        }
    }

    fn record(&mut self, code: u16, delta: bool) -> Result<NodeRecord> {
        self.charge_record()?;
        self.ensure_layout(code)?;
        let definitions = self
            .effective
            .get(&code)
            .ok_or_else(|| invalid(format!("type {code}; expected effective layout")))?;
        super::record_decode::read(
            &mut self.cursor,
            code,
            definitions,
            delta,
            self.limits.maximum_field_elements,
        )
    }

    fn charge_record(&mut self) -> Result<()> {
        if self.count >= self.limits.maximum_records {
            return Err(invalid(format!(
                "transmit record count {}; expected at most {}",
                self.count + 1,
                self.limits.maximum_records
            )));
        }
        self.count += 1;
        Ok(())
    }

    fn ensure_layout(&mut self, code: u16) -> Result<()> {
        if self.layouts.contains_key(&code) {
            return Ok(());
        }
        let layout = super::layout_codec::read(&mut self.cursor, code, self.base)?;
        let fields = super::definitions::effective_layout(self.base, code, &layout)?;
        self.layouts.insert(code, layout);
        self.effective.insert(code, fields);
        Ok(())
    }

    fn deltas(&mut self) -> Result<TransmitBody> {
        let mut sections = Vec::new();
        loop {
            let records = self.record_list(true, false)?;
            if self.cursor.remaining() == 0 {
                sections.push(DeltaSection {
                    records,
                    changes: None,
                });
                return Ok(TransmitBody::Deltas(sections));
            }
            let changes = Some(self.changes()?);
            sections.push(DeltaSection { records, changes });
        }
    }

    fn changes(&mut self) -> Result<DeltaChanges> {
        let new_nodes = self.new_nodes()?;
        let count = self.cursor.u32()?;
        let modified = self.record_list(true, true)?;
        Ok(DeltaChanges {
            new_nodes,
            count,
            modified,
        })
    }

    fn new_nodes(&mut self) -> Result<Vec<NewNode>> {
        let mut nodes = Vec::new();
        loop {
            let NodeReference::Node(index) = self.cursor.index()? else {
                return Ok(nodes);
            };
            if nodes.len() >= self.limits.maximum_records {
                return Err(invalid(format!(
                    "new-node count {}; expected at most {}",
                    nodes.len() + 1,
                    self.limits.maximum_records
                )));
            }
            nodes.push(NewNode {
                index,
                node_type: self.cursor.u16()?,
            });
        }
    }
}
