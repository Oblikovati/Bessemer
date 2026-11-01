//! Complete fixed-slot PostInfo read/build/edit against independent dialog helper expectations (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared projection support also serves families with nested archives."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::*;
#[path = "simulation_post_info/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;

const LIMITS: SimulationPostInfoLimits = SimulationPostInfoLimits {
    maximum_bytes: 3000000,
    maximum_fields: 117,
    maximum_entries: 0x100000,
    maximum_text_bytes: 0x200000,
    maximum_characters: 0x100000,
};
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/projection.bin");

#[path = "simulation_post_info/contracts.rs"]
mod contracts;
#[path = "simulation_post_info/text_limits.rs"]
mod text_limits;

fn encoded_value(value: &SimulationPostInfoValue) -> (usize, Vec<u8>) {
    match value {
        SimulationPostInfoValue::Signed(value) => (0, value.to_le_bytes().to_vec()),
        SimulationPostInfoValue::RawWordBits(value) => (7, value.to_le_bytes().to_vec()),
        SimulationPostInfoValue::Float32Bits(value) => (8, value.to_le_bytes().to_vec()),
        SimulationPostInfoValue::Text(text) => (6, text.to_bytes()),
    }
}

fn project_info(body: &SimulationPostInfo) -> Vec<u8> {
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

fn infos_from_projection(mut cursor: &[u8], expected_count: usize) -> Vec<SimulationPostInfo> {
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        let family = read_word(&mut cursor);
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        let fields = cursor;
        consume_expected_fields(&mut cursor);
        if family != 1 {
            continue;
        }
        let body = SimulationPostInfo::from_bytes(raw, version, LIMITS).unwrap();
        assert_eq!(project_info(&body), fields[..fields.len() - cursor.len()]);
        assert_eq!(body.to_bytes(), raw);
        cases.push(body);
    }
    assert_eq!(cases.len(), expected_count);
    cases
}

fn owned_infos() -> Vec<SimulationPostInfo> {
    infos_from_projection(EXPECTED, 70)
}

#[test]
fn owned_complete_fields_builds_and_exact_coverage() {
    for body in owned_infos() {
        let built = SimulationPostInfo::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
        assert_eq!(built.fields(), body.fields());
        assert_eq!(built.to_bytes(), body.to_bytes());
        assert_ne!(built.field_id(0).unwrap(), body.field_id(0).unwrap());
        let coverage = body.coverage();
        assert_eq!(coverage.checked_bytes, body.to_bytes().len());
        assert_eq!(coverage.checked_fields, body.fields().len());
        assert_eq!(
            coverage.completion,
            SimulationPostInfoCompletion::CompletedBody
        );
        assert_eq!(coverage.requested_pattern_error_bits, 0);
        assert_eq!(coverage.body_count, 1);
        assert_eq!(coverage.embedded_archives, 0);
        assert_eq!(
            coverage.declared_entries,
            if body.version().value() == 700002 {
                body.to_bytes()[468..].len() / 2
            } else {
                0
            }
        );
    }
}

#[test]
fn every_owned_proper_prefix_and_trailing_byte_is_rejected() {
    for body in owned_infos() {
        let mut raw = body.to_bytes();
        for end in 0..raw.len() {
            assert!(
                SimulationPostInfo::from_bytes(&raw[..end], body.version(), LIMITS).is_err(),
                "{end}"
            );
        }
        raw.push(0);
        assert!(SimulationPostInfo::from_bytes(&raw, body.version(), LIMITS).is_err());
    }
}

#[test]
fn all_public_info_types_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostInfo>();
    assert_threads::<SimulationPostInfoField>();
    assert_threads::<SimulationPostInfoValue>();
    assert_threads::<SimulationPostInfoText>();
    assert_threads::<SimulationPostInfoLimits>();
    assert_threads::<SimulationPostInfoEdit>();
    assert_threads::<SimulationPostInfoCoverage>();
    assert_threads::<SimulationPostInfoFieldId>();
}

#[test]
#[ignore = "Requires fresh complete native dialog packets from freeze_post_dialog.py."]
fn native_complete_info_fields_match_every_successful_helper() {
    let packet = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    infos_from_projection(&packet, 120);
}

#[test]
#[ignore = "Requires fresh complete native dialog packets from freeze_post_dialog.py."]
fn native_all_info_attempts_match_reference_acceptance() {
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
        if family != 1 {
            continue;
        }
        let parsed = SimulationPostInfo::from_bytes(raw, version, LIMITS);
        assert_eq!(parsed.is_ok(), accepted == 1, "{version:?}");
        counts[accepted as usize] += 1;
        if let Ok(body) = parsed {
            assert_eq!(body.to_bytes(), raw);
        }
    }
    assert_eq!(counts, [0, 120]);
}
