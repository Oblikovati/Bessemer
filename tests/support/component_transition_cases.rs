use super::*;
const VIEWER: &str = "root.tessellation.components[0].viewer_comp_data";

#[test]
fn component_state_fields_and_discarded_pointer_obey_their_individual_boundaries() {
    for case in cases()
        .into_iter()
        .filter(|case| case.kind == DisplayDocumentKind::Assembly)
    {
        let archive = parsed(case);
        for (field, present) in [
            ("config_description", case.version > 0x279c),
            ("fixed", case.version > 0x279d),
            ("temp_fixed", case.version > 0x27b1),
            ("flexible", case.version > 0x27c1),
            ("unknown_obj", (0x27b7..0x27e3).contains(&case.version)),
        ] {
            assert_eq!(
                has_field(&archive, &format!("{VIEWER}.{field}")),
                present,
                "{} {field}",
                case.version
            );
        }
    }
}

fn edited_transient(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{VIEWER}.unknown_obj.flexible"), 23);
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn transient_viewers_retain_self_aliases_and_independent_edits() {
    for version in [0x27c2, 0x27e2] {
        let case = selected(version, DisplayDocumentKind::Assembly, 134);
        let archive = parsed(case);
        let pointer = |suffix: &str| {
            archive
                .contents()
                .fields
                .iter()
                .find_map(|field| {
                    if field.path.as_ref() != format!("{VIEWER}.{suffix}") {
                        return None;
                    }
                    match &field.value {
                        DisplayFieldValue::ObjectTag(tag) => Some(tag),
                        _ => None,
                    }
                })
                .unwrap()
        };
        let DisplayObjectPointer::New(identity) = pointer("unknown_obj").target else {
            panic!("expected new viewer")
        };
        assert_eq!(
            pointer("unknown_obj.unknown_obj").target,
            DisplayObjectPointer::Reference(identity)
        );
        assert_eq!(
            edited_transient(&archive).to_bytes().unwrap(),
            selected(version, DisplayDocumentKind::Assembly, 135).bytes
        );
        let mut contents = archive.contents().clone();
        let field = contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == format!("{VIEWER}.unknown_obj"))
            .unwrap();
        let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
            panic!("expected pointer")
        };
        tag.runtime_class.as_mut().unwrap().name = "moNodeName_c".into();
        tag.new_class.as_mut().unwrap().name = "moNodeName_c".into();
        assert!(
            archive
                .with_contents(contents, LIMITS)
                .unwrap_err()
                .to_string()
                .contains("moViewerComponentData_c")
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn transient_viewer_prefixes_and_parallel_edits_do_not_publish_partial_or_shared_mutations() {
    let case = selected(0x27c2, DisplayDocumentKind::Assembly, 134);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_transient(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap(),
                selected(case.version, case.kind, 135).bytes
            );
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
#[test]
fn transient_viewer_document_edit_reopens_and_reverts_authentic_input() {
    let case = selected(0x27c2, DisplayDocumentKind::Assembly, 134);
    let archive = parsed(case);
    let edited = edited_transient(&archive);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    let initial = include_bytes!("../fixtures/blank-assembly.SLDASM");
    let native = archive_support::decode_document(initial, case.kind);
    let mut document = bessemer::Document::from_bytes(initial).unwrap();
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
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}
