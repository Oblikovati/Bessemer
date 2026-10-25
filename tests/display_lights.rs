//! Modern old-light context, independent scalar oracles and isolated complete-stream edits.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/light_trace.rs"]
mod light_trace;
const SPOT: &[u8] = include_bytes!("fixtures/display-lists/lights-spot.bin");
const POINT: &[u8] = include_bytes!("fixtures/display-lists/lights-point.bin");
const EMPTY: &[u8] = include_bytes!("fixtures/display-lists/lights-empty-properties.bin");
const EDITED: &[u8] = include_bytes!("fixtures/display-lists/lights-edited.bin");
const ROOT: &str = "root.old_lights.items[1]";

fn parsed(bytes: &[u8]) -> DisplayArchive {
    DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap()
}

#[test]
fn complete_spot_point_and_empty_property_layouts_match_independent_fields_and_bytes() {
    let fixtures: [(&[u8], &[u8]); 4] = [
        (
            SPOT,
            include_bytes!("fixtures/display-lists/lights-spot.txt"),
        ),
        (
            POINT,
            include_bytes!("fixtures/display-lists/lights-point.txt"),
        ),
        (
            EMPTY,
            include_bytes!("fixtures/display-lists/lights-empty-properties.txt"),
        ),
        (
            EDITED,
            include_bytes!("fixtures/display-lists/lights-edited.txt"),
        ),
    ];
    for (bytes, expected) in fixtures {
        let archive = parsed(bytes);
        assert_eq!(light_trace::trace(&archive), expected);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn base_flags_property_variants_and_signed_zero_remain_raw_observations() {
    let archive = parsed(SPOT);
    let table = "root.old_lights.items[1].enabled_in_pv_initial=i:-5\nroot.old_lights.items[1].keep_light_in_scene=i:6\nroot.old_lights.items[1].enabled_in_pv=i:-7\nroot.old_lights.items[1].pv_fog_enabled=i:-10\nroot.old_lights.items[1].user_property.count=i:4\nroot.old_lights.items[1].user_property.entries[1].value=i:-123\nroot.old_lights.items[1].user_property.entries[3].type=i:4294967295\n";
    assert_eq!(archive_support::check_scalar_table(&archive, table), 7);
    let path = format!("{ROOT}.user_property.entries[0].value");
    let value = &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value;
    let DisplayFieldValue::F32(value) = value else {
        panic!("stored float")
    };
    assert_eq!(value.bits(), 0x80000000);
    let absent = format!("{ROOT}.user_property.entries[3].value");
    assert!(
        !archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.as_ref() == absent)
    );
    let archive = parsed(EMPTY);
    assert_eq!(
        archive_support::check_scalar_table(
            &archive,
            "root.old_lights.items[1].user_property.count=i:4294967295\n"
        ),
        1
    );
}

#[test]
fn explicit_empty_count_spellings_keep_empty_semantics_without_normalization() {
    let archive = parsed(EMPTY);
    for count in [0, 0x80000000, 0x80000001, u32::MAX] {
        let mut contents = archive.contents().clone();
        let path = format!("{ROOT}.user_property.count");
        contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = DisplayFieldValue::U32(count);
        let changed = archive.with_contents(contents, LIMITS).unwrap();
        assert_eq!(light_trace::trace(&changed), light_trace::trace(&archive));
    }
}

fn edited_exponent(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let path = format!("{ROOT}.exponent");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_edit_reopens_in_document_preserves_other_streams_and_reverts() {
    let archive = parsed(SPOT);
    let edited = edited_exponent(&archive);
    assert_eq!(edited.to_bytes().unwrap(), EDITED);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        SPOT
    );
    let original =
        bessemer::Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let written = document.to_bytes().unwrap();
    assert_eq!(
        archive_support::decode_document(&written, DisplayDocumentKind::Part).contents(),
        edited.contents()
    );
    let reopened = bessemer::Document::from_bytes(&written).unwrap();
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
}

#[test]
fn exact_budgets_and_every_new_light_prefix_are_checked() {
    for (bytes, end, entries) in [(SPOT, 3868, 57), (POINT, 3789, 57), (EMPTY, 3810, 53)] {
        for length in 3507..end {
            assert!(DisplayArchive::from_bytes(&bytes[..length], context(), LIMITS).is_err());
        }
        let archive = parsed(bytes);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_bytes: bytes.len(),
            maximum_fields: archive.contents().fields.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(bytes, context(), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), context(), exact).is_ok());
        for limited in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: bytes.len() - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(bytes, context(), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn impossible_property_counts_and_changed_variant_shapes_cannot_publish_a_partial_light() {
    let archive = parsed(SPOT);
    for (suffix, value) in [
        ("count", 1),
        ("count", 0x7fffffff),
        ("entries[0].type", 2),
        ("entries[3].type", 0),
    ] {
        let mut contents = archive.contents().clone();
        let path = format!("{ROOT}.user_property.{suffix}");
        contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = DisplayFieldValue::U32(value);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, "root.old_lights.has_lights", 0);
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), SPOT);
}

#[test]
fn lights_own_source_bytes_and_parallel_edits_match_the_independent_result() {
    let mut source = SPOT.to_vec();
    let archive = parsed(&source);
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_exponent(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), EDITED);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), SPOT);
}
