use super::super::annotation_view_tests::LIMITS;
use super::super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;
#[path = "../support/component_legacy_cases.rs"]
mod cases;

fn decode_legacy(
    case: &cases::ComponentLegacyCase<'_>,
    raw: &[u8],
    limits: ModelFieldLimits,
) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        raw,
        ArchiveVersion::new(case.version),
        limits,
        super::super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    if case.presence != 0 && case.kind > 0 {
        reader.object("following_alias", &mut map, ModelObjectRole::Any)?;
    }
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn all_bare_legacy_profiles_preserve_exact_bytes_values_and_alias_map_slots() {
    let mut checked = 0;
    for case in cases::component_legacy_cases()
        .iter()
        .filter(|case| case.context == 0)
    {
        let (fields, slots) = decode_legacy(case, case.raw, LIMITS).unwrap();
        let expected = if case.presence != 0 && case.kind == 2 {
            5
        } else {
            3
        };
        assert_eq!(slots, expected);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), case.raw);
        assert_legacy_scalars(&fields, case);
        checked += 1;
    }
    assert_eq!(checked, 270);
}

fn assert_legacy_scalars(fields: &[ModelField], case: &cases::ComponentLegacyCase<'_>) {
    for (suffix, value) in [
        (".unknown_84", ModelFieldValue::I32(case.presence)),
        (".unknown_a0", ModelFieldValue::I32(-19)),
    ] {
        let field = fields
            .iter()
            .find(|field| field.path.ends_with(suffix))
            .unwrap();
        assert_eq!(field.value, value);
    }
    let time = fields
        .iter()
        .find(|field| field.path.ends_with(".legacy_reference.time_word"));
    if case.presence == 0 {
        assert!(time.is_none());
        return;
    }
    let expected = [0, 0x80000000, u32::MAX][case.kind] ^ u32::from(case.edited);
    assert_eq!(time.unwrap().value, ModelFieldValue::U32(expected));
}

#[test]
fn every_bare_profile_prefix_and_appended_byte_fails_without_partial_output() {
    for case in cases::component_legacy_cases()
        .iter()
        .filter(|case| case.context == 0)
    {
        for end in 0..case.raw.len() {
            assert!(decode_legacy(case, &case.raw[..end], LIMITS).is_err());
        }
        assert!(decode_legacy(case, &[case.raw, &[0]].concat(), LIMITS).is_err());
        assert!(decode_legacy(case, case.raw, LIMITS).is_ok());
    }
}

#[test]
fn fresh_targets_charge_one_object_and_obey_exact_field_budgets() {
    let case = fresh_legacy_case();
    let (fields, _) = decode_legacy(&case, case.raw, LIMITS).unwrap();
    let exact = ModelFieldLimits {
        maximum_objects: 2,
        maximum_fields: fields.len(),
        ..LIMITS
    };
    assert_eq!(decode_legacy(&case, case.raw, exact).unwrap().0, fields);
    let objects = ModelFieldLimits {
        maximum_objects: 1,
        ..exact
    };
    let field_cap = ModelFieldLimits {
        maximum_fields: fields.len() - 1,
        ..exact
    };
    assert!(decode_legacy(&case, case.raw, objects).is_err());
    assert!(decode_legacy(&case, case.raw, field_cap).is_err());
}

fn fresh_legacy_case() -> cases::ComponentLegacyCase<'static> {
    cases::component_legacy_cases()
        .into_iter()
        .find(|case| case.context == 0 && case.presence != 0 && case.kind == 2)
        .unwrap()
}
