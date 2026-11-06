//! Independent record-108 body reconstruction and integer-only width oracle.
#[path = "flow_field_values/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_field_values/rebuild.rs"]
mod rebuild;
#[path = "flow_field_values/types.rs"]
mod types;
use std::io::Read;
use types::*;

#[test]
fn every_field_body_context_interval_and_raw_pattern_reconstructs_exactly() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 203);
    for case in cases {
        assert_eq!(
            rebuild::decoded(&case),
            case.frame.decoded,
            "{}",
            case.frame.name
        );
        assert_eq!(case.flags, [false, true, false, false]);
        check_transport(&case);
    }
}

fn check_transport(case: &FieldCase) {
    assert_eq!(&case.frame.stored[..4], case.version.to_le_bytes());
    let body = &case.frame.stored[4..];
    let mut decoded = Vec::new();
    if case.frame.compressed {
        flate2::read::ZlibDecoder::new(body)
            .read_to_end(&mut decoded)
            .unwrap();
    } else {
        decoded.extend_from_slice(body);
    }
    assert_eq!(decoded, case.frame.decoded);
    assert_eq!(case.body_end as usize, decoded.len());
    check_byte_counts(case, body.len(), decoded.len());
}

#[test]
fn every_native_width_outcome_matches_independent_bit_decomposition() {
    let cases = fixture::width_cases();
    assert_eq!(cases.len(), 315);
    assert_eq!(
        cases.iter().filter(|case| case.expected.is_some()).count(),
        289
    );
    for case in cases {
        assert_eq!(rebuild::projected_width(&case), case.expected);
    }
}

#[test]
fn native_version_prefix_and_conditional_object_layout_have_independent_totals() {
    for case in fixture::cases().into_iter().take(3) {
        assert_eq!((case.version, case.frame.stored.len()), (5, 301710));
        assert_eq!(case.byte_counts, [4, 301706, 377220, 377224]);
        let FieldProjection::Legacy { table, objects, .. } = case.projection else {
            panic!("native layout; expected legacy");
        };
        assert_eq!(table, [14]);
        check_native_objects(&objects);
        assert_eq!(4 + 3368 * 14 * 8, case.body_end);
    }
}

fn check_byte_counts(case: &FieldCase, stored: usize, decoded: usize) {
    assert_eq!(
        case.byte_counts,
        [4, stored as u32, decoded as u32, 4 + decoded as u32]
    );
}

fn check_native_objects(objects: &[FieldObject]) {
    assert_eq!(objects.len(), 5952);
    assert_eq!(
        objects.iter().filter(|object| object.width == 14).count(),
        3368
    );
    assert_eq!(
        objects.iter().filter(|object| object.width == 0).count(),
        2584
    );
    assert_eq!(
        objects
            .iter()
            .map(|object| object.bits.len())
            .sum::<usize>(),
        47152
    );
}
