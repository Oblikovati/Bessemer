use super::header_imports::ModelHeaderLoadPlan;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, encode_fields, lightweight,
};
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;
use crate::{Document, Result};
use std::sync::Arc;

const VERSION: ArchiveVersion = ArchiveVersion::new(19000);
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 10000,
    maximum_fields: 3000,
    maximum_entries: 3000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn native_lightweight() -> Vec<u8> {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document.read_stream("Contents/Config-0").unwrap()[..2921].to_vec()
}

fn decode(bytes: &[u8]) -> Result<Vec<ModelField>> {
    let header = Arc::new(super::header_load_plan::native_header());
    let plan = ModelHeaderLoadPlan::new(header, VERSION, LIMITS.maximum_entries)?;
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        VERSION,
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    let imports = lightweight::read(&mut reader, &mut map, plan)?;
    assert_eq!(imports.imports.slots().len(), 1);
    assert_eq!(imports.node.unknown_2c, Some(0x8100));
    assert_eq!(imports.raw_1614, 0);
    reader.finish()
}

#[test]
fn complete_owned_lightweight_section_reconstructs_without_source_bytes() {
    let mut bytes = native_lightweight();
    let fields = decode(&bytes).unwrap();
    let reconstructed = encode_fields(&fields, LIMITS.maximum_bytes).unwrap();
    assert_eq!(reconstructed, bytes);
    bytes.fill(0xa5);
    assert_eq!(
        encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
        reconstructed
    );
    assert_eq!(decode(&reconstructed).unwrap(), fields);
}

#[test]
fn native_lightweight_fields_match_reviewed_reference_values() {
    let fields = decode(&native_lightweight()).unwrap();
    for (path, expected) in reviewed_values() {
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.as_ref() == path)
                .unwrap()
                .value,
            expected
        );
    }
}

fn reviewed_values() -> [(&'static str, ModelFieldValue); 5] {
    [
        ("root.lightweight_size", ModelFieldValue::I32(2903)),
        (
            "root.spatial.reference_plane.back_face_color",
            ModelFieldValue::U32(6759173),
        ),
        (
            "root.manager.unknown_110",
            ModelFieldValue::I32(-1431655766),
        ),
        (
            "root.manager.unknown_114",
            ModelFieldValue::I32(-1145324613),
        ),
        ("root.model_tail.unknown_12f8", ModelFieldValue::U32(101)),
    ]
}

#[test]
fn all_native_lightweight_scalar_fields_match_the_independent_oracle() {
    let fields = decode(&native_lightweight()).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-lightweight.txt");
    let count = super::node_units::assert_oracle(&fields, oracle);
    assert_eq!(count, 477);
}

#[test]
fn every_adjusted_native_prefix_still_rejects_incomplete_lightweight_bodies() {
    let bytes = native_lightweight();
    for length in 18..bytes.len() {
        let mut prefix = bytes[..length].to_vec();
        prefix[14..18].copy_from_slice(&((length - 18) as i32).to_le_bytes());
        assert!(decode(&prefix).is_err(), "prefix {length}");
    }
}

#[test]
fn independent_parallel_lightweight_maps_produce_identical_owned_fields() {
    let bytes = native_lightweight();
    let expected = decode(&bytes).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| decode(&bytes).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

#[test]
fn lightweight_size_truncation_and_trailing_input_never_publish() {
    let mut bytes = native_lightweight();
    for length in [0, 13, 17, 18, 100, 2440, 2920] {
        assert!(decode(&bytes[..length]).is_err());
    }
    bytes.push(0);
    assert!(decode(&bytes).is_err());
    for size in [-1_i32, 0, 2902, 2904, i32::MAX] {
        let mut altered = native_lightweight();
        altered[14..18].copy_from_slice(&size.to_le_bytes());
        assert!(decode(&altered).is_err(), "declared {size}");
    }
}

#[test]
fn ordinary_model_context_is_explicit_and_unsupported_versions_poison_reads() {
    for (version, context) in [
        (19000, ModelArchiveContext::Unspecified),
        (0x27cb, ModelArchiveContext::OrdinaryPartExternalXml),
    ] {
        let mut reader =
            ModelDecoder::new_in_context(&[], ArchiveVersion::new(version), LIMITS, context)
                .unwrap();
        let error = reader.require_ordinary_part().unwrap_err();
        assert_eq!(reader.finish().unwrap_err(), error);
    }
}
