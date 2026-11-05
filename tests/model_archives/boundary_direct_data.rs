use super::super::annotation_view_tests::{LIMITS, decode};
use super::super::{ModelField, ModelFieldValue, encode_fields};
#[path = "../support/boundary_direct_cases.rs"]
mod cases;

const SCENARIOS: [[i32; 4]; 10] = [
    [0, 0, 0, 0],
    [i32::MIN, 0, 0, 0],
    [-1, 0, 0, 0],
    [2, 0, 0, 0],
    [2, 1, 2, 3],
    [1, -1, -1, 3],
    [-1, i32::MIN, 2, i32::MIN],
    [i32::MIN, i32::MAX, i32::MIN, -1],
    [0, 1, 0, 0],
    [2, 255, 2, 3],
];

fn boundary_field<'a>(fields: &'a [ModelField], suffix: &str) -> Option<&'a ModelFieldValue> {
    fields
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .map(|field| &field.value)
}

#[test]
fn all_bare_direct_boundaries_match_golden_values_and_exact_gate_bytes() {
    let mut checked = 0;
    for case in cases::boundary_direct_cases()
        .iter()
        .filter(|case| case.context == 0)
    {
        let fields = decode(case.version, case.raw).unwrap();
        assert_eq!(encode_fields(&fields, 30000).unwrap(), case.raw);
        assert_boundary_values(&fields, case);
        checked += 1;
    }
    assert_eq!(checked, 78);
}

fn assert_boundary_scalar(fields: &[ModelField], suffix: &str, expected: ModelFieldValue) {
    assert_eq!(boundary_field(fields, suffix), Some(&expected));
}

fn assert_boundary_values(fields: &[ModelField], case: &cases::BoundaryDirectCase<'_>) {
    let [temporary, presence, pairs, values] = SCENARIOS[case.style];
    assert_boundary_scalar(fields, ".temporary_count", ModelFieldValue::I32(temporary));
    assert_boundary_times(fields, case, temporary, presence);
    if case.version <= 0x332c {
        assert_eq!(boundary_field(fields, ".has_direct_model_data"), None);
        return;
    }
    assert_boundary_scalar(fields, ".unknown_ec", ModelFieldValue::I32(-1));
    assert_boundary_scalar(
        fields,
        ".has_direct_model_data",
        ModelFieldValue::I32(presence),
    );
    if presence != 0 {
        assert_downstream_values(fields, case, pairs, values);
    }
}

fn assert_boundary_times(
    fields: &[ModelField],
    case: &cases::BoundaryDirectCase<'_>,
    temporary: i32,
    presence: i32,
) {
    if temporary <= 0 {
        return;
    }
    let first = if case.edited && presence == 0 {
        0x11223344
    } else {
        u32::MAX
    };
    for (name, value) in [("time_0", first), ("time_8", 0x80000000)] {
        let suffix = format!(".temporary_records[0].{name}");
        assert_boundary_scalar(fields, &suffix, ModelFieldValue::U32(value));
    }
}

fn assert_direct_value(fields: &[ModelField], suffix: &str, value: i32) {
    let suffix = format!(".direct_model_data.{suffix}");
    assert_boundary_scalar(fields, &suffix, ModelFieldValue::I32(value));
}

fn assert_downstream_values(
    fields: &[ModelField],
    case: &cases::BoundaryDirectCase<'_>,
    pairs: i32,
    values: i32,
) {
    assert_direct_value(fields, "pair_count", pairs);
    assert_direct_value(fields, "value_count", values);
    let expected = if case.edited { 42 } else { -17 };
    assert_direct_value(fields, "unknown_38", expected);
    if pairs > 0 {
        assert_direct_value(fields, "pairs[0].key", -7);
        assert_direct_value(fields, "pairs[1].key", -7);
    }
    if values > 0 {
        assert_direct_value(fields, "values[0]", 0);
        assert_direct_value(fields, "values[2]", 0);
    }
    let tail = boundary_field(fields, ".direct_model_data.values_40[0]");
    assert_eq!(tail.is_some(), case.version >= 0x3359);
}

#[test]
fn bare_boundaries_reject_all_proper_prefixes_extra_bytes_and_unsupported_contexts() {
    for case in cases::boundary_direct_cases()
        .iter()
        .filter(|case| case.context == 0)
    {
        for end in 0..case.raw.len() {
            assert!(
                decode(case.version, &case.raw[..end]).is_err(),
                "{} prefix {end}",
                case.version
            );
        }
        assert!(decode(case.version, &[case.raw, &[0]].concat()).is_err());
        assert!(decode(0x27cb, case.raw).is_err());
        assert!(decode(case.version, case.raw).is_ok());
    }
}

#[test]
fn direct_boundary_fields_and_positive_entries_obey_explicit_resource_caps() {
    let case = cases::boundary_direct_cases()
        .into_iter()
        .find(|case| case.context == 0 && case.version == 19000 && case.style == 4)
        .unwrap();
    let fields = decode(case.version, case.raw).unwrap();
    assert_boundary_field_budget(case.version, case.raw, fields.len());
    let limits = super::super::ModelFieldLimits {
        maximum_entries: 0,
        ..LIMITS
    };
    assert!(decode_boundary_with_limits(case.version, case.raw, limits).is_err());
}

fn assert_boundary_field_budget(version: u32, raw: &[u8], count: usize) {
    let exact = super::super::ModelFieldLimits {
        maximum_fields: count,
        ..LIMITS
    };
    assert_eq!(
        decode_boundary_with_limits(version, raw, exact)
            .unwrap()
            .len(),
        count
    );
    let below = super::super::ModelFieldLimits {
        maximum_fields: count - 1,
        ..exact
    };
    assert!(decode_boundary_with_limits(version, raw, below).is_err());
}

fn decode_boundary_with_limits(
    version: u32,
    raw: &[u8],
    limits: super::super::ModelFieldLimits,
) -> crate::Result<Vec<ModelField>> {
    use super::super::{ModelDecoder, ModelObjectRole};
    use crate::archive_codec::ObjectReader;
    let mut reader = ModelDecoder::new_in_context(
        raw,
        crate::header::ArchiveVersion::new(version),
        limits,
        super::super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.object("object", &mut ObjectReader::new(), ModelObjectRole::Any)?;
    reader.finish()
}
