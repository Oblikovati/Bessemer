use super::{grammar, invalid, unsupported};
use crate::Result;
use quick_xml::events::{BytesDecl, BytesStart};

pub(super) fn validate(declaration: &BytesDecl<'_>, position: usize) -> Result<()> {
    if position != 0 {
        return Err(invalid(format!(
            "XML declaration at byte {position}; expected byte 0 after physical BOM"
        )));
    }
    let start = BytesStart::from_content(declaration.as_ref(), 3);
    let mut names = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| {
            invalid(format!(
                "XML declaration attribute {error}; expected version, encoding, standalone order"
            ))
        })?;
        let name = attribute.key.as_ref();
        require_value(name, &attribute.value)?;
        names.push(name.to_owned());
    }
    require_order(&names)
}

fn require_order(names: &[String]) -> Result<()> {
    let permitted = [
        vec!["version"],
        vec!["version", "encoding"],
        vec!["version", "standalone"],
        vec!["version", "encoding", "standalone"],
    ];
    if !permitted
        .iter()
        .any(|order| order.iter().copied().eq(names.iter().map(String::as_str)))
    {
        return Err(invalid(format!(
            "XML declaration attributes {names:?}; expected version then optional encoding and standalone"
        )));
    }
    Ok(())
}

fn require_value(name: &str, value: &str) -> Result<()> {
    grammar::qualified(name)?;
    match name {
        "version" if value != "1.0" => {
            return Err(unsupported(format!("XML version {value:?}; expected 1.0")));
        }
        "encoding" => require_label(value)?,
        "standalone" if value != "yes" && value != "no" => {
            return Err(invalid(format!(
                "XML standalone {value:?}; expected yes or no"
            )));
        }
        _ => (),
    }
    Ok(())
}

fn require_label(value: &str) -> Result<()> {
    if !value.is_empty()
        && value.as_bytes()[0].is_ascii_alphabetic()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
    {
        return Ok(());
    }
    Err(invalid(format!(
        "XML encoding label {value:?}; expected XML EncName"
    )))
}
