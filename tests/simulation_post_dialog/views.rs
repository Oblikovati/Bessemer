use super::*;

fn range_words(view: SimulationPostDialogRangeView) -> [u32; 10] {
    [
        view.stored_combination as u32,
        view.normalized_combination as u32,
        u32::from(view.user_minimum_selected),
        u32::from(view.user_maximum_selected),
        view.baseline_minimum_bits,
        view.baseline_maximum_bits,
        view.user_minimum_bits,
        view.user_maximum_bits,
        view.selected_minimum_bits,
        view.selected_maximum_bits,
    ]
}

fn scale_words(view: SimulationPostDialogDeformationView) -> [u32; 5] {
    let origin = match view.source {
        SimulationPostDialogScaleSource::Stored528 => 0,
        SimulationPostDialogScaleSource::Stored52c => 1,
        SimulationPostDialogScaleSource::ConstantOne => 2,
    };
    [
        view.selector as u32,
        view.scale_528_bits,
        view.scale_52c_bits,
        view.selected_scale_bits,
        origin,
    ]
}

#[test]
fn complete_owned_range_and_scale_views_match_independent_raw_bit_expectations() {
    let mut cursor =
        include_bytes!("../fixtures/simulation-post-dialog/parent-views.bin").as_slice();
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 225);
    for body in owned_dialogs() {
        let original = body.to_bytes();
        let mut words = vec![body.version().value() as u32];
        words.extend(range_words(body.range_view().unwrap()));
        words.extend(scale_words(body.deformation_scale_view().unwrap()));
        for word in words {
            assert_eq!(read_word(&mut cursor), word, "{:?}", body.version());
        }
        assert_eq!(body.to_bytes(), original);
    }
    assert!(cursor.is_empty());
}

#[test]
fn parent_prefix_views_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostDialogRangeView>();
    assert_threads::<SimulationPostDialogDeformationView>();
    assert_threads::<SimulationPostDialogScaleSource>();
}

fn assert_branch_selection(
    body: &SimulationPostDialog,
    code: i32,
    selector: i32,
    values: [u32; 6],
) {
    assert_range_selection(body.range_view().unwrap(), code, values);
    let scale = body.deformation_scale_view().unwrap();
    assert_eq!(scale.selector, selector);
    let selected = match selector {
        0 => values[0],
        1 => values[1],
        _ => 0x3f800000,
    };
    assert_eq!(scale.selected_scale_bits, selected);
}

fn assert_range_selection(view: SimulationPostDialogRangeView, code: i32, values: [u32; 6]) {
    assert_eq!(view.stored_combination, code);
    let minimum = if code == 1 || code == 2 {
        values[4]
    } else {
        values[2]
    };
    let maximum = if code == 1 || code == 3 {
        values[5]
    } else {
        values[3]
    };
    assert_eq!(view.selected_minimum_bits, minimum);
    assert_eq!(view.selected_maximum_bits, maximum);
}

fn assert_branch_rotations(body: &SimulationPostDialog, code: i32, selector: i32) {
    let patterns: [u32; 5] = [0, 0x80000000, 1, 0x7f800000, 0x7fc00042];
    for rotation in 0..patterns.len() {
        let values: [u32; 6] =
            std::array::from_fn(|index| patterns[(rotation + index) % patterns.len()]);
        let mut raw = body.to_bytes();
        raw[52..56].copy_from_slice(&selector.to_le_bytes());
        raw[56..60].copy_from_slice(&code.to_le_bytes());
        for (index, value) in values.iter().enumerate() {
            raw[60 + 4 * index..64 + 4 * index].copy_from_slice(&value.to_le_bytes());
        }
        let parsed = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS).unwrap();
        assert_branch_selection(&parsed, code, selector, values);
        assert_eq!(parsed.to_bytes(), raw);
    }
}

#[test]
fn all_210_code_selector_and_degenerate_pattern_branches_preserve_exact_bits() {
    let body = owned_dialogs().remove(0);
    for code in [0, 1, 2, 3, -1, i32::MIN, i32::MAX] {
        for selector in [0, 1, 2, -1, i32::MIN, i32::MAX] {
            assert_branch_rotations(&body, code, selector);
        }
    }
}

#[test]
fn isolated_state_edits_refresh_the_view_without_changing_the_source() {
    let source = owned_dialogs().remove(0);
    let before = source.range_view().unwrap();
    let edit = source
        .edit_field(
            &source.field_id(14).unwrap(),
            SimulationPostDialogValue::Signed(1),
        )
        .unwrap();
    let after = edit.archive().range_view().unwrap();
    assert_eq!(source.range_view().unwrap(), before);
    assert_eq!(after.stored_combination, 1);
    assert_eq!(after.normalized_combination, 1);
    assert!(after.user_minimum_selected && after.user_maximum_selected);
    assert_eq!(after.selected_minimum_bits, before.user_minimum_bits);
    assert_eq!(after.selected_maximum_bits, before.user_maximum_bits);
}
