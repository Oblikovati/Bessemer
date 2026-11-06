use super::*;
#[test]
fn every_proper_known_backend_prefix_and_nonzero_version_or_negative_count_is_rejected() {
    for case in cases() {
        for end in 0..case.end as usize {
            assert!(open(&case.raw[..end]).is_err());
        }
        for position in
            std::iter::once(0).chain(case.tables.iter().map(|table| table.start as usize))
        {
            for version in 1..=255 {
                let mut raw = case.raw.clone();
                raw[position] = version;
                assert!(open(&raw).is_err());
            }
        }
        for table in &case.tables {
            let mut raw = case.raw.clone();
            let start = table.start as usize + 1;
            raw[start..start + 4].copy_from_slice(&(-1i32).to_le_bytes());
            assert!(open(&raw).is_err());
        }
    }
}
#[test]
fn inactive_outer_versions_missing_tables_and_layout_changing_edits_are_rejected() {
    let prefix = prefix(&[0; 11]).replace_version_byte(1);
    assert!(prefix.is_err());
    let view = open(&[0; 11]).unwrap();
    assert!(view.replace_declared_count(0, 1).is_err());
    assert!(view.replace_declared_count(0, -1).is_err());
    assert!(view.replace_declared_count(2, 0).is_err());
    assert!(view.replace_first_kind(0, 1).is_err());
    assert!(view.replace_first_key(0, [0; 16]).is_err());
    assert!(view.replace_first_kind(usize::MAX, 0).is_err());
    assert!(view.replace_first_key(usize::MAX, [0; 16]).is_err());
    check_populated_rejections();
}
fn check_populated_rejections() {
    let case = cases().remove(0);
    let view = open(&case.raw).unwrap();
    assert!(view.replace_declared_count(0, 0).is_err());
    assert!(view.replace_first_kind(0, 2).is_err());
    let other = cases()
        .into_iter()
        .find(|case| case.tables[0].kind == Some(0))
        .unwrap();
    assert!(open(&other.raw).unwrap().replace_first_kind(0, 1).is_err());
}
#[test]
fn backend_types_are_send_sync_and_eight_workers_match_every_actual_read_and_edit() {
    assert_send_sync::<FlowMediumBackendStop>();
    assert_send_sync::<FlowMediumBackendCoverage>();
    assert_send_sync::<FlowMediumBackendPrefix>();
    assert_send_sync::<FlowMediumBackendTable<'static>>();
    assert_send_sync::<FlowMediumBackendEdit>();
    let cases = cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn assert_send_sync<T: Send + Sync>() {}
fn transcript(cases: &[BackendCase]) -> Vec<u8> {
    let mut output = Vec::new();
    for case in cases {
        let mut raw = case.raw.clone();
        let view = open(&raw).unwrap();
        raw.fill(255);
        output.extend_from_slice(&(view.coverage().known_backend_bytes as u64).to_le_bytes());
        output.extend_from_slice(view.opaque_tail());
        for table in view.tables() {
            output.extend_from_slice(&table.declared_count().to_le_bytes());
            let count = if table.declared_count() == 0 { 0 } else { 7 };
            let edit = view.replace_declared_count(table.ordinal(), count).unwrap();
            output.extend_from_slice(edit.result().metadata_prefix().source().as_bytes());
        }
        assert_eq!(view.backend_bytes(), case.raw);
    }
    output
}

#[test]
fn inactive_metadata_prefix_cannot_be_misrepresented_as_a_completed_empty_backend() {
    let source = prefix(&[]);
    let edit = source
        .source()
        .replace_payload(source.record(), &[128, 0, 255])
        .unwrap();
    let id = edit.result().records().nth(1).unwrap().id;
    let inactive = edit
        .result()
        .record_424_metadata_prefix(&id, options())
        .unwrap();
    let error = inactive.medium_backend_prefix().unwrap_err();
    assert!(error.cause.contains("-128"));
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("424"));
    assert_eq!(inactive.opaque_tail(), [0, 255]);
}
