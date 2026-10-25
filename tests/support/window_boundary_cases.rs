use super::*;

#[test]
fn earlier_environment_body_units_and_edw_members_follow_their_version_bounds() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version <= 9000 && case.scenario == 128)
    {
        let archive = parsed(case);
        let body = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation.bodies[0]"
        } else {
            "root.tessellation.components[0].comp_part_tess_data.bodies[0]"
        };
        assert_eq!(
            has_field(&archive, "root.user_model_env.unknown_1c4"),
            case.version >= 0x1ff1
        );
        assert_eq!(
            has_field(&archive, &format!("{body}.unknown_str")),
            case.version > 0x2017
        );
        assert_eq!(
            has_field(&archive, "root.units_of_measurement.length_units"),
            case.version > 0x1fb2
        );
        let annotation = "root.views[0].display_items.items[1].data.edw_annot_data[0]";
        assert_eq!(
            has_field(&archive, &format!("{annotation}.unknown_i")),
            case.version > 0x1fa8
        );
        assert_eq!(
            has_field(&archive, &format!("{annotation}.unknown_strings2.count")),
            case.version > 0x1fd4
        );
    }
}

#[test]
fn minimal_annotation_rows_fit_the_exact_earlier_physical_stride() {
    for case in cases().into_iter().filter(|case| case.scenario == 136) {
        let archive = parsed(case);
        let prefix = "root.views[0].display_items.items[0].data.edw_annot_data";
        assert_eq!(
            archive_support::check_scalar_table(
                &archive,
                &format!("{prefix}.count=i:100\nroot.has_shaded_data=i:0\n")
            ),
            2
        );
        let width =
            16 + 4 * usize::from(case.version > 0x1fa8) + 4 * usize::from(case.version > 0x1fd4);
        let start = case.bytes.len() - 4 - 100 * width;
        assert_eq!(&case.bytes[start - 4..start], 100_i32.to_le_bytes());
        if case.version <= 0x1fd4 {
            assert!(100 * width + 4 < 100 * 24);
        }
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
fn uncounted_window_versions_remain_explicitly_unsupported_for_each_document_kind() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(8000, kind, 128);
        for version in 0x1fae..0x1fcf {
            let context = DisplayArchiveContext {
                version: DisplayVersion::new(version),
                ..case_context(case)
            };
            let error =
                DisplayArchive::from_bytes(case.bytes, context.clone(), LIMITS).unwrap_err();
            assert_eq!(error.class, bessemer::FailureClass::Unsupported);
            assert!(error.to_string().contains("open-window"));
            assert!(error.to_string().contains(&version.to_string()));
            assert!(DisplayArchive::new(parsed(case).contents().clone(), context, LIMITS).is_err());
        }
    }
    for version in [0x1fad, 0x1fcf] {
        assert!(
            parsed(selected(version, DisplayDocumentKind::Part, 128))
                .to_bytes()
                .is_ok()
        );
    }
}

#[test]
fn earlier_prefixes_and_independent_edits_preserve_completed_streams() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(8000, kind, 128);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{kind:?} prefix {end}"
            );
        }
        let archive = parsed(case);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(8000, kind, 129).bytes);
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
#[test]
fn older_document_edits_preserve_other_streams_and_parallel_archive_isolation() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(8000, kind, 128);
        let initial: &[u8] = if kind == DisplayDocumentKind::Part {
            include_bytes!("../fixtures/blank-part.SLDPRT")
        } else {
            include_bytes!("../fixtures/blank-assembly.SLDASM")
        };
        let original = bessemer::Document::from_bytes(initial).unwrap();
        let native = archive_support::decode_document(initial, kind);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(8000, kind, 129).bytes);
            }
        });
        let edited = edited_ambient(&archive);
        let mut document = original.clone();
        document
            .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(
            DisplayArchive::from_bytes(
                &reopened.read_stream("Contents/DisplayLists").unwrap(),
                case_context(case),
                LIMITS
            )
            .unwrap()
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
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
