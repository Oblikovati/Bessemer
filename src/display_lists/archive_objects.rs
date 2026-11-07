use super::{
    DisplayClassDescriptor, DisplayFieldValue, DisplayObjectTag, archive_decode::DisplayDecoder,
    archive_error::unsupported,
};
use crate::{
    Result,
    archive_codec::{
        ArchiveObjectRole as ObjectRole, ObjectFraming, ObjectRecord, require_object_role,
    },
};

impl DisplayDecoder<'_> {
    pub fn object(&mut self, label: &str, expected: &[&str]) -> Result<()> {
        self.object_with_role(label, ObjectRole::Declared(expected))
    }

    pub fn cobject(&mut self, label: &str) -> Result<()> {
        self.object_with_role(label, ObjectRole::Any)
    }

    fn object_with_role(&mut self, label: &str, role: ObjectRole<'_>) -> Result<()> {
        let framed = self.objects.framed_object(&mut self.cursor)?;
        let record = framed.record;
        require_object_role(&record, role, label, super::archive_error::invalid)?;
        let mut tag = display_tag(framed.framing, &record);
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
        self.array_with_role(label, ObjectRole::Any)
    }

    pub fn su_count(&mut self, label: &str) -> Result<usize> {
        let short = self.u16(&format!("{label}.count_word"))?;
        let count = if short == u16::MAX {
            self.u32(&format!("{label}.count_dword"))? as usize
        } else {
            short as usize
        };
        self.entries(count, label)?;
        Ok(count)
    }

    fn array_with_role(&mut self, label: &str, role: ObjectRole<'_>) -> Result<()> {
        let count = self.su_count(label)?;
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

fn display_tag(framing: ObjectFraming, record: &ObjectRecord) -> DisplayObjectTag {
    let new_class = match (framing.tag, record) {
        (0xffff, ObjectRecord::New { class, .. }) => Some(DisplayClassDescriptor {
            schema: class.schema,
            name: class.name.clone(),
        }),
        _ => None,
    };
    DisplayObjectTag {
        target: super::DisplayObjectPointer::Null,
        runtime_class: None,
        tag: framing.tag,
        extended: framing.extended,
        new_class,
    }
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
        "moComponentProperty" => super::modern_component_properties::property(archive),
        "moComponentPropertyHolder" => super::modern_component_properties::holder(archive),
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
