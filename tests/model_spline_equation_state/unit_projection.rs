use super::*;
use crate::archive_codec::TextEncoding;

const PREFIX: &str = "sketch.splines[0]";

fn saved_equation_group() -> Vec<ModelField> {
    let mut fields: Vec<ModelField> = (0..6)
        .map(|index| ModelField {
            path: format!("{PREFIX}.equation_strings[{index}]").into(),
            value: ModelFieldValue::Text(
                ArchiveString::new(&index.to_string(), TextEncoding::Latin1).unwrap(),
            ),
        })
        .collect();
    for suffix in ["equation_word", "unknown_1e0"] {
        fields.push(ModelField {
            path: format!("{PREFIX}.{suffix}").into(),
            value: ModelFieldValue::I32(i32::MIN),
        });
    }
    fields
}

#[test]
fn same_scope_group_preserves_each_slot_and_full_signed_words() {
    let fields = saved_equation_group();
    let scope = EquationScope {
        prefix: PREFIX,
        range: 0..8,
    };
    let (first, group) = equation_group(&fields, &scope).unwrap();
    assert_eq!(first, 0);
    for (index, text) in equation_strings(group, PREFIX).unwrap().iter().enumerate() {
        assert_eq!(text.text(), index.to_string());
    }
    assert_eq!(
        equation_i32(&group[6], PREFIX, ".equation_word").unwrap(),
        i32::MIN
    );
    assert_eq!(
        equation_i32(&group[7], PREFIX, ".unknown_1e0").unwrap(),
        i32::MIN
    );
}

#[test]
fn group_rejects_every_incomplete_span_and_invalid_physical_extent() {
    let fields = saved_equation_group();
    let ranges = (0..8).map(|end| 0..end).chain([1..8, 0..9, 9..10]);
    for range in ranges {
        let scope = EquationScope {
            prefix: PREFIX,
            range,
        };
        assert!(equation_group(&fields, &scope).is_err());
    }
}

#[test]
fn string_group_rejects_foreign_paths_reordered_slots_and_wrong_types() {
    for index in 0..6 {
        let mut fields = saved_equation_group();
        fields[index].path = "other.splines[0].equation_strings[0]".into();
        assert!(equation_strings(fields.as_slice().try_into().unwrap(), PREFIX).is_err());
        fields[index].path = format!("{PREFIX}.equation_strings[{index}]").into();
        fields[index].value = ModelFieldValue::I32(0);
        assert!(equation_strings(fields.as_slice().try_into().unwrap(), PREFIX).is_err());
    }
    let mut fields = saved_equation_group();
    fields.swap(1, 2);
    assert!(equation_strings(fields.as_slice().try_into().unwrap(), PREFIX).is_err());
}

#[test]
fn signed_words_reject_cross_scope_paths_and_unsigned_storage() {
    for index in [6, 7] {
        let suffix = if index == 6 {
            ".equation_word"
        } else {
            ".unknown_1e0"
        };
        let mut fields = saved_equation_group();
        fields[index].path = format!("other{suffix}").into();
        assert!(equation_i32(&fields[index], PREFIX, suffix).is_err());
        fields[index].path = format!("{PREFIX}{suffix}").into();
        fields[index].value = ModelFieldValue::U32(0x80000000);
        assert!(equation_i32(&fields[index], PREFIX, suffix).is_err());
    }
}
