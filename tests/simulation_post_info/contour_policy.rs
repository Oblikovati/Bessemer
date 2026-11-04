use super::*;

const REVERSE_CASES: [(i32, i32); 8] = [
    (0, 0),
    (1, 1),
    (2, 4),
    (3, 6),
    (-1, 6),
    (99, 6),
    (i32::MIN, 6),
    (i32::MAX, 6),
];
const PLOT_CODES: [i32; 14] = [
    i32::MIN,
    -1,
    0,
    21,
    22,
    23,
    56,
    57,
    58,
    69,
    70,
    71,
    99,
    i32::MAX,
];

#[test]
fn typed_contour_preserves_raw_codes_and_the_noninvertible_conversions() {
    for (raw, preference) in views::CONTOUR_FORWARD_CASES {
        let code = SimulationPostInfoContourCode::from_raw(raw);
        assert_eq!(code.raw(), raw);
        assert_eq!(code.preference_integer(), preference);
        assert_eq!(code.raw(), raw);
    }
    for (preference, raw) in REVERSE_CASES {
        assert_eq!(
            SimulationPostInfoContourCode::from_preference(preference).raw(),
            raw
        );
    }
    let original = SimulationPostInfoContourCode::from_raw(5);
    let roundtrip = SimulationPostInfoContourCode::from_preference(original.preference_integer());
    assert_eq!((original.raw(), roundtrip.raw()), (5, 6));
}

#[test]
fn defaults_require_explicit_plot_context_and_only_three_codes_override() {
    for (preference, normal) in REVERSE_CASES {
        assert_eq!(
            SimulationPostInfoContourCode::default_from_preference(preference, None).raw(),
            normal
        );
        for plot in PLOT_CODES {
            let expected = if [22, 57, 70].contains(&plot) {
                4
            } else {
                normal
            };
            assert_eq!(
                SimulationPostInfoContourCode::default_from_preference(preference, Some(plot))
                    .raw(),
                expected
            );
        }
    }
}

fn changed_comparison_value(value: &SimulationPostInfoValue) -> SimulationPostInfoValue {
    match value {
        SimulationPostInfoValue::Signed(word) => SimulationPostInfoValue::Signed(word ^ 1),
        SimulationPostInfoValue::RawWordBits(bits) => {
            SimulationPostInfoValue::RawWordBits(bits ^ 1)
        }
        SimulationPostInfoValue::Float32Bits(bits) => {
            SimulationPostInfoValue::Float32Bits(bits ^ 1)
        }
        SimulationPostInfoValue::Text(text) => {
            let payload = [text.payload(), &[0, 0xd8]].concat();
            SimulationPostInfoValue::Text(
                SimulationPostInfoText::from_payload(&payload, LIMITS.maximum_text_bytes).unwrap(),
            )
        }
    }
}

fn assert_comparison_field(info: &SimulationPostInfo, ordinal: usize) {
    let original = info.to_bytes();
    let value = changed_comparison_value(&info.fields()[ordinal].value);
    let edit = info
        .edit_field(&info.field_id(ordinal).unwrap(), value)
        .unwrap();
    let expected = [1, 2, 4, 3].contains(&ordinal);
    assert_eq!(info.diff_ogl_render(edit.archive()).unwrap(), expected);
    assert_eq!(edit.archive().diff_ogl_render(info).unwrap(), expected);
    assert_eq!(info.to_bytes(), original);
}

#[test]
fn named_comparison_selects_exactly_four_words_for_every_owned_field() {
    for info in infos_from_projection(EXPECTED, 70) {
        assert!(!info.diff_ogl_render(&info).unwrap());
        for ordinal in 0..info.fields().len() {
            assert_comparison_field(&info, ordinal);
        }
    }
}

#[test]
fn equal_preferences_do_not_hide_distinct_stored_contour_words() {
    let info = infos_from_projection(EXPECTED, 70).remove(0);
    let id = info.field_id(2).unwrap();
    let first = info
        .edit_field(&id, SimulationPostInfoValue::Signed(5))
        .unwrap();
    let second = info
        .edit_field(&id, SimulationPostInfoValue::Signed(6))
        .unwrap();
    assert_eq!(first.archive().header_view().unwrap().preference_integer, 3);
    assert_eq!(
        second.archive().header_view().unwrap().preference_integer,
        3
    );
    assert!(first.archive().diff_ogl_render(second.archive()).unwrap());
}

#[test]
fn contour_codes_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostInfoContourCode>();
}
