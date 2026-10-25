//! Complete markup-ink grids retain dimensions, states and nonminimal strings.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as InkCase;

const INK: &str = "root.markup_ink_views";

fn cases() -> Vec<InkCase> {
    complete_cases::decode(include_bytes!("fixtures/display-lists/markup-ink.bin"))
}

fn selected(scenario: u8) -> InkCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == 19000
                && case.kind == DisplayDocumentKind::Part
                && case.scenario == scenario
        })
        .unwrap()
}

fn case_context(case: InkCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: InkCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_populated_and_empty_ink_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 9);
    let mut compared = 0;
    for case in cases() {
        assert!(case.expected.is_empty());
        let archive = parsed(case);
        compared += archive_support::check_scalar_table(&archive, case.scalars);
        let actual = archive
            .contents()
            .fields
            .iter()
            .filter(|field| field.path.starts_with(&format!("{INK}.views[")))
            .count();
        assert_eq!(actual, case.scalars.lines().count());
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(compared, 104);
}

#[test]
fn empty_rows_dimensions_minimal_strings_and_signed_state_values_are_not_normalized() {
    let populated = parsed(selected(0));
    let table = format!(
        "{INK}.views[0].configurations[0].suppressed_state=i:-2147483648\n{INK}.views[0].configurations[1].suppressed_state=i:2147483647\n{INK}.views[0].configurations[0].is_shown=i:-3\n"
    );
    assert_eq!(archive_support::check_scalar_table(&populated, &table), 3);
    for (scenario, views, configs) in [(2, 0, 7), (3, 2, 0), (4, 0, 0), (5, 1, 1), (6, 1, 0)] {
        let archive = parsed(selected(scenario));
        let table = format!("{INK}.view_count=i:{views}\n{INK}.configuration_count=i:{configs}\n");
        assert_eq!(archive_support::check_scalar_table(&archive, &table), 2);
        assert_eq!(
            ink_text_count(&archive),
            views as usize * (2 + configs as usize)
        );
    }
}

fn ink_text_count(archive: &DisplayArchive) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|field| {
            field.path.starts_with(&format!("{INK}.views["))
                && matches!(field.value, DisplayFieldValue::Text(_))
        })
        .count()
}

fn edited_state(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        &format!("{INK}.views[0].configurations[0].is_shown"),
        23,
    );
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_state_edit_reopens_without_other_stream_changes_and_reverts() {
    let case = selected(0);
    let archive = parsed(case);
    let edited = edited_state(&archive);
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

#[test]
fn exact_ink_grid_budgets_include_dimensions_and_every_configuration_row() {
    for case in cases() {
        let archive = parsed(case);
        let extra = match case.scenario {
            2 => 7,
            3 => 2,
            4 => 0,
            5 => 3,
            6 => 1,
            _ => 8,
        };
        // The source roots charge a zero-view, one-configuration ink dimension.
        let base = if case.version == 18000 { 67 } else { 52 };
        let objects = if case.version == 18000 { 49 } else { 46 };
        let exact = DisplayArchiveLimits {
            maximum_entries: base + extra,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {:?} {}",
            case.version,
            case.kind,
            case.scenario
        );
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_entries: exact.maximum_entries - 1,
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
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} {} limits {limits:?}",
                case.version,
                case.kind,
                case.scenario
            );
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn invalid_dimensions_shape_changes_prefixes_and_trailing_bytes_fail_closed() {
    let case = selected(5);
    let archive = parsed(case);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    for dimension in ["view_count", "configuration_count"] {
        for value in [-1, 0, i32::MAX] {
            let mut contents = archive.contents().clone();
            archive_support::set_integer(&mut contents, &format!("{INK}.{dimension}"), value);
            assert!(
                archive.with_contents(contents, LIMITS).is_err(),
                "{dimension}={value}"
            );
        }
    }
    let mut trailing = case.bytes.to_vec();
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
}

#[test]
fn ink_results_own_sources_and_concurrent_edits_match_independent_bytes() {
    let case = selected(0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_state(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn grid_product_is_checked_against_wire_bytes_and_older_layouts_omit_ink() {
    let archive = parsed(selected(5));
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{INK}.view_count"), 2);
    archive_support::set_integer(&mut contents, &format!("{INK}.configuration_count"), 2);
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.to_string().contains("ink dimensions 2 x 2"));
    for case in complete_cases::decode(include_bytes!("fixtures/display-lists/old-shaded.bin")) {
        let archive = parsed(case);
        assert!(
            archive
                .contents()
                .fields
                .iter()
                .all(|field| !field.path.starts_with(INK))
        );
    }
}
