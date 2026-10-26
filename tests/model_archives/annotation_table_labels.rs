use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, encode_fields};
use super::{annotation_defaults, label_defaults, table_defaults};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/annotation-table-label-cases.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 3000,
    maximum_entries: 3000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    match kind {
        0 => annotation_defaults::read(&mut reader)?,
        1 => table_defaults::read(&mut reader)?,
        2 => label_defaults::read(&mut reader)?,
        _ => panic!("unexpected defaults fixture kind {kind}"),
    }
    reader.finish()
}

const NATIVE_GROUPS: [(u8, &[u8], &str, usize); 3] = [
    (
        0,
        include_bytes!("../fixtures/model-archives/native-annotation-defaults.bin"),
        include_str!("../fixtures/model-archives/native-annotation-defaults.txt"),
        325,
    ),
    (
        1,
        include_bytes!("../fixtures/model-archives/native-table-defaults.bin"),
        include_str!("../fixtures/model-archives/native-table-defaults.txt"),
        234,
    ),
    (
        2,
        include_bytes!("../fixtures/model-archives/native-label-defaults.bin"),
        include_str!("../fixtures/model-archives/native-label-defaults.txt"),
        463,
    ),
];

#[test]
fn all_default_groups_match_independent_scalar_cases_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (33, 10188));
}

#[test]
fn owned_annotation_table_and_label_groups_match_every_native_field() {
    for (kind, bytes, oracle, fields) in NATIVE_GROUPS {
        let decoded = decode(kind, 19000, bytes).unwrap();
        assert_eq!(super::node_units::assert_oracle(&decoded, oracle), fields);
        assert_eq!(encode_fields(&decoded, 10000).unwrap(), bytes);
    }
}

#[test]
fn every_default_case_prefix_and_trailing_byte_rejects_partial_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}

#[test]
fn owned_defaults_survive_input_changes_and_parallel_reconstruction() {
    for (kind, bytes, _, _) in NATIVE_GROUPS {
        let mut source = bytes.to_vec();
        let fields = decode(kind, 19000, &source).unwrap();
        source.fill(0xa5);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..4)
                .map(|_| scope.spawn(|| encode_fields(&fields, 10000).unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), bytes);
            }
        });
    }
}
