use super::*;
fn messages_with_opaque(bytes: usize) -> FlowMessageLists {
    let messages = open(&compact(3), options()).unwrap();
    let source = messages.source();
    let opaque = source.records().nth(2).unwrap().id;
    let edit = source.replace_payload(&opaque, &vec![0xa5; bytes]).unwrap();
    let record = edit.result().records().nth(1).unwrap().id;
    edit.result()
        .record_301_message_lists(&record, options())
        .unwrap()
}
#[test]
fn foreign_text_identity_errors_stay_small_with_large_unrelated_opaque_payloads() {
    let foreign = open(&compact(3), options())
        .unwrap()
        .text_at(0)
        .unwrap()
        .id();
    for bytes in [0, 65_536, 524_288] {
        let messages = messages_with_opaque(bytes);
        let error = messages.text(&foreign).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(
            error.cause.len() < 512,
            "foreign text error length {} for {bytes} opaque bytes",
            error.cause.len()
        );
        assert!(!error.cause.contains("wire: [") && !error.cause.contains("spans: ["));
        assert_eq!(
            messages.text_at(0).unwrap().raw_bytes(),
            &[65, 0, 66, 0, 0, 0]
        );
    }
}
fn compact(version: i32) -> Vec<u8> {
    let mut raw = Vec::new();
    for value in [version, 1, 4, 3] {
        raw.extend_from_slice(&value.to_le_bytes());
    }
    if version == 1 {
        raw.extend_from_slice(&[0, 255, 7]);
    } else {
        raw.extend_from_slice(&[65, 0, 66, 0, 0, 0]);
    }
    raw.extend_from_slice(&0i32.to_le_bytes());
    if version == 3 {
        raw.extend_from_slice(&0i32.to_le_bytes());
    }
    raw
}
#[test]
fn independent_legacy_byte_length_empty_list_presence_and_modern_prefix_oracles() {
    assert_eq!(compact(1).len(), 23);
    for version in 0..=3 {
        let messages = open(&compact(version), options()).unwrap();
        let item = messages.item(0).unwrap();
        assert_eq!(item.first_list().string_count(), 0);
        assert_eq!(item.second_list().is_some(), version == 3);
        let text = item.primary();
        assert_eq!(text.stored_count(), 3);
        assert_eq!(
            text.prefix_bytes(),
            if version == 1 {
                None
            } else {
                Some(&[65, 0, 66, 0][..])
            }
        );
        assert_eq!(messages.coverage().strings, 1);
        check_diagnostics(&messages, version);
    }
}
fn check_diagnostics(messages: &FlowMessageLists, version: i32) {
    use FlowMessageDiagnostic::*;
    let text = if version == 1 {
        LegacyStringInterpretationNotReproduced
    } else {
        NativeStringModeAssumed
    };
    assert_eq!(
        messages.diagnostics(),
        [
            text,
            NativeRuntimeObjectsNotReconstructed,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_compact_prefix_extension_signed_count_and_version_failure_is_explicit() {
    for version in 0..=3 {
        let raw = compact(version);
        for end in 0..raw.len() {
            assert!(open(&raw[..end], options()).is_err());
        }
        let mut extended = raw.clone();
        extended.push(0);
        assert!(open(&extended, options()).is_err());
        reject_counts(&raw);
    }
    for version in [-1i32, 4, i32::MAX] {
        let mut raw = compact(3);
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert!(open(&raw, options()).is_err());
    }
}
#[test]
fn all_resource_axes_accept_exact_usage_and_reject_one_less() {
    for version in 0..=3 {
        let raw = compact(version);
        let coverage = open(&raw, options()).unwrap().coverage();
        let exact = exact_bounds(coverage);
        assert!(open(&raw, exact).is_ok());
        for axis in 0..6 {
            let mut reduced = exact;
            let bound = bound_axis(&mut reduced, axis);
            *bound -= 1;
            assert!(
                open(&raw, reduced).is_err(),
                "axis {axis} version {version}"
            );
        }
    }
}
fn exact_bounds(coverage: FlowMessageCoverage) -> FlowMessageListOptions {
    FlowMessageListOptions {
        maximum_payload_bytes: coverage.payload_bytes,
        maximum_items: coverage.items,
        maximum_strings: coverage.strings,
        maximum_text_bytes: coverage.text_bytes,
        maximum_values: coverage.stored_values,
        maximum_scopes: coverage.scopes,
        ..options()
    }
}
fn bound_axis(bounds: &mut FlowMessageListOptions, axis: usize) -> &mut usize {
    match axis {
        0 => &mut bounds.maximum_payload_bytes,
        1 => &mut bounds.maximum_items,
        2 => &mut bounds.maximum_strings,
        3 => &mut bounds.maximum_text_bytes,
        4 => &mut bounds.maximum_values,
        _ => &mut bounds.maximum_scopes,
    }
}
#[test]
fn empty_modern_records_require_context_but_legacy_ignores_it_and_zero_actual_budgets_work() {
    for version in 0i32..=3 {
        let mut raw = version.to_le_bytes().to_vec();
        raw.extend_from_slice(&0i32.to_le_bytes());
        let exact = exact_bounds(open(&raw, options()).unwrap().coverage());
        assert!(open(&raw, exact).is_ok());
        let absent = FlowMessageListOptions {
            string_mode: None,
            ..exact
        };
        assert_eq!(open(&raw, absent).is_ok(), version == 1);
    }
    let raw = compact(1);
    let ignored = FlowMessageListOptions {
        string_mode: None,
        ..options()
    };
    assert_eq!(
        open(&raw, ignored).unwrap().coverage(),
        open(&raw, options()).unwrap().coverage()
    );
}
#[test]
fn terminal_nul_failure_and_interior_nul_edit_reparse_without_mutating_source() {
    let messages = open(&compact(3), options()).unwrap();
    let text = messages.text_at(0).unwrap();
    let id = text.id();
    assert!(messages.replace_text_unit(&id, 2, 77).is_err());
    let edit = messages.replace_text_unit(&id, 0, 0).unwrap();
    assert_eq!(
        edit.result().text_at(0).unwrap().prefix_bytes(),
        Some(&[][..])
    );
    assert_eq!(
        messages.text_at(0).unwrap().prefix_bytes(),
        Some(&[65, 0, 66, 0][..])
    );
    assert!(edit.result().text(&id).is_err());
    assert!(messages.replace_text_byte(&id, 0, 1).is_err());
    assert!(messages.replace_text_unit(&id, usize::MAX, 0).is_err());
    assert!(messages.replace_member(usize::MAX, 0).is_err());
    check_modern_noop(&messages, &id);
}
#[test]
fn legacy_bytes_wrong_encoding_foreign_ids_and_caller_mutation_preserve_isolated_inputs() {
    let mut raw = compact(1);
    let messages = open(&raw, options()).unwrap();
    raw.fill(99);
    let text = messages.text_at(0).unwrap();
    let id = text.id();
    assert_eq!(text.raw_bytes(), &[0, 255, 7]);
    assert!(messages.replace_text_unit(&id, 0, 65).is_err());
    assert!(messages.replace_text_byte(&id, usize::MAX, 65).is_err());
    check_foreign(&messages);
    let edit = messages.replace_text_byte(&id, 0, 254).unwrap();
    assert_eq!(
        edit.result().text_at(0).unwrap().raw_bytes(),
        &[254, 255, 7]
    );
    assert_eq!(messages.text_at(0).unwrap().raw_bytes(), &[0, 255, 7]);
}
fn assert_shared<T: Send + Sync>() {}
#[test]
fn all_public_message_types_are_send_and_sync() {
    assert_shared::<FlowMessageListOptions>();
    assert_shared::<FlowMessageLists>();
    assert_shared::<FlowMessageItem<'static>>();
    assert_shared::<FlowMessageStringList<'static>>();
    assert_shared::<FlowMessageText<'static>>();
    assert_shared::<FlowMessageTextId>();
    assert_shared::<FlowMessageCoverage>();
    assert_shared::<FlowMessageListEdit>();
    assert_shared::<FlowMessageTextRole>();
    assert_shared::<FlowMessageTextEncoding>();
    assert_shared::<FlowMessageDiagnostic>();
}
#[test]
fn eight_workers_produce_identical_isolated_message_edits() {
    let messages = open(&compact(3), options()).unwrap();
    let expected = messages
        .replace_member(0, 123)
        .unwrap()
        .result()
        .source()
        .as_bytes()
        .to_vec();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| check_worker(&messages, &expected)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(messages.decoded_bytes(), compact(3));
}

fn reject_counts(raw: &[u8]) {
    for offset in [4, 12, raw.len() - 4] {
        for count in [-1i32, i32::MAX] {
            let mut invalid = raw.to_vec();
            invalid[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&invalid, options()).is_err());
        }
    }
}

fn check_modern_noop(messages: &FlowMessageLists, id: &FlowMessageTextId) {
    let noop = messages.replace_text_unit(id, 2, 0).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        messages.source().as_bytes()
    );
    state_carrier::check_history(noop.carrier_edit());
}

fn check_foreign(messages: &FlowMessageLists) {
    let foreign = open(&compact(3), options())
        .unwrap()
        .text_at(0)
        .unwrap()
        .id();
    assert!(messages.text(&foreign).is_err());
    assert!(messages.replace_text_byte(&foreign, 0, 0).is_err());
}

fn check_worker(messages: &FlowMessageLists, expected: &[u8]) {
    for _ in 0..32 {
        let edit = messages.replace_member(0, 123).unwrap();
        assert_eq!(edit.result().source().as_bytes(), expected);
        assert!(
            messages
                .replace_text_unit(&messages.text_at(0).unwrap().id(), 2, 1)
                .is_err()
        );
    }
}
