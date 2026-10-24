use bessemer::display_lists::{ArchiveString, DisplayFieldValue};

pub(crate) fn scalar(value: &DisplayFieldValue) -> Option<String> {
    let encoded = match value {
        DisplayFieldValue::U8(value) => format!("i:{value}"),
        DisplayFieldValue::U16(value) => format!("i:{value}"),
        DisplayFieldValue::I16(value) => format!("i:{value}"),
        DisplayFieldValue::I32(value) => format!("i:{value}"),
        DisplayFieldValue::U32(value) => format!("i:{value}"),
        DisplayFieldValue::F32(value) => format!("f:{:016x}", f64::from(value.value()).to_bits()),
        DisplayFieldValue::F64(value) => format!("f:{:016x}", value.bits()),
        DisplayFieldValue::Text(value) => text(value),
        _ => return None,
    };
    Some(encoded)
}

fn text(value: &ArchiveString) -> String {
    let encoded = value
        .text()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("s:{encoded}")
}
