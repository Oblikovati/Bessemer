//! Complete conditional flat EGR framing, with stored live/deleted/unallocated states retained.
use bessemer::simulation::*;
use std::io::Read;
#[path = "common/egr_packets.rs"]
mod checks;
#[path = "common/egr_export.rs"]
mod export;
#[path = "common/egr_failures.rs"]
mod failures;
#[path = "common/egr_groups.rs"]
mod groups;
#[path = "common/egr_names.rs"]
mod names;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
#[path = "common/egr_ranges.rs"]
mod ranges;
use packet::Cursor;

fn limits() -> SimulationEgrLimits {
    SimulationEgrLimits { maximum_rows: 1000 }
}
fn profile() -> SimulationEgrProfile {
    SimulationEgrProfile::ConstructorBoundFlatCFile
}

fn owned() -> Vec<u8> {
    inflate(include_bytes!("fixtures/simulation-egr/tables.bin"))
}

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut raw)
        .unwrap();
    raw
}

#[test]
fn all_complete_owned_metadata_profiles_reconstruct_exactly_and_match_actual_reader_regions() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"EGR1");
    assert_eq!(cursor.word(), 14);
    for _ in 0..14 {
        owned_table(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn owned_table(cursor: &mut Cursor<'_>) {
    let raw = cursor.span();
    let image = checks::table(raw);
    let parts = image.parts();
    let rebuilt = SimulationEgrArchive::from_parts(parts.clone(), profile(), limits()).unwrap();
    assert_eq!(rebuilt.to_bytes(), raw);
    assert_eq!(rebuilt.parts(), parts);
    assert_ne!(rebuilt.revision(), image.revision());
    assert_eq!(image.revision(), image.clone().revision());
    assert_eq!(image.profile(), profile());
    assert_eq!(image.limits().maximum_rows, 1000);
    checks::metadata(&image, cursor.span());
    ranges::assert_identity_guards(&image);
    assert_descriptor(&image, raw, cursor.span());
}

#[test]
fn all_egroup_integer_permutations_wrapping_and_incomplete_name_states_match_actual_reader() {
    let raw = inflate(include_bytes!("fixtures/simulation-egr/groups.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"EGRO");
    assert_eq!(cursor.word(), 936);
    let mut totals = [0usize; 3];
    for _ in 0..936 {
        groups::group_case(&mut cursor, &mut totals);
    }
    assert_eq!(totals, [468, 396, 72]);
    assert!(cursor.0.is_empty());
}

#[test]
fn every_live_field_interval_and_isolated_edit_matches_independent_complete_file_goldens() {
    let raw = inflate(include_bytes!("fixtures/simulation-egr/fields.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"EGRF");
    assert_eq!(cursor.word(), 14);
    let mut total = 0;
    for _ in 0..14 {
        let encoded = cursor.span();
        let image = checks::table(encoded);
        let count = cursor.word();
        for _ in 0..count {
            ranges::field_case(&image, &mut cursor);
        }
        total += count;
        assert_eq!(image.to_bytes(), encoded);
    }
    assert_eq!(total, 3992);
    assert!(cursor.0.is_empty());
}

#[test]
fn all_solver_name_codes_destinations_and_null_source_states_match_actual_reader_projections() {
    let raw = inflate(include_bytes!("fixtures/simulation-egr/names.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"EGRN");
    assert_eq!(cursor.word(), 183);
    let mut totals = [0usize; 2];
    for _ in 0..183 {
        names::name_case(&mut cursor, &mut totals);
    }
    assert_eq!(totals, [150, 33]);
    assert!(cursor.0.is_empty());
}

#[test]
fn every_file_prefix_and_extra_byte_fails_without_publishing_partial_regions() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    cursor.take(8);
    for _ in 0..14 {
        let encoded = cursor.span();
        for length in 0..encoded.len() {
            assert!(
                SimulationEgrArchive::from_bytes(&encoded[..length], profile(), limits()).is_err()
            );
        }
        let mut extra = encoded.to_vec();
        extra.push(0);
        assert!(SimulationEgrArchive::from_bytes(&extra, profile(), limits()).is_err());
        cursor.span();
        cursor.span();
    }
}

fn assert_descriptor(image: &SimulationEgrArchive, raw: &[u8], expected: &[u8]) {
    let descriptor = image.descriptor_writer_projection();
    assert_eq!(descriptor.projected_descriptor_bytes.as_ref(), expected);
    assert_eq!(
        descriptor.original_descriptor_bytes.as_ref(),
        &raw[128..128 + image.fields().len() * 8]
    );
    assert_eq!(descriptor.alignment_bytes.as_ref(), image.field_alignment());
    assert_eq!(
        descriptor.descriptor_bytes_changed,
        descriptor.original_descriptor_bytes != descriptor.projected_descriptor_bytes
    );
    assert!(!descriptor.complete_native_save);
    assert_eq!(descriptor.source.to_bytes(), raw);
}
