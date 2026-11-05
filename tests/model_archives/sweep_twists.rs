use super::annotation_view_tests::decode;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/sweep-twist/bare-cases.bin");
type TwistCase = (u32, [u8; 17], &'static [u8]);

fn cases() -> Vec<TwistCase> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"BWT1");
    assert_eq!(cursor.u32().unwrap(), 6144);
    let result = (0..6144).map(|_| next_twist(&mut cursor)).collect();
    assert_eq!(cursor.remaining(), 0);
    result
}

fn next_twist(cursor: &mut ArchiveCursor<'static>) -> TwistCase {
    let version = cursor.u32().unwrap();
    let expected: [u8; 17] = cursor.take(17).unwrap().try_into().unwrap();
    let length = cursor.u32().unwrap() as usize;
    (version, expected, cursor.take(length).unwrap())
}

#[test]
fn all_sweep_twist_bytes_and_ieee_patterns_match_owned_wire_and_slots() {
    for (version, expected, raw) in cases() {
        let (fields, slots) = decode_graph(version, raw).unwrap();
        assert_eq!((fields.len(), slots), (4, 3));
        assert_eq!(super::encode_fields(&fields[1..], 17).unwrap(), expected);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        for (field, suffix) in fields[1..].iter().zip(["raw_08", "raw_10", "raw_18"]) {
            assert!(field.path.ends_with(suffix));
        }
    }
}

#[test]
fn sweep_twist_proper_prefixes_trailing_bytes_and_unproved_schemas_fail() {
    for (version, _, raw) in cases().into_iter().step_by(1536) {
        for end in 0..raw.len() {
            assert!(decode(version, &raw[..end]).is_err());
        }
        assert!(decode(version, &[raw, &[0]].concat()).is_err());
        assert!(decode(0x27cb, raw).is_err());
        for schema in [0u16, 2, u16::MAX] {
            let mut changed = raw.to_vec();
            changed[2..4].copy_from_slice(&schema.to_le_bytes());
            let error = decode(version, &changed).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Unsupported);
            assert!(error.cause.contains(&schema.to_string()));
        }
    }
}

fn twist_with_limits(
    raw: &[u8],
    limits: super::ModelFieldLimits,
) -> crate::Result<Vec<super::ModelField>> {
    super::direct_collection_tests::decode_limited(19000, raw, limits, |reader, map| {
        reader.object("twist", map, super::ModelObjectRole::Any)
    })
}

#[test]
fn sweep_twist_enforces_byte_field_depth_and_object_budgets() {
    let (_, _, raw) = cases().pop().unwrap();
    let limits = super::annotation_view_tests::LIMITS;
    assert!(twist_with_limits(raw, limits).is_ok());
    let mut bounded = [limits; 4];
    bounded[0].maximum_bytes = raw.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_depth = 0;
    bounded[3].maximum_objects = 0;
    for limit in bounded {
        assert!(twist_with_limits(raw, limit).is_err());
    }
}
