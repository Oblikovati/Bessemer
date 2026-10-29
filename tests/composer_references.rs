//! Complete native lexical and supplied-state branch projections.
#[path = "common/composer_reference_oracle.rs"]
mod reference;
use bessemer::composer::references::*;
use reference::*;
#[path = "common/composer_links.rs"]
mod links;

fn owned() -> Oracle {
    let mut cursor = Cursor(include_bytes!("fixtures/composer-references/owned.bin"));
    let oracle = cursor.oracle();
    assert!(cursor.0.is_empty());
    oracle
}

#[test]
fn complete_raw_utf16_splits_match_frozen_native_branch_projection() {
    for expected in owned().get("splits").list() {
        let input = expected.get("input").units();
        let split = input.split();
        assert_eq!(split.original, input);
        assert_eq!(
            split.marker.map_or(-1, |at| at as i64),
            expected.get("marker").number()
        );
        assert_eq!(
            split.separator.map_or(-1, |at| at as i64),
            expected.get("separator").number()
        );
        assert_eq!(split.file.is_some(), expected.get("external").boolean());
        assert_eq!(split.guid, expected.get("guid").units());
        match expected.get("file") {
            Oracle::Null => assert!(split.file.is_none()),
            expected => assert_eq!(split.file.as_ref(), Some(&expected.units())),
        }
    }
}

#[test]
fn every_lexical_helper_matches_frozen_mixed_separator_and_dot_quirks() {
    for expected in owned().get("paths").list() {
        let input = text(expected.get("input").text());
        let actual = lexical_results(&input);
        for (actual, key) in actual
            .iter()
            .zip(["file", "folder", "base_path", "base", "initial"])
        {
            assert_eq!(*actual, text(expected.get(key).text()));
        }
        let fallback = input.fallback_file_keys(&text("C:\\project\\part.smgXml"));
        for (actual, expected) in fallback.iter().zip(expected.get("fallback").list()) {
            assert_eq!(*actual, text(expected.text()));
        }
    }
}

#[test]
fn all_supplied_primary_override_null_and_registration_branches_match_source() {
    for expected in owned().get("servers").list() {
        check_server_case(expected);
    }
}

fn check_server_case(expected: &Oracle) {
    let server = expected_server(expected);
    assert_lookup(
        server.get(&text("FILE"), &text("g"), 42).unwrap(),
        expected.get("before"),
    );
    let edit = server
        .register(
            &text("file"),
            &text("g"),
            expected.get("incoming").number() as u64,
        )
        .unwrap();
    assert_registration(&edit, expected.get("registration"));
    assert_lookup(
        edit.server.get(&text("fIlE"), &text("g"), 42).unwrap(),
        expected.get("after"),
    );
    check_server_isolation(&server, &edit.server, expected);
}

fn assert_registration(actual: &ComposerRegistration, expected: &Oracle) {
    assert_eq!(
        actual.fidelity,
        ComposerReferenceFidelity::ExactSuppliedState
    );
    assert_registration_completion(actual);
    assert_eq!(actual.pointer, expected.get("pointer").number() as u64);
    assert_eq!(
        actual.detach_incoming_first_parent_if_present,
        expected
            .get("detach_incoming_first_parent_if_present")
            .boolean()
    );
    if check_inserted_registration(actual, expected) {
        return;
    }
    let branch = expected_unchanged_branch(expected.get("reason").text());
    assert_eq!(actual.branch, branch);
    assert_eq!(actual.parent, actual.result);
}

fn check_inserted_registration(actual: &ComposerRegistration, expected: &Oracle) -> bool {
    if !expected.get("inserted").boolean() {
        return false;
    }
    assert_eq!(
        actual.branch,
        ComposerRegistrationBranch::Inserted(expected_tier(expected.get("tier")).unwrap())
    );
    assert_ne!(actual.parent, actual.result);
    true
}

#[test]
fn every_fallback_stop_position_and_duplicate_attempt_matches_frozen_source() {
    for expected in owned().get("lookups").list() {
        let supplied = lookup_supplied(expected);
        let server = ComposerGeodeServer::from_files(&supplied, limits()).unwrap();
        let actual = server
            .lookup_nonempty(
                &text(expected.get("file").text()),
                &text(expected.get("project").text()),
                &text("g"),
            )
            .unwrap();
        assert_attempts(&actual, expected.get("result"));
        assert_eq!(&actual.revision, server.revision());
        assert_eq!(actual.coverage, server.coverage());
        assert_eq!(actual.diagnostics, server.diagnostics());
    }
}

#[test]
fn boundaries_reject_nul_nonascii_empty_merge_and_duplicate_supplied_keys() {
    assert!(ComposerReferenceText::from_units(&[65, 0, 66]).is_err());
    assert!(ComposerReferenceText::from_text("bad\0text").is_err());
    assert!(text("é.xml").initial_file_key().is_err());
    let server = ComposerGeodeServer::from_files(&[], limits()).unwrap();
    for invalid in [
        text("é"),
        ComposerReferenceText::from_units(&[0xd800]).unwrap(),
    ] {
        assert!(server.get(&invalid, &text("g"), 0).is_err());
        assert!(server.get(&text("file"), &invalid, 0).is_err());
        assert!(server.register(&text("file"), &invalid, 7).is_err());
    }
    assert!(
        server
            .lookup_nonempty(&text("file"), &text("project"), &text(""))
            .is_err()
    );
    reject_duplicate_maps();
}

fn supplied(primary: &[(&str, u64)], overrides: &[(&str, u64)]) -> ComposerSuppliedFile {
    let convert = |entries: &[(&str, u64)]| {
        entries
            .iter()
            .map(|(key, pointer)| (text(key), *pointer))
            .collect()
    };
    ComposerSuppliedFile {
        file: text("FiLe"),
        primary: convert(primary),
        overrides: convert(overrides),
    }
}

#[test]
fn all_server_budgets_reject_seed_and_registration_without_changing_input() {
    let exact = ComposerServerLimits {
        maximum_files: 1,
        maximum_entries: 1,
        maximum_key_bytes: 5,
    };
    let server = ComposerGeodeServer::from_files(&[supplied(&[("g", 0)], &[])], exact).unwrap();
    reject_small_seed_budgets(exact);
    assert!(server.register(&text("new"), &text("g"), 9).is_err());
    assert!(server.register(&text("file"), &text("g"), 9).is_err());
    assert!(server.register(&text("file"), &text("h"), 9).is_err());
    assert_eq!(server.get(&text("file"), &text("g"), 9).unwrap().pointer, 0);
    assert_eq!(server.coverage().entries, 1);
    assert_eq!(
        server.coverage().mode,
        ComposerReferenceCheck::ExactSuppliedState
    );
}

#[test]
fn completed_empty_keys_null_continuation_and_full_u64_tokens_are_preserved() {
    let server = ComposerGeodeServer::from_files(&[], limits()).unwrap();
    assert_eq!(server.coverage().files, 0);
    let first = server.register(&text(""), &text(""), u64::MAX).unwrap();
    assert_eq!(
        first.server.get(&text(""), &text(""), 0).unwrap().pointer,
        u64::MAX
    );
    check_null_fallback();
}

#[test]
fn published_snapshots_are_isolated_and_thread_independent() {
    let mut original = supplied(&[("g", 0)], &[]);
    let server = ComposerGeodeServer::from_files(&[original.clone()], limits()).unwrap();
    original.primary[0].1 = 77;
    drop(original);
    let threads = concurrent_registrations(&server);
    for thread in threads {
        let edit = thread.join().unwrap();
        assert_registered_token(&edit.server, 9);
        assert_eq!(edit.coverage, edit.server.coverage());
        assert_eq!(edit.diagnostics, edit.server.diagnostics());
        assert_eq!(&edit.parent, server.revision());
    }
    assert_eq!(server.get(&text("file"), &text("g"), 0).unwrap().pointer, 0);
    assert_public_threads();
}

fn assert_public_threads() {
    fn check<T: Send + Sync>() {}
    check::<ComposerReferenceText>();
    check::<ComposerReferenceSplit>();
    check::<ComposerServerLimits>();
    check::<ComposerSuppliedFile>();
    check::<ComposerGeodeTier>();
    check::<ComposerGeodeLookup>();
    check::<ComposerRegistrationBranch>();
    check::<ComposerReferenceCheck>();
    check::<ComposerServerCoverage>();
    check::<ComposerReferenceDiagnostic>();
    check::<ComposerRegistration>();
    check::<ComposerLookupAttempt>();
    check::<ComposerReferenceLookup>();
    check::<ComposerGeodeServer>();
    check::<ComposerServerRevision>();
    check::<ComposerReferenceCompletion>();
    check::<ComposerReferenceFidelity>();
}

#[test]
#[ignore = "requires frozen references and complete scene/XML native probes"]
fn every_installed_reference_matches_all_supplied_state_fallback_attempts() {
    let raw = std::fs::read("target/composer-references-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    let expected = cursor.oracle();
    assert!(cursor.0.is_empty());
    let inputs = std::fs::read("target/composer-links-native.bin").unwrap();
    let cases = links::link_cases(&inputs);
    assert_eq!((cases.len(), expected.list().len()), (38, 38));
    let mut totals = (0, 0, 0);
    let mut encoded = Vec::new();
    for (case, expected) in cases.iter().zip(expected.list()) {
        let index = links::check_case(case);
        check_native_case(&index, expected, &mut totals, &mut encoded);
    }
    assert_eq!(totals, (4445, 4416, 4532));
    std::fs::write("target/composer-references-lookups.bin", encoded).unwrap();
}

fn check_native_case(
    index: &bessemer::composer::links::ComposerXmlLinks,
    expected: &Oracle,
    totals: &mut (usize, usize, usize),
    encoded: &mut Vec<u8>,
) {
    let server = native_server(index, expected);
    let project = text(expected.get("project").text());
    let file = project.initial_file_key().unwrap();
    let references = expected.get("references").list();
    assert_eq!(index.references().len(), references.len());
    for (occurrence, expected) in index.references().iter().zip(references) {
        check_native_reference(
            &server, &file, &project, occurrence, expected, totals, encoded,
        );
    }
}

fn lexical_results(input: &ComposerReferenceText) -> [ComposerReferenceText; 5] {
    [
        input.file_name(),
        input.file_folder(),
        input.file_base_path(),
        input.file_base(),
        input.initial_file_key().unwrap(),
    ]
}

fn check_server_isolation(
    server: &ComposerGeodeServer,
    result: &ComposerGeodeServer,
    expected: &Oracle,
) {
    assert_lookup(
        result.get(&text("file"), &text("G"), 42).unwrap(),
        expected.get("missing"),
    );
    assert_lookup(
        server.get(&text("FILE"), &text("g"), 42).unwrap(),
        expected.get("before"),
    );
}

fn expected_unchanged_branch(reason: &str) -> ComposerRegistrationBranch {
    match reason {
        "existing_nonnull_geode" => ComposerRegistrationBranch::ExistingNonnull,
        "inner_map_unchanged" => ComposerRegistrationBranch::InnerUnchanged,
        reason => panic!("{reason}; expected retained native registration branch"),
    }
}

fn lookup_supplied(expected: &Oracle) -> Vec<ComposerSuppliedFile> {
    match expected.get("seeded") {
        Oracle::Null => vec![],
        key => vec![ComposerSuppliedFile {
            file: text(key.text()),
            primary: vec![(text("g"), 123)],
            overrides: vec![],
        }],
    }
}

fn reject_duplicate_maps() {
    let duplicate = supplied(&[("g", 0), ("g", 7)], &[]);
    assert!(ComposerGeodeServer::from_files(&[duplicate], limits()).is_err());
    let duplicate = supplied(&[], &[("g", 0), ("g", 7)]);
    assert!(ComposerGeodeServer::from_files(&[duplicate], limits()).is_err());
    assert!(
        ComposerGeodeServer::from_files(&[supplied(&[], &[]), supplied(&[], &[])], limits())
            .is_err()
    );
}

fn reject_small_seed_budgets(exact: ComposerServerLimits) {
    for limits in [
        ComposerServerLimits {
            maximum_files: 0,
            ..exact
        },
        ComposerServerLimits {
            maximum_entries: 0,
            ..exact
        },
        ComposerServerLimits {
            maximum_key_bytes: 4,
            ..exact
        },
    ] {
        assert!(ComposerGeodeServer::from_files(&[supplied(&[("g", 0)], &[])], limits).is_err());
    }
}

fn check_null_fallback() {
    let first = ComposerSuppliedFile {
        file: text("geometry.geom"),
        primary: vec![(text("g"), 0)],
        overrides: vec![],
    };
    let server = ComposerGeodeServer::from_files(&[first], limits()).unwrap();
    let lookup = server
        .lookup_nonempty(&text("geometry.geom"), &text("geometry.xml"), &text("g"))
        .unwrap();
    assert_eq!(lookup.pointer, 0);
    assert_eq!(lookup.attempts.len(), 4);
    assert!(lookup.attempts.iter().all(|attempt| attempt.result.found));
    assert_eq!(
        server
            .get(&text("missing"), &text("g"), u64::MAX)
            .unwrap()
            .pointer,
        u64::MAX
    );
}

fn concurrent_registrations(
    server: &ComposerGeodeServer,
) -> Vec<std::thread::JoinHandle<ComposerRegistration>> {
    (0..6)
        .map(|_| {
            let snapshot = server.clone();
            std::thread::spawn(move || snapshot.register(&text("file"), &text("g"), 9).unwrap())
        })
        .collect()
}

fn native_server(
    index: &bessemer::composer::links::ComposerXmlLinks,
    expected: &Oracle,
) -> ComposerGeodeServer {
    let primary = index
        .guid_groups()
        .iter()
        .map(|group| {
            assert_eq!(group.candidates.len(), 1);
            (
                ComposerReferenceText::from_units(&group.identifier).unwrap(),
                group.candidates[0].entry_index() as u64 + 1,
            )
        })
        .collect();
    let supplied = ComposerSuppliedFile {
        file: text(expected.get("geometry").text()),
        primary,
        overrides: vec![],
    };
    ComposerGeodeServer::from_files(&[supplied], limits()).unwrap()
}

fn check_native_reference(
    server: &ComposerGeodeServer,
    file: &ComposerReferenceText,
    project: &ComposerReferenceText,
    occurrence: &bessemer::composer::links::ComposerXmlReference,
    expected: &Oracle,
    totals: &mut (usize, usize, usize),
    encoded: &mut Vec<u8>,
) {
    assert_eq!(occurrence.identifier.as_ref(), expected.get("input").text());
    let split = text(&occurrence.identifier).split();
    assert!(split.file.is_none());
    let actual = server.lookup_nonempty(file, project, &split.guid).unwrap();
    assert_attempts(&actual, expected.get("lookup"));
    encode_lookup(encoded, &actual);
    *totals = (
        totals.0 + 1,
        totals.1 + usize::from(actual.pointer != 0),
        totals.2 + actual.attempts.len(),
    );
}

fn expected_server(expected: &Oracle) -> ComposerGeodeServer {
    let supplied = ComposerSuppliedFile {
        file: text("FiLe"),
        primary: entries(expected.get("primary")),
        overrides: entries(expected.get("override")),
    };
    ComposerGeodeServer::from_files(&[supplied], limits()).unwrap()
}

fn assert_registered_token(server: &ComposerGeodeServer, expected: u64) {
    assert_eq!(
        server.get(&text("file"), &text("g"), 0).unwrap().pointer,
        expected
    );
}

fn assert_registration_completion(actual: &ComposerRegistration) {
    let expected = if actual.parent == actual.result {
        bessemer::display_lists::DisplayEditCompletion::Unchanged
    } else {
        bessemer::display_lists::DisplayEditCompletion::Changed
    };
    assert_eq!(actual.completion, expected);
}
