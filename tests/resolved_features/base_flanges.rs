use super::annotation_view_tests::LIMITS;
use super::configuration_manager::SavedHelperState;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/base-flanges/cases.bin");
type FlangeRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<FlangeRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"BFL1");
    assert_eq!(cursor.u32().unwrap(), 202);
    let slots: Vec<_> = (0..202).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn state(kind: u8) -> SavedHelperState {
    if kind == 0 {
        return SavedHelperState::Inactive;
    }
    SavedHelperState::Active
}

#[test]
fn complete_flange_branches_match_independent_fields_bytes_and_object_slots() {
    for (kind, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) =
            decode_graph(version, bytes, Some(state(kind)), LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
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
fn flange_records_reject_every_truncation_trailing_bytes_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], Some(state(kind)), LIMITS).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), Some(state(kind)), LIMITS).is_err());
        assert!(decode_graph(0x27cb, bytes, Some(state(kind)), LIMITS).is_err());
    }
}

#[test]
fn flange_handle_counts_reject_negative_and_unbounded_collections() {
    for (kind, version, bytes, _, _) in records().into_iter().take(48) {
        let fields = decode_graph(version, bytes, Some(state(kind)), LIMITS)
            .unwrap()
            .0;
        for count in [-1i32, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.payload.handle_arrays.958.count",
                &count.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, Some(state(kind)), LIMITS).is_err());
        }
    }
}

#[test]
fn flange_declared_pointers_reject_cached_root_of_the_wrong_class() {
    for (kind, version, bytes, _, _) in records().into_iter().take(48) {
        let fields = decode_graph(version, bytes, Some(state(kind)), LIMITS)
            .unwrap()
            .0;
        for suffix in [
            "profile_888",
            "handle_arrays.958.sketch",
            "handle_arrays.958.handles[0]",
        ] {
            let label = format!("root.object.payload.{suffix}");
            if !fields.iter().any(|field| field.path.as_ref() == label) {
                continue;
            }
            let changed =
                super::manager_context_tests::replace_field(&fields, &label, &2u16.to_le_bytes());
            assert!(
                decode_graph(version, &changed, Some(state(kind)), LIMITS).is_err(),
                "{label}"
            );
        }
    }
}
