use super::{ModelDecoder, ModelObjectRole};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::header::{FieldRule, WireKind};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for rule in crate::header::external_field_rules() {
        if (rule.enabled)(reader.version()) {
            field(reader, map, rule)?;
        }
    }
    Ok(())
}

pub(super) fn shared_file(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for rule in crate::header::external_field_rules()
        .iter()
        .take_while(|rule| rule.name != "flags")
    {
        if (rule.enabled)(reader.version()) {
            field(reader, map, rule)?;
        }
    }
    Ok(())
}

fn field(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, rule: &FieldRule) -> Result<()> {
    let path = field_path(rule.name);
    match rule.kind {
        WireKind::U8 => reader.u8(&path).map(|_| ()),
        WireKind::U16 => reader.u16(&path).map(|_| ()),
        WireKind::U32 => reader.u32(&path).map(|_| ()),
        WireKind::I32 => reader.i32(&path).map(|_| ()),
        WireKind::Text => reader.text(&path),
        WireKind::SharedName => reader.object(
            &path,
            map,
            ModelObjectRole::Declared(&["moCStringHandle_c"]),
        ),
        other => unsupported_kind(reader, &path, other),
    }
}

fn field_path(name: &str) -> String {
    if let Some(name) = name.strip_prefix("shared_") {
        return format!("file.{name}");
    }
    match name {
        "path" | "name" | "imported_path" | "pdm_id" => format!("file.{name}"),
        "configuration_name" => "referenced_configuration.name".into(),
        "configuration_id" => "referenced_configuration.id".into(),
        _ => name.into(),
    }
}

pub(super) fn unsupported_kind(
    reader: &mut ModelDecoder<'_>,
    path: &str,
    kind: WireKind,
) -> Result<()> {
    reader.checked(Err(BessemerError::new(
        "model archive", path, FailureClass::Unsupported,
        format!("external field {path:?} kind {kind:?}; expected a scalar, CString or modern shared-name pointer"),
    )))
}
