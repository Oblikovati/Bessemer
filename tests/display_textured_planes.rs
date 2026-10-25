//! Documented textured-plane framing retains null bitmap pointers and complete transform bodies.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as PlaneCase;
#[allow(
    dead_code,
    reason = "Plane traces reuse the scalar/transform formatter; display-item traces have dedicated tests."
)]
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

const PLANES: &str = "root.textured_planes";

fn cases() -> Vec<PlaneCase> {
    complete_cases::decode(include_bytes!("fixtures/display-lists/textured-planes.bin"))
}

fn selected(scenario: u8) -> PlaneCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == 19000
                && case.kind == DisplayDocumentKind::Part
                && case.scenario == scenario
        })
        .unwrap()
}

fn case_context(case: PlaneCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: PlaneCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn plane_trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = view_trace::Trace::new();
    for field in &archive.contents().fields {
        if field.path.starts_with(PLANES) && matches!(field.value, DisplayFieldValue::Transform(_))
        {
            item_trace::insert(&mut rows, &field.path, &field.value);
        }
    }
    rows.into_iter()
        .filter(|(_, value)| !value.starts_with("a:"))
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

#[test]
fn all_textured_plane_snapshots_match_independent_transform_fields_and_exact_bytes() {
    assert_eq!(cases().len(), 15);
    let mut fields = 0;
    for case in cases() {
        assert!(case.scalars.is_empty());
        let archive = parsed(case);
        assert_eq!(plane_trace(&archive), case.expected);
        fields += case.expected.iter().filter(|byte| **byte == b'\n').count();
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 15261);
}

#[test]
fn all_flag_bytes_and_noncanonical_presence_values_retain_the_selected_payload() {
    let archive = parsed(selected(4));
    let expected = format!("{PLANES}.count=i:1024\n");
    assert_eq!(archive_support::check_scalar_table(&archive, &expected), 1);
    let transforms: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::Transform(value) if field.path.starts_with(PLANES) => Some(value),
            _ => None,
        })
        .collect();
    assert_eq!(transforms.len(), 1024);
    for (index, transform) in transforms.iter().enumerate() {
        assert_eq!(transform.has_matrix, [0, 1, 2, 255][index / 256]);
        assert_eq!(transform.matrix.is_some(), index >= 256);
        assert_eq!(transform.flags, (index % 256) as u8);
        assert!(match (transform.flags & 0xf0, &transform.perspective) {
            (0x10, PerspectivePayload::Base(_))
            | (0x20, PerspectivePayload::Extended { .. })
            | (
                0x40,
                PerspectivePayload::Matrix5 {
                    discarded: -1234567,
                    ..
                },
            ) => true,
            (0x10 | 0x20 | 0x40, _) => false,
            (_, PerspectivePayload::None) => true,
            _ => false,
        });
    }
    let pointers: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter(|field| field.path.starts_with(PLANES) && field.path.ends_with(".sketch_bitmap"))
        .collect();
    assert_eq!(pointers.len(), 1024);
    assert!(pointers.iter().all(|field|matches!(&field.value,DisplayFieldValue::ObjectTag(tag) if tag.target==DisplayObjectPointer::Null)));
}

fn edited_translation(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let path = format!("{PLANES}.items[0].xform");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::Transform(transform) = &mut field.value else {
        panic!("compact plane transform")
    };
    transform.translation[0] = StoredDouble::from_bits(23.0_f64.to_bits());
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_transform_edit_reopens_preserves_other_streams_and_reverts() {
    let case = selected(0);
    let archive = parsed(case);
    let edited = edited_translation(&archive);
    assert_eq!(edited.to_bytes().unwrap(), selected(1).bytes);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    let initial = include_bytes!("fixtures/blank-part.SLDPRT");
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let original = bessemer::Document::from_bytes(initial).unwrap();
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        archive_support::decode_document(&reopened.to_bytes().unwrap(), DisplayDocumentKind::Part)
            .contents(),
        edited.contents()
    );
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}

fn budget(case: PlaneCase) -> (usize, usize) {
    let assembly = case.kind == DisplayDocumentKind::Assembly;
    let (base, objects) = match case.version {
        7000 => {
            if assembly {
                (96, 54)
            } else {
                (88, 52)
            }
        }
        9000 => {
            if assembly {
                (108, 54)
            } else {
                (100, 52)
            }
        }
        10000 => {
            if assembly {
                (95, 54)
            } else {
                (87, 52)
            }
        }
        11000 => {
            if assembly {
                (99, 54)
            } else {
                (91, 52)
            }
        }
        18000 => (68, 49),
        _ => {
            if assembly {
                (53, 46)
            } else {
                (134, 49)
            }
        }
    };
    let planes = match case.scenario {
        2 => 1,
        3 => 0,
        4 => 1024,
        _ => 4,
    };
    (base + planes, objects)
}

#[test]
fn exact_budgets_include_planes_without_allocating_bitmap_objects() {
    for case in cases() {
        let archive = parsed(case);
        let (entries, objects) = budget(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: objects - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn new_bitmap_bodies_remain_unsupported_and_unrelated_classes_cannot_fill_the_pointer() {
    let case = selected(2);
    let pointer_at = case.bytes.len() - 68;
    assert_eq!(
        &case.bytes[pointer_at - 4..pointer_at],
        &1_i32.to_le_bytes()
    );
    assert_eq!(&case.bytes[pointer_at..pointer_at + 2], &[0, 0]);
    for (name, expected) in [
        ("moSketchBitmap_c", "Unsupported"),
        ("moNodeName_c", "Invalid"),
    ] {
        let mut changed = case.bytes[..pointer_at].to_vec();
        changed.extend_from_slice(&[255, 255, 1, 0]);
        changed.extend_from_slice(&(name.len() as u16).to_le_bytes());
        changed.extend_from_slice(name.as_bytes());
        changed.extend_from_slice(&case.bytes[pointer_at + 2..]);
        let error = DisplayArchive::from_bytes(&changed, case_context(case), LIMITS).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        assert!(error.to_string().contains(name));
    }
    let mut alias = case.bytes.to_vec();
    alias[pointer_at..pointer_at + 2].copy_from_slice(&51_u16.to_le_bytes());
    let error = DisplayArchive::from_bytes(&alias, case_context(case), LIMITS).unwrap_err();
    assert!(error.to_string().contains("moVisualProperties_c"));
}

#[test]
fn negative_impossible_counts_truncations_and_trailing_bytes_fail_closed() {
    let case = selected(2);
    let archive = parsed(case);
    for value in [-1, i32::MAX] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, &format!("{PLANES}.count"), value);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut trailing = case.bytes.to_vec();
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
}

#[test]
fn plane_archives_own_sources_and_parallel_edits_match_independent_bytes() {
    let case = selected(0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_translation(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
