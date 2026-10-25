use super::*;

pub(super) fn cases() -> Vec<UiCase> {
    [
        include_bytes!("../fixtures/display-lists/component-properties.bin").as_slice(),
        include_bytes!("../fixtures/display-lists/component-property-holders-part.bin").as_slice(),
        include_bytes!("../fixtures/display-lists/component-property-holders-assembly.bin")
            .as_slice(),
    ]
    .into_iter()
    .flat_map(complete_cases::decode)
    .collect()
}

fn selected_property(version: u32, kind: DisplayDocumentKind, scenario: u8) -> UiCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

#[test]
fn property_font_slot_counts_and_raw_visibility_members_follow_exact_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 0) {
        let archive = parsed(case);
        let count = if case.version < 0x13ef {
            26
        } else if case.version < 0x23b3 {
            30
        } else {
            31
        };
        assert_eq!(
            archive
                .contents()
                .fields
                .iter()
                .filter(
                    |field| field.path.starts_with(&format!("{PREFIX}.line_fonts["))
                        && field.path.ends_with(".name")
                )
                .count(),
            count
        );
        assert_eq!(
            ui_field(&archive, "ent_vis_prop.layer"),
            &DisplayFieldValue::U16(u16::MAX)
        );
        assert_eq!(ui_field(&archive, "all_views"), &DisplayFieldValue::U8(255));
        for (label, present) in [
            ("added_for_envelop", case.version >= 0x147c),
            ("empty_prop", case.version > 0x3738),
        ] {
            assert_eq!(
                archive
                    .contents()
                    .fields
                    .iter()
                    .any(|field| field.path.as_ref() == format!("{PREFIX}.{label}")),
                present,
                "{} {label}",
                case.version
            );
        }
    }
}

fn edited_property(archive: &DisplayArchive, holder: bool) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let prefix = if holder {
        format!("{PREFIX}.property")
    } else {
        PREFIX.into()
    };
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("{prefix}.all_views"))
        .unwrap();
    field.value = DisplayFieldValue::U8(23);
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn property_edits_match_independent_bytes_and_preserve_concurrent_sources() {
    for (scenario, holder) in [(0, false), (16, true)] {
        let case = selected_property(19000, DisplayDocumentKind::Assembly, scenario);
        let archive = parsed(case);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_property(&archive, holder).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(
                    worker.join().unwrap(),
                    selected_property(19000, DisplayDocumentKind::Assembly, scenario + 2).bytes
                );
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn holder_cycles_shared_properties_and_unsigned_occurrences_keep_their_map_roles() {
    let archive = parsed(selected_property(19000, DisplayDocumentKind::Part, 21));
    let DisplayFieldValue::ObjectTag(root) = ui_root(&archive) else {
        panic!("expected holder definition")
    };
    let DisplayObjectPointer::New(root_id) = root.target else {
        panic!("expected holder ID")
    };
    let DisplayFieldValue::ObjectTag(cycle) = ui_field(&archive, "unknown_8.unknown_8") else {
        panic!("expected owner cycle")
    };
    assert_eq!(cycle.target, DisplayObjectPointer::Reference(root_id));
    let DisplayFieldValue::ObjectTag(definition) = ui_field(&archive, "unknown_8.property") else {
        panic!("expected property definition")
    };
    let DisplayObjectPointer::New(id) = definition.target else {
        panic!("expected property ID")
    };
    let DisplayFieldValue::ObjectTag(alias) = ui_field(&archive, "property") else {
        panic!("expected shared property alias")
    };
    assert_eq!(alias.target, DisplayObjectPointer::Reference(id));
    assert_eq!(
        ui_field(&archive, "occurrences[1]"),
        &DisplayFieldValue::U32(u32::MAX)
    );
    let handle = archive.object_handle(id).unwrap();
    assert_eq!(
        archive.resolve_object(&handle).unwrap().path.as_ref(),
        format!("{PREFIX}.unknown_8.property")
    );
}

fn ui_root(archive: &DisplayArchive) -> &DisplayFieldValue {
    &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == PREFIX)
        .unwrap()
        .value
}

#[test]
fn su_occurrence_counts_do_not_consume_a_fictitious_64_bit_extension() {
    let archive = parsed(selected_property(19000, DisplayDocumentKind::Part, 19));
    assert_eq!(
        ui_field(&archive, "occurrences.count_word"),
        &DisplayFieldValue::U16(u16::MAX)
    );
    assert_eq!(
        ui_field(&archive, "occurrences.count_dword"),
        &DisplayFieldValue::U32(3)
    );
    let mut contents = archive.contents().clone();
    contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("{PREFIX}.occurrences.count_dword"))
        .unwrap()
        .value = DisplayFieldValue::U32(u32::MAX);
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert!(
        !archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.as_ref() == format!("{PREFIX}.occurrences.count_high"))
    );
    let empty = parsed(selected_property(19000, DisplayDocumentKind::Part, 20));
    assert_eq!(
        ui_field(&empty, "occurrences.count_dword"),
        &DisplayFieldValue::U32(0)
    );
}

#[test]
fn holder_null_records_and_declared_property_roles_remain_distinct() {
    let empty = parsed(selected_property(19000, DisplayDocumentKind::Part, 17));
    for leaf in ["unknown_8", "property"] {
        let DisplayFieldValue::ObjectTag(tag) = ui_field(&empty, leaf) else {
            panic!("expected null object tag")
        };
        assert_eq!(tag.target, DisplayObjectPointer::Null);
    }
    assert_eq!(
        ui_field(&empty, "occurrences.count_word"),
        &DisplayFieldValue::U16(0)
    );
    let archive = parsed(selected_property(19000, DisplayDocumentKind::Part, 16));
    let mut contents = archive.contents().clone();
    let cycle = ui_field(&archive, "unknown_8").clone();
    contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("{PREFIX}.property"))
        .unwrap()
        .value = cycle;
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.cause.contains("expected one of"));
    assert!(error.cause.contains("moComponentProperty"));
}

#[test]
fn fonts_and_visibility_preserve_raw_width_bits_and_huge_occurrences_fail_physically() {
    let archive = parsed(selected_property(19000, DisplayDocumentKind::Part, 0));
    for leaf in [
        "line_fonts[1].custom_width",
        "ent_vis_prop.line_width.custom_width",
    ] {
        let DisplayFieldValue::F32(value) = ui_field(&archive, leaf) else {
            panic!("expected stored float")
        };
        assert_eq!(value.bits(), 0x7f800042);
    }
    assert_eq!(
        ui_field(&archive, "ent_vis_prop.unknown_4"),
        &DisplayFieldValue::U8(254)
    );
    assert_eq!(
        ui_field(&archive, "ent_vis_prop.unknown_8"),
        &DisplayFieldValue::U32(0xfedcba98)
    );
    let holder = parsed(selected_property(19000, DisplayDocumentKind::Part, 19));
    let mut contents = holder.contents().clone();
    contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("{PREFIX}.occurrences.count_dword"))
        .unwrap()
        .value = DisplayFieldValue::U32(u32::MAX);
    let error = holder
        .with_contents(
            contents,
            DisplayArchiveLimits {
                maximum_entries: usize::MAX,
                ..LIMITS
            },
        )
        .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("expected unsigned words fitting"));
}

#[test]
fn complete_property_and_holder_prefixes_never_publish_partial_graphs() {
    for (version, scenario) in [(5000, 0), (5000, 16), (19000, 21)] {
        let case = selected_property(version, DisplayDocumentKind::Part, scenario);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "version {version}, scenario {scenario}, prefix {end}"
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
}

#[test]
fn component_property_document_edits_reopen_preserve_other_streams_and_revert() {
    for (scenario, holder) in [(0, false), (16, true)] {
        let case = selected_property(19000, DisplayDocumentKind::Assembly, scenario);
        let mut source =
            bessemer::Document::from_bytes(include_bytes!("../fixtures/blank-assembly.SLDASM"))
                .unwrap();
        source
            .replace_stream("Contents/DisplayLists", case.bytes.to_vec())
            .unwrap();
        let original_bytes = source.to_bytes().unwrap();
        let original = bessemer::Document::from_bytes(&original_bytes).unwrap();
        let archive =
            archive_support::decode_document(&original_bytes, DisplayDocumentKind::Assembly);
        let changed = edited_property(&archive, holder);
        let mut output = original.clone();
        output
            .replace_stream("Contents/DisplayLists", changed.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
        assert_eq!(
            archive_support::decode_document(
                &output.to_bytes().unwrap(),
                DisplayDocumentKind::Assembly
            )
            .contents(),
            changed.contents()
        );
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
