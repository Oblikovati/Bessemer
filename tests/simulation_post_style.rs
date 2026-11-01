//! Complete fixed-slot PostStyle read/build/edit against independent dialog helper expectations (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared projection support also serves families with nested archives."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::*;
#[path = "simulation_post_style/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;

const LIMITS: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/projection.bin");

#[path = "support/post_color_views.rs"]
mod color_expectations;
#[path = "simulation_post_style/colors.rs"]
mod colors;
#[path = "simulation_post_style/contracts.rs"]
mod contracts;
#[path = "simulation_post_style/text_limits.rs"]
mod text_limits;

fn encoded_value(value: &SimulationPostStyleValue) -> (usize, Vec<u8>) {
    match value {
        SimulationPostStyleValue::Signed(value) => (0, value.to_le_bytes().to_vec()),
        SimulationPostStyleValue::RawWordBits(value) => (7, value.to_le_bytes().to_vec()),
        SimulationPostStyleValue::Float32Bits(value) => (8, value.to_le_bytes().to_vec()),
        SimulationPostStyleValue::Text(text) => (6, text.to_bytes()),
    }
}

fn project_style(body: &SimulationPostStyle) -> Vec<u8> {
    let mut output = Vec::new();
    project_word(&mut output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        let (tag, raw) = encoded_value(&field.value);
        project_word(&mut output, field.path.len());
        output.extend_from_slice(field.path.as_bytes());
        project_word(&mut output, tag);
        project_word(&mut output, span.0);
        project_word(&mut output, span.1);
        assert_eq!(span.1 - span.0, raw.len());
        output.extend(raw);
    }
    output
}

fn styles_from_projection(mut cursor: &[u8], expected_count: usize) -> Vec<SimulationPostStyle> {
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        let family = read_word(&mut cursor);
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        let fields = cursor;
        consume_expected_fields(&mut cursor);
        if family != 2 {
            continue;
        }
        let body = SimulationPostStyle::from_bytes(raw, version, LIMITS).unwrap();
        assert_eq!(project_style(&body), fields[..fields.len() - cursor.len()]);
        assert_eq!(body.to_bytes(), raw);
        cases.push(body);
    }
    assert_eq!(cases.len(), expected_count);
    cases
}

fn owned_styles() -> Vec<SimulationPostStyle> {
    styles_from_projection(EXPECTED, 200)
}

#[test]
fn owned_complete_fields_builds_and_exact_coverage() {
    for body in owned_styles() {
        let built =
            SimulationPostStyle::from_fields(body.fields().to_vec(), body.version(), LIMITS)
                .unwrap();
        assert_eq!(built.fields(), body.fields());
        assert_eq!(built.to_bytes(), body.to_bytes());
        assert_ne!(built.field_id(0).unwrap(), body.field_id(0).unwrap());
        assert_style_coverage(&body);
    }
}

#[test]
fn every_owned_proper_prefix_and_trailing_byte_is_rejected() {
    for body in owned_styles() {
        let mut raw = body.to_bytes();
        for end in 0..raw.len() {
            assert!(
                SimulationPostStyle::from_bytes(&raw[..end], body.version(), LIMITS).is_err(),
                "{end}"
            );
        }
        raw.push(0);
        assert!(SimulationPostStyle::from_bytes(&raw, body.version(), LIMITS).is_err());
    }
}

#[test]
fn all_public_style_types_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostStyle>();
    assert_threads::<SimulationPostStyleField>();
    assert_threads::<SimulationPostStyleValue>();
    assert_threads::<SimulationPostStyleText>();
    assert_threads::<SimulationPostStyleLimits>();
    assert_threads::<SimulationPostStyleEdit>();
    assert_threads::<SimulationPostStyleCoverage>();
    assert_threads::<SimulationPostStyleFieldId>();
}

#[test]
#[ignore = "Requires fresh complete native dialog packets from freeze_post_dialog.py."]
fn native_complete_style_fields_match_every_successful_helper() {
    let packet = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    styles_from_projection(&packet, 52);
}

#[test]
#[ignore = "Requires fresh complete native dialog packets from freeze_post_dialog.py."]
fn native_all_style_attempts_match_reference_acceptance() {
    let packet = std::fs::read("target/simulation-post-dialog-native.bin").unwrap();
    let mut cursor = packet.as_slice();
    let mut counts = [0; 2];
    while !cursor.is_empty() {
        let family = read_word(&mut cursor);
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let accepted = read_word(&mut cursor);
        read_word(&mut cursor);
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        if family != 2 {
            continue;
        }
        assert_native_style(raw, version, accepted);
        counts[accepted as usize] += 1;
    }
    assert_eq!(counts, [68, 52]);
}

fn assert_style_coverage(body: &SimulationPostStyle) {
    let coverage = body.coverage();
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(
        coverage.completion,
        SimulationPostStyleCompletion::CompletedBody
    );
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.embedded_archives, 0);
    assert_eq!(coverage.declared_entries, style_character_count(body));
}

fn style_character_count(body: &SimulationPostStyle) -> usize {
    body.fields()
        .iter()
        .map(|field| match &field.value {
            SimulationPostStyleValue::Text(text) => text.character_count(),
            _ => 0,
        })
        .sum::<usize>()
}

fn assert_native_style(raw: &[u8], version: SimulationPersistVersion, accepted: u32) {
    let parsed = SimulationPostStyle::from_bytes(raw, version, LIMITS);
    assert_eq!(parsed.is_ok(), accepted == 1, "{version:?}");
    if let Ok(body) = parsed {
        assert_eq!(body.to_bytes(), raw);
    }
}
