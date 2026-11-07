//! Complete shared line-style and layer-manager objects retain every stored field and alias.
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared fixture helpers also support scalar integer mutation tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as UiCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared view formatting is also tested through view-specific fixtures."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

const PREFIX: &str = "root.scene.concept.feature_list.nodes[1]";

fn cases() -> Vec<UiCase> {
    [
        include_bytes!("fixtures/display-lists/line-styles.bin").as_slice(),
        include_bytes!("fixtures/display-lists/layer-maps.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(complete_cases::decode)
    .chain(component_properties::cases())
    .collect()
}

fn selected(version: u32, kind: DisplayDocumentKind, scenario: u8) -> UiCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: UiCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: UiCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn ui_field<'a>(archive: &'a DisplayArchive, suffix: &str) -> &'a DisplayFieldValue {
    &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == format!("{PREFIX}.{suffix}"))
        .unwrap()
        .value
}

fn edited(archive: &DisplayArchive, layers: bool) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let leaf = if layers {
        "attributes[1].value"
    } else {
        "segments[5]"
    };
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("{PREFIX}.{leaf}"))
        .unwrap();
    field.value = if layers {
        DisplayFieldValue::I32(23)
    } else {
        DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()))
    };
    archive.with_contents(contents, LIMITS).unwrap()
}

fn check_trace(actual: &[u8], expected: &[u8], case: UiCase) {
    for (index, (actual, expected)) in actual
        .split(|byte| *byte == b'\n')
        .zip(expected.split(|byte| *byte == b'\n'))
        .enumerate()
    {
        assert_eq!(
            std::str::from_utf8(actual).unwrap(),
            std::str::from_utf8(expected).unwrap(),
            "{} {:?} {} row {index}",
            case.version,
            case.kind,
            case.scenario
        );
    }
    assert_eq!(
        actual.len(),
        expected.len(),
        "{} {:?} {} trace length",
        case.version,
        case.kind,
        case.scenario
    );
}

#[test]
fn all_ui_objects_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 149);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        trace.extend(item_trace::trace(&archive));
        check_trace(&trace, case.expected, case);
        fields += case
            .expected
            .iter()
            .filter(|value| **value == b'\n')
            .count();
        fields += archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        let definition = archive
            .contents()
            .fields
            .iter()
            .find(|field| field.path.as_ref() == PREFIX)
            .unwrap();
        let DisplayFieldValue::ObjectTag(tag) = &definition.value else {
            panic!("expected object tag")
        };
        let DisplayObjectPointer::New(id) = tag.target else {
            panic!("expected loaded object")
        };
        let handle = archive.object_handle(id).unwrap();
        assert_eq!(archive.resolve_object(&handle).unwrap(), definition);
        let alias = archive
            .contents()
            .fields
            .iter()
            .find(|field| field.path.as_ref() == "root.scene.floor_selection")
            .unwrap();
        let DisplayFieldValue::ObjectTag(alias) = &alias.value else {
            panic!("expected alias tag")
        };
        assert_eq!(alias.target, DisplayObjectPointer::Reference(id));
        assert_eq!(alias.runtime_class, tag.runtime_class);
    }
    assert_eq!(fields, 128607);
}

#[test]
fn old_layer_counts_keep_short_dword_and_qword_spelling_and_raw_attributes() {
    for scenario in [16, 19, 20, 21] {
        let archive = parsed(selected(0x1be4, DisplayDocumentKind::Part, scenario));
        assert_eq!(
            ui_field(&archive, "layers[0].key"),
            &DisplayFieldValue::I32(-7)
        );
        assert_eq!(
            ui_field(&archive, "layers[1].key"),
            &DisplayFieldValue::I32(-7)
        );
        assert_eq!(
            ui_field(&archive, "old_attributes[0].value"),
            &DisplayFieldValue::I32(-12)
        );
        let qword = archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.as_ref() == format!("{PREFIX}.layers.count_high"));
        assert_eq!(qword, scenario >= 20);
    }
    let archive = parsed(selected(0x1be5, DisplayDocumentKind::Part, 16));
    assert_eq!(
        ui_field(&archive, "layers.count"),
        &DisplayFieldValue::I32(3)
    );
    assert_eq!(
        ui_field(&archive, "attributes[2].key"),
        &DisplayFieldValue::I32(99)
    );
    assert_eq!(
        ui_field(&archive, "attributes[2].value"),
        &DisplayFieldValue::I32(i32::MIN)
    );
}

#[test]
fn full_word_line_style_count_and_noncanonical_flag_retain_all_double_bits() {
    let archive = parsed(selected(19000, DisplayDocumentKind::Part, 3));
    assert_eq!(
        ui_field(&archive, "segments.count"),
        &DisplayFieldValue::U16(u16::MAX)
    );
    assert_eq!(
        ui_field(&archive, "thick_thin"),
        &DisplayFieldValue::U8(255)
    );
    let DisplayFieldValue::F64(value) = ui_field(&archive, "segments[5]") else {
        panic!("expected stored double")
    };
    assert_eq!(value.bits(), 0x7ff0000000000042);
    assert_eq!(
        ui_field(&archive, "segments[65534]"),
        &DisplayFieldValue::F64(StoredDouble::from_bits(1))
    );
}

#[test]
fn independent_edits_reopen_preserve_inputs_and_remain_isolated_across_threads() {
    for (scenario, layers) in [(0, false), (16, true)] {
        let case = selected(19000, DisplayDocumentKind::Part, scenario);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited(&archive, layers)))
                .collect();
            for worker in workers {
                let changed = worker.join().unwrap();
                assert_eq!(
                    changed.to_bytes().unwrap(),
                    selected(19000, DisplayDocumentKind::Part, scenario + 2).bytes
                );
                assert_eq!(
                    DisplayArchive::from_bytes(
                        &changed.to_bytes().unwrap(),
                        case_context(case),
                        LIMITS
                    )
                    .unwrap()
                    .contents(),
                    changed.contents()
                );
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

fn budgets() -> Vec<(usize, usize)> {
    [
        include_bytes!("fixtures/display-lists/ui-object-budgets.bin").as_slice(),
        include_bytes!("fixtures/display-lists/component-property-budgets.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(read_budgets)
    .collect()
}

fn read_budgets(bytes: &'static [u8]) -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_count, require_finished, take_u32};
    let mut remaining = bytes;
    let count = take_u32(&mut remaining).unwrap();
    require_count(count, remaining.len(), 8).unwrap();
    let rows = (0..count)
        .map(|_| {
            (
                take_u32(&mut remaining).unwrap() as usize,
                take_u32(&mut remaining).unwrap() as usize,
            )
        })
        .collect();
    require_finished(remaining).unwrap();
    rows
}

#[test]
fn exact_global_limits_charge_all_style_segments_layers_and_attributes() {
    assert_eq!(budgets().len(), cases().len());
    for (case, (entries, objects)) in cases().into_iter().zip(budgets()) {
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            ..LIMITS
        };
        assert_eq!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact)
                .unwrap()
                .contents(),
            archive.contents(),
            "{} {:?} {} entries {entries}",
            case.version,
            case.kind,
            case.scenario
        );
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
        ] {
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} {} entries {entries}",
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
fn impossible_extended_counts_and_wrong_declared_roles_fail_without_mutating_archives() {
    for (version, scenario, suffix, value) in [
        (
            5000,
            20,
            "layers.count_high",
            DisplayFieldValue::U32(u32::MAX),
        ),
        (0x1be5, 16, "layers.count", DisplayFieldValue::I32(-1)),
        (19000, 0, "segments.count", DisplayFieldValue::U16(u16::MAX)),
    ] {
        let case = selected(version, DisplayDocumentKind::Part, scenario);
        let archive = parsed(case);
        let mut contents = archive.contents().clone();
        contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == format!("{PREFIX}.{suffix}"))
            .unwrap()
            .value = value;
        assert!(archive.with_contents(contents, LIMITS).is_err());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
    let archive = parsed(selected(19000, DisplayDocumentKind::Part, 0));
    let mut contents = archive.contents().clone();
    let alias = contents
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.scene.floor_selection")
        .unwrap()
        .value
        .clone();
    contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.walls[0].wall.viz_prop")
        .unwrap()
        .value = alias;
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.cause.contains("expected one of"));
    assert!(error.cause.contains("moVisualProperties_c"));
}

#[test]
fn every_small_ui_object_prefix_and_trailing_byte_is_rejected() {
    for scenario in [0, 16, 20] {
        let case = selected(5000, DisplayDocumentKind::Part, scenario);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "scenario {scenario}, prefix {end}"
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
}

#[test]
fn ui_object_document_edits_preserve_other_streams_and_authentically_revert() {
    for (scenario, layers) in [(0, false), (16, true)] {
        let case = selected(19000, DisplayDocumentKind::Part, scenario);
        let mut document =
            bessemer::Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
        document
            .replace_stream("Contents/DisplayLists", case.bytes.to_vec())
            .unwrap();
        let original_bytes = document.to_bytes().unwrap();
        let original = bessemer::Document::from_bytes(&original_bytes).unwrap();
        let archive = archive_support::decode_document(&original_bytes, DisplayDocumentKind::Part);
        let changed = edited(&archive, layers);
        let mut output = original.clone();
        output
            .replace_stream("Contents/DisplayLists", changed.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
        for stream in original
            .streams()
            .filter(|stream| stream.name != "Contents/DisplayLists")
        {
            assert_eq!(
                reopened.read_stream(stream.name).unwrap(),
                original.read_stream(stream.name).unwrap()
            );
        }
        output
            .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(output.to_bytes().unwrap(), original_bytes);
    }
}

#[path = "support/component_property_cases.rs"]
mod component_properties;
