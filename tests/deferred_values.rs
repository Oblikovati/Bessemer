//! Independent conditional installed-writer blobs and bounded owned reconstruction (#227).
use bessemer::database_hash::{HashBufferContext, HashBufferProfile, HashPageBuffer};
use bessemer::deferred_values::*;
use bessemer::{Document, FailureClass};

const PACKET: &[u8] = include_bytes!("fixtures/deferred-values/cases.bin");
const NATIVE: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
type WriterCase = (u8, u8, u64, &'static [u8]);

fn limits() -> DeferredValueLimits {
    DeferredValueLimits {
        maximum_bytes: 4_000_000,
        maximum_values: 200000,
        maximum_depth: 64,
    }
}

fn cases() -> Vec<WriterCase> {
    assert_eq!(&PACKET[..4], b"MPK1");
    let count = u32::from_le_bytes(PACKET[4..8].try_into().unwrap()) as usize;
    assert_eq!(count, 548);
    let mut position = 8;
    let mut cases = Vec::new();
    for _ in 0..count {
        let control = PACKET[position];
        let kind = PACKET[position + 1];
        let meta = u64::from_le_bytes(PACKET[position + 2..position + 10].try_into().unwrap());
        let length =
            u32::from_le_bytes(PACKET[position + 10..position + 14].try_into().unwrap()) as usize;
        position += 14;
        let end = position + length;
        cases.push((control, kind, meta, &PACKET[position..end]));
        position = end;
    }
    assert_eq!(position, PACKET.len());
    cases
}

fn pattern(length: usize) -> Vec<u8> {
    (0..length)
        .map(|index| ((index * 73) ^ 0xa5) as u8)
        .collect()
}

fn expected(kind: u8, meta: u64) -> DeferredValue {
    if kind <= 11 {
        return expected_scalar(kind, meta);
    }
    expected_container(kind, meta)
}

fn expected_scalar(kind: u8, meta: u64) -> DeferredValue {
    match kind {
        0 => DeferredValue::Null,
        1 => DeferredValue::Boolean(meta != 0),
        2 => DeferredValue::Float32Bits(meta as u32),
        3 => DeferredValue::Float64Bits(meta),
        4..=7 if (meta as i64) < 0 => DeferredValue::NegativeInteger(meta as i64),
        4..=11 => DeferredValue::UnsignedInteger(meta),
        _ => panic!("scalar kind {kind}; expected0..11"),
    }
}

fn expected_container(kind: u8, meta: u64) -> DeferredValue {
    match kind {
        12 => DeferredValue::StringBytes(pattern(meta as usize)),
        13 => DeferredValue::BinaryBytes(pattern(meta as usize)),
        14 => DeferredValue::Array(
            (0..meta)
                .map(|i| DeferredValue::UnsignedInteger(i % 128))
                .collect(),
        ),
        15 => expected_pairs(meta),
        16 => DeferredValue::Extension {
            type_byte: meta as u8,
            bytes: pattern((meta >> 8) as usize),
        },
        _ => panic!("kind {kind}; expected installed variant0..16"),
    }
}

fn expected_pairs(count: u64) -> DeferredValue {
    DeferredValue::ObjectPairs(
        (0..count)
            .map(|index| {
                (
                    DeferredValue::StringBytes(b"k".to_vec()),
                    DeferredValue::UnsignedInteger(index % 128),
                )
            })
            .collect(),
    )
}

#[test]
fn all_seventeen_writers_both_profiles_and_width_boundaries_match_independent_goldens() {
    for (control, kind, meta, raw) in cases() {
        let profile = DeferredValueProfile::from_control(control).unwrap();
        let document = DeferredValueDocument::from_bytes(raw, profile, limits()).unwrap();
        let value = expected(kind, meta);
        assert_eq!(
            document.value(),
            &value,
            "control{control}/kind{kind}/meta{meta}"
        );
        assert_eq!(document.profile(), profile);
        assert_eq!(document.to_bytes(), raw);
        assert_eq!(
            DeferredValueDocument::from_value(&value, profile, limits()).unwrap(),
            document
        );
        assert_eq!(document.coverage().encoded_bytes, raw.len());
        assert_eq!(document.coverage().values, expected_value_count(kind, meta));
    }
}

fn expected_value_count(kind: u8, meta: u64) -> usize {
    match kind {
        14 => 1 + meta as usize,
        15 => 1 + 2 * meta as usize,
        _ => 1,
    }
}

#[test]
fn ordered_duplicate_recursive_keys_and_mixed_nested_values_match_literal_bytes() {
    let encodings: [&[u8]; 2] = [
        include_bytes!("fixtures/deferred-values/nested-0.bin"),
        include_bytes!("fixtures/deferred-values/nested-1.bin"),
    ];
    for (control, raw) in encodings.into_iter().enumerate() {
        let profile = DeferredValueProfile::from_control(control as u8).unwrap();
        let document = DeferredValueDocument::from_bytes(raw, profile, limits()).unwrap();
        assert_eq!(document.value(), &nested_value());
        assert_eq!(document.coverage().values, 13);
        assert_eq!(document.coverage().deepest_depth, 2);
        assert_eq!(
            DeferredValueDocument::from_value(&nested_value(), profile, limits())
                .unwrap()
                .to_bytes(),
            raw
        );
    }
}

fn nested_value() -> DeferredValue {
    let key = DeferredValue::Array(vec![
        DeferredValue::StringBytes(vec![0, 255]),
        DeferredValue::UnsignedInteger(256),
    ]);
    let value = DeferredValue::ObjectPairs(vec![
        (
            DeferredValue::Boolean(true),
            DeferredValue::Extension {
                type_byte: 255,
                bytes: vec![1, 0, 2],
            },
        ),
        (
            DeferredValue::Boolean(true),
            DeferredValue::Float32Bits(0x7f800001),
        ),
    ]);
    DeferredValue::ObjectPairs(vec![(key.clone(), value), (key, DeferredValue::Null)])
}

#[test]
fn every_other_control_value_is_rejected_without_inference() {
    for control in 2..=u8::MAX {
        let error = DeferredValueProfile::from_control(control).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains(&control.to_string()));
    }
    let raw = [0xcd, 1, 2];
    let big =
        DeferredValueDocument::from_bytes(&raw, DeferredValueProfile::BigEndianControl0, limits())
            .unwrap();
    let little = DeferredValueDocument::from_bytes(
        &raw,
        DeferredValueProfile::LittleEndianControl1,
        limits(),
    )
    .unwrap();
    assert_eq!(big.value(), &DeferredValue::UnsignedInteger(258));
    assert_eq!(little.value(), &DeferredValue::UnsignedInteger(513));
}

#[test]
fn all_proper_prefixes_of_small_values_and_every_appended_byte_fail() {
    for (control, _, _, raw) in cases().into_iter().filter(|row| row.3.len() < 300) {
        let profile = DeferredValueProfile::from_control(control).unwrap();
        for end in 0..raw.len() {
            assert!(DeferredValueDocument::from_bytes(&raw[..end], profile, limits()).is_err());
        }
        assert!(
            DeferredValueDocument::from_bytes(&[raw, &[0]].concat(), profile, limits()).is_err()
        );
    }
}

#[test]
fn unproved_nonminimal_forms_and_reserved_tags_are_not_silently_normalized() {
    for raw in nonminimal_forms() {
        let error = DeferredValueDocument::from_bytes(
            &raw,
            DeferredValueProfile::BigEndianControl0,
            limits(),
        )
        .unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported, "{raw:?}: {error}");
    }
}

fn nonminimal_forms() -> Vec<Vec<u8>> {
    vec![
        vec![0xcc, 0],
        vec![0xd0, 0xff],
        vec![0xd0, 127],
        vec![0xd9, 1, b'a'],
        vec![0xc5, 0, 1, b'a'],
        vec![0xdc, 0, 0],
        vec![0xde, 0, 0],
        vec![0xc7, 1, 0xff, 0],
        vec![0xc1],
    ]
}

#[test]
fn negative_constructor_rejects_nonnegative_inputs() {
    for value in [0, 1, i64::MAX] {
        assert!(
            DeferredValueDocument::from_value(
                &DeferredValue::NegativeInteger(value),
                DeferredValueProfile::BigEndianControl0,
                limits()
            )
            .is_err()
        );
    }
}

#[test]
fn impossible_counts_and_lengths_fail_before_large_allocations() {
    let mut generous = limits();
    generous.maximum_values = usize::MAX;
    for raw in [
        vec![0xdd, 255, 255, 255, 255],
        vec![0xdf, 255, 255, 255, 255],
        vec![0xdb, 255, 255, 255, 255],
        vec![0xc6, 255, 255, 255, 255],
        vec![0xc9, 255, 255, 255, 255, 0],
    ] {
        let error = DeferredValueDocument::from_bytes(
            &raw,
            DeferredValueProfile::BigEndianControl0,
            generous,
        )
        .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
    }
}

#[test]
fn exact_and_one_below_bytes_values_keys_and_depth_budgets_are_enforced() {
    let profile = DeferredValueProfile::BigEndianControl0;
    let raw = [0x82, 0xa1, b'k', 0, 0xa1, b'k', 1];
    let exact = DeferredValueLimits {
        maximum_bytes: raw.len(),
        maximum_values: 5,
        maximum_depth: 1,
    };
    let document = DeferredValueDocument::from_bytes(&raw, profile, exact).unwrap();
    assert_eq!(document.coverage().values, 5);
    assert_eq!(document.coverage().deepest_depth, 1);
    for lower in one_below_limits(exact) {
        assert!(DeferredValueDocument::from_bytes(&raw, profile, lower).is_err());
        assert!(DeferredValueDocument::from_value(document.value(), profile, lower).is_err());
    }
}

fn one_below_limits(exact: DeferredValueLimits) -> [DeferredValueLimits; 3] {
    [
        DeferredValueLimits {
            maximum_bytes: exact.maximum_bytes - 1,
            ..exact
        },
        DeferredValueLimits {
            maximum_values: 4,
            ..exact
        },
        DeferredValueLimits {
            maximum_depth: 0,
            ..exact
        },
    ]
}

#[test]
fn host_depth_ceiling_is_checked_before_recursive_construction() {
    let profile = DeferredValueProfile::BigEndianControl0;
    let raw = [vec![0x91; 64], vec![0xc0]].concat();
    let deepest = DeferredValueDocument::from_bytes(&raw, profile, limits()).unwrap();
    assert_eq!(deepest.coverage().deepest_depth, 64);
    assert!(
        DeferredValueDocument::from_bytes(
            &[vec![0x91; 65], vec![0xc0]].concat(),
            profile,
            limits()
        )
        .is_err()
    );
    let mut invalid = limits();
    invalid.maximum_depth = 65;
    assert!(DeferredValueDocument::from_bytes(&[0xc0], profile, invalid).is_err());
}

#[test]
fn empty_containers_and_blobs_have_complete_single_value_coverage() {
    let profile = DeferredValueProfile::BigEndianControl0;
    for raw in [
        vec![0x90],
        vec![0x80],
        vec![0xa0],
        vec![0xc4, 0],
        vec![0xc7, 0, 0xff],
    ] {
        let empty = DeferredValueDocument::from_bytes(&raw, profile, limits()).unwrap();
        assert_eq!(empty.to_bytes(), raw);
        assert_eq!(empty.coverage().values, 1);
    }
}

#[test]
fn constructed_values_and_parsed_bytes_have_independent_ownership() {
    let profile = DeferredValueProfile::LittleEndianControl1;
    let mut value = DeferredValue::ObjectPairs(vec![(
        DeferredValue::StringBytes(b"\0\xff".to_vec()),
        DeferredValue::Float64Bits(0x7ff0000000000001),
    )]);
    let document = DeferredValueDocument::from_value(&value, profile, limits()).unwrap();
    value = DeferredValue::Null;
    assert_eq!(value, DeferredValue::Null);
    let expected = document.to_bytes();
    let mut input = expected.clone();
    let reopened = DeferredValueDocument::from_bytes(&input, profile, limits()).unwrap();
    input.fill(0);
    assert_eq!(reopened, document);
    assert_eq!(reopened.to_bytes(), expected);
    let mut bad = limits();
    bad.maximum_bytes = 0;
    assert!(DeferredValueDocument::from_value(document.value(), profile, bad).is_err());
    assert_eq!(document.to_bytes(), expected);
}

#[test]
fn explicit_constructed_blob_can_be_written_through_an_opaque_native_storage_slot() {
    let source = Document::from_bytes(NATIVE).unwrap();
    let stream = "Contents/3DExperienceExchange2";
    let raw = source.read_stream(stream).unwrap();
    let context = storage_context();
    let page = HashPageBuffer::from_bytes(&raw[8192..12288], context).unwrap();
    let cell = &page.cells()[0];
    assert_eq!(cell.value().len(), 155);
    for profile in [
        DeferredValueProfile::BigEndianControl0,
        DeferredValueProfile::LittleEndianControl1,
    ] {
        let blob = storage_blob(profile, 155);
        assert_eq!(blob.to_bytes().len(), 155);
        let edit = page.edit_value(cell.offset(), &blob.to_bytes()).unwrap();
        let mut replacement = raw.clone();
        replacement[8192..12288].copy_from_slice(&edit.buffer().to_bytes());
        check_constructed_storage(&source, stream, replacement, cell.offset(), &blob);
    }
}

fn storage_context() -> HashBufferContext {
    HashBufferContext {
        profile: HashBufferProfile::InstalledInline,
        engine_extent: 4096,
        maximum_bytes: 4096,
        maximum_entries: 8,
    }
}

fn storage_blob(profile: DeferredValueProfile, size: usize) -> DeferredValueDocument {
    let value = DeferredValue::Array(vec![
        DeferredValue::UnsignedInteger(0x1234),
        DeferredValue::BinaryBytes(pattern(size - 6)),
    ]);
    DeferredValueDocument::from_value(&value, profile, limits()).unwrap()
}

fn check_constructed_storage(
    source: &Document,
    stream: &str,
    replacement: Vec<u8>,
    offset: u16,
    blob: &DeferredValueDocument,
) {
    let mut document = source.clone();
    document
        .replace_stream(stream, replacement.clone())
        .unwrap();
    let fresh = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(fresh.read_stream(stream).unwrap(), replacement);
    assert_eq!(
        read_constructed_blob(&fresh, stream, offset, blob.profile()),
        *blob
    );
    assert_unrelated_storage_streams(source, &fresh, stream);
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

fn assert_unrelated_storage_streams(source: &Document, fresh: &Document, stream: &str) {
    for entry in source.streams().filter(|entry| entry.name != stream) {
        assert_eq!(
            fresh.read_stream(entry.name).unwrap(),
            source.read_stream(entry.name).unwrap()
        );
    }
}

fn read_constructed_blob(
    document: &Document,
    stream: &str,
    offset: u16,
    profile: DeferredValueProfile,
) -> DeferredValueDocument {
    let raw = document.read_stream(stream).unwrap();
    let page = HashPageBuffer::from_bytes(&raw[8192..12288], storage_context()).unwrap();
    let cell = page
        .cells()
        .iter()
        .find(|cell| cell.offset() == offset)
        .unwrap();
    DeferredValueDocument::from_bytes(cell.value(), profile, limits()).unwrap()
}

#[test]
fn all_public_types_are_send_sync_and_shared_writes_match_on_eight_workers() {
    check_public_types();
    let value = expected_pairs(16);
    let document = DeferredValueDocument::from_value(
        &value,
        DeferredValueProfile::LittleEndianControl1,
        limits(),
    )
    .unwrap();
    let expected = document.to_bytes();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| document.to_bytes()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn check_public_types() {
    fn check<T: Send + Sync>() {}
    check::<DeferredValue>();
    check::<DeferredValueDocument>();
    check::<DeferredValueProfile>();
    check::<DeferredValueLimits>();
    check::<DeferredValueCoverage>();
}
