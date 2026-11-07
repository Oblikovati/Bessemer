//! Complete Source-frozen record11 fields, UTF16 units and explicit load-mode boundaries.
#[path = "flow_load_state/api.rs"]
mod api;
#[path = "flow_load_state/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_field_values/stored_fields.rs"]
mod stored_fields;
#[path = "flow_load_state/types.rs"]
mod types;
use packet::*;
use types::*;

#[test]
fn complete_load_state_payloads_rebuild_every_field_scope_count_and_stored_unit() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 356);
    for case in cases {
        check_complete(&case);
    }
}

fn check_complete(case: &LoadCase) {
    let rebuilt = rebuild(case);
    assert_eq!(rebuilt, case.raw, "{}", case.name);
    assert_eq!(case.end as usize, case.raw.len());
    assert_eq!(case.version, 18);
    assert_eq!(case.scopes[0].path, "[]");
    assert_eq!((case.scopes[0].start, case.scopes[0].end), (0, case.end));
    assert!(case.complete && !case.runtime && !case.engineering);
    for scope in &case.scopes {
        assert!(scope.start <= scope.end && scope.end <= case.end);
    }
    check_header(case);
    check_counts(case);
    check_texts(case);
    check_tail(case);
}

fn rebuild(case: &LoadCase) -> Vec<u8> {
    let mut rebuilt = Vec::new();
    for field in &case.fields {
        assert_eq!(
            field.offset as usize,
            rebuilt.len(),
            "{}:{}",
            case.name,
            field.path
        );
        assert_eq!(field.raw.len(), field.count as usize * field.width());
        rebuilt.extend_from_slice(&field.raw);
    }
    rebuilt
}

fn check_header(case: &LoadCase) {
    assert_eq!(
        u32::from_le_bytes(case.raw[4..8].try_into().unwrap()),
        case.count
    );
    assert_eq!(
        i32::from_le_bytes(case.raw[8..12].try_into().unwrap()),
        case.member
    );
    assert_eq!(
        u32::from_le_bytes(case.raw[12..16].try_into().unwrap()),
        case.width
    );
    assert_eq!(case.texts.len(), case.count as usize * 3);
}

fn check_counts(case: &LoadCase) {
    for &offset in &case.counts {
        let field = case
            .fields
            .iter()
            .find(|field| field.offset == offset && !field.raw.is_empty())
            .unwrap();
        assert_eq!((field.encoding, field.count), (1, 1));
        assert!(i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()) >= 0);
    }
}

fn check_texts(case: &LoadCase) {
    for text in &case.texts {
        check_text(case, text);
    }
}

fn check_text(case: &LoadCase, text: &LoadText) {
    assert_eq!(text.end - text.start, 4 + text.units * 2);
    let count = &case.raw[text.start as usize..text.start as usize + 4];
    assert_eq!(u32::from_le_bytes(count.try_into().unwrap()), text.units);
    let raw = &case.raw[text.start as usize + 4..text.end as usize];
    let (units, remainder) = raw.as_chunks::<2>();
    assert!(remainder.is_empty());
    let prefix = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(text.units as usize);
    assert_eq!(prefix, text.prefix_units as usize);
    assert_eq!(&raw[..prefix * 2], text.prefix_raw);
    let scope = case
        .scopes
        .iter()
        .find(|scope| scope.path == text.path)
        .unwrap();
    assert_eq!((scope.start, scope.end), (text.start, text.end));
}

fn check_tail(case: &LoadCase) {
    assert_eq!(case.mode_tail.is_some(), !case.load_mode && case.count > 0);
    if let Some(start) = case.mode_tail {
        assert_eq!(case.end - start, 192);
    }
    if case.count == 0 {
        assert_eq!(case.raw.len(), 16);
    }
}

#[test]
fn native_payload_and_derived_true_mode_prefix_keep_distinct_origins_and_complete_extents() {
    let cases = fixture::cases();
    let native = &cases[0];
    let derived = &cases[1];
    check_origin(native, (1, false, 7, 40, 11474));
    check_origin(derived, (2, true, 7, 40, 11282));
    assert_eq!(&native.raw[..11282], derived.raw);
    assert_eq!(native.mode_tail, Some(11282));
    assert_eq!(derived.mode_tail, None);
    check_complete(native);
    check_complete(derived);
    for case in &cases[2..] {
        assert_eq!(case.origin, 0);
    }
}

fn check_origin(case: &LoadCase, expected: (u8, bool, u32, u32, u32)) {
    assert_eq!(
        (
            case.origin,
            case.load_mode,
            case.count,
            case.width,
            case.end
        ),
        expected
    );
}

#[test]
fn complete_raw_utf16_corpus_keeps_all_units_surrogates_empty_nonterminated_and_nul_hidden_suffixes()
 {
    let cases = fixture::cases();
    let mut units = std::collections::BTreeSet::new();
    for field in cases
        .iter()
        .flat_map(|case| &case.fields)
        .filter(|field| field.encoding == 4)
    {
        units.extend(
            field
                .raw
                .as_chunks::<2>()
                .0
                .iter()
                .map(|raw| u16::from_le_bytes(*raw)),
        );
    }
    assert_eq!(units, (0..=u16::MAX).collect());
    check_text_variants(&cases);
}

#[test]
fn zero_width_present_arrays_remain_distinct_from_empty_record_omissions_and_unsigned_numeric_bits()
{
    let cases = fixture::cases();
    let empty = cases
        .iter()
        .find(|case| case.count == 0 && case.width == i32::MAX as u32)
        .unwrap();
    assert_eq!(empty.fields.len(), 4);
    assert_eq!(empty.scopes.len(), 1);
    assert!(empty.texts.is_empty());
    let present = cases
        .iter()
        .find(|case| case.count > 0 && case.width == 0)
        .unwrap();
    let arrays: Vec<_> = present
        .fields
        .iter()
        .filter(|field| field.path.starts_with("[\"shared_arrays\","))
        .collect();
    assert_eq!(arrays.len(), 4);
    assert!(arrays.iter().all(|field| field.count == 0));
    check_numeric_patterns(&cases);
}

fn check_numeric_patterns(cases: &[LoadCase]) {
    let words = stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), 2);
    let wide = stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), 3);
    for bit in 0..32 {
        assert!(
            words.contains(&u64::from(1u32 << bit)) && words.contains(&u64::from(!(1u32 << bit)))
        );
    }
    for bit in 0..64 {
        assert!(wide.contains(&(1u64 << bit)) && wide.contains(&!(1u64 << bit)));
    }
    for special in [
        0,
        0x8000000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
    ] {
        assert!(wide.contains(&special));
    }
}

fn check_text_variants(cases: &[LoadCase]) {
    let texts: Vec<_> = cases.iter().flat_map(|case| &case.texts).collect();
    assert!(texts.iter().any(|text| text.units == 0));
    assert!(
        texts
            .iter()
            .any(|text| text.units > 0 && text.prefix_units == text.units)
    );
    assert!(texts.iter().any(|text| text.units > text.prefix_units + 1));
    assert!(texts.iter().any(|text| text.prefix_raw == [0, 0xd8]));
}
