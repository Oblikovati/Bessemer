use super::annotation_view_tests::decode;

fn object(class: &str, body: &[u8]) -> Vec<u8> {
    let mut bytes = vec![255, 255, 1, 0];
    bytes.extend_from_slice(&(class.len() as u16).to_le_bytes());
    bytes.extend_from_slice(class.as_bytes());
    bytes.extend_from_slice(body);
    bytes
}

#[test]
fn context_chain_bodies_preserve_signed_state_and_exact_framing() {
    // Closed-form wire layouts from contextchains.py and assemblyreferences.py.
    let cases: &[(&str, &[u8])] = &[
        ("moChainComponent_c", &[0, 0, 0, 0, 152, 186, 220, 254]),
        (
            "moContextChainComponent_c",
            &[0, 0, 0, 0, 152, 186, 220, 254, 0, 0, 249, 255, 255, 255],
        ),
        (
            "moContextUnitComponent_c",
            &[0, 0, 0, 0, 249, 255, 255, 255, 1, b'A', 254, 255, 255, 255],
        ),
        (
            "moReferenceChain_c",
            &[
                255, 255, 0, 0, 249, 255, 255, 255, 1, b'A', 254, 255, 255, 255,
            ],
        ),
        ("moExtFeatureList_c", &[2, 0, 0, 0, 0, 0, 0, 0]),
    ];
    for (class, body) in cases {
        let bytes = object(class, body);
        let fields = decode(19000, &bytes).unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        for length in 0..bytes.len() {
            assert!(
                decode(19000, &bytes[..length]).is_err(),
                "{class} prefix {length}"
            );
        }
        assert!(decode(19000, &[bytes.as_slice(), &[0]].concat()).is_err());
        assert!(decode(0x27cb, &bytes).is_err());
    }
}

#[test]
fn context_chain_owner_rejects_reference_to_its_own_incompatible_class() {
    let bytes = object("moChainComponent_c", &[2, 0, 0, 0, 0, 0, 0, 0]);
    assert!(decode(19000, &bytes).is_err());
}

#[test]
fn external_feature_list_requires_nonnegative_i32_count_and_typed_members() {
    for body in [
        &[255, 255, 255, 255][..],
        &[255, 255, 255, 127][..],
        &[2, 0, 0, 0, 0, 0][..],
        &[1, 0, 0, 0, 2, 0][..],
    ] {
        assert!(decode(19000, &object("moExtFeatureList_c", body)).is_err());
    }
    let bytes = object("moExtFeatureList_c", &[0, 0, 0, 0]);
    assert_eq!(
        super::encode_fields(&decode(19000, &bytes).unwrap(), 30000).unwrap(),
        bytes
    );
}

#[test]
fn external_features_match_reference_version_boundaries_and_reject_partial_bodies() {
    let mut cursor = crate::archive_codec::ArchiveCursor::new(include_bytes!(
        "../fixtures/resolved-features/external-feature-cases.bin"
    ));
    let mut count = 0;
    while cursor.remaining() != 0 {
        let version = cursor.u32().unwrap();
        let length = cursor.u32().unwrap() as usize;
        let bytes = cursor.take(length).unwrap();
        let fields = decode(version, bytes).unwrap();
        assert_external_feature_state(version, &fields);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        for prefix in 0..bytes.len() {
            assert!(decode(version, &bytes[..prefix]).is_err());
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        let null = super::manager_context_tests::replace_field(
            &fields,
            "root.object.name_handle",
            &[0, 0],
        );
        assert!(decode(version, &null).is_err());
        count += 1;
    }
    assert_eq!(count, 6);
}

fn assert_external_feature_state(version: u32, fields: &[super::ModelField]) {
    for (path, expected) in [("raw_44", -7), ("raw_58", -9), ("configuration.id", -2)] {
        let field = fields
            .iter()
            .find(|field| field.path.as_ref() == format!("root.object.{path}"))
            .unwrap();
        assert_eq!(field.value, super::ModelFieldValue::I32(expected));
    }
    for (path, present) in [
        ("file.pdm_id", version > 0x2f82),
        ("file.unknown_35", version >= 0x3330),
    ] {
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == format!("root.object.{path}")),
            present
        );
    }
    let timestamp = fields
        .iter()
        .find(|field| field.path.as_ref() == "root.object.file.timestamp")
        .unwrap();
    assert_eq!(timestamp.value, super::ModelFieldValue::U32(0xfedcba98));
}
