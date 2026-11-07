//! Complete owned embedded wrapper operations in the explicit Simulation context (#134).
use bessemer::display_lists::{PerspectiveBase, PerspectivePayload};
use bessemer::model_archives::{
    CompactTransform, ModelArchiveLimits, ModelEditCompletion, ModelFieldValue,
    SimulationEmbeddedArchive, SimulationEmbeddedEdit, SimulationEmbeddedProfile, StoredDouble,
};

const FIXTURE: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/embedded.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 10000,
    maximum_fields: 1000,
    maximum_entries: 1000,
    maximum_depth: 32,
    maximum_objects: 100,
};

fn fixture_word(cursor: &mut &[u8]) -> usize {
    let (word, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(word.try_into().unwrap()) as usize
}

fn next_embedded(cursor: &mut &[u8]) -> (&'static [u8], usize, usize) {
    let size = fixture_word(cursor);
    let wrapper_end = fixture_word(cursor);
    let first_end = fixture_word(cursor);
    let offset = FIXTURE.len() - cursor.len();
    *cursor = &cursor[size..];
    (&FIXTURE[offset..offset + size], wrapper_end, first_end)
}

fn read_embedded(raw: &[u8]) -> SimulationEmbeddedArchive {
    SimulationEmbeddedArchive::from_bytes(raw, SimulationEmbeddedProfile::Modern, LIMITS).unwrap()
}

fn first_embedded() -> &'static [u8] {
    let mut cursor = FIXTURE;
    next_embedded(&mut cursor).0
}

fn assert_complete_archive(raw: &[u8]) {
    let archive = read_embedded(raw);
    assert_eq!(archive.to_bytes().unwrap(), raw);
    let rebuilt = SimulationEmbeddedArchive::from_fields(
        archive.fields().to_vec(),
        SimulationEmbeddedProfile::Modern,
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), raw);
    assert_eq!(archive.object_map_slots(), 3);
    assert_eq!(archive.coverage().new_objects, 1);
    assert_eq!(archive.coverage().wire_bytes, raw.len());
    assert_eq!(archive.diagnostics().len(), 2);
}

#[test]
fn populated_embedded_wrappers_reconstruct_every_owned_byte_and_construct_exact_fields() {
    let mut cursor = FIXTURE;
    let mut count = 0;
    while !cursor.is_empty() {
        let (raw, _, _) = next_embedded(&mut cursor);
        assert_complete_archive(raw);
        count += 1;
    }
    assert_eq!(count, 198);
}

#[test]
fn embedded_noop_edits_preserve_revision_identity() {
    let raw = first_embedded();
    let source = read_embedded(raw);
    let unchanged = source.edit_fields(source.fields().to_vec()).unwrap();
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(
        source.field_handle(0).unwrap(),
        unchanged.archive().field_handle(0).unwrap()
    );
    assert!(unchanged.changes().is_empty());
    assert_eq!(unchanged.unchanged_fields(), source.fields().len());
}

#[test]
fn embedded_edits_own_source_and_replacement() {
    let raw = first_embedded();
    let source = read_embedded(raw);
    let mut replacement = source.fields().to_vec();
    let index = replacement
        .iter()
        .position(|field| field.path.as_ref() == "root.object.unknown_60")
        .unwrap();
    replacement[index].value = ModelFieldValue::U16(17);
    let edit = source.edit_fields(replacement).unwrap();
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.changes().len(), 1);
    assert_edit_history(&source, &edit, index);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert!(source.field_handle(source.fields().len()).is_err());
}

fn assert_edit_history(
    source: &SimulationEmbeddedArchive,
    edit: &SimulationEmbeddedEdit,
    index: usize,
) {
    assert_eq!(
        edit.changes()[0].parents(),
        &[source.field_handle(index).unwrap()]
    );
    assert_eq!(
        edit.archive().fields()[index].value,
        ModelFieldValue::U16(17)
    );
    assert_eq!(edit.coverage(), edit.archive().coverage());
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(
        edit.fidelity(),
        bessemer::model_archives::ModelEditFidelity::ExactStoredFields
    );
}

fn assert_modern_rejected(raw: &[u8]) {
    assert!(
        SimulationEmbeddedArchive::from_bytes(raw, SimulationEmbeddedProfile::Modern, LIMITS)
            .is_err()
    );
}

#[test]
fn embedded_profiles_reject_wrong_versions_prefixes_and_extra_bytes() {
    let raw = first_embedded();
    for end in 0..raw.len() {
        assert_modern_rejected(&raw[..end]);
    }
    let mut trailing = raw.to_vec();
    trailing.push(0);
    assert_modern_rejected(&trailing);
}

#[test]
fn embedded_profiles_require_the_selected_version_family() {
    let raw = first_embedded();
    assert!(
        SimulationEmbeddedArchive::from_bytes(raw, SimulationEmbeddedProfile::Legacy4281, LIMITS)
            .is_err()
    );
    for version in [0_u32, 4281, 4306, 19001, u32::MAX] {
        let mut invalid = raw.to_vec();
        invalid[..4].copy_from_slice(&version.to_le_bytes());
        assert_modern_rejected(&invalid);
    }
}

#[test]
fn embedded_published_results_and_operation_results_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationEmbeddedArchive>();
    shared::<SimulationEmbeddedEdit>();
    shared::<SimulationEmbeddedProfile>();
}

#[test]
fn embedded_archive_limits_fail_without_publishing_partial_results() {
    let raw = first_embedded();
    for limits in rejection_limits(raw.len()) {
        assert!(
            SimulationEmbeddedArchive::from_bytes(raw, SimulationEmbeddedProfile::Modern, limits)
                .is_err()
        );
    }
    assert_complete_archive(raw);
}

fn rejection_limits(bytes: usize) -> [ModelArchiveLimits; 4] {
    [
        ModelArchiveLimits {
            maximum_bytes: bytes - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ]
}

#[test]
fn embedded_archives_own_source_bytes_and_returned_buffers() {
    let raw = first_embedded();
    let mut input = raw.to_vec();
    let archive = read_embedded(&input);
    input.fill(0xa5);
    let mut returned = archive.to_bytes().unwrap();
    returned.fill(0x5a);
    assert_eq!(archive.to_bytes().unwrap(), raw);
    let mut invalid = archive.fields().to_vec();
    invalid.swap(0, 1);
    assert!(archive.edit_fields(invalid).is_err());
    assert_eq!(archive.to_bytes().unwrap(), raw);
}

#[test]
fn legacy_and_modern_null_roots_are_complete_empty_results() {
    for (version, profile) in [
        (4281_u32, SimulationEmbeddedProfile::Legacy4281),
        (4307, SimulationEmbeddedProfile::Modern),
    ] {
        let mut raw = version.to_le_bytes().to_vec();
        raw.extend_from_slice(&[0, 0]);
        let archive = SimulationEmbeddedArchive::from_bytes(&raw, profile, LIMITS).unwrap();
        assert_eq!(archive.to_bytes().unwrap(), raw);
        assert_eq!(archive.coverage().new_objects, 0);
        assert_eq!(archive.object_map_slots(), 1);
    }
}

#[test]
fn undeclared_embedded_classes_never_fall_back_to_ordinary_readers() {
    let name = b"moCameraFeature_c";
    let mut raw = 19000_u32.to_le_bytes().to_vec();
    raw.extend_from_slice(&[0xff, 0xff, 1, 0]);
    raw.extend_from_slice(&(name.len() as u16).to_le_bytes());
    raw.extend_from_slice(name);
    let error =
        SimulationEmbeddedArchive::from_bytes(&raw, SimulationEmbeddedProfile::Modern, LIMITS)
            .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert!(error.cause.contains("moCameraFeature_c"));
    assert!(error.cause.contains("Simulation embedded reader"));
}

#[test]
fn concurrent_embedded_operations_are_independent_of_thread_and_prior_work() {
    let source = read_embedded(first_embedded());
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let source = &source;
            scope.spawn(move || {
                for _ in 0..32 {
                    let edit = source.edit_fields(source.fields().to_vec()).unwrap();
                    assert_eq!(edit.archive().to_bytes().unwrap(), first_embedded());
                    assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
                    assert!(edit.changes().is_empty());
                }
            });
        }
    });
}

fn append_double_patterns(output: &mut Vec<u8>, values: impl IntoIterator<Item = StoredDouble>) {
    for value in values {
        output.extend_from_slice(&value.bits().to_le_bytes());
    }
}

fn append_perspective_base(output: &mut Vec<u8>, base: &PerspectiveBase) {
    append_double_patterns(output, base.eye_point);
    append_double_patterns(output, [base.plane_distance]);
}

fn perspective_patterns(payload: &PerspectivePayload) -> Vec<u8> {
    let mut output = Vec::new();
    match payload {
        PerspectivePayload::None => (),
        PerspectivePayload::Base(base) => append_perspective_base(&mut output, base),
        PerspectivePayload::Extended {
            base,
            minimum_far_clip,
            camera_fov_plane,
        } => {
            append_perspective_base(&mut output, base);
            append_double_patterns(&mut output, [*minimum_far_clip, *camera_fov_plane]);
        }
        PerspectivePayload::Matrix5 { discarded, matrix } => {
            output.extend_from_slice(&discarded.to_le_bytes());
            append_double_patterns(&mut output, matrix.iter().copied());
        }
    }
    output
}

fn transform_patterns(transform: &CompactTransform) -> Vec<u8> {
    let mut output = vec![transform.has_matrix];
    if let Some(matrix) = transform.matrix {
        append_double_patterns(&mut output, matrix.into_iter().flatten());
    }
    append_double_patterns(&mut output, transform.translation);
    append_double_patterns(&mut output, [transform.scaling]);
    output.push(transform.flags);
    output.extend_from_slice(&perspective_patterns(&transform.perspective));
    output
}

fn named_transform<'a>(archive: &'a SimulationEmbeddedArchive, path: &str) -> &'a CompactTransform {
    let field = archive
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let ModelFieldValue::Transform(transform) = &field.value else {
        panic!("expected {path} transform")
    };
    transform
}

#[test]
fn direct_transform_fields_match_every_independent_raw_pattern_and_boundary() {
    let mut cursor = FIXTURE;
    while !cursor.is_empty() {
        let (raw, wrapper_end, first_end) = next_embedded(&mut cursor);
        let archive = read_embedded(raw);
        let first = named_transform(&archive, "root.object.transform_a8");
        let second = named_transform(&archive, "root.object.transform_130");
        assert_eq!(transform_patterns(first), raw[wrapper_end..first_end]);
        assert_eq!(transform_patterns(second), raw[first_end..]);
    }
}
