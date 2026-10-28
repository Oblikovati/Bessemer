use crate::Result;
use crate::archive_codec::{
    FramedObjectRecord, ObjectClassView, ObjectRecord, require_object_role, write_object_framing,
};

pub(super) use crate::archive_codec::ArchiveObjectRole as ModelObjectRole;

pub(super) fn require_role(
    record: &ObjectRecord,
    role: ModelObjectRole<'_>,
    label: &str,
) -> Result<()> {
    if representation_role(record, role) || super::weld_symbols::assignable(record, role) {
        return Ok(());
    }
    require_object_role(record, role, label, super::invalid)
}

fn representation_role(record: &ObjectRecord, role: ModelObjectRole<'_>) -> bool {
    let ModelObjectRole::Declared(expected) = role else {
        return false;
    };
    if !expected.contains(&"moEntityIdRep_c") {
        return false;
    }
    let class = match record {
        ObjectRecord::Null => return false,
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. } => class,
    };
    super::entity_representations::layout(&class.name).is_some()
}

fn class_view(framed: &FramedObjectRecord) -> Result<Option<ObjectClassView<'_>>> {
    if framed.framing.tag != 0xffff {
        return Ok(None);
    }
    let ObjectRecord::New { class, .. } = &framed.record else {
        return Err(super::invalid(format!(
            "MFC tag 0xFFFF with {:?}; expected a new object",
            framed.record
        )));
    };
    Ok(Some(ObjectClassView {
        name: &class.name,
        schema: class.schema,
    }))
}

pub(super) fn tag_size(framed: &FramedObjectRecord) -> Result<usize> {
    let class = class_view(framed)?;
    Ok(class.map_or(
        if framed.framing.extended.is_some() {
            6
        } else {
            2
        },
        |class| 6 + class.name.len(),
    ))
}

pub(super) fn encode_tag(framed: &FramedObjectRecord, output: &mut Vec<u8>) -> Result<()> {
    let class = class_view(framed)?;
    write_object_framing(
        &framed.framing,
        class,
        |bytes| {
            output.extend_from_slice(bytes);
            Ok(())
        },
        super::invalid,
    )
}
