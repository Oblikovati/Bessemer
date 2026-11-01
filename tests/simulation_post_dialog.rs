//! Complete parent dialog fields against frozen independent owned/native helper expectations (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared projection support also serves other complete persistence families."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::*;
#[path = "simulation_post_dialog/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[allow(
    dead_code,
    reason = "The nested history adapter is shared with other persistence families."
)]
#[path = "support/binary_load_history.rs"]
mod load_history;

const LIMITS: SimulationPostDialogLimits = SimulationPostDialogLimits {
    maximum_bytes: 10000000,
    maximum_fields: 200000,
    maximum_entries: 10000000,
    maximum_text_bytes: 0x200000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 1000000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};

fn encoded_value(value: &SimulationPostDialogValue) -> (usize, Vec<u8>) {
    match value {
        SimulationPostDialogValue::Signed(value) => (0, value.to_le_bytes().to_vec()),
        SimulationPostDialogValue::RawWordBits(value) => (7, value.to_le_bytes().to_vec()),
        SimulationPostDialogValue::Float32Bits(value) => (8, value.to_le_bytes().to_vec()),
        SimulationPostDialogValue::Raw64(raw) => (9, raw.to_vec()),
        SimulationPostDialogValue::PersistText(text) => {
            (3, [text.header(), text.payload()].concat())
        }
        SimulationPostDialogValue::CharacterText(text) => (6, text.to_bytes()),
        SimulationPostDialogValue::CountedBytes(frame) => (10, frame.to_bytes()),
        SimulationPostDialogValue::Embedded(archive) => (5, archive.to_bytes().unwrap()),
    }
}

fn project_dialog(body: &SimulationPostDialog) -> Vec<u8> {
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

fn dialogs_from_projection(mut cursor: &[u8], expected_count: usize) -> Vec<SimulationPostDialog> {
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        let family = read_word(&mut cursor);
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        let fields = cursor;
        consume_expected_fields(&mut cursor);
        if family != 0 {
            continue;
        }
        let expected = &fields[..fields.len() - cursor.len()];
        let body = assert_projected_dialog(raw, version, expected);
        cases.push(body);
    }
    assert_eq!(cases.len(), expected_count);
    cases
}

fn owned_dialogs() -> Vec<SimulationPostDialog> {
    dialogs_from_projection(
        include_bytes!("fixtures/simulation-post-dialog/projection.bin"),
        225,
    )
}

#[test]
fn complete_owned_parent_fields_and_construction_match_independent_expectations() {
    for body in owned_dialogs() {
        let built =
            SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), LIMITS)
                .unwrap();
        assert_eq!(built.fields(), body.fields());
        assert_eq!(built.field_offsets(), body.field_offsets());
        assert_eq!(built.to_bytes(), body.to_bytes());
        assert_ne!(built.field_id(0).unwrap(), body.field_id(0).unwrap());
        assert_eq!(body.coverage().checked_bytes, body.to_bytes().len());
        assert_eq!(body.coverage().checked_fields, body.fields().len());
        assert_eq!(
            body.coverage().completion,
            SimulationPostDialogCompletion::CompletedBody
        );
    }
}

#[test]
#[ignore = "Requires fresh complete dialog native field packets from freeze_post_dialog.py."]
fn complete_native_parent_fields_match_independent_expectations() {
    let packet = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    dialogs_from_projection(&packet, 224);
}

#[test]
#[ignore = "Requires fresh expanded legacy dialog packets from freeze_post_dialog.py."]
fn expanded_legacy_failures_remain_unresolved_under_their_retained_versions() {
    let packet = std::fs::read("target/simulation-post-dialog-additional-legacy.bin").unwrap();
    let mut cursor = packet.as_slice();
    let mut count = 0;
    while !cursor.is_empty() {
        assert_eq!(read_word(&mut cursor), 3);
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        assert_eq!(read_word(&mut cursor), 0);
        assert_eq!(read_word(&mut cursor), 0);
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        assert!(SimulationPostDialog::from_bytes(raw, version, LIMITS).is_err());
        count += 1;
    }
    assert_eq!(count, 251);
}

#[test]
fn published_parent_parts_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostDialog>();
    assert_threads::<SimulationPostDialogField>();
    assert_threads::<SimulationPostDialogValue>();
    assert_threads::<SimulationPostDialogText>();
    assert_threads::<SimulationPostDialogBytes>();
    assert_threads::<SimulationPostDialogLimits>();
    assert_threads::<SimulationPostDialogEdit>();
    assert_threads::<SimulationPostDialogCoverage>();
    assert_threads::<SimulationPostDialogFieldId>();
    assert_threads::<SimulationPostDialogCompletion>();
    assert_threads::<SimulationPostDialogCheckMode>();
    assert_threads::<SimulationPostDialogDiagnostic>();
    assert_threads::<SimulationPostDialogHistory>();
    assert_threads::<SimulationPostDialogHistoryParent>();
    assert_threads::<SimulationPostDialogNestedHistory>();
    assert_threads::<SimulationPostDialogNestedParent>();
    assert_threads::<SimulationPostDialogRelation>();
}

#[path = "simulation_post_dialog/edit_matrix.rs"]
mod edit_matrix;
#[allow(
    dead_code,
    reason = "Shared history assertions also serve other persistence families."
)]
#[path = "support/typed_load_history.rs"]
mod typed_history;
#[allow(
    dead_code,
    reason = "Shared span arithmetic also serves other persistence families."
)]
#[path = "support/typed_load_values.rs"]
mod typed_values;

#[path = "simulation_post_dialog/contracts.rs"]
mod contracts;

fn assert_projected_dialog(
    raw: &[u8],
    version: SimulationPersistVersion,
    expected: &[u8],
) -> SimulationPostDialog {
    let body = SimulationPostDialog::from_bytes(raw, version, LIMITS).unwrap();
    assert_eq!(project_dialog(&body), expected, "{version:?}");
    assert_eq!(body.to_bytes(), raw);
    body
}

#[path = "simulation_post_dialog/persist_text.rs"]
mod persist_text;

#[path = "simulation_post_dialog/budgets.rs"]
mod budgets;

#[path = "simulation_post_dialog/concurrency.rs"]
mod concurrency;

#[path = "simulation_post_dialog/extents.rs"]
mod extents;

#[path = "simulation_post_dialog/character_caps.rs"]
mod character_caps;

#[path = "simulation_post_dialog/embedded_budgets.rs"]
mod embedded_budgets;

#[path = "simulation_post_dialog/source_budgets.rs"]
mod source_budgets;

#[path = "simulation_post_dialog/byte_caps.rs"]
mod byte_caps;

#[path = "simulation_post_dialog/count_caps.rs"]
mod count_caps;

#[path = "simulation_post_dialog/process_exports.rs"]
mod process_exports;

#[path = "simulation_post_dialog/reference_caps.rs"]
mod reference_caps;

#[path = "simulation_post_dialog/embedded_boundaries.rs"]
mod embedded_boundaries;
