use super::super::{ModelObjectRole, configuration_io, invalid, object_body, object_values};
use super::{ModelDecoder, ModelFieldValue};
use crate::Result;
use crate::archive_codec::{ArchiveClass, MaterialRead, ObjectReader, ObjectRecord};
use crate::header::{ArchiveVersion, FieldRole, FieldRule, HEADER_VERSION_MARKER, Phase, WireKind};

/// Decode an inline header on the caller's map, without external-header imports.
/// Example: `inline_header::read(reader, map)?` after its fresh header tag.
pub(in crate::model_archives) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    let outer = reader.archive_version();
    let result = header_fields(reader, map, outer);
    reader.version = outer;
    reader.checked(result)
}

fn header_fields(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    outer: ArchiveVersion,
) -> Result<()> {
    let mut body = outer;
    for rule in crate::header::inline_header_rules() {
        reader.version = match rule.phase {
            Phase::Outer => outer,
            Phase::Body => body,
        };
        if !(rule.enabled)(reader.version()) {
            continue;
        }
        if rule.name == "users" {
            let users = pointer(reader, map, rule.name, role(rule.role))?;
            body = user_version(reader, &users, outer)?;
            continue;
        }
        field(reader, map, rule)?;
    }
    Ok(())
}

fn user_version(
    reader: &ModelDecoder<'_>,
    users: &ObjectRecord,
    outer: ArchiveVersion,
) -> Result<ArchiveVersion> {
    if matches!(users, ObjectRecord::Null) {
        return Ok(outer);
    }
    let fields = reader.object_fields(users);
    let count = object_values::stored_object_count(fields, users, "items.count")?;
    if count < 2 {
        return Ok(outer);
    }
    let value =
        object_values::stored_object_value(fields, users, &format!("items[{}]", count - 1))?;
    let ModelFieldValue::Text(text) = value else {
        return Err(invalid(format!(
            "users last field {value:?}; expected CString"
        )));
    };
    if !text.text().contains(HEADER_VERSION_MARKER) {
        return Ok(outer);
    }
    ArchiveVersion::decimal(&text.text().replace(HEADER_VERSION_MARKER, ""))
}

fn role(role: FieldRole) -> ModelObjectRole<'static> {
    match role {
        FieldRole::Any => ModelObjectRole::Any,
        FieldRole::Strings => ModelObjectRole::Declared(&["su_CStringArray"]),
        FieldRole::ObjectList | FieldRole::RevisionLogs => ModelObjectRole::Declared(&["suObList"]),
        FieldRole::Log => ModelObjectRole::Declared(&["moLogs_c"]),
        FieldRole::Stamp => ModelObjectRole::Declared(&["moStamp_c"]),
        FieldRole::ExternalList => ModelObjectRole::Declared(&["moExtObjectList_c"]),
        FieldRole::External => ModelObjectRole::Declared(&["moExtObject_c"]),
        FieldRole::Handle => ModelObjectRole::Declared(&["moCStringHandle_c"]),
    }
}

fn pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<ObjectRecord> {
    let record = reader.object_tag(label, map, role)?;
    if let ObjectRecord::New { class, .. } = &record {
        reader.scope(label, |reader| header_object(reader, map, class))?;
    }
    Ok(record)
}

fn header_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moHeader_c" => read(reader, map),
        "su_CStringArray" => configuration_io::strings(reader, "items"),
        "suObList" | "moExtObjectList_c" => {
            pointer_list(reader, map, "items", ModelObjectRole::Any)
        }
        "moLogs_c" => logs(reader, map),
        "moStamp_c" => stamp(reader),
        "moCStringHandle_c" => reader.text("value"),
        "moExtObject_c" => external(reader, map),
        _ => object_body::read(reader, map, class),
    }
}

fn pointer_list(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<()> {
    for index in 0..reader.object_count(&format!("{label}.count"))? {
        pointer(reader, map, &format!("{label}[{index}]"), role)?;
    }
    Ok(())
}

fn logs(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    pointer_list(
        reader,
        map,
        "items",
        ModelObjectRole::Declared(&["moStamp_c"]),
    )?;
    reader.i32("id")?;
    reader.text("unknown_50")
}

fn stamp(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let stamp = reader.scalar(crate::history::decode::read_stamp)?;
    reader.field(
        "revision_status",
        ModelFieldValue::U16(stamp.revision_status),
    )?;
    reader.field("unknown_0c", ModelFieldValue::U16(stamp.unknown_0c))?;
    reader.field("unknown_10", ModelFieldValue::U16(stamp.unknown_10))?;
    reader.field("time", ModelFieldValue::U32(stamp.time))?;
    reader.field("comment", ModelFieldValue::Text(stamp.comment))
}

fn external(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() < 0xf18 {
        return reader.checked(Err(invalid(format!(
            "inline external version {}; expected >= 0xF18",
            reader.version()
        ))));
    }
    for rule in crate::header::external_field_rules() {
        if (rule.enabled)(reader.version()) {
            field(reader, map, rule)?;
        }
    }
    Ok(())
}

fn field(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, rule: &FieldRule) -> Result<()> {
    match rule.kind {
        WireKind::U8 => reader.u8(rule.name).map(|_| ()),
        WireKind::U16 => reader.u16(rule.name).map(|_| ()),
        WireKind::U32 => reader.u32(rule.name).map(|_| ()),
        WireKind::I32 => reader.i32(rule.name).map(|_| ()),
        WireKind::Text => reader.text(rule.name),
        WireKind::Reference => pointer(reader, map, rule.name, role(rule.role)).map(|_| ()),
        WireKind::References => pointer_list(reader, map, rule.name, role(rule.role)),
        WireKind::Texts => configuration_io::strings(reader, rule.name),
        WireKind::I32x3 => integers(reader, rule.name, 3),
        WireKind::I32x6 => integers(reader, rule.name, 6),
        WireKind::Bounds => bounds(reader, rule.name),
        WireKind::SharedName => shared_name(reader, map, rule.name),
    }
}

fn integers(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.i32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn bounds(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    if reader.i32(&format!("{label}.has_box"))? == 0 {
        return Ok(());
    }
    for index in 0..10 {
        reader.f64(&format!("{label}.box[{index}]"))?;
    }
    Ok(())
}

fn shared_name(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    if reader.version() < 0xfa7 {
        return reader.text(label);
    }
    pointer(
        reader,
        map,
        label,
        ModelObjectRole::Declared(&["moCStringHandle_c"]),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "../../tests/resolved_features/inline_headers.rs"]
mod tests;
