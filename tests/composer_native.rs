//! Installed Composer corpus gate; ordinary tests use owned independent ZIP fixtures.
use bessemer::composer::{ComposerArchive, ComposerLimits};

fn take<'a>(raw: &mut &'a [u8], length: usize) -> &'a [u8] {
    let (part, rest) = raw.split_at(length);
    *raw = rest;
    part
}

fn word(raw: &mut &[u8]) -> usize {
    u32::from_le_bytes(take(raw, 4).try_into().unwrap()) as usize
}

#[test]
#[ignore = "requires freeze_composer.py --export and the installed 38-file corpus"]
fn all_native_composer_archives_force_exact_reconstruction_and_match_all_550_independent_payloads()
{
    let packet = std::fs::read("target/composer-native-probe.bin").unwrap();
    let mut raw = packet.as_slice();
    assert_eq!(take(&mut raw, 4), b"SMGP");
    assert_eq!(word(&mut raw), 38);
    let limits = ComposerLimits {
        maximum_bytes: 64000000,
        maximum_members: 10000,
        maximum_member_bytes: 64000000,
        maximum_expanded_bytes: 256000000,
    };
    let mut members = 0;
    let mut edits = b"SMGE".to_vec();
    edits.extend_from_slice(&38u32.to_le_bytes());
    for index in 0..38 {
        let length = word(&mut raw);
        let count = word(&mut raw);
        let original = take(&mut raw, length);
        let archive = ComposerArchive::from_bytes(original, limits).unwrap();
        assert_eq!(archive.to_bytes(), original);
        assert_eq!(archive.members().len(), count);
        let first = archive.members().next().unwrap().id;
        let edited = archive
            .edit_member(&first, b"Bessemer native member replacement")
            .unwrap()
            .archive()
            .to_bytes();
        edits.extend_from_slice(&(original.len() as u32).to_le_bytes());
        edits.extend_from_slice(&(edited.len() as u32).to_le_bytes());
        edits.extend_from_slice(original);
        edits.extend_from_slice(&edited);
        for member in archive.members() {
            let name_length = word(&mut raw);
            let payload_length = word(&mut raw);
            assert_eq!(member.name.as_bytes(), take(&mut raw, name_length));
            assert_eq!(
                archive.read_member(&member.id).unwrap(),
                take(&mut raw, payload_length)
            );
            members += 1;
        }
        println!("COMPOSER\t{index}\taccepted\t{count}");
    }
    assert!(raw.is_empty());
    assert_eq!(members, 550);
    std::fs::write("target/composer-native-edits.bin", edits).unwrap();
}
