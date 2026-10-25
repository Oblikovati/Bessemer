use super::*;

#[test]
fn admitted_simple_text_records_retain_the_temporary_float_bits() {
    let case = complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/early-view-roots-part.bin"
    ))
    .into_iter()
    .find(|case| case.version == 8000 && case.scenario == 160)
    .unwrap();
    let archive = parsed(case);
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| {
            field.path.as_ref() == "root.views[0].display_items.items[2].data.unknown_v1c05"
        })
        .unwrap();
    let DisplayFieldValue::F32(value) = field.value else {
        panic!("expected stored f32")
    };
    assert_eq!(value.bits(), 0x7f800042);
    assert_eq!(
        DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
}

pub(super) fn early_entries(case: RangeCase) -> usize {
    let prefix = early_prefix_entries(case.version);
    if case.scenario == 168 {
        return prefix + 101;
    }
    let items = if case.version <= 0x1c15 {
        3
    } else if case.version <= 0x1c49 {
        5
    } else {
        9 + 2 * usize::from(case.version > 0x1c4b)
    };
    let shaded = if case.kind == DisplayDocumentKind::Part {
        61
    } else {
        62
    };
    prefix + items + shaded - 2 * usize::from(case.version <= 0x1c23)
        + usize::from(case.scenario >= 192)
}

fn early_prefix_entries(version: u32) -> usize {
    let named = if version <= 0x1be7 {
        0
    } else if version <= 0x1bf4 {
        2
    } else {
        6
    };
    let annotation = if version <= 0x1c39 { 0 } else { 6 };
    44 + named + annotation + 6 * usize::from(version > 0x1c18)
}

#[test]
fn temporary_float_edits_match_independent_bytes_and_preserve_source() {
    let case = selected(8000, DisplayDocumentKind::Part, 160);
    let original = parsed(case);
    let mut contents = original.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref() == "root.views[0].display_items.items[2].data.unknown_v1c05"
        })
        .unwrap();
    field.value =
        DisplayFieldValue::F32(bessemer::tessellation::StoredFloat::from_bits(0x41b80000));
    let edited = original.with_contents(contents, LIMITS).unwrap();
    assert_eq!(
        edited.to_bytes().unwrap(),
        selected(8000, DisplayDocumentKind::Part, 162).bytes
    );
    assert_eq!(original.to_bytes().unwrap(), case.bytes);
    assert_eq!(
        DisplayArchive::from_bytes(&edited.to_bytes().unwrap(), case_context(case), LIMITS)
            .unwrap()
            .contents(),
        edited.contents()
    );
}

#[test]
fn old_annotation_ids_and_temporary_float_have_exact_version_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 160) {
        let archive = parsed(case);
        let has_ids = archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.ends_with("unknown_old_ids[1]"));
        assert_eq!(
            has_ids,
            (0x1c16..0x1c4a).contains(&case.version),
            "{}",
            case.version
        );
        assert_eq!(
            has_field(
                &archive,
                "root.views[0].display_items.items[2].data.unknown_v1c05"
            ),
            (0x1c05..0x1fb5).contains(&case.version),
            "{}",
            case.version
        );
    }
}

#[test]
fn earliest_minimal_annotations_accept_twelve_byte_records() {
    for version in [0x1c4a, 0x1c4b, 0x1c4c, 7999] {
        let case = selected(version, DisplayDocumentKind::Part, 168);
        let archive = parsed(case);
        assert!(
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.ends_with("edw_annot_data[99].unknown_0"))
        );
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

#[test]
fn earliest_annotation_prefixes_and_impossible_id_counts_fail_without_source_changes() {
    for (version, scenario) in [(0x1c16, 160), (0x1c4a, 168)] {
        let case = selected(version, DisplayDocumentKind::Part, scenario);
        let archive = parsed(case);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "version {version}, prefix {end}"
            );
        }
        if scenario == 160 {
            let mut contents = archive.contents().clone();
            let count = contents
                .fields
                .iter_mut()
                .find(|field| field.path.ends_with("unknown_old_ids.count"))
                .unwrap();
            count.value = DisplayFieldValue::I32(i32::MAX);
            assert!(archive.with_contents(contents, LIMITS).is_err());
        }
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
