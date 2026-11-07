//! Historical holder and nested chain interoperability against complete source-backed images (#134).
use bessemer::model_archives::{
    ModelArchiveLimits, ModelFieldValue, ModelObjectRecord, ModelObjectTag,
    SimulationEmbeddedArchive, SimulationEmbeddedProfile,
};

const HOLDERS: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/holders.bin");
const CHAINS: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/chains.bin");
const REFERENCES: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/references.bin");
const DEPTHS: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/depth.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 10000,
    maximum_fields: 1000,
    maximum_entries: 1000,
    maximum_depth: 32,
    maximum_objects: 100,
};

fn next_packet<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let (length, remaining) = cursor.split_at(4);
    let size = u32::from_le_bytes(length.try_into().unwrap()) as usize;
    let (raw, tail) = remaining.split_at(size);
    *cursor = tail;
    raw
}

fn read_component(raw: &[u8]) -> SimulationEmbeddedArchive {
    let version = u32::from_le_bytes(raw[..4].try_into().unwrap());
    let profile = if version == 4281 {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS).unwrap()
}

fn roundtrip_corpus(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        let raw = next_packet(&mut cursor);
        let archive = read_component(raw);
        assert_noop_archive(&archive, raw);
        count += 1;
    }
    count
}

fn assert_noop_archive(archive: &SimulationEmbeddedArchive, raw: &[u8]) {
    let edit = archive.edit_fields(archive.fields().to_vec()).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), raw);
    assert_eq!(edit.archive().to_bytes().unwrap(), raw);
    assert!(edit.changes().is_empty());
}

#[test]
fn historical_holders_cover_all_word_and_optional_text_gates() {
    assert_eq!(roundtrip_corpus(HOLDERS), 66);
    let mut cursor = HOLDERS;
    let archive = read_component(next_packet(&mut cursor));
    let word = archive
        .fields()
        .iter()
        .find(|field| field.path.ends_with(".unknown_34"))
        .unwrap();
    assert_eq!(word.value, ModelFieldValue::I32(i32::MIN));
    assert_eq!(archive.object_map_slots(), 3);
}

#[test]
fn chains_features_and_external_files_preserve_complete_historical_layouts() {
    assert_eq!(roundtrip_corpus(CHAINS), 140);
}

#[test]
fn complete_reference_registry_matches_owned_roots_and_historical_neighbors() {
    assert_eq!(roundtrip_corpus(REFERENCES), 1191);
}

#[test]
fn complete_reference_registry_rejects_every_prefix_and_extra_byte() {
    assert_all_prefixes_fail(REFERENCES);
}

fn assert_all_prefixes_fail(mut cursor: &[u8]) {
    while !cursor.is_empty() {
        let raw = next_packet(&mut cursor);
        let profile = if raw[..4] == 4281_u32.to_le_bytes() {
            SimulationEmbeddedProfile::Legacy4281
        } else {
            SimulationEmbeddedProfile::Modern
        };
        for end in 0..raw.len() {
            assert!(SimulationEmbeddedArchive::from_bytes(&raw[..end], profile, LIMITS).is_err());
        }
        let mut trailing = raw.to_vec();
        trailing.push(0);
        assert!(SimulationEmbeddedArchive::from_bytes(&trailing, profile, LIMITS).is_err());
    }
}

#[test]
fn holder_and_chain_prefixes_never_publish_partial_archives() {
    assert_all_prefixes_fail(HOLDERS);
    assert_all_prefixes_fail(CHAINS);
}

fn modern_holder() -> SimulationEmbeddedArchive {
    let mut cursor = HOLDERS;
    loop {
        let raw = next_packet(&mut cursor);
        if raw[..4] == 19000_u32.to_le_bytes() {
            return read_component(raw);
        }
    }
}

#[test]
fn holder_unsupported_optional_bodies_fail_without_mutating_the_source() {
    let archive = modern_holder();
    let original = archive.to_bytes().unwrap();
    for suffix in ["unknown_84", "has_reference_info", "has_transform_c8"] {
        let mut fields = archive.fields().to_vec();
        let index = fields
            .iter()
            .position(|field| field.path.ends_with(suffix))
            .unwrap();
        fields[index].value = ModelFieldValue::I32(-1);
        let error = archive.edit_fields(fields).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert!(error.cause.contains("-1"));
        assert_eq!(archive.to_bytes().unwrap(), original);
    }
}

#[test]
#[ignore = "Requires explicitly extracted native embedded archives"]
fn complete_native_embedded_archives_reconstruct_and_construct_exactly() {
    let path = std::env::var("BESSEMER_EMBEDDED_NATIVE_INPUT").expect(
        "BESSEMER_EMBEDDED_NATIVE_INPUT=<unset>; expected an explicit extracted native packet path",
    );
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable native archive packet")
    });
    let mut cursor = packet.as_slice();
    let mut failures = std::collections::BTreeMap::<String, usize>::new();
    let mut count = 0;
    while !cursor.is_empty() {
        inspect_native_archive(next_packet(&mut cursor), &mut failures);
        count += 1;
    }
    assert_eq!(count, 242);
    assert!(
        failures.is_empty(),
        "remaining native archive failures: {failures:?}"
    );
}

fn inspect_native_archive(raw: &[u8], failures: &mut std::collections::BTreeMap<String, usize>) {
    let profile = if raw[..4] == 4281_u32.to_le_bytes() {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    match SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS) {
        Ok(archive) => {
            assert_noop_archive(&archive, raw);
        }
        Err(error) => {
            *failures
                .entry(format!(
                    "{:?}:{}:{}",
                    error.class, error.entity, error.cause
                ))
                .or_default() += 1;
        }
    }
}

fn tagged_archive(name: &[u8], body: &[u8]) -> Vec<u8> {
    let mut raw = 19000_u32.to_le_bytes().to_vec();
    raw.extend_from_slice(&[0xff, 0xff, 1, 0]);
    raw.extend_from_slice(&(name.len() as u16).to_le_bytes());
    raw.extend_from_slice(name);
    raw.extend_from_slice(body);
    raw
}

#[test]
fn unit_and_chain_components_require_null_owners() {
    let owner = tagged_archive(b"moExtFeatureList_c", &0_i32.to_le_bytes());
    for name in [
        b"moUnitComponent_c".as_slice(),
        b"moChainComponent_c".as_slice(),
    ] {
        let raw = tagged_archive(name, &owner[4..]);
        let error =
            SimulationEmbeddedArchive::from_bytes(&raw, SimulationEmbeddedProfile::Modern, LIMITS)
                .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert!(error.cause.contains("expected null"));
    }
}

#[test]
fn nonzero_external_unit_flags_are_explicit_unsupported_states() {
    for flag in [1, 255] {
        let raw = tagged_archive(b"moUnitComponent_c", &[0, 0, flag]);
        let error =
            SimulationEmbeddedArchive::from_bytes(&raw, SimulationEmbeddedProfile::Modern, LIMITS)
                .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert!(error.cause.contains(&format!("flag {flag}")));
    }
}

#[test]
fn feature_counts_keep_the_fixed_reference_cap_with_larger_caller_budgets() {
    let limits = ModelArchiveLimits {
        maximum_bytes: 50000,
        maximum_entries: 20000,
        ..LIMITS
    };
    for count in [-1_i32, 10001] {
        let mut body = count.to_le_bytes().to_vec();
        if count > 0 {
            body.extend(std::iter::repeat_n(0, count as usize * 2));
        }
        let raw = tagged_archive(b"moExtFeatureList_c", &body);
        let error =
            SimulationEmbeddedArchive::from_bytes(&raw, SimulationEmbeddedProfile::Modern, limits)
                .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains(&count.to_string()));
    }
}

fn alias_archive() -> SimulationEmbeddedArchive {
    let mut cursor = REFERENCES;
    while !cursor.is_empty() {
        let raw = next_packet(&mut cursor);
        if raw[10..].starts_with(b"moReferenceChain_c") {
            return read_component(raw);
        }
    }
    panic!("owned reference fixture; expected a shared-alias reference chain")
}

fn tagged_field<'a>(archive: &'a SimulationEmbeddedArchive, suffix: &str) -> &'a ModelObjectTag {
    let field = archive
        .fields()
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected {suffix} object tag")
    };
    tag
}

#[test]
fn nested_object_aliases_and_cached_classes_keep_independent_literal_map_slots() {
    let archive = alias_archive();
    assert_eq!(archive.object_map_slots(), 8);
    let alias = tagged_field(&archive, "features[1]");
    assert!(
        matches!(&alias.record, ModelObjectRecord::Reference { index, class } if index.value() == 6 && class.name == "moCStringHandle_c")
    );
    let cached = tagged_field(&archive, "features[2]");
    assert_eq!(cached.framing.tag, 0x8005);
    assert!(
        matches!(&cached.record, ModelObjectRecord::New { index, class } if index.value() == 7 && class.schema == 7)
    );
    assert_eq!(archive.coverage().object_aliases, 1);
    assert_noop_archive(&archive, &archive.to_bytes().unwrap());
}

#[test]
fn representation_depth_cap_is_independent_of_larger_operation_budgets() {
    let mut cursor = DEPTHS;
    for expected_depth in [128_u32, 129] {
        let (depth, rest) = cursor.split_at(4);
        assert_eq!(
            u32::from_le_bytes(depth.try_into().unwrap()),
            expected_depth
        );
        cursor = rest;
        assert_representation_depth(next_packet(&mut cursor), expected_depth);
    }
    assert!(cursor.is_empty());
}

fn assert_representation_depth(raw: &[u8], depth: u32) {
    let limits = ModelArchiveLimits {
        maximum_depth: 512,
        maximum_objects: 256,
        ..LIMITS
    };
    let result =
        SimulationEmbeddedArchive::from_bytes(raw, SimulationEmbeddedProfile::Modern, limits);
    if depth == 128 {
        assert_noop_archive(&result.unwrap(), raw);
    } else {
        assert!(result.unwrap_err().cause.contains("depth 128"));
    }
}
