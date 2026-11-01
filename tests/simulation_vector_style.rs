//! C-886 complete raw vector groups and independently composed local write effects (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves complete field/history projections."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const REQUESTS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/vector-style-requests.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/vector-style-projection.bin");
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

fn from_words(words: [u32; 8]) -> SimulationVectorStyleControls {
    SimulationVectorStyleControls {
        size_raw: words[0] as i32,
        density_first_raw: words[1] as i32,
        density_second_raw: words[2] as i32,
        color_selector_raw: words[3] as i32,
        color: SimulationRgbaColor::from_bits(words[4..].try_into().unwrap()),
    }
}

fn words_of(controls: SimulationVectorStyleControls) -> [u32; 8] {
    let color = controls.color.bits();
    [
        controls.size_raw as u32,
        controls.density_first_raw as u32,
        controls.density_second_raw as u32,
        controls.color_selector_raw as u32,
        color[0],
        color[1],
        color[2],
        color[3],
    ]
}

fn read_request(cursor: &mut &[u8]) -> Vec<u32> {
    let original = std::array::from_fn(|_| read_word(cursor));
    let before = from_words(original);
    let active = read_word(cursor);
    assert!(active <= 1);
    let requests: [u32; 5] = std::array::from_fn(|_| read_word(cursor));
    let outputs = [
        before.after_size_write(requests[0] as i32, active == 1),
        before.after_density_writes(requests[1] as i32, requests[2] as i32, active == 1),
        before.after_color_copy_nearest_even(requests[3] as i32, requests[4], active == 1),
    ];
    let mut result = Vec::new();
    for output in outputs {
        let expected: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
        assert_eq!(words_of(output), expected);
        result.extend(expected);
    }
    assert_eq!(words_of(before), original);
    result
}

fn replay_requests(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 2922);
    let output = (0..2922).flat_map(|_| read_request(&mut cursor)).collect();
    assert!(cursor.is_empty());
    output
}

fn stored_vector_controls(
    family: u32,
    version: u32,
    raw: &[u8],
) -> bessemer::Result<SimulationVectorStyleControls> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let controls = parent.vector_style_controls();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.vector_style_controls(), controls);
        return controls;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let controls = style.vector_style_controls();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.vector_style_controls(), controls);
    controls
}

fn assert_view(
    cursor: &mut &[u8],
    actual: bessemer::Result<SimulationVectorStyleControls>,
    available: u32,
) -> Vec<u32> {
    if available == 0 {
        let error = actual.unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "style.word_28c");
        return vec![0; 9];
    }
    assert_eq!(available, 1);
    let expected: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(words_of(actual.unwrap()), expected);
    [vec![1], expected.to_vec()].concat()
}

fn replay_views(mut cursor: &[u8], count: usize, expected_present: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    let mut present = 0;
    for _ in 0..count {
        let family = read_word(&mut cursor);
        let version = read_word(&mut cursor);
        let length = read_word(&mut cursor) as usize;
        let available = read_word(&mut cursor);
        let raw = take_projection(&mut cursor, length);
        let actual = stored_vector_controls(family, version, raw);
        output.extend(assert_view(&mut cursor, actual, available));
        present += available as usize;
    }
    assert!(cursor.is_empty());
    assert_eq!(present, expected_present);
    output
}

#[test]
fn all_local_requests_match_independent_raw_stores_and_source_whole_color_copy() {
    replay_requests(REQUESTS);
}

#[test]
fn every_owned_vector_group_preserves_raw_bytes_and_older_absence() {
    replay_views(VIEWS, 425, 385);
}

#[test]
fn whole_color_copy_replaces_alpha_and_null_active_object_preserves_every_bit() {
    let original = from_words([0xffffffff, 0x80000000, 2, 7, 0, 1, 0x7fa00042, 0xffa00042]);
    let copied = original.after_color_copy_nearest_even(-1, 0xff332211, true);
    assert_eq!(
        copied.color,
        SimulationRgbaColor::from_colorref_nearest_even(0x00332211)
    );
    assert_eq!(copied.color.bits()[3], 0x3f800000);
    assert_eq!(copied.color_selector_raw, -1);
    assert_eq!(
        original.after_color_copy_nearest_even(1, 0, false),
        original
    );
    assert_eq!(original.color.bits()[3], 0xffa00042);
}

#[test]
fn all_public_values_and_eight_workers_match_serial_requests_and_views() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationVectorStyleControls>();
    let requests = replay_requests(REQUESTS);
    let views = replay_views(VIEWS, 425, 385);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                assert_eq!(replay_requests(REQUESTS), requests);
                assert_eq!(replay_views(VIEWS, 425, 385), views);
            });
        }
    });
}

#[test]
#[ignore = "Requires fresh complete pinned native vector-style projections."]
fn every_native_vector_group_preserves_raw_bytes_and_older_absence() {
    replay_views(
        &std::fs::read("target/simulation-vector-style-projection-native.bin").unwrap(),
        276,
        224,
    );
}

#[test]
#[ignore = "Requires native projections; output supports independent process comparison."]
fn export_vector_requests_and_views_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-vector-style-projection-native.bin").unwrap();
    let output = [
        replay_requests(REQUESTS),
        replay_views(VIEWS, 425, 385),
        replay_views(&native, 276, 224),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_VECTOR_STYLE_OUTPUT") {
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
