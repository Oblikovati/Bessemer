use super::validate::EffectiveLayouts;
use super::wire::TransmitWriter;
use super::{
    DeltaSection, NodeRecord, NodeReference, RecordList, TransmitBody, TransmitContents, invalid,
};
use crate::Result;
use std::collections::BTreeSet;

struct Encoder<'a> {
    writer: TransmitWriter,
    contents: &'a TransmitContents,
    effective: &'a EffectiveLayouts,
    seen: BTreeSet<u16>,
}

pub(super) fn file(contents: &TransmitContents, effective: &EffectiveLayouts) -> Result<Vec<u8>> {
    let mut encoder = Encoder {
        writer: TransmitWriter::new(contents.format),
        contents,
        effective,
        seen: BTreeSet::new(),
    };
    encoder.writer.string32(&contents.title)?;
    encoder.writer.string32(&contents.schema_key.encoded())?;
    encoder.writer.u16(contents.n_types);
    encoder.writer.u32(0);
    encoder.body(&contents.body)?;
    Ok(encoder.writer.output)
}

impl Encoder<'_> {
    fn body(&mut self, body: &TransmitBody) -> Result<()> {
        match body {
            TransmitBody::Partition(records) => self.record_list(records)?,
            TransmitBody::Deltas(sections) => {
                for section in sections {
                    self.section(section)?;
                }
            }
        }
        Ok(())
    }

    fn section(&mut self, section: &DeltaSection) -> Result<()> {
        self.record_list(&section.records)?;
        if let Some(changes) = &section.changes {
            self.changes(changes)?;
        }
        Ok(())
    }

    fn record_list(&mut self, list: &RecordList) -> Result<()> {
        for record in &list.records {
            self.record(record)?;
        }
        self.writer.u16(1);
        self.writer.index(list.terminator);
        Ok(())
    }

    fn record(&mut self, record: &NodeRecord) -> Result<()> {
        self.writer.u16(record.node_type);
        if self.seen.insert(record.node_type) {
            self.layout(record.node_type)?;
        }
        if let Some(length) = record.length {
            self.writer.u32(length);
        }
        self.writer.index(record.index);
        for values in &record.fields {
            super::field_encode::write(&mut self.writer, values)?;
        }
        Ok(())
    }

    fn layout(&mut self, code: u16) -> Result<()> {
        let layout = self
            .contents
            .layouts
            .get(&code)
            .ok_or_else(|| invalid(format!("type {code}; expected validated layout")))?;
        let fields = self
            .effective
            .get(&code)
            .ok_or_else(|| invalid(format!("type {code}; expected effective fields")))?;
        super::layout_codec::write(&mut self.writer, layout, fields.len())
    }

    fn changes(&mut self, changes: &super::DeltaChanges) -> Result<()> {
        for node in &changes.new_nodes {
            self.writer.index(NodeReference::Node(node.index));
            self.writer.u16(node.node_type);
        }
        self.writer.index(NodeReference::Null);
        self.writer.u32(changes.count);
        self.record_list(&changes.modified)
    }
}
