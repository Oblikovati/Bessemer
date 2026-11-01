use super::*;

const CONTOUR_FORWARD_CASES: [(i32, i32); 10] = [
    (0, 0),
    (1, 1),
    (4, 2),
    (5, 3),
    (6, 3),
    (2, 6),
    (3, 6),
    (-1, 6),
    (i32::MIN, 6),
    (i32::MAX, 6),
];

fn kind_tag(kind: Option<SimulationPostInfoPrimitiveKind>) -> u32 {
    match kind {
        Some(SimulationPostInfoPrimitiveKind::Cylinder) => 0,
        Some(SimulationPostInfoPrimitiveKind::Plane) => 1,
        Some(SimulationPostInfoPrimitiveKind::Sphere) => 2,
        Some(SimulationPostInfoPrimitiveKind::Iso) => 3,
        None => 4,
    }
}

fn direction_tag(token: Option<SimulationPostInfoDirectionToken>) -> u32 {
    match token {
        Some(SimulationPostInfoDirectionToken::Yon) => 0,
        Some(SimulationPostInfoDirectionToken::Hither) => 1,
        Some(SimulationPostInfoDirectionToken::Both) => 2,
        None => 3,
    }
}

fn assert_slot(info: &SimulationPostInfo, raw: &[u8], index: usize, cursor: &mut &[u8]) {
    let group = if index < 6 {
        SimulationPostInfoSlotGroup::Section
    } else {
        SimulationPostInfoSlotGroup::Iso
    };
    let view = info.cut_primitive(group, index % 6).unwrap();
    assert_eq!((view.group, view.index), (group, index % 6));
    assert_eq!(kind_tag(view.primitive_kind), read_word(cursor));
    assert_eq!(direction_tag(view.direction_token), read_word(cursor));
    assert_eq!(view.negated_d_bits, read_word(cursor));
    let mut words = vec![view.primitive_code as u32, view.direction_code as u32];
    words.extend(view.normal_bits);
    words.extend(view.center_bits);
    words.push(view.shared_scalar_bits);
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    assert_eq!(bytes, raw[28 + index * 36..64 + index * 36]);
}

fn assert_header(info: &SimulationPostInfo, raw: &[u8], preference: u32) {
    let view = info.header_view().unwrap();
    let bytes: Vec<u8> = view
        .raw_words
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert_eq!(bytes, raw[..28]);
    assert_eq!(view.contour_type, view.raw_words[2] as i32);
    assert_eq!(view.preference_integer, preference as i32);
    assert_eq!(view.section_active_count, view.raw_words[5] as i32);
    assert_eq!(view.iso_active_count, view.raw_words[6] as i32);
}

#[test]
fn all_sixty_owned_slots_match_complete_independent_names_vectors_and_sign_aliases() {
    let mut cursor = include_bytes!("../fixtures/simulation-post-dialog/cut-views.bin").as_slice();
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 5);
    let version = SimulationPersistVersion::new(700335).unwrap();
    for _ in 0..5 {
        let raw = take_projection(&mut cursor, 460);
        let info = SimulationPostInfo::from_bytes(raw, version, LIMITS).unwrap();
        assert_header(&info, raw, read_word(&mut cursor));
        for index in 0..12 {
            assert_slot(&info, raw, index, &mut cursor);
        }
        assert_eq!(info.to_bytes(), raw);
    }
    assert!(cursor.is_empty());
}

#[test]
fn physical_slot_bounds_reject_indices_before_multiplication() {
    let info = infos_from_projection(EXPECTED, 70).remove(0);
    for group in [
        SimulationPostInfoSlotGroup::Section,
        SimulationPostInfoSlotGroup::Iso,
    ] {
        for index in [6, usize::MAX] {
            let error = info.cut_primitive(group, index).unwrap_err();
            assert_eq!(error.class, bessemer::FailureClass::Invalid);
            assert!(error.cause.contains(&index.to_string()));
            assert!(error.cause.contains("0..6"));
        }
    }
}

#[test]
fn contour_forward_conversion_retains_original_codes_and_source_after_edits() {
    let info = infos_from_projection(EXPECTED, 70).remove(0);
    let original = info.to_bytes();
    let id = info.field_id(2).unwrap();
    for (code, preference) in CONTOUR_FORWARD_CASES {
        let edit = info
            .edit_field(&id, SimulationPostInfoValue::Signed(code))
            .unwrap();
        let view = edit.archive().header_view().unwrap();
        assert_eq!(
            (view.contour_type, view.preference_integer),
            (code, preference)
        );
        assert_eq!(info.to_bytes(), original);
    }
}

#[test]
fn named_postinfo_views_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationPostInfoCutView>();
    assert_threads::<SimulationPostInfoHeaderView>();
    assert_threads::<SimulationPostInfoSlotGroup>();
    assert_threads::<SimulationPostInfoPrimitiveKind>();
    assert_threads::<SimulationPostInfoDirectionToken>();
}

fn assert_native_slot_roles(info: &SimulationPostInfo, group: SimulationPostInfoSlotGroup) {
    let (kind, token) = match group {
        SimulationPostInfoSlotGroup::Section => (
            SimulationPostInfoPrimitiveKind::Plane,
            SimulationPostInfoDirectionToken::Hither,
        ),
        SimulationPostInfoSlotGroup::Iso => (
            SimulationPostInfoPrimitiveKind::Iso,
            SimulationPostInfoDirectionToken::Yon,
        ),
    };
    for index in 0..6 {
        let view = info.cut_primitive(group, index).unwrap();
        assert_eq!(view.primitive_kind, Some(kind));
        assert_eq!(view.direction_token, Some(token));
        assert_eq!(view.negated_d_bits, view.shared_scalar_bits ^ 0x80000000);
    }
}

#[test]
#[ignore = "Requires freshly verified complete native dialog/PostInfo projections."]
fn native_named_roles_match_all_retained_physical_slots_under_both_helper_arguments() {
    let packet = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    for info in infos_from_projection(&packet, 120) {
        let header = info.header_view().unwrap();
        assert_eq!((header.contour_type, header.preference_integer), (6, 3));
        assert_eq!(
            (header.section_active_count, header.iso_active_count),
            (1, 1)
        );
        assert_native_slot_roles(&info, SimulationPostInfoSlotGroup::Section);
        assert_native_slot_roles(&info, SimulationPostInfoSlotGroup::Iso);
    }
}
