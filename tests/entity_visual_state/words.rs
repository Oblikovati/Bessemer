use super::*;

fn literal_words() -> EntityVisualStoredWords {
    EntityVisualStoredWords {
        layer_number: 23,
        by_layer_flags: 0xa5a5,
        layer_attribute_cache: 0xfd,
        color_word: 0x80563412,
        line_style: 0x8000,
        standard_width: -32768,
        custom_width: Some(StoredFloat::from_bits(0x80000000)),
    }
}

#[test]
fn every_by_layer_word_and_known_role_preserves_all_unselected_values() {
    for raw in 0..=u16::MAX {
        verify_by_layer_word(raw);
    }
}

fn verify_by_layer_word(raw: u16) {
    let words = EntityVisualStoredWords {
        by_layer_flags: raw,
        ..literal_words()
    };
    assert_eq!(words.by_layer_unresolved_bits(), raw & 0xffe0);
    for (role, mask) in [
        (EntityVisualByLayerRole::Color, 1),
        (EntityVisualByLayerRole::Font, 2),
        (EntityVisualByLayerRole::Width, 4),
        (EntityVisualByLayerRole::FontSp, 8),
        (EntityVisualByLayerRole::WidthSp, 16),
    ] {
        verify_by_layer_role(words, role, mask);
    }
}

fn verify_by_layer_role(words: EntityVisualStoredWords, role: EntityVisualByLayerRole, mask: u16) {
    assert_eq!(words.by_layer(role), words.by_layer_flags & mask != 0);
    for enabled in [false, true] {
        let raw = words.by_layer_flags;
        let expected = EntityVisualStoredWords {
            by_layer_flags: if enabled { raw | mask } else { raw & !mask },
            ..words
        };
        assert_eq!(
            words
                .edited(EntityVisualStoredEdit::ByLayerRole(role, enabled), "owned")
                .unwrap(),
            expected
        );
    }
}

#[test]
fn every_cache_byte_and_both_roles_preserve_unresolved_bits() {
    for raw in 0..=u8::MAX {
        let words = EntityVisualStoredWords {
            layer_attribute_cache: raw,
            ..literal_words()
        };
        assert_eq!(words.cache_unresolved_bits(), raw & 0xfc);
        for (role, mask) in [
            (EntityVisualCacheRole::Visible, 1),
            (EntityVisualCacheRole::Printable, 2),
        ] {
            verify_cache_role(words, role, mask);
        }
    }
}

fn verify_cache_role(words: EntityVisualStoredWords, role: EntityVisualCacheRole, mask: u8) {
    let raw = words.layer_attribute_cache;
    assert_eq!(words.cached(role), raw & mask != 0);
    for enabled in [false, true] {
        let expected = EntityVisualStoredWords {
            layer_attribute_cache: if enabled { raw | mask } else { raw & !mask },
            ..words
        };
        assert_eq!(
            words
                .edited(EntityVisualStoredEdit::CacheRole(role, enabled), "owned")
                .unwrap(),
            expected
        );
    }
}

#[test]
fn rgb_order_high_bytes_and_exact_missing_sentinel_are_independent() {
    for high in 0..=u8::MAX {
        let words = EntityVisualStoredWords {
            color_word: u32::from(high) << 24 | 0x563412,
            ..literal_words()
        };
        assert_eq!(words.color_rgb(), [0x12, 0x34, 0x56]);
        assert_eq!(words.color_unresolved_high_byte(), high);
        let edited = words
            .edited(EntityVisualStoredEdit::ColorRgb([7, 19, 251]), "owned")
            .unwrap();
        assert_eq!(edited.color_word, u32::from(high) << 24 | 0xfb1307);
        assert!(!words.color_is_missing());
    }
    let missing = EntityVisualStoredWords {
        color_word: u32::MAX,
        ..literal_words()
    };
    assert!(missing.color_is_missing());
    assert_eq!(missing.color_rgb(), [255, 255, 255]);
}

fn verify_raw_request(edit: EntityVisualStoredEdit, expected: EntityVisualStoredWords) {
    assert_eq!(literal_words().edited(edit, "owned").unwrap(), expected);
    assert_eq!(
        EntityVisualStoredWords::from_scalars(&expected.scalars(), "owned").unwrap(),
        expected
    );
}

#[test]
fn raw_layer_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        layer_number: 65535,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::LayerNumber(65535), expected);
}

#[test]
fn raw_by_layer_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        by_layer_flags: 65535,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::ByLayerFlags(65535), expected);
}

#[test]
fn raw_cache_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        layer_attribute_cache: 255,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::LayerAttributeCache(255), expected);
}

#[test]
fn raw_color_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        color_word: u32::MAX,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::ColorWord(u32::MAX), expected);
}

#[test]
fn raw_style_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        line_style: 65535,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::LineStyle(65535), expected);
}

#[test]
fn raw_selector_request_retains_the_complete_record() {
    let expected = EntityVisualStoredWords {
        standard_width: 32767,
        ..literal_words()
    };
    verify_raw_request(EntityVisualStoredEdit::StandardWidth(32767), expected);
}

#[test]
fn complete_width_domains_preserve_every_raw_float_payload() {
    for standard in [i16::MIN, -1, 0, 1, i16::MAX] {
        for bits in [
            0, 0x80000000, 0x7fc01234, 0x7f800042, 0x7f800000, 0xff800000,
        ] {
            verify_width_case(standard, bits);
        }
    }
}

fn verify_width_case(standard: i16, bits: u32) {
    let words = literal_words();
    let custom = Some(StoredFloat::from_bits(bits));
    let edit = EntityVisualStoredEdit::LineWidth { standard, custom };
    let expected = EntityVisualStoredWords {
        standard_width: standard,
        custom_width: custom,
        ..words
    };
    assert_eq!(words.edited(edit, "owned").unwrap(), expected);
    verify_custom_width_only(words, custom);
}

#[test]
fn scalar_roundtrips_keep_present_and_absent_custom_fields() {
    for custom_width in [None, literal_words().custom_width] {
        let words = EntityVisualStoredWords {
            custom_width,
            ..literal_words()
        };
        assert_eq!(
            EntityVisualStoredWords::from_scalars(&words.scalars(), "owned").unwrap(),
            words
        );
    }
}

#[test]
fn malformed_scalar_records_report_the_expected_shape() {
    let mut wrong_custom = literal_words().scalars();
    wrong_custom[6] = VisualScalar::U8(0);
    for values in [
        vec![],
        vec![VisualScalar::U32(0)],
        vec![VisualScalar::U16(0); 7],
        wrong_custom,
    ] {
        let error = EntityVisualStoredWords::from_scalars(&values, "owned").unwrap_err();
        assert!(error.cause.contains("expected a complete"));
    }
}

#[test]
fn width_requests_reject_mismatched_physical_presence() {
    let absent = EntityVisualStoredWords {
        custom_width: None,
        ..literal_words()
    };
    assert!(
        absent
            .edited(
                EntityVisualStoredEdit::CustomWidth(StoredFloat::from_bits(0)),
                "owned"
            )
            .is_err()
    );
    let invalid = EntityVisualStoredEdit::LineWidth {
        standard: 0,
        custom: None,
    };
    assert!(literal_words().edited(invalid, "owned").is_err());
}

#[test]
fn absent_float_profiles_can_edit_their_stored_selector() {
    let absent = EntityVisualStoredWords {
        custom_width: None,
        ..literal_words()
    };
    let request = EntityVisualStoredEdit::LineWidth {
        standard: -1,
        custom: None,
    };
    let expected = EntityVisualStoredWords {
        standard_width: -1,
        ..absent
    };
    assert_eq!(absent.edited(request, "owned").unwrap(), expected);
}

#[test]
fn published_word_role_and_request_types_are_send_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<EntityVisualStoredWords>();
    assert_traits::<EntityVisualStoredEdit>();
    assert_traits::<EntityVisualByLayerRole>();
    assert_traits::<EntityVisualCacheRole>();
    assert_traits::<crate::model_archives::ModelEntityVisualStoredState>();
}

fn verify_custom_width_only(words: EntityVisualStoredWords, custom: Option<StoredFloat>) {
    let only_custom = EntityVisualStoredWords {
        custom_width: custom,
        ..words
    };
    assert_eq!(
        words
            .edited(
                EntityVisualStoredEdit::CustomWidth(custom.unwrap()),
                "owned"
            )
            .unwrap(),
        only_custom
    );
}
