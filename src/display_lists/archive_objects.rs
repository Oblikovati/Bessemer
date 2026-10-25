use super::{
    DisplayClassDescriptor, DisplayFieldValue, DisplayObjectTag, archive_decode::DisplayDecoder,
    archive_error::unsupported,
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ObjectRecord},
};

#[derive(Clone, Copy)]
enum ObjectRole<'a> {
    Declared(&'a [&'a str]),
    CObject,
}

impl DisplayDecoder<'_> {
    pub fn object(&mut self, label: &str, expected: &[&str]) -> Result<()> {
        self.object_with_role(label, ObjectRole::Declared(expected))
    }

    pub fn cobject(&mut self, label: &str) -> Result<()> {
        self.object_with_role(label, ObjectRole::CObject)
    }

    fn object_with_role(&mut self, label: &str, role: ObjectRole<'_>) -> Result<()> {
        let start = self.input.len() - self.cursor.remaining();
        let record = self.objects.object(&mut self.cursor)?;
        let end = self.input.len() - self.cursor.remaining();
        require_role(&record, role, label)?;
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
        self.array_with_role(label, ObjectRole::Declared(expected))
    }

    pub fn cobject_array(&mut self, label: &str) -> Result<()> {
        self.array_with_role(label, ObjectRole::CObject)
    }

    fn array_with_role(&mut self, label: &str, role: ObjectRole<'_>) -> Result<()> {
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
            self.object_with_role(&format!("{label}[{index}]"), role)?;
        }
        Ok(())
    }
}

fn require_role(record: &ObjectRecord, role: ObjectRole<'_>, label: &str) -> Result<()> {
    let ObjectRole::Declared(expected) = role else {
        return Ok(());
    };
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
        "uiViewBucket_c" => super::modern_bucket::view(archive),
        "gcBucketEdgeData_c" => super::modern_bucket_poly::edge(archive),
        "moAmbientLight_c" | "moLightFeature_c" => super::modern_lights::ambient(archive),
        "moDirectionLight_c" => super::modern_lights::directional(archive),
        "moSpotLight_c" => super::modern_lights::spot(archive),
        "moPointLight_c" => super::modern_lights::point(archive),
        "moVisualProperties_c" => super::modern_material::visual(archive),
        "moNodeName_c" => archive.text("name"),
        "moLineStyle_c" => super::modern_line_style::read(archive),
        "uiLayerMgr_c" => super::modern_layers::read(archive),
        "pwSceneData_c" => super::modern_pw_scene::read(archive),
        other => dispatch_tessellation(archive, other),
    }
}

fn dispatch_tessellation(archive: &mut DisplayDecoder<'_>, class: &str) -> Result<()> {
    match class {
        "uoTempAssemblySHDData_c" | "uoTempSubAssemblySHDData_c" => {
            super::modern_assembly::shaded_component(archive)
        }
        "moViewerComponentData_c" => super::modern_component::read(archive).map(|_| ()),
        "uoTempPartTessData_c" => super::modern_part_tess::read(archive),
        "uoTempBodyTessData_c" => super::modern_body_tess::body(archive),
        "uoTempFaceTessData_c" => super::modern_body_tess::face(archive),
        "uoBodyPropInfo_c" => super::modern_body_tess::properties(archive),
        other => dispatch_graphics(archive, other),
    }
}

fn dispatch_graphics(archive: &mut DisplayDecoder<'_>, class: &str) -> Result<()> {
    match class {
        "moGraphicsConnectRefMgrData_c" => super::graphics_reference::manager(archive),
        "moGraphicsMagneticConnectRefData_c" => super::graphics_reference::magnetic(archive),
        "moGraphicsRefWrapData_c" => super::graphics_reference::wrapper(archive),
        other => Err(unsupported(format!(
            "runtime class {other:?}; expected an implemented display-list object body"
        ))),
    }
}
