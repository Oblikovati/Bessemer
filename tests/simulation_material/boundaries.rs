use super::*;

pub(super) fn owned_body(version: i32, presence: Option<i32>) -> SimulationStudyMaterial {
    let mut cursor = OWNED;
    while !cursor.is_empty() {
        let context = SimulationPersistVersion::new(word(&mut cursor) as i32).unwrap();
        assert_eq!(word(&mut cursor), 1);
        let size = word(&mut cursor);
        word(&mut cursor);
        let raw = take(&mut cursor, size);
        let spans = word(&mut cursor);
        take(&mut cursor, spans * 8);
        if context.value() != version {
            continue;
        }
        let body = SimulationStudyMaterial::from_bytes(raw, context, LIMITS).unwrap();
        if body_presence(&body) == presence {
            return body;
        }
    }
    panic!("version {version}, presence {presence:?}; expected complete owned Material");
}

fn body_presence(body: &SimulationStudyMaterial) -> Option<i32> {
    body.fields().iter().find_map(|field| {
        if field.path.as_ref() != "property0.curve_present" {
            return None;
        }
        let SimulationMaterialValue::Signed(value) = field.value else {
            panic!("{:?}; expected signed stored presence", field.value);
        };
        Some(value)
    })
}

fn text_at<'a>(body: &'a SimulationStudyMaterial, path: &str) -> &'a SimulationPersistText {
    let index = body
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap();
    let field = body.field(&body.field_id(index).unwrap()).unwrap();
    let SimulationMaterialValue::Text(text) = &field.value else {
        panic!("{path}: {:?}; expected raw persisted text", field.value);
    };
    text
}

#[test]
fn absent_positive_table_counts_are_retained_without_payload_or_availability_assumptions() {
    let absent = owned_body(891107, Some(0));
    assert_eq!(absent.coverage().property_count, 2);
    assert_eq!(absent.coverage().delegated_count, 3);
    assert_eq!(absent.coverage().point_pairs, 7);
    assert_eq!(absent.coverage().declared_entries, 10019);
    assert!(
        !absent
            .fields()
            .iter()
            .any(|field| field.path.starts_with("property0.pairs["))
    );
    for presence in [-1, 1, 2] {
        let populated = owned_body(891107, Some(presence));
        assert_eq!(populated.coverage().point_pairs, 10);
        assert_eq!(populated.coverage().declared_entries, 22);
    }
}

#[test]
fn null_empty_odd_surrogate_and_embedded_nul_text_keep_distinct_exact_frames() {
    let body = owned_body(891107, Some(1));
    assert!(text_at(&body, "text20").is_null());
    assert_eq!(text_at(&body, "text28").utf16_units(), Some(vec![]));
    assert_eq!(text_at(&body, "text58").utf16_units(), Some(vec![0xd800]));
    let odd = text_at(&body, "property0.auxiliary_text");
    assert!(!odd.is_null());
    assert_eq!(odd.payload(), b"3\0\0");
    assert_eq!(odd.utf16_units(), None);
    assert_eq!(
        text_at(&body, "property0.name").utf16_units(),
        Some(vec![65, 0, 66])
    );
}

#[test]
fn complete_empty_property_lists_remain_valid_completed_bodies_with_zero_declared_entries() {
    let body = owned_body(891107, None);
    assert_eq!(body.coverage().property_count, 0);
    assert_eq!(body.coverage().delegated_count, 0);
    assert_eq!(body.coverage().point_pairs, 0);
    assert_eq!(body.coverage().declared_entries, 0);
    let limits = SimulationMaterialLimits {
        maximum_entries: 0,
        ..LIMITS
    };
    let parsed =
        SimulationStudyMaterial::from_bytes(&body.to_bytes(), body.version(), limits).unwrap();
    assert_eq!(
        parsed.coverage().completion,
        SimulationMaterialCompletion::CompletedBody
    );
}

#[test]
fn exact_prefix_byte_limit_excludes_a_much_larger_following_carrier() {
    let source = owned_body(891107, Some(1));
    let raw = source.to_bytes();
    let mut carrier = raw.clone();
    carrier.extend(vec![0xa5; 2000000]);
    let limits = SimulationMaterialLimits {
        maximum_bytes: raw.len(),
        ..LIMITS
    };
    let (body, consumed) =
        SimulationStudyMaterial::from_prefix(&carrier, source.version(), limits).unwrap();
    assert_eq!(consumed, raw.len());
    assert_eq!(body.to_bytes(), raw);
    assert_text_parents(&body, &raw);
    assert!(carrier[consumed..].iter().all(|byte| *byte == 0xa5));
    assert!(SimulationStudyMaterial::from_bytes(&carrier, source.version(), limits).is_err());
    let short = SimulationMaterialLimits {
        maximum_bytes: raw.len() - 1,
        ..limits
    };
    assert!(SimulationStudyMaterial::from_prefix(&carrier, source.version(), short).is_err());
}

#[test]
fn source_and_constructed_fields_have_owned_parents_and_revision_scoped_identities() {
    let source = owned_body(891107, Some(1));
    let original = source.to_bytes();
    let mut input = original.clone();
    let body = SimulationStudyMaterial::from_bytes(&input, source.version(), LIMITS).unwrap();
    input.fill(0);
    assert_eq!(body.to_bytes(), original);
    assert_text_parents(&body, &original);
    assert!(body.field(&source.field_id(0).unwrap()).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    let id = body.field_id(0).unwrap();
    assert_eq!(body.clone().field(&id).unwrap(), &body.fields()[0]);
}

#[test]
fn constructors_reject_mismatched_field_paths_and_wire_kinds_without_source_changes() {
    let source = owned_body(891107, Some(1));
    let original = source.to_bytes();
    let mut fields = source.fields().to_vec();
    fields[0].path = "fabricated.text".into();
    let error = SimulationStudyMaterial::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.text") && error.cause.contains("text20"));
    let mut fields = source.fields().to_vec();
    fields[0].value = SimulationMaterialValue::Signed(0);
    let error = SimulationStudyMaterial::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("Signed(0)") && error.cause.contains("Text"));
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn material_read_snapshots_and_public_contracts_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationStudyMaterial>();
    shared::<SimulationMaterialField>();
    shared::<SimulationMaterialValue>();
    shared::<SimulationMaterialFieldId>();
    shared::<SimulationMaterialLimits>();
    shared::<SimulationMaterialBoundary>();
    shared::<SimulationMaterialCheckMode>();
    shared::<SimulationMaterialCompletion>();
    shared::<SimulationMaterialCoverage>();
    shared::<SimulationMaterialDiagnostic>();
}
