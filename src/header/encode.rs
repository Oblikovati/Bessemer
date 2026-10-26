use std::collections::BTreeMap;

use super::fields::FieldView;
use super::{
    HeaderContents, HeaderObject, HeaderObjectId, HeaderReference, SharedFileName, invalid, values,
};
use crate::Result;
use crate::archive_codec::{ArchiveMapIndex, ObjectWriter};

enum Task<'a> {
    Object(HeaderReference),
    Field(FieldView<'a>),
}

pub(super) fn header(contents: &HeaderContents) -> Result<Vec<u8>> {
    let mut encoder = Encoder {
        contents,
        output: Vec::new(),
        map: ObjectWriter::new(),
        identities: BTreeMap::new(),
        tasks: vec![Task::Object(HeaderReference::Object(contents.root))],
    };
    while let Some(task) = encoder.tasks.pop() {
        match task {
            Task::Object(reference) => encoder.object(reference)?,
            Task::Field(value) => encoder.field(value)?,
        }
    }
    Ok(encoder.output)
}

struct Encoder<'a> {
    contents: &'a HeaderContents,
    output: Vec<u8>,
    map: ObjectWriter,
    identities: BTreeMap<HeaderObjectId, ArchiveMapIndex>,
    tasks: Vec<Task<'a>>,
}

impl<'a> Encoder<'a> {
    fn object(&mut self, reference: HeaderReference) -> Result<()> {
        let HeaderReference::Object(id) = reference else {
            self.output.extend_from_slice(&0u16.to_le_bytes());
            return Ok(());
        };
        if let Some(index) = self.identities.get(&id) {
            ObjectWriter::reference(*index, &mut self.output);
            return Ok(());
        }
        self.new_object(id)
    }

    fn new_object(&mut self, id: HeaderObjectId) -> Result<()> {
        let node = self.contents.objects.get(&id).ok_or_else(|| {
            invalid(format!(
                "object {}; expected validated graph identity",
                id.value()
            ))
        })?;
        let class = node.archive_class();
        let index = self.map.object(&class, &mut self.output)?;
        self.identities.insert(id, index);
        self.body(&node.object)
    }

    fn body(&mut self, object: &'a HeaderObject) -> Result<()> {
        match object {
            HeaderObject::Stamp(stamp) => {
                crate::history::encode::write_stamp(&mut self.output, stamp)?
            }
            HeaderObject::StringHandle(text) => text.encode(&mut self.output)?,
            object => self
                .tasks
                .extend(object.views().into_iter().flatten().rev().map(Task::Field)),
        }
        Ok(())
    }

    fn field(&mut self, value: FieldView<'a>) -> Result<()> {
        match value {
            FieldView::Reference(reference) => self.tasks.push(Task::Object(reference)),
            FieldView::SharedName(SharedFileName::Handle(reference)) => {
                self.tasks.push(Task::Object(*reference))
            }
            FieldView::References(items) => {
                values::write_count(items.len(), &mut self.output)?;
                self.tasks
                    .extend(items.iter().rev().map(|reference| Task::Object(*reference)));
            }
            _ => values::write_scalar(value, &mut self.output)?,
        }
        Ok(())
    }
}
