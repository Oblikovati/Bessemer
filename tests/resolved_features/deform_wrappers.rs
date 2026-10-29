use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, solid_reference_tests::decode_graph};
use crate::archive_codec::ObjectRecord;
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/deformations/callers/cases.bin"),
        b"CPW1",
        30,
    )
}
pub(super) fn fresh_wrapper_class(caller: u8, path: &str) -> String {
    let (_, version, bytes, _, _) = records().into_iter().find(|r| r.0 == caller * 6).unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected real wrapper tag")
    };
    let ObjectRecord::New { class, .. } = &tag.record else {
        panic!("expected full fresh curve-point body")
    };
    assert_eq!(class.name, "moCurvePointRef_w");
    class.name.clone()
}
#[test]
fn curve_point_wrapper_callers_match_all_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}
#[test]
fn curve_point_wrapper_callers_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn curve_point_keeps_fresh_and_cached_wrapper_and_point_roles() {
    let records = super::trim_edge_identifier_tests::records_with_count(
        include_bytes!("../fixtures/deformations/cases.bin"),
        b"DFM1",
        384,
    );
    let (_, version, original, _, _) = records
        .into_iter()
        .find(|r| r.0 == 92 && r.1 == 19000)
        .unwrap();
    let mut bytes = original.to_vec();
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    for classes in [
        super::model_references::WRAPPER_CLASSES,
        super::geometric_references::POINT_REFERENCE_CLASSES,
    ] {
        check_parent_roles(version, &bytes, classes);
    }
}
fn check_parent_roles(version: u32, bytes: &[u8], classes: &[&str]) {
    use super::{ModelArchiveLimits, ModelDecoder, ModelObjectRole};
    let limits = ModelArchiveLimits {
        maximum_bytes: 30000,
        maximum_fields: 10000,
        maximum_entries: 10000,
        maximum_depth: 64,
        maximum_objects: 256,
    };
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        crate::header::ArchiveVersion::new(version),
        limits,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    let mut map = crate::archive_codec::ObjectReader::new();
    reader
        .object("fresh", &mut map, ModelObjectRole::Declared(classes))
        .unwrap();
    reader
        .object("cached", &mut map, ModelObjectRole::Declared(classes))
        .unwrap();
    assert_eq!(
        super::encode_fields(&reader.finish().unwrap(), 30000).unwrap(),
        bytes
    );
}
