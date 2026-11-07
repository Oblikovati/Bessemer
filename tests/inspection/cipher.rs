use super::{InspectionProfile, cbc, rijndael, tables};

#[test]
fn legacy_cbc_matches_all_nine_independent_framework_vectors() {
    let mut cursor = crate::archive_codec::ArchiveCursor::new(include_bytes!(
        "../fixtures/standalone-inspection/legacy-vectors.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"ILV1");
    assert_eq!(cursor.u32().unwrap(), 9);
    for _ in 0..9 {
        let plain_length = cursor.u32().unwrap() as usize;
        let cipher_length = cursor.u32().unwrap() as usize;
        let plain = cursor.take(plain_length).unwrap();
        let cipher = cursor.take(cipher_length).unwrap();
        assert_eq!(
            cbc::decode(InspectionProfile::Historical2015, cipher, 10000).unwrap(),
            plain
        );
        assert_eq!(
            cbc::encode(InspectionProfile::Historical2015, plain, 10000).unwrap(),
            cipher
        );
    }
    assert_eq!(cursor.remaining(), 0);
}

#[test]
fn current_cbc_matches_the_independent_framework_xml_vector() {
    let expected = [
        0x97, 0x37, 0xe0, 0x4e, 0x6f, 0x9a, 0xda, 0x0b, 0x7c, 0xdc, 0xfc, 0x03, 0xab, 0x03, 0x72,
        0x3a,
    ];
    assert_eq!(
        cbc::encode(InspectionProfile::Current2020, b"<Project/>", 16).unwrap(),
        expected
    );
    assert_eq!(
        cbc::decode(InspectionProfile::Current2020, &expected, 16).unwrap(),
        b"<Project/>"
    );
}

#[test]
fn substitutions_are_inverse_permutations_and_rounds_preserve_every_byte() {
    for byte in 0..=255_u8 {
        assert_eq!(
            tables::INVERSE[usize::from(tables::SBOX[usize::from(byte)])],
            byte
        );
        for profile in [
            InspectionProfile::Current2020,
            InspectionProfile::Historical2015,
        ] {
            let block = [byte; 16];
            let keys = rijndael::round_keys(profile);
            assert_eq!(
                rijndael::decrypt(rijndael::encrypt(block, &keys), &keys),
                block
            );
        }
    }
}

#[test]
fn all_padding_widths_empty_input_and_full_blocks_roundtrip() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        for length in 0..=64 {
            let plain = vec![0xa5; length];
            let stored = cbc::encode(profile, &plain, 80).unwrap();
            assert_eq!(stored.len(), length + 16 - length % 16);
            assert_eq!(cbc::decode(profile, &stored, 80).unwrap(), plain);
            assert!(cbc::decode(profile, &stored, stored.len() - 1).is_err());
        }
    }
}

#[test]
fn cipher_lengths_overflow_and_invalid_padding_reject() {
    for length in [0, 1, 15, 17, 31] {
        assert!(cbc::decode(InspectionProfile::Current2020, &vec![0; length], 64).is_err());
    }
    assert!(cbc::padded_length(usize::MAX, usize::MAX).is_err());
    assert!(cbc::encode(InspectionProfile::Current2020, &[0; 16], 31).is_err());
    assert!(cbc::decode(InspectionProfile::Current2020, &[0; 16], 16).is_err());
}

#[test]
fn invalid_padding_rejects_zero_oversized_and_unequal_trailing_bytes() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let keys = rijndael::round_keys(profile);
        for count in [0, 17, 255, 2] {
            let mut block = [3; 16];
            block[15] = count;
            let stored = rijndael::encrypt(block, &keys);
            assert!(cbc::decode(profile, &stored, 16).is_err());
        }
    }
}
