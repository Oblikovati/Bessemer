use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/feature-comments/cases.bin");
type CommentRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<CommentRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"FCM1");
    assert_eq!(cursor.u32().unwrap(), 48);
    let slots: Vec<_> = (0..48).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

#[test]
fn comment_objects_match_every_independent_primitive_reconstructed_byte_and_slot() {
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn comment_objects_reject_every_prefix_trailing_byte_and_old_context() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn comment_owners_reject_cached_comment_objects_and_direct_extra_has_no_slot() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, slots)| *kind == 0 && *version == 19000 && *slots == 3)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    for label in ["feature_78", "chain_80"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("moNewFeatComment_c"),
            "{}",
            error.cause
        );
    }
}

#[test]
fn comment_text_times_use_four_unsigned_bytes_and_extra_order_is_native() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode(version, bytes).unwrap();
        if kind == 2 {
            let time = fields
                .iter()
                .find(|field| field.path.as_ref() == "root.object.time_18")
                .unwrap();
            assert_eq!(
                super::encode_fields(std::slice::from_ref(time), 30000)
                    .unwrap()
                    .len(),
                4
            );
        }
        if kind == 1 && version >= 0x3b10 {
            let names: Vec<_> = fields
                .iter()
                .skip(1)
                .map(|field| field.path.as_ref())
                .collect();
            assert_eq!(
                names,
                [
                    "root.object.raw_0c",
                    "root.object.raw_08",
                    "root.object.raw_10"
                ]
            );
        }
    }
}
