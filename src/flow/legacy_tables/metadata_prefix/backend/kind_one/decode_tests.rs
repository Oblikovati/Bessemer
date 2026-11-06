use super::*;
fn body_options() -> FlowMediumBodyOptions {
    FlowMediumBodyOptions {
        maximum_body_bytes: 161,
        maximum_wide_words: 11,
        maximum_name_units: 1,
    }
}
fn mixed_payload() -> Vec<u8> {
    let mut raw = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    raw.extend_from_slice(&2i32.to_le_bytes());
    raw.extend_from_slice(&2i32.to_le_bytes());
    raw.extend_from_slice(&[0xa5; 16]);
    let mut two = vec![0; 198];
    two[136..140].copy_from_slice(&3i32.to_le_bytes());
    raw.extend_from_slice(&two);
    raw.extend_from_slice(&1i32.to_le_bytes());
    raw.extend_from_slice(&[0x5a; 16]);
    raw.extend_from_slice(&[0; 161]);
    raw.extend_from_slice(&[0; 17]);
    raw
}
fn carrier(raw: &[u8]) -> FlowCarrier {
    let mut wire = 8u16.to_le_bytes().to_vec();
    wire.extend_from_slice(&424u32.to_le_bytes());
    wire.extend_from_slice(&(raw.len() as u64).to_le_bytes());
    wire.extend_from_slice(raw);
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 1024,
            maximum_payload_bytes: 1024,
            maximum_records: 1,
        },
    )
    .unwrap()
}
fn backend(raw: &[u8]) -> FlowMediumBackendPrefix {
    let carrier = carrier(raw);
    let id = carrier.latest(424).unwrap().unwrap().id;
    let options = FlowMetadataPrefixOptions {
        string_mode: FlowMetadataStringMode::NativeUtf16,
        maximum_payload_bytes: 1024,
        maximum_vectors: 1,
        maximum_objects: 1,
        maximum_values: 1,
        maximum_string_units: 1,
    };
    carrier
        .record_424_metadata_prefix(&id, options)
        .unwrap()
        .medium_backend_prefix()
        .unwrap()
}
#[test]
fn selected_body_after_kind_two_uses_its_explicit_offset_without_widening_first_body_api() {
    let backend = backend(&mixed_payload());
    assert!(backend.first_kind_one_body(body_options()).is_err());
    let body = FlowMediumKindOne::read_at(&backend, 253, body_options()).unwrap();
    assert_eq!(body.payload_interval(), (253, 414));
    assert_eq!(body.body_bytes(), [0; 161]);
    assert_eq!(body.opaque_tail(), [0; 17]);
    assert_eq!(body.curves().count(), 7);
    assert_eq!(body.coverage().wide_words, 11);
    assert_eq!(body.coverage().name_units, 0);
    assert!(!body.coverage().full_record_schema);
}
#[test]
fn selected_body_edits_preserve_prior_kind_two_bytes_noops_and_fresh_reversal() {
    let backend = backend(&mixed_payload());
    let body = FlowMediumKindOne::read_at(&backend, 253, body_options()).unwrap();
    let noop = body.replace_state_byte(0).unwrap();
    assert_eq!(
        noop.result().backend_prefix().metadata_prefix().record(),
        backend.metadata_prefix().record()
    );
    let edit = body.replace_state_byte(255).unwrap();
    let mut expected = mixed_payload();
    expected[413] = 255;
    let result = edit.result().backend_prefix().metadata_prefix();
    assert_eq!(result.payload_bytes(), expected);
    assert_eq!(body.state_byte(), 0);
    check_fresh_reverse(&expected, &backend);
}
fn check_fresh_reverse(raw: &[u8], original: &FlowMediumBackendPrefix) {
    let fresh = backend(raw);
    let after = FlowMediumKindOne::read_at(&fresh, 253, body_options()).unwrap();
    let reverse = after.replace_state_byte(0).unwrap();
    assert_eq!(
        reverse.prefix_edit().carrier_edit().result().as_bytes(),
        original.metadata_prefix().source().as_bytes()
    );
}
#[test]
fn selected_body_keeps_independent_positive_limits_and_rejects_invalid_offsets() {
    let backend = backend(&mixed_payload());
    let options = body_options();
    for invalid in [
        FlowMediumBodyOptions {
            maximum_body_bytes: 0,
            ..options
        },
        FlowMediumBodyOptions {
            maximum_wide_words: 0,
            ..options
        },
        FlowMediumBodyOptions {
            maximum_name_units: 0,
            ..options
        },
    ] {
        assert!(FlowMediumKindOne::read_at(&backend, 253, invalid).is_err());
    }
    check_invalid_offsets(&backend, options);
}
fn check_invalid_offsets(backend: &FlowMediumBackendPrefix, options: FlowMediumBodyOptions) {
    for start in [backend.metadata_prefix().payload_bytes().len(), usize::MAX] {
        assert!(FlowMediumKindOne::read_at(backend, start, options).is_err());
    }
}
