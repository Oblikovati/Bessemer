use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;
use crate::header::ArchiveVersion;
type GeometryRecord = (i32, u8, &'static [u8], &'static str);
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
fn records() -> Vec<GeometryRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/deformations/geometry.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"DFG1");
    let count = cursor.u32().unwrap();
    let mut records = Vec::new();
    for _ in 0..count {
        let version = cursor.i32().unwrap();
        let style = cursor.u8().unwrap();
        let width = cursor.u32().unwrap();
        let size = cursor.u32().unwrap();
        let bytes = cursor.take(width as usize).unwrap();
        let oracle = std::str::from_utf8(cursor.take(size as usize).unwrap()).unwrap();
        records.push((version, style, bytes, oracle));
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(count, 66);
    records
}
fn decode(bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.scope("geometry", super::deform_geometry::read)?;
    reader.finish()
}
#[test]
fn deform_geometry_fixed_widths_match_independent_native_totals() {
    for (version, _, bytes, oracle) in records() {
        let expected = match version {
            i32::MIN..=0 => 372,
            1 => 380,
            2 => 396,
            3 => 516,
            4 => 576,
            5 => 584,
            6 => 588,
            _ => 592,
        };
        assert_eq!(bytes.len(), expected);
        let fields = decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.ends_with("words_f8_134[15]"))
                .unwrap()
                .value,
            ModelFieldValue::I32(version)
        );
    }
}
#[test]
fn deform_geometry_rejects_every_prefix_trailing_byte_and_perspective_operator() {
    for (_, _, bytes, _) in records() {
        for end in 0..bytes.len() {
            assert!(decode(&bytes[..end]).is_err());
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(&trailing).is_err());
    }
    let (_, _, bytes, _) = records()
        .into_iter()
        .find(|r| r.0 == 3 && r.1 == 2)
        .unwrap();
    let fields = decode(bytes).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.geometry.transform_5f0[28]",
        &0xf0_u32.to_le_bytes(),
    );
    assert!(decode(&changed).is_err());
}
#[test]
fn deform_geometry_requires_the_explicit_ordinary_archive_context() {
    let (_, _, bytes, _) = records()[0];
    let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    assert!(super::deform_geometry::read(&mut reader).is_err());
}
