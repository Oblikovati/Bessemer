//! Negative ZIP32 profile and integrity checks through the complete public operation.
use bessemer::{
    FailureClass,
    composer::{ComposerArchive, ComposerCompression, ComposerLimits, ComposerMemberInput},
};

const LIMITS: ComposerLimits = ComposerLimits {
    maximum_bytes: 100000,
    maximum_members: 20,
    maximum_member_bytes: 10000,
    maximum_expanded_bytes: 20000,
};

fn short(raw: &[u8], at: usize) -> usize {
    u16::from_le_bytes(raw[at..at + 2].try_into().unwrap()) as usize
}
fn long(raw: &[u8], at: usize) -> usize {
    u32::from_le_bytes(raw[at..at + 4].try_into().unwrap()) as usize
}
fn put_short(raw: &mut [u8], at: usize, value: u16) {
    raw[at..at + 2].copy_from_slice(&value.to_le_bytes());
}
fn put_long(raw: &mut [u8], at: usize, value: usize) {
    raw[at..at + 4].copy_from_slice(&(value as u32).to_le_bytes());
}
fn end(raw: &[u8]) -> usize {
    raw.windows(4)
        .rposition(|bytes| bytes == b"PK\x05\x06")
        .unwrap()
}

fn rejected(raw: &[u8], expected: FailureClass) {
    let failure = ComposerArchive::from_bytes(raw, LIMITS).unwrap_err();
    assert_eq!(failure.operation, "composer archive");
    assert_eq!(failure.class, expected, "{failure}");
    assert!(failure.cause.contains("expected"));
}

#[test]
fn encrypted_descriptor_split_zip64_and_unknown_method_inputs_are_explicitly_unsupported() {
    let original = include_bytes!("fixtures/composer/stored.smg");
    let tail = end(original);
    let central = long(original, tail + 16) + 3;
    for flag in [1, 8, 0x1000] {
        let mut raw = original.to_vec();
        put_short(&mut raw, 3 + 6, flag);
        put_short(&mut raw, central + 8, flag);
        rejected(&raw, FailureClass::Unsupported);
    }
    for (at, value) in [
        (tail + 4, 1),
        (central + 34, 1),
        (central + 6, 45),
        (central + 10, 99),
    ] {
        let mut raw = original.to_vec();
        put_short(&mut raw, at, value);
        rejected(&raw, FailureClass::Unsupported);
    }
    let mut raw = original.to_vec();
    put_long(&mut raw, central + 24, u32::MAX as usize);
    rejected(&raw, FailureClass::Unsupported);
    let mut raw = original.to_vec();
    let extra = 3 + 30 + short(original, 3 + 26);
    put_short(&mut raw, extra, 1);
    rejected(&raw, FailureClass::Unsupported);
}

#[test]
fn stored_crc_and_both_local_central_agreement_and_physical_partition_are_required() {
    let original = include_bytes!("fixtures/composer/stored.smg");
    let central = long(original, end(original) + 16) + 3;
    let mut raw = original.to_vec();
    put_long(&mut raw, 3 + 14, 0);
    put_long(&mut raw, central + 16, 0);
    rejected(&raw, FailureClass::Integrity);
    let mut raw = original.to_vec();
    raw[3 + 30] ^= 1;
    rejected(&raw, FailureClass::Invalid);
    let mut raw = original.to_vec();
    put_long(&mut raw, central + 42, 3);
    rejected(&raw, FailureClass::Invalid);
    let mut raw = original.to_vec();
    let payload = 3 + 30 + short(original, 3 + 26) + short(original, 3 + 28);
    raw[payload] ^= 1;
    rejected(&raw, FailureClass::Integrity);
}

fn single_deflated() -> Vec<u8> {
    ComposerArchive::from_members(
        vec![ComposerMemberInput {
            name: "a".into(),
            contents: b"hello world".repeat(40),
            compression: ComposerCompression::Deflated,
        }],
        b"",
        LIMITS,
    )
    .unwrap()
    .to_bytes()
}

#[test]
fn raw_deflate_must_terminate_and_consume_exactly_its_declared_span() {
    let original = single_deflated();
    let old_end = end(&original);
    let central = long(&original, old_end + 16) + 3;
    let stored = long(&original, 3 + 18);
    for (removed, suffix) in [(1, Vec::new()), (0, b"JUNK".to_vec())] {
        let mut raw = original[..central - removed].to_vec();
        raw.extend_from_slice(&suffix);
        raw.extend_from_slice(&original[central..]);
        let new_central = central - removed + suffix.len();
        let new_end = old_end - removed + suffix.len();
        put_long(&mut raw, 3 + 18, stored - removed + suffix.len());
        put_long(&mut raw, new_central + 20, stored - removed + suffix.len());
        put_long(&mut raw, new_end + 16, new_central - 3);
        rejected(&raw, FailureClass::Integrity);
    }
}

#[test]
fn invalid_utf8_extra_framing_and_constructor_limits_never_publish_partial_results() {
    let original = include_bytes!("fixtures/composer/stored.smg");
    let central = long(original, end(original) + 16) + 3;
    let mut raw = original.to_vec();
    raw[3 + 30] = 255;
    raw[central + 46] = 255;
    rejected(&raw, FailureClass::Invalid);
    let mut raw = original.to_vec();
    let extra = central + 46 + short(original, central + 28);
    put_short(&mut raw, extra + 2, 65535);
    rejected(&raw, FailureClass::Invalid);
    let member = ComposerMemberInput {
        name: String::new(),
        contents: Vec::new(),
        compression: ComposerCompression::Stored,
    };
    assert!(ComposerArchive::from_members(vec![member], b"", LIMITS).is_err());
    assert!(ComposerArchive::from_members(Vec::new(), &vec![0; 65536], LIMITS).is_err());
}
