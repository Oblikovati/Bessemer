use super::{
    DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits,
    DisplayFieldValue as Value, DisplayObjectTag, DisplayTransform, TransformContents,
    archive_error::invalid, output::DisplayOutput,
};
use crate::Result;

pub(super) fn encode(
    contents: &DisplayArchiveContents,
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<Vec<u8>> {
    super::view_support::require_size(
        contents.fields.len(),
        limits.maximum_fields,
        "display-list fields",
    )?;
    let mut output = DisplayOutput::new(limits.maximum_bytes);
    for field in &contents.fields {
        write_value(&mut output, &field.value, context, limits)?;
    }
    Ok(output.finish())
}

fn write_value(
    output: &mut DisplayOutput,
    value: &Value,
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<()> {
    match value {
        Value::U8(value) => output.bytes(&[*value]),
        Value::U16(value) => output.u16(*value),
        Value::I16(value) => output.bytes(&value.to_le_bytes()),
        Value::I32(value) => output.i32(*value),
        Value::U32(value) => output.u32(*value),
        Value::F32(value) => output.bytes(&value.bits().to_le_bytes()),
        Value::F64(value) => output.double(*value),
        Value::Text(value) => output.string(value),
        Value::Transform(value) => output.compact(value),
        _ => write_compound(output, value, context, limits),
    }
}

fn write_compound(
    output: &mut DisplayOutput,
    value: &Value,
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<()> {
    match value {
        Value::FixedTransform(value) => output
            .bytes(&DisplayTransform::new(TransformContents::Fixed(value.clone()))?.to_bytes()),
        Value::Box(value) => {
            let mut bytes = Vec::new();
            crate::archive_codec::write_box(value, &mut bytes);
            output.bytes(&bytes)
        }
        Value::ObjectTag(value) => write_tag(output, value),
        Value::ExternalFacet { .. } => Ok(()),
        Value::Facet(value) => output.bytes(&crate::tessellation::write_embedded(
            value,
            super::modern_body_tess::facet_context(context),
            output.remaining(),
        )?),
        _ => write_views(output, value, context, limits),
    }
}

fn write_views(
    output: &mut DisplayOutput,
    value: &Value,
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<()> {
    let limits = super::modern_root::view_limits(limits);
    let bytes = match value {
        Value::NamedViews(value) => super::named_write::encode(context.version, value, limits)?,
        Value::AnnotationViews(value) => {
            super::annotation_write::encode(context.version, value, limits)?
        }
        Value::SpatialViews(value) => super::spatial_write::encode(context.version, value, limits)?,
        other => {
            return Err(invalid(format!(
                "view packet {other:?}; expected named, annotation or spatial body"
            )));
        }
    };
    output.bytes(&bytes)
}

fn write_tag(output: &mut DisplayOutput, tag: &DisplayObjectTag) -> Result<()> {
    if tag.extended.is_some() != (tag.tag == 0x7fff)
        || tag.new_class.is_some() != (tag.tag == 0xffff)
    {
        return Err(invalid(format!(
            "MFC tag {:#06x} with extension/class presence {}/{}; expected framing selected by 0x7FFF/0xFFFF",
            tag.tag,
            tag.extended.is_some(),
            tag.new_class.is_some()
        )));
    }
    output.u16(tag.tag)?;
    if let Some(value) = tag.extended {
        output.u32(value)?;
    }
    if let Some(class) = &tag.new_class {
        write_class(output, class)?;
    }
    Ok(())
}

fn write_class(output: &mut DisplayOutput, class: &super::DisplayClassDescriptor) -> Result<()> {
    if class.name.is_empty() || !class.name.is_ascii() {
        return Err(invalid(format!(
            "class name {:?}; expected nonempty ASCII",
            class.name
        )));
    }
    let length = u16::try_from(class.name.len()).map_err(|_| {
        invalid(format!(
            "class name length {}; expected u16",
            class.name.len()
        ))
    })?;
    output.u16(class.schema)?;
    output.u16(length)?;
    output.bytes(class.name.as_bytes())
}
