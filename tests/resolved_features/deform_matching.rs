use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, encode_fields};
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;
const BYTES: &[u8] = include_bytes!("../fixtures/deformations/matching.bin");
const ORACLE: &str = include_str!("../fixtures/deformations/matching.tsv");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
fn decode(bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    let counts = super::deform_matching::first(&mut reader, &mut map)?;
    assert_eq!(counts, [1]);
    assert_eq!(bytes.len() - reader.remaining(), 144);
    super::deform_matching::second(&mut reader, &mut map, 10, &counts)?;
    reader.finish()
}
#[test]
fn deform_minimum_matching_records_keep_nulls_and_direct_widths() {
    assert_eq!(BYTES.len(), 144 + 68);
    let fields = decode(BYTES).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        BYTES,
        ORACLE,
        ORACLE.lines().count(),
    );
    assert_eq!(encode_fields(&fields, 30000).unwrap(), BYTES);
}
#[test]
fn deform_minimum_matching_records_reject_every_prefix_and_extra_byte() {
    for end in 0..BYTES.len() {
        assert!(decode(&BYTES[..end]).is_err());
    }
    let mut changed = BYTES.to_vec();
    changed.push(0);
    assert!(decode(&changed).is_err());
}
