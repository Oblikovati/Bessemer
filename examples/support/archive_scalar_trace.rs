use bessemer::display_lists::{ArchiveString, DisplayFieldValue};

pub(crate) fn scalar(value: &DisplayFieldValue) -> Option<String> {
    let encoded = match value {
        DisplayFieldValue::U8(value) => format!("i:{value}"),
        DisplayFieldValue::U16(value) => format!("i:{value}"),
        DisplayFieldValue::I16(value) => format!("i:{value}"),
        DisplayFieldValue::I32(value) => format!("i:{value}"),
        DisplayFieldValue::U32(value) => format!("i:{value}"),
        DisplayFieldValue::F32(value) => format!("f:{:016x}", float_bits(*value)),
        DisplayFieldValue::F64(value) => format!("f:{:016x}", value.bits()),
        DisplayFieldValue::Text(value) => text(value),
        _ => return None,
    };
    Some(encoded)
}

fn float_bits(value: bessemer::tessellation::StoredFloat) -> u64 {
    let bits = value.bits();
    let payload = bits & 0x007f_ffff;
    if bits & 0x7f80_0000 != 0x7f80_0000 || payload == 0 {
        return f64::from(value.value()).to_bits();
    }
    // #35: native widening can quiet signaling NaNs; tracing must retain the stored payload.
    (u64::from(bits & 0x8000_0000) << 32) | 0x7ff0_0000_0000_0000 | (u64::from(payload) << 29)
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
