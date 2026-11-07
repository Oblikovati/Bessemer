use std::collections::BTreeMap;

use super::context::VersionContext;
use super::fields::{FieldValue, Phase, WireKind};
use super::kinds::ClassKind;
use super::{
    ArchiveVersion, HeaderContents, HeaderNode, HeaderObject, HeaderObjectId, HeaderReference,
    SharedFileName, invalid, values,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ArchiveString, ObjectReader, ObjectRecord};

enum Destination {
    Root,
    Field(HeaderObjectId, usize, bool),
    Element(HeaderObjectId, usize, usize),
}
enum Task {
    Object(Destination, ArchiveVersion),
    Fields(HeaderObjectId, usize),
}

struct PendingNode {
    kind: ClassKind,
    schema: u16,
    context: VersionContext,
    fields: Vec<Option<FieldValue>>,
}

struct Decoder<'a> {
    cursor: ArchiveCursor<'a>,
    map: ObjectReader,
    maximum_objects: usize,
    root: Option<HeaderObjectId>,
    pending: BTreeMap<HeaderObjectId, PendingNode>,
    objects: BTreeMap<HeaderObjectId, HeaderNode>,
    tasks: Vec<Task>,
}

pub(super) fn header(
    bytes: &[u8],
    version: ArchiveVersion,
    maximum_objects: usize,
) -> Result<HeaderContents> {
    let mut decoder = Decoder {
        cursor: ArchiveCursor::new(bytes),
        map: ObjectReader::new(),
        maximum_objects,
        root: None,
        pending: BTreeMap::new(),
        objects: BTreeMap::new(),
        tasks: vec![Task::Object(Destination::Root, version)],
    };
    decoder.complete()
}

impl Decoder<'_> {
    fn complete(&mut self) -> Result<HeaderContents> {
        while let Some(task) = self.tasks.pop() {
            match task {
                Task::Object(destination, version) => self.object(destination, version)?,
                Task::Fields(id, offset) => self.fields(id, offset)?,
            }
        }
        self.cursor.finish()?;
        let root = self
            .root
            .ok_or_else(|| invalid("root null; expected moHeader_c object"))?;
        Ok(HeaderContents {
            root,
            objects: std::mem::take(&mut self.objects),
        })
    }

    fn object(&mut self, destination: Destination, version: ArchiveVersion) -> Result<()> {
        match self.map.object(&mut self.cursor)? {
            ObjectRecord::Null => self.assign(destination, HeaderReference::Null),
            ObjectRecord::Reference { index, .. } => {
                self.assign(destination, HeaderReference::Object(HeaderObjectId(index)))
            }
            ObjectRecord::New { index, class } => {
                let identity = HeaderObjectId(index);
                self.assign(destination, HeaderReference::Object(identity))?;
                self.new_node(identity, ClassKind::parse(&class)?, class.schema, version)
            }
        }
    }

    fn new_node(
        &mut self,
        identity: HeaderObjectId,
        kind: ClassKind,
        schema: u16,
        version: ArchiveVersion,
    ) -> Result<()> {
        let count = self.pending.len() + self.objects.len();
        if count >= self.maximum_objects {
            return Err(invalid(format!(
                "object count {}; expected at most {}",
                count + 1,
                self.maximum_objects
            )));
        }
        super::context::require_supported(kind, version)?;
        self.reserve_body(identity, kind, schema, version)
    }

    fn direct_body(&mut self, kind: ClassKind) -> Result<Option<HeaderObject>> {
        let direct = match kind {
            ClassKind::Stamp => Some(HeaderObject::Stamp(crate::history::decode::read_stamp(
                &mut self.cursor,
            )?)),
            ClassKind::Handle => Some(HeaderObject::StringHandle(ArchiveString::decode(
                &mut self.cursor,
            )?)),
            _ => None,
        };
        Ok(direct)
    }

    fn reserve_body(
        &mut self,
        identity: HeaderObjectId,
        kind: ClassKind,
        schema: u16,
        version: ArchiveVersion,
    ) -> Result<()> {
        if let Some(object) = self.direct_body(kind)? {
            self.objects.insert(identity, HeaderNode { schema, object });
            return Ok(());
        }
        let fields = (0..kind.rules().len()).map(|_| None).collect();
        self.pending.insert(
            identity,
            PendingNode {
                kind,
                schema,
                context: VersionContext::new(version),
                fields,
            },
        );
        self.tasks.push(Task::Fields(identity, 0));
        Ok(())
    }

    fn assign(&mut self, destination: Destination, reference: HeaderReference) -> Result<()> {
        match destination {
            Destination::Root => {
                if let HeaderReference::Object(id) = reference {
                    self.root = Some(id);
                }
            }
            Destination::Field(id, offset, shared) => {
                let value = if shared {
                    FieldValue::SharedName(SharedFileName::Handle(reference))
                } else {
                    FieldValue::Reference(reference)
                };
                self.pending_node(id)?.fields[offset] = Some(value);
            }
            Destination::Element(id, offset, element) => {
                self.assign_element(id, offset, element, reference)?;
            }
        }
        Ok(())
    }

    fn assign_element(
        &mut self,
        id: HeaderObjectId,
        offset: usize,
        element: usize,
        reference: HeaderReference,
    ) -> Result<()> {
        let Some(FieldValue::References(items)) = &mut self.pending_node(id)?.fields[offset] else {
            return Err(invalid("list destination absent; expected reference array"));
        };
        items[element] = reference;
        Ok(())
    }

    fn pending_node(&mut self, id: HeaderObjectId) -> Result<&mut PendingNode> {
        self.pending.get_mut(&id).ok_or_else(|| {
            invalid(format!(
                "pending object {}; expected reserved node",
                id.value()
            ))
        })
    }

    fn fields(&mut self, identity: HeaderObjectId, offset: usize) -> Result<()> {
        self.override_version(identity, offset)?;
        let node = self.pending_node(identity)?;
        let Some(rule) = node.kind.rules().get(offset) else {
            return self.finish_node(identity);
        };
        let context = node.context;
        self.tasks.push(Task::Fields(identity, offset + 1));
        if !context.enabled(rule) {
            return Ok(());
        }
        let version = match rule.phase {
            Phase::Outer => context.outer,
            Phase::Body => context.body,
        };
        self.read_field(identity, offset, rule.kind, version)
    }

    fn override_version(&mut self, identity: HeaderObjectId, offset: usize) -> Result<()> {
        let node = self.pending_node(identity)?;
        if node.kind != ClassKind::Header || offset != 2 {
            return Ok(());
        }
        let Some(FieldValue::Reference(users)) = node.fields[1] else {
            return Err(invalid("users field absent; expected object reference"));
        };
        let context = node.context.with_users(users, &self.objects)?;
        self.pending_node(identity)?.context = context;
        Ok(())
    }

    fn read_field(
        &mut self,
        identity: HeaderObjectId,
        offset: usize,
        kind: WireKind,
        version: ArchiveVersion,
    ) -> Result<()> {
        match kind {
            WireKind::Reference => self.tasks.push(Task::Object(
                Destination::Field(identity, offset, false),
                version,
            )),
            WireKind::SharedName if version.value() >= 0xfa7 => self.tasks.push(Task::Object(
                Destination::Field(identity, offset, true),
                version,
            )),
            WireKind::References => self.read_references(identity, offset, version)?,
            _ => self.scalar_field(identity, offset, kind)?,
        }
        Ok(())
    }

    fn scalar_field(
        &mut self,
        identity: HeaderObjectId,
        offset: usize,
        kind: WireKind,
    ) -> Result<()> {
        let value = match kind {
            WireKind::SharedName => FieldValue::SharedName(SharedFileName::Text(
                ArchiveString::decode(&mut self.cursor)?,
            )),
            _ => values::read_scalar(&mut self.cursor, kind)?,
        };
        self.pending_node(identity)?.fields[offset] = Some(value);
        Ok(())
    }

    fn read_references(
        &mut self,
        identity: HeaderObjectId,
        offset: usize,
        version: ArchiveVersion,
    ) -> Result<()> {
        let count = self.cursor.collection_count(2)?;
        self.pending_node(identity)?.fields[offset] =
            Some(FieldValue::References(vec![HeaderReference::Null; count]));
        for element in (0..count).rev() {
            self.tasks.push(Task::Object(
                Destination::Element(identity, offset, element),
                version,
            ));
        }
        Ok(())
    }

    fn finish_node(&mut self, identity: HeaderObjectId) -> Result<()> {
        let node = self.pending.remove(&identity).ok_or_else(|| {
            invalid(format!(
                "object {}; expected unfinished body",
                identity.value()
            ))
        })?;
        let object = node.kind.assemble(node.fields)?;
        self.objects.insert(
            identity,
            HeaderNode {
                schema: node.schema,
                object,
            },
        );
        Ok(())
    }
}
