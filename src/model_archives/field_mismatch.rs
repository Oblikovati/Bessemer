use super::{ModelField, ModelFieldValue, ModelObjectRecord};

pub(super) fn construction_mismatch(
    supplied: &[ModelField],
    expected: &[ModelField],
) -> crate::BessemerError {
    let index = supplied
        .iter()
        .zip(expected)
        .position(|(left, right)| left != right)
        .unwrap_or(supplied.len().min(expected.len()));
    super::invalid(format!(
        "field {index} supplied {}; expected {}; supplied field count {}, expected {}; expected exact schema paths, types and object identities",
        field_summary(supplied.get(index)),
        field_summary(expected.get(index)),
        supplied.len(),
        expected.len()
    ))
}

fn field_summary(field: Option<&ModelField>) -> String {
    let Some(field) = field else {
        return "missing field".into();
    };
    format!("{} = {}", brief(&field.path), value_summary(&field.value))
}

fn value_summary(value: &ModelFieldValue) -> String {
    match value {
        ModelFieldValue::U8(value) => format!("u8 {value}"),
        ModelFieldValue::U16(value) => format!("u16 {value}"),
        ModelFieldValue::I16(value) => format!("i16 {value}"),
        ModelFieldValue::I32(value) => format!("i32 {value}"),
        ModelFieldValue::U32(value) => format!("u32 {value}"),
        ModelFieldValue::U64(value) => format!("u64 {value}"),
        ModelFieldValue::F32(value) => format!("f32 bits {:#010x}", value.bits()),
        ModelFieldValue::F64(value) => format!("f64 bits {:#018x}", value.bits()),
        ModelFieldValue::Text(value) => {
            format!("CString {:?} {}", value.encoding(), brief(value.text()))
        }
        ModelFieldValue::ObjectTag(value) => object_summary(&value.record),
        ModelFieldValue::Transform(_) => "compact transform".into(),
        ModelFieldValue::Kernel(value) => format!(
            "kernel partition: {} records, {} expanded bytes",
            value.record_count(),
            value.expanded_bytes()
        ),
    }
}

fn object_summary(record: &ModelObjectRecord) -> String {
    let (kind, index, class) = match record {
        ModelObjectRecord::Null => return "null pointer".into(),
        ModelObjectRecord::New { index, class } => ("new", index, class),
        ModelObjectRecord::Reference { index, class } => ("reference", index, class),
    };
    format!(
        "{kind} map index {}, class {}, schema {}",
        index.value(),
        brief(&class.name),
        class.schema
    )
}

fn brief(value: &str) -> String {
    let mut escaped = value.chars().flat_map(char::escape_debug);
    let prefix: String = escaped.by_ref().take(160).collect();
    let suffix = if escaped.next().is_some() { "…" } else { "" };
    format!("\"{prefix}{suffix}\" ({} bytes)", value.len())
}

#[cfg(test)]
mod tests {
    use super::{brief, construction_mismatch};
    use crate::model_archives::{ModelField, ModelFieldValue};

    #[test]
    fn missing_fields_and_control_characters_are_bounded_and_explicit() {
        let field = ModelField {
            path: "root.\nvalue".into(),
            value: ModelFieldValue::U32(9),
        };
        let error = construction_mismatch(&[], &[field]);
        assert!(error.cause.contains("missing field"));
        assert!(error.cause.contains("root.\\nvalue"));
        assert!(error.cause.contains("u32 9"));
        assert!(brief(&"x".repeat(100000)).len() < 200);
    }
}
