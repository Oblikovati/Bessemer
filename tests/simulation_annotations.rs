//! Independent C-887 requests and material gates preserve raw controls and complete archive bytes (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves complete field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const POLICIES: &[u8] = include_bytes!("fixtures/simulation-post-dialog/annotation-policy.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/annotation-projection.bin");
const PARENTS: SimulationPostDialogLimits = SimulationPostDialogLimits {
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
const STYLES: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn read_controls(cursor: &mut &[u8]) -> SimulationExtremaAnnotationControls {
    SimulationExtremaAnnotationControls {
        minimum_raw: read_word(cursor) as i32,
        maximum_raw: read_word(cursor) as i32,
    }
}

fn request_flags(controls: SimulationExtremaAnnotationControls) -> [u32; 3] {
    [
        u32::from(controls.minimum_requested()),
        u32::from(controls.maximum_requested()),
        u32::from(controls.needs_value_position_sets()),
    ]
}

fn policy_case(cursor: &mut &[u8]) -> [u32; 4] {
    let controls = read_controls(cursor);
    let original = controls;
    let render = read_word(cursor) as i32;
    let plot = read_word(cursor);
    assert!(plot <= 1);
    let expected: [u32; 4] = std::array::from_fn(|_| read_word(cursor));
    let flags = request_flags(controls);
    let actual = [
        flags[0],
        flags[1],
        flags[2],
        u32::from(controls.glyph_material_adjustment_requested(render, plot == 1)),
    ];
    assert_eq!(actual, expected);
    assert_eq!(controls, original);
    actual
}

fn replay_policies(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 43350);
    let output = (0..43350).flat_map(|_| policy_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    output
}

fn stored_controls(
    family: u32,
    version: u32,
    raw: &[u8],
) -> bessemer::Result<SimulationExtremaAnnotationControls> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let controls = parent.extrema_annotation_controls();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.extrema_annotation_controls(), controls);
        return controls;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let controls = style.extrema_annotation_controls();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.extrema_annotation_controls(), controls);
    controls
}

fn assert_view(
    cursor: &mut &[u8],
    actual: bessemer::Result<SimulationExtremaAnnotationControls>,
    present: u32,
) -> Vec<u32> {
    if present == 0 {
        let error = actual.unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "style.word_234");
        return vec![0; 6];
    }
    assert_eq!(present, 1);
    let expected = read_controls(cursor);
    let flags: [u32; 3] = std::array::from_fn(|_| read_word(cursor));
    let controls = actual.unwrap();
    assert_eq!(controls, expected);
    assert_eq!(request_flags(controls), flags);
    [
        vec![1, controls.minimum_raw as u32, controls.maximum_raw as u32],
        flags.to_vec(),
    ]
    .concat()
}

fn replay_views(mut cursor: &[u8], count: usize, expected_present: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    let mut found = 0;
    for _ in 0..count {
        let family = read_word(&mut cursor);
        let version = read_word(&mut cursor);
        let length = read_word(&mut cursor) as usize;
        let present = read_word(&mut cursor);
        let raw = take_projection(&mut cursor, length);
        output.extend(assert_view(
            &mut cursor,
            stored_controls(family, version, raw),
            present,
        ));
        found += present as usize;
    }
    assert!(cursor.is_empty());
    assert_eq!(found, expected_present);
    output
}

#[test]
fn all_raw_control_classes_and_material_contexts_match_reviewed_truth_tables() {
    replay_policies(POLICIES);
}

#[test]
fn all_owned_annotation_views_preserve_raw_values_bytes_and_old_absence() {
    replay_views(VIEWS, 425, 405);
}

#[test]
fn minimum_only_and_negative_controls_keep_independent_annotation_requests() {
    let controls = SimulationExtremaAnnotationControls {
        minimum_raw: -1,
        maximum_raw: 0,
    };
    assert_eq!(request_flags(controls), [1, 0, 1]);
    assert!(controls.glyph_material_adjustment_requested(-1, true));
    assert!(!controls.glyph_material_adjustment_requested(0, true));
    assert!(!controls.glyph_material_adjustment_requested(1, false));
    assert!(controls.needs_value_position_sets());
}

#[test]
fn raw_preference_copy_provenance_composes_with_annotation_roles() {
    let initial = SimulationStyleRawWords::initializer_words();
    for pointed in [None, Some(70), Some(73), Some(-1)] {
        let copied = initial.after_local_preference_copy(u32::MAX, 0x80000000, pointed);
        let controls = SimulationExtremaAnnotationControls {
            minimum_raw: copied.word_234.unwrap() as i32,
            maximum_raw: copied.word_238.unwrap() as i32,
        };
        let clear = matches!(pointed, Some(70 | 73));
        assert_eq!(
            request_flags(controls),
            if clear { [0, 0, 0] } else { [1, 1, 1] }
        );
        assert_eq!(initial.word_234, Some(0));
        assert_eq!(initial.word_238, Some(0));
    }
}

#[test]
fn public_controls_and_eight_workers_match_serial_policies_and_views() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationExtremaAnnotationControls>();
    let serial = (replay_policies(POLICIES), replay_views(VIEWS, 425, 405));
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                assert_eq!(
                    (replay_policies(POLICIES), replay_views(VIEWS, 425, 405)),
                    serial
                )
            });
        }
    });
}

#[test]
#[ignore = "Requires fresh complete pinned native annotation projections."]
fn all_native_annotation_views_preserve_raw_values_and_bytes() {
    replay_views(
        &std::fs::read("target/simulation-annotation-projection-native.bin").unwrap(),
        276,
        276,
    );
}

#[test]
#[ignore = "Requires native projections; output supports independent process comparison."]
fn export_annotation_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-annotation-projection-native.bin").unwrap();
    let output = [
        replay_policies(POLICIES),
        replay_views(VIEWS, 425, 405),
        replay_views(&native, 276, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_ANNOTATION_OUTPUT") {
        std::fs::write(
            path,
            output
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
