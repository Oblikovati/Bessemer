use super::annotation_view_tests::{LIMITS, decode};
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-text-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-text-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-text-edited.bin");

#[test]
fn complete_dimension_text_matches_nested_array_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (128, 5760));
}

#[test]
fn dimension_text_truncation_and_trailing_bytes_never_publish_partial_results() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
}

#[test]
fn dimension_text_edit_matches_independent_wire_and_leaves_original_isolated() {
    let mut source = ORIGINAL.to_vec();
    let original = decode(19000, &source).unwrap();
    source.fill(0xa5);
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.unknown_130")
        .unwrap();
    field.value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn dimension_text_negative_and_impossible_nested_counts_fail_before_iteration() {
    for path in [
        "group_count",
        "group_sizes[0]",
        "point_counts_80[0]",
        "curve_count",
    ] {
        for value in [-1, i32::MAX] {
            let mut fields = decode(19000, ORIGINAL).unwrap();
            let name = format!("root.object.{path}");
            let field = fields
                .iter_mut()
                .find(|field| field.path.as_ref() == name)
                .unwrap();
            field.value = ModelFieldValue::I32(value);
            let bytes = encode_fields(&fields, 30000).unwrap();
            assert!(decode(19000, &bytes).is_err(), "{path}={value}");
        }
    }
}

fn decode_budget(maximum_entries: usize) -> Result<Vec<ModelField>> {
    let limits = super::ModelFieldLimits {
        maximum_entries,
        ..LIMITS
    };
    let mut reader = ModelDecoder::new_in_context(
        ORIGINAL,
        ArchiveVersion::new(19000),
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.object("object", &mut ObjectReader::new(), ModelObjectRole::Any)?;
    reader.finish()
}

#[test]
fn dimension_text_aggregate_budget_counts_each_array_entry_once() {
    assert!(decode_budget(19).is_ok());
    assert!(decode_budget(18).is_err());
}

#[test]
fn dimension_text_length_sums_reject_negative_values_and_usize_overflow() {
    use super::sketch_text::add_length;
    assert_eq!(add_length(7, 0, "lengths").unwrap(), 7);
    assert_eq!(add_length(7, 3, "lengths").unwrap(), 10);
    assert!(
        add_length(7, -1, "lengths")
            .unwrap_err()
            .cause
            .contains("-1")
    );
    assert!(add_length(usize::MAX, 1, "lengths").is_err());
}

#[test]
fn dimension_text_concurrent_writes_are_byte_identical() {
    let fields = decode(19000, ORIGINAL).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}
