//! Complete Composer carrier operations against independent ZIP/reference fixtures.
use bessemer::composer::{
    ComposerArchive, ComposerCompression, ComposerLimits, ComposerMemberInput,
};

const LIMITS: ComposerLimits = ComposerLimits {
    maximum_bytes: 100000,
    maximum_members: 20,
    maximum_member_bytes: 10000,
    maximum_expanded_bytes: 20000,
};

#[test]
fn complete_owned_containers_keep_original_bytes_names_duplicates_and_payloads() {
    for bytes in [
        include_bytes!("fixtures/composer/empty.smg").as_slice(),
        include_bytes!("fixtures/composer/stored.smg"),
        include_bytes!("fixtures/composer/deflated.smg"),
        include_bytes!("fixtures/composer/false-end.smg"),
    ] {
        let archive = ComposerArchive::from_bytes(bytes, LIMITS).unwrap();
        assert_eq!(archive.to_bytes(), bytes);
        assert_eq!(archive.coverage().wire_bytes, bytes.len());
        for member in archive.members() {
            assert_eq!(
                archive.read_member(&member.id).unwrap().len(),
                member.expanded_bytes as usize
            );
        }
    }
    let archive =
        ComposerArchive::from_bytes(include_bytes!("fixtures/composer/deflated.smg"), LIMITS)
            .unwrap();
    assert!(archive.member_named("same").is_err());
    assert!(archive.member_named("missing").is_err());
    assert_eq!(
        archive
            .read_member(&archive.member_named("α/file").unwrap())
            .unwrap(),
        b"data".repeat(200)
    );
}

#[test]
fn complete_member_edits_reopen_revert_and_keep_source_and_foreign_identities_isolated() {
    let original = include_bytes!("fixtures/composer/deflated.smg");
    let source = ComposerArchive::from_bytes(original, LIMITS).unwrap();
    let id = source.member_named("α/file").unwrap();
    let edit = source
        .edit_member(&id, b"Bessemer Composer payload")
        .unwrap();
    let result = edit.archive();
    assert!(!edit.is_unchanged());
    assert!(result.read_member(&id).is_err());
    let (parent, child) = edit.derived_member().unwrap();
    assert_eq!(source.read_member(parent).unwrap(), b"data".repeat(200));
    assert_eq!(
        result.read_member(child).unwrap(),
        b"Bessemer Composer payload"
    );
    let reopened = ComposerArchive::from_bytes(&result.to_bytes(), LIMITS).unwrap();
    assert_eq!(
        reopened
            .read_member(&reopened.member_named("α/file").unwrap())
            .unwrap(),
        b"Bessemer Composer payload"
    );
    assert_eq!(source.to_bytes(), original);
    assert!(
        source
            .edit_member(&id, source.read_member(&id).unwrap())
            .unwrap()
            .is_unchanged()
    );
}

#[test]
fn new_empty_stored_and_deflated_carriers_are_complete_and_thread_safe() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ComposerArchive>();
    send_sync::<bessemer::composer::ComposerEdit>();
    send_sync::<bessemer::composer::ComposerMemberId>();
    send_sync::<bessemer::composer::ComposerMember>();
    send_sync::<bessemer::composer::ComposerMemberInput>();
    send_sync::<bessemer::composer::ComposerCoverage>();
    send_sync::<bessemer::composer::ComposerCheckMode>();
    send_sync::<bessemer::composer::ComposerDiagnostic>();
    send_sync::<bessemer::composer::ComposerLimits>();
    send_sync::<bessemer::composer::ComposerCompression>();
    send_sync::<bessemer::composer::ComposerEditCompletion>();
    send_sync::<bessemer::composer::ComposerEditFidelity>();
    let empty = ComposerArchive::from_members(Vec::new(), b"comment", LIMITS).unwrap();
    assert_eq!(empty.members().len(), 0);
    for compression in [ComposerCompression::Stored, ComposerCompression::Deflated] {
        let archive = ComposerArchive::from_members(
            vec![ComposerMemberInput {
                name: "Ω".into(),
                contents: b"new payload".to_vec(),
                compression,
            }],
            b"comment",
            LIMITS,
        )
        .unwrap();
        assert_eq!(
            archive
                .read_member(&archive.member_named("Ω").unwrap())
                .unwrap(),
            b"new payload"
        );
        assert_eq!(archive.comment(), b"comment");
    }
}

#[test]
fn stored_replacement_matches_independent_zip_writer_golden_and_preserves_metadata() {
    let archive =
        ComposerArchive::from_bytes(include_bytes!("fixtures/composer/stored.smg"), LIMITS)
            .unwrap();
    let id = archive.member_named("α/file").unwrap();
    let edit = archive
        .edit_member(&id, b"Bessemer Composer payload")
        .unwrap();
    assert_eq!(
        edit.archive().to_bytes(),
        include_bytes!("fixtures/composer/stored-edited.smg")
    );
    assert_eq!(
        edit.coverage().mode,
        bessemer::composer::ComposerCheckMode::ExactContainer
    );
    assert_eq!(
        edit.completion(),
        bessemer::composer::ComposerEditCompletion::Changed
    );
    assert_eq!(
        edit.fidelity(),
        bessemer::composer::ComposerEditFidelity::ExactMemberContents
    );
    assert_eq!(edit.diagnostics(), archive.diagnostics());
    let revert_id = edit.archive().member_named("α/file").unwrap();
    assert_eq!(
        edit.archive()
            .edit_member(&revert_id, archive.read_member(&id).unwrap())
            .unwrap()
            .archive()
            .to_bytes(),
        archive.to_bytes()
    );
}

#[test]
fn cp437_and_independent_central_order_are_retained_through_complete_construction_and_edits() {
    let cp437 =
        ComposerArchive::from_bytes(include_bytes!("fixtures/composer/cp437.smg"), LIMITS).unwrap();
    let member = cp437.members().next().unwrap();
    assert_eq!(member.name.chars().count(), 128);
    assert_eq!(
        member.name.as_ref(),
        include_str!("fixtures/composer/cp437-name.txt")
    );
    assert_eq!(member.name.chars().next(), Some('Ç'));
    assert_eq!(cp437.read_member(&member.id).unwrap(), b"cp437");
    let bytes = include_bytes!("fixtures/composer/reordered.smg");
    let archive = ComposerArchive::from_bytes(bytes, LIMITS).unwrap();
    assert_eq!(archive.to_bytes(), bytes);
    assert_eq!(
        archive
            .members()
            .map(|member| member.name.to_string())
            .collect::<Vec<_>>(),
        ["same", "α/file", "same", "empty"]
    );
    let id = archive.members().next().unwrap().id;
    let edit = archive
        .edit_member(&id, b"first duplicate changed")
        .unwrap();
    assert_eq!(
        edit.archive().members().next().unwrap().name.as_ref(),
        "same"
    );
    assert_eq!(
        edit.archive()
            .read_member(edit.derived_member().unwrap().1)
            .unwrap(),
        b"first duplicate changed"
    );
}

#[test]
fn every_owned_prefix_trailing_byte_and_resource_budget_fails_without_source_mutation() {
    let bytes = include_bytes!("fixtures/composer/deflated.smg");
    for end in 0..bytes.len() {
        assert!(
            ComposerArchive::from_bytes(&bytes[..end], LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut extra = bytes.to_vec();
    extra.push(0);
    assert!(ComposerArchive::from_bytes(&extra, LIMITS).is_err());
    for limits in [
        ComposerLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        },
        ComposerLimits {
            maximum_members: 1,
            ..LIMITS
        },
        ComposerLimits {
            maximum_member_bytes: 1,
            ..LIMITS
        },
        ComposerLimits {
            maximum_expanded_bytes: 1,
            ..LIMITS
        },
    ] {
        assert!(ComposerArchive::from_bytes(bytes, limits).is_err());
    }
    let source = ComposerArchive::from_bytes(bytes, LIMITS).unwrap();
    assert!(
        source
            .edit_member(
                &source.member_named("α/file").unwrap(),
                &vec![0; LIMITS.maximum_member_bytes + 1]
            )
            .is_err()
    );
    let foreign = ComposerArchive::from_bytes(bytes, LIMITS)
        .unwrap()
        .members()
        .next()
        .unwrap()
        .id;
    assert!(source.read_member(&foreign).is_err());
    assert_eq!(source.to_bytes(), bytes);
}

#[test]
fn source_buffer_mutation_and_parallel_replacements_cannot_change_published_archives() {
    let expected = include_bytes!("fixtures/composer/deflated.smg");
    let mut input = expected.to_vec();
    let source = ComposerArchive::from_bytes(&input, LIMITS).unwrap();
    input.fill(0);
    assert_eq!(source.to_bytes(), expected);
    let id = source.member_named("α/file").unwrap();
    let single = source
        .edit_member(&id, b"parallel replacement")
        .unwrap()
        .archive()
        .to_bytes();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..6)
            .map(|_| {
                scope.spawn(|| {
                    source
                        .edit_member(&id, b"parallel replacement")
                        .unwrap()
                        .archive()
                        .to_bytes()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), single);
        }
    });
    assert_eq!(source.to_bytes(), expected);
}
