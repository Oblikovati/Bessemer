//! Complete owned message-list framing and modern/legacy string transfer oracles.
#[path = "flow_message_lists/api.rs"]
mod api;
#[path = "flow_message_lists/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
#[path = "flow_field_values/stored_fields.rs"]
mod stored_fields;
use fixture::{MessageCase, MessageText};
use packet::*;
use std::collections::BTreeSet;

#[test]
fn every_message_payload_rebuilds_all_source_fields_scopes_counts_and_version_boundaries() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 992);
    for case in cases {
        check_complete(&case);
        check_scopes(&case);
        check_counts(&case);
        assert_eq!(
            i32::from_le_bytes(case.raw[..4].try_into().unwrap()),
            case.version
        );
        assert_eq!(
            u32::from_le_bytes(case.raw[4..8].try_into().unwrap()),
            case.items
        );
        if case.version != 1 {
            assert_eq!(case.mode, Some(0));
        }
        assert!(case.complete && !case.interpretation && !case.engineering);
    }
}
fn check_complete(case: &MessageCase) {
    let mut raw = Vec::new();
    for field in &case.fields {
        assert_eq!(
            raw.len(),
            field.offset as usize,
            "{}:{}",
            case.name,
            field.path
        );
        assert_eq!(field.raw.len(), field.count as usize * field.width());
        raw.extend_from_slice(&field.raw);
    }
    assert_eq!(raw, case.raw, "{}", case.name);
}
fn check_scopes(case: &MessageCase) {
    assert_eq!(case.scopes[0], ("[]".into(), 0, case.raw.len() as u32));
    let paths: BTreeSet<_> = case.scopes.iter().map(|(path, _, _)| path).collect();
    assert_eq!(paths.len(), case.scopes.len());
    for (_, start, end) in &case.scopes {
        assert!(start <= end && *end as usize <= case.raw.len());
    }
    let second = case
        .scopes
        .iter()
        .filter(|(path, _, _)| path.ends_with("\"second_strings\"]"))
        .count();
    assert_eq!(
        second,
        if case.version == 3 {
            case.items as usize
        } else {
            0
        }
    );
}
fn check_counts(case: &MessageCase) {
    let unique: BTreeSet<_> = case.counts.iter().collect();
    assert_eq!(unique.len(), case.counts.len());
    for offset in &case.counts {
        let field = case
            .fields
            .iter()
            .find(|field| field.offset == *offset && field.count > 0)
            .unwrap();
        assert_eq!((field.encoding, field.count), (1, 1));
        assert!(i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()) >= 0);
    }
}
#[test]
fn every_text_retains_exact_transfer_length_optional_projection_and_source_scope() {
    for case in fixture::cases() {
        for text in &case.texts {
            check_text(&case, text);
        }
    }
}
fn check_text(case: &MessageCase, text: &MessageText) {
    assert_eq!(text.legacy, case.version == 1);
    let scope = case
        .scopes
        .iter()
        .find(|(path, _, _)| path == &text.path)
        .unwrap();
    assert_eq!((scope.1, scope.2), (text.start, text.end));
    let field = text_field(case, text);
    assert_eq!((field.offset, field.count), (text.start + 4, text.count));
    assert_eq!(
        text.end - text.start,
        4 + text.count * if text.legacy { 1 } else { 2 }
    );
    if text.legacy {
        assert_eq!(field.encoding, 0);
        assert!(text.prefix.is_none());
    } else {
        check_modern_prefix(text, field);
    }
}
fn text_field<'a>(case: &'a MessageCase, text: &MessageText) -> &'a stored_fields::StoredField {
    let role = if text.legacy {
        "raw_bytes"
    } else {
        "raw_units"
    };
    let path = format!("{},\"{role}\"]", text.path.strip_suffix(']').unwrap());
    case.fields.iter().find(|field| field.path == path).unwrap()
}
fn check_modern_prefix(text: &MessageText, field: &stored_fields::StoredField) {
    assert_eq!(field.encoding, 4);
    let (units, remainder) = field.raw.as_chunks::<2>();
    assert!(remainder.is_empty());
    assert!(units.is_empty() || units.last() == Some(&[0, 0]));
    let prefix = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len());
    assert_eq!(text.prefix.as_deref(), Some(&field.raw[..prefix * 2]));
}
#[test]
fn every_utf16_unit_legacy_byte_and_distinct_normalization_form_has_owned_coverage() {
    let cases = fixture::cases();
    let units = utf16_units(&cases);
    assert_eq!(units, (0..=u16::MAX).collect());
    let bytes: BTreeSet<_> = cases
        .iter()
        .flat_map(|case| &case.fields)
        .filter(|field| field.encoding == 0)
        .flat_map(|field| field.raw.iter().copied())
        .collect();
    assert_eq!(bytes, (0..=u8::MAX).collect());
    check_representations(&cases);
}
fn check_representations(cases: &[MessageCase]) {
    let nfc = cases.iter().find(|case| case.name == "nfc-v0").unwrap();
    let nfd = cases.iter().find(|case| case.name == "nfd-v0").unwrap();
    assert_ne!(nfc.raw, nfd.raw);
    let odd = cases
        .iter()
        .find(|case| case.name == "legacy-length3")
        .unwrap();
    assert!(
        odd.texts
            .iter()
            .any(|text| text.count == 3 && text.prefix.is_none())
    );
}
fn utf16_units(cases: &[MessageCase]) -> BTreeSet<u16> {
    cases
        .iter()
        .flat_map(|case| &case.fields)
        .filter(|field| field.encoding == 4)
        .flat_map(|field| field.raw.as_chunks::<2>().0.iter())
        .map(|unit| u16::from_le_bytes(*unit))
        .collect()
}
#[test]
fn member_word_patterns_keep_every_single_bit_and_complement_without_enum_projection() {
    let cases = fixture::cases();
    let patterns = stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), 2);
    for bit in 0..32 {
        assert!(patterns.contains(&u64::from(1u32 << bit)));
        assert!(patterns.contains(&u64::from(!(1u32 << bit))));
    }
}
#[test]
fn native_message_and_independent_lists_preserve_the_complete_original_payload() {
    let cases = fixture::cases();
    let native: Vec<_> = cases.iter().filter(|case| case.native).collect();
    assert_eq!(native.len(), 1);
    let case = native[0];
    assert_eq!(
        (case.version, case.mode, case.items, case.raw.len()),
        (3, Some(0), 1, 328)
    );
    check_native_fields(case);
    assert_eq!(case.texts.len(), 3);
    check_complete(case);
}
fn check_native_fields(case: &MessageCase) {
    let member = case
        .fields
        .iter()
        .find(|field| field.path == "[\"items\",0,\"member_8_pattern\"]")
        .unwrap();
    assert_eq!(
        u32::from_le_bytes(member.raw.as_slice().try_into().unwrap()),
        4
    );
    assert_eq!(
        native_text(&case.texts[0]),
        "A vortex crosses the pressure opening"
    );
}
fn native_text(text: &MessageText) -> String {
    let raw = text.prefix.as_ref().unwrap();
    let units: Vec<_> = raw
        .as_chunks::<2>()
        .0
        .iter()
        .map(|unit| u16::from_le_bytes(*unit))
        .collect();
    String::from_utf16(&units).unwrap()
}
