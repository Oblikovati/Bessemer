use super::{
    DisplayClassDescriptor, DisplayFieldValue, DisplayObjectTag, archive_decode::DisplayDecoder,
    archive_error::unsupported,
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ObjectRecord},
};

impl DisplayDecoder<'_> {
    pub fn object(&mut self, label: &str, expected: &[&str]) -> Result<()> {
        let start = self.input.len() - self.cursor.remaining();
        let record = self.objects.object(&mut self.cursor)?;
        let end = self.input.len() - self.cursor.remaining();
        require_role(&record, expected, label)?;
        let mut tag = read_tag(&self.input[start..end])?;
        resolve_target(&mut tag, &record)?;
        self.field(label, DisplayFieldValue::ObjectTag(tag))?;
        if let ObjectRecord::New { class, .. } = record {
            self.charge_object()?;
            self.scope(label, |archive| dispatch(archive, &class.name))?;
        }
        Ok(())
    }

    pub fn object_array(&mut self, label: &str, expected: &[&str]) -> Result<()> {
        let short = self.u16(&format!("{label}.count_word"))?;
        let count = if short == u16::MAX {
            self.u32(&format!("{label}.count_dword"))? as usize
        } else {
            short as usize
        };
        self.entries(count, label)?;
        if count > self.cursor.remaining() / 2 {
            return Err(super::archive_error::invalid(format!(
                "{label} count {count}; expected object tags fitting remaining bytes"
            )));
        }
        for index in 0..count {
            self.object(&format!("{label}[{index}]"), expected)?;
        }
        Ok(())
    }
}

fn require_role(record: &ObjectRecord, expected: &[&str], label: &str) -> Result<()> {
    let class = match record {
        ObjectRecord::Null => return Ok(()),
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. } => class,
    };
    // #31: map validity alone does not prove that an alias satisfies its declared pointer role.
    if expected.contains(&class.name.as_str()) {
        return Ok(());
    }
    Err(super::archive_error::invalid(format!(
        "object role {label:?} has runtime class {:?}; expected one of {expected:?}",
        class.name
    )))
}

fn read_tag(bytes: &[u8]) -> Result<DisplayObjectTag> {
    let mut cursor = ArchiveCursor::new(bytes);
    let tag = cursor.u16()?;
    let extended = if tag == 0x7fff {
        Some(cursor.u32()?)
    } else {
        None
    };
    let new_class = if tag == 0xffff {
        Some(read_descriptor(&mut cursor)?)
    } else {
        None
    };
    cursor.finish()?;
    Ok(DisplayObjectTag {
        target: super::DisplayObjectPointer::Null,
        runtime_class: None,
        tag,
        extended,
        new_class,
    })
}

fn read_descriptor(cursor: &mut ArchiveCursor<'_>) -> Result<DisplayClassDescriptor> {
    let schema = cursor.u16()?;
    let length = cursor.u16()? as usize;
    let name = cursor
        .take(length)?
        .iter()
        .map(|byte| char::from(*byte))
        .collect();
    Ok(DisplayClassDescriptor { schema, name })
}

fn resolve_target(tag: &mut DisplayObjectTag, record: &ObjectRecord) -> Result<()> {
    use super::{DisplayObjectId, DisplayObjectPointer};
    let (index, class, is_new) = match record {
        ObjectRecord::Null => return Ok(()),
        ObjectRecord::New { index, class } => (index, class, true),
        ObjectRecord::Reference { index, class } => (index, class, false),
    };
    let id = DisplayObjectId::new(index.value())?;
    tag.target = if is_new {
        DisplayObjectPointer::New(id)
    } else {
        DisplayObjectPointer::Reference(id)
    };
    tag.runtime_class = Some(DisplayClassDescriptor {
        schema: class.schema,
        name: class.name.clone(),
    });
    Ok(())
}

fn dispatch(archive: &mut DisplayDecoder<'_>, class: &str) -> Result<()> {
    match class {
        "uiUserModelEnv_c" => super::modern_ui::environment(archive),
        "uiLineFontMgr_c" => super::modern_ui::fonts(archive),
        "uiLFConfig_c" => super::modern_ui::font(archive),
        "moAmbientLight_c" | "moLightFeature_c" => super::modern_lights::ambient(archive),
        "moDirectionLight_c" => super::modern_lights::directional(archive),
        "moVisualProperties_c" => super::modern_material::visual(archive),
        "uoTempAssemblySHDData_c" | "uoTempSubAssemblySHDData_c" => {
            super::modern_assembly::shaded_component(archive)
        }
        "moViewerComponentData_c" => super::modern_component::read(archive),
        "uoTempPartTessData_c" => super::modern_part_tess::read(archive),
        other => Err(unsupported(format!(
            "runtime class {other:?}; expected an implemented display-list object body"
        ))),
    }
}
