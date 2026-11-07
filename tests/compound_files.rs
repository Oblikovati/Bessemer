//! Complete compound carrier reconstruction and isolated stream replacement.
#[path = "common/compound_failures.rs"]
mod failures;
#[path = "common/compound_files.rs"]
mod reference;
use bessemer::compound::*;
use reference::*;

#[test]
fn complete_owned_streams_metadata_and_independent_canonical_goldens_match() {
    let raw = owned();
    let cases = cases(&raw, false);
    assert_eq!(cases.len(), 10);
    for case in cases {
        check_case(&case);
    }
}

#[test]
fn replacements_cross_mini_cutoff_shrink_empty_and_grow_without_source_mutation() {
    let raw = owned();
    let cases = cases(&raw, false);
    for case in &cases[2..4] {
        let file = check_case(case);
        let entry = file.entry_at_path("S/B").unwrap();
        for size in [0, 1, 63, 64, 65, 4095, 4096, 4097, 12000] {
            let replacement = vec![0x5a; size];
            let edit = file.edit_stream(entry.id(), &replacement).unwrap();
            assert_equivalent(&file, &edit.file, Some((entry.id().slot(), &replacement)));
            assert_eq!(file.to_bytes(), case.bytes);
            assert_edit_contract(&file, &edit, entry.id().slot());
        }
    }
}

fn assert_edit_contract(original: &CompoundFile, edit: &CompoundEdit, changed: usize) {
    assert_eq!(
        edit.completion,
        bessemer::display_lists::DisplayEditCompletion::Changed
    );
    assert_eq!(
        edit.fidelity,
        CompoundEditFidelity::ExactLogicalStreamsAndDirectoryMetadata
    );
    assert_eq!(&edit.history.parent, original.revision());
    assert_eq!(&edit.history.result, edit.file.revision());
    assert_ne!(edit.history.parent, edit.history.result);
    assert_eq!(edit.history.entries.len(), original.entries().len());
    assert_history_entries(original, edit, changed);
    assert_eq!(edit.coverage, edit.file.coverage());
    assert_eq!(edit.diagnostics, edit.file.diagnostics());
}

#[test]
fn no_op_revision_foreign_ids_paths_and_non_stream_guards_are_explicit() {
    let raw = owned();
    let cases = cases(&raw, false);
    let file = check_case(&cases[2]);
    let entry = file.entry_at_path("S/A").unwrap();
    let edit = file
        .edit_stream(entry.id(), file.stream(entry.id()).unwrap())
        .unwrap();
    assert_eq!(edit.file.revision(), file.revision());
    assert_eq!(edit.file.to_bytes(), file.to_bytes());
    assert_eq!(
        edit.completion,
        bessemer::display_lists::DisplayEditCompletion::Unchanged
    );
    let other = check_case(&cases[2]);
    assert!(other.stream(entry.id()).is_err());
    assert!(file.entry_at_path("s/A").is_err());
    assert!(file.entry_at_path("S/missing").is_err());
    reject_non_streams(&file);
}

#[test]
#[ignore = "requires freeze_compound.py --export against installed compound carriers"]
fn all_installed_compound_streams_reconstruct_and_every_stream_edit_is_isolated() {
    let raw = std::fs::read("target/compound-files-native.bin").unwrap();
    let cases = cases(&raw, true);
    assert_eq!(cases.len(), 43);
    let mut bundle = b"CFBE".to_vec();
    bundle.extend_from_slice(&528_u32.to_le_bytes());
    let mut streams = 0;
    for (ordinal, case) in cases.iter().enumerate() {
        let file = check_case(case);
        streams += append_native_edits(&mut bundle, ordinal, &file);
    }
    assert_eq!(streams, 528);
    std::fs::write("target/compound-files-native-edits.bin", bundle).unwrap();
}

fn append_native_edits(bundle: &mut Vec<u8>, ordinal: usize, file: &CompoundFile) -> usize {
    let mut count = 0;
    for entry in file
        .entries()
        .iter()
        .filter(|entry| entry.kind() == CompoundEntryKind::Stream)
    {
        let replacement = changed_payload(file.stream(entry.id()).unwrap());
        let edit = file.edit_stream(entry.id(), &replacement).unwrap();
        assert_equivalent(file, &edit.file, Some((entry.id().slot(), &replacement)));
        assert_edit_contract(file, &edit, entry.id().slot());
        let bytes = edit.file.to_bytes();
        for value in [ordinal, entry.id().slot(), bytes.len()] {
            bundle.extend_from_slice(&(value as u32).to_le_bytes());
        }
        bundle.extend_from_slice(&bytes);
        count += 1;
    }
    count
}

#[test]
fn snapshots_survive_source_mutation_drop_and_concurrent_isolated_edits() {
    let packet = owned();
    let cases = cases(&packet, false);
    let mut source = cases[2].bytes.to_vec();
    let file = CompoundFile::from_bytes(&source, LIMITS).unwrap();
    source.fill(0xff);
    drop(source);
    let workers = concurrent_edits(&file);
    let mut expected = None;
    for worker in workers {
        let edit = worker.join().unwrap();
        let encoded = edit.file.to_bytes();
        if let Some(expected) = &expected {
            assert_eq!(&encoded, expected);
        } else {
            expected = Some(encoded);
        }
        assert_equivalent(&file, &edit.file, Some((2, b"isolated")));
    }
    assert_eq!(file.to_bytes(), cases[2].bytes);
}

#[test]
fn public_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<CompoundFile>();
    check::<CompoundLimits>();
    check::<CompoundRevision>();
    check::<CompoundEntryId>();
    check::<CompoundEntryKind>();
    check::<CompoundEntry>();
    check::<CompoundCheckMode>();
    check::<CompoundCoverage>();
    check::<CompoundDiagnostic>();
    check::<CompoundEditFidelity>();
    check::<CompoundEntryHistory>();
    check::<CompoundHistory>();
    check::<CompoundEdit>();
}

#[test]
fn raw_utf16_names_and_root_color_zero_survive_exact_reconstruction() {
    let packet = owned();
    let cases = cases(&packet, false);
    let mut raw = cases[2].bytes.to_vec();
    let directory = (word_at(&raw, 48) + 1) * 512;
    raw[directory + 2 * 128..directory + 2 * 128 + 2].copy_from_slice(&0xd800_u16.to_le_bytes());
    let file = CompoundFile::from_bytes(&raw, LIMITS).unwrap();
    let entry = file
        .entries()
        .iter()
        .find(|entry| entry.id().slot() == 2)
        .unwrap();
    assert_eq!(entry.name_units(), &[0xd800]);
    let reopened = CompoundFile::from_bytes(&file.reconstructed_bytes().unwrap(), LIMITS).unwrap();
    assert_equivalent(&file, &reopened, None);
}

fn assert_history_entries(original: &CompoundFile, edit: &CompoundEdit, changed: usize) {
    for relation in edit.history.entries.iter() {
        assert_eq!(relation.parent.revision(), original.revision());
        assert_eq!(relation.result.revision(), edit.file.revision());
        assert_eq!(relation.parent.slot(), relation.result.slot());
        assert_eq!(relation.replaced_stream, relation.parent.slot() == changed);
        assert!(original.entry(&relation.parent).is_ok());
        assert!(edit.file.entry(&relation.result).is_ok());
        assert!(edit.file.entry(&relation.parent).is_err());
    }
}

fn reject_non_streams(file: &CompoundFile) {
    let root = file.entry_at_path("").unwrap();
    assert!(file.stream(root.id()).is_err());
    assert!(file.edit_stream(root.id(), b"invalid").is_err());
}

fn concurrent_edits(file: &CompoundFile) -> Vec<std::thread::JoinHandle<CompoundEdit>> {
    (0..6)
        .map(|_| {
            let file = file.clone();
            std::thread::spawn(move || {
                file.edit_stream(file.entry_at_path("S/A").unwrap().id(), b"isolated")
                    .unwrap()
            })
        })
        .collect::<Vec<_>>()
}

#[test]
fn red_root_107_retains_color_and_declares_directory_validation_limits() {
    let packet = owned();
    let cases = cases(&packet, false);
    let colored = check_case(&cases[6]);
    assert_eq!(colored.entries()[0].directory_record()[67], 0);
    assert!(
        colored
            .diagnostics()
            .contains(&CompoundDiagnostic::DirectoryCollationAndBalanceNotChecked)
    );
}

#[test]
fn storage_allocation_words_remain_raw_without_an_invented_user_stream_size() {
    let packet = owned();
    let cases = cases(&packet, false);
    let mut raw = cases[2].bytes.to_vec();
    let offset = (word_at(&raw, 48) + 1) * 512 + 128;
    raw[offset + 116..offset + 128].fill(0xff);
    let file = CompoundFile::from_bytes(&raw, LIMITS).unwrap();
    let storage = file.entry_at_path("S").unwrap();
    assert_eq!(storage.logical_size(), 0);
    assert_eq!(
        storage.directory_record().as_slice(),
        &raw[offset..offset + 128]
    );
    assert!(file.stream(storage.id()).is_err());
    let reopened = CompoundFile::from_bytes(&file.reconstructed_bytes().unwrap(), LIMITS).unwrap();
    assert_equivalent(&file, &reopened, None);
}

#[test]
fn growing_into_extended_difat_and_shrinking_back_matches_independent_goldens() {
    let packet = owned();
    let cases = cases(&packet, false);
    let small = check_case(&cases[2]);
    let large = check_case(&cases[4]);
    let original = small.entry_at_path("S/B").unwrap();
    let expanded = small
        .edit_stream(
            original.id(),
            large
                .stream(large.entry_at_path("S/B").unwrap().id())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(expanded.file.to_bytes(), cases[4].canonical.unwrap());
    assert_shrinks_to_golden(&expanded.file, &small, cases[2].canonical.unwrap());
    assert_eq!(small.to_bytes(), cases[2].bytes);
}

#[test]
fn preallocated_109_capacity_is_owned_but_not_added_to_logical_stream_bytes() {
    let packet = owned();
    let cases = cases(&packet, false);
    let minimal = check_case(&cases[2]);
    for case in &cases[7..10] {
        let file = check_case(case);
        assert_equivalent(&minimal, &file, None);
        assert_eq!(file.coverage().logical_stream_bytes, 7040);
        assert_eq!(
            file.reconstructed_bytes().unwrap(),
            cases[2].canonical.unwrap()
        );
        assert_eq!(file.to_bytes(), case.bytes);
    }
}

fn assert_shrinks_to_golden(expanded: &CompoundFile, original: &CompoundFile, golden: &[u8]) {
    let original_id = original.entry_at_path("S/B").unwrap().id();
    let result_id = expanded.entry_at_path("S/B").unwrap().id();
    let restored = expanded
        .edit_stream(result_id, original.stream(original_id).unwrap())
        .unwrap();
    assert_eq!(restored.file.to_bytes(), golden);
}
