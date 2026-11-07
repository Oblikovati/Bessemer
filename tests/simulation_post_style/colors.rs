use super::*;

const CHANNELS: [SimulationColorChannel; 4] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
    SimulationColorChannel::Alpha,
];

#[test]
fn rgba_values_preserve_nonfinite_payloads_and_offer_finite_channels_separately() {
    let bits = [0x80000000, 1, 0x7f800000, 0x7fa00042];
    let color = SimulationRgbaColor::from_bits(bits);
    assert_eq!(
        SimulationRgbaColor::from_bytes(&color.to_bytes()).unwrap(),
        color
    );
    assert_eq!(color.bits(), bits);
    for (index, channel) in CHANNELS.into_iter().enumerate() {
        assert_eq!(color.channel_bits(channel), bits[index]);
        let expected = if index < 2 { Some(bits[index]) } else { None };
        assert_eq!(color.finite_channel(channel).map(f32::to_bits), expected);
    }
    for length in [0, 15, 17] {
        assert!(SimulationRgbaColor::from_bytes(&vec![0; length]).is_err());
    }
}

#[test]
fn every_owned_style_color_matches_all_four_typed_fields_and_physical_bytes() {
    for style in owned_styles() {
        let raw = style.to_bytes();
        let colors = style.colors().unwrap();
        assert!(!colors.is_empty());
        for color in colors {
            let (start, end) = color.wire_range;
            assert_eq!(end - start, 16);
            assert_eq!(color.color.to_bytes(), raw[start..end]);
            assert!(
                style
                    .fields()
                    .iter()
                    .any(|field| field.path.as_ref() == format!("{}.component_0", color.path))
            );
        }
        assert_eq!(style.to_bytes(), raw);
    }
}

#[test]
fn color_views_and_channels_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationRgbaColor>();
    assert_threads::<SimulationColorChannel>();
    assert_threads::<SimulationPostColorView>();
}

#[test]
fn all_owned_style_colors_match_independent_source_paths_spans_bits_and_finite_values() {
    let actual = owned_styles()
        .into_iter()
        .map(|style| (style.version().value(), style.colors().unwrap()));
    let count = color_expectations::assert_packet(
        include_bytes!("../fixtures/simulation-post-dialog/rgba-views.bin"),
        2,
        actual,
    );
    assert_eq!(count, 200);
}

#[test]
#[ignore = "Requires fresh independent complete native RGBA and field projections."]
fn all_native_style_colors_match_independent_source_expectations() {
    let fields = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    let colors = std::fs::read("target/simulation-post-rgba-native.bin").unwrap();
    let actual = styles_from_projection(&fields, 52)
        .into_iter()
        .map(|style| (style.version().value(), style.colors().unwrap()));
    assert_eq!(color_expectations::assert_packet(&colors, 2, actual), 52);
}

#[test]
fn eight_workers_match_serial_color_queries_for_every_owned_style() {
    color_expectations::assert_threaded(&owned_styles(), SimulationPostStyle::colors);
}

#[test]
#[ignore = "Requires fresh native inputs; optional output supports process replay."]
fn complete_color_projection_for_two_process_replay() {
    let native = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    let mut styles = owned_styles();
    styles.extend(styles_from_projection(&native, 52));
    let contexts = styles
        .into_iter()
        .map(|style| (style.version().value(), style.colors().unwrap()));
    let projected = color_expectations::project_collections(2, contexts);
    if let Some(path) = std::env::var_os("BESSEMER_POST_RGBA_OUTPUT") {
        std::fs::write(path, projected).unwrap();
    }
}
