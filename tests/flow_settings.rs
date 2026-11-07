//! Exact supplied-context cache projection and optional physical identity distinctions.
#[path = "flow_settings/fixture.rs"]
mod fixture;
#[path = "flow_carriers/fixture.rs"]
mod native;
use bessemer::{FailureClass, flow::*};

fn carrier(records: &[(u32, &[u8])]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    for &(identifier, payload) in records {
        wire.extend(identifier.to_le_bytes());
        wire.extend((payload.len() as u64).to_le_bytes());
        wire.extend(payload);
    }
    FlowCarrier::from_bytes(&wire, native::limits()).unwrap()
}

#[test]
fn all_73984_source_projections_match_exact_inputs_actions_and_normal_return_cache() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 73984);
    for case in cases {
        let selected = select_flow_static_settings(case.incoming, case.cached, case.precedence);
        assert_eq!(selected.incoming(), case.incoming);
        assert_eq!(selected.cached_before(), case.cached);
        assert_eq!(selected.source_precedence_byte(), case.precedence);
        assert_eq!(selected.action(), case.action);
        assert_eq!(selected.load_required(), case.load);
        assert_eq!(selected.cached_after_loaders_return(), case.after);
        assert_eq!(selected.overwrite_warning(), case.warning);
        assert_projection_coverage(&selected, case.precedence);
    }
}

fn assert_projection_coverage(selected: &FlowSettingsSelection, precedence: u8) {
    let coverage = selected.coverage();
    assert_eq!(
        coverage.mode,
        FlowSettingsCheckMode::ExactNormalReturnSelection
    );
    assert_eq!(coverage.guid_equality_considered, precedence == 0);
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(
        selected.diagnostics(),
        &[
            FlowSettingsDiagnostic::LoaderSequenceNotExecuted,
            FlowSettingsDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn native_identity_fields_include_physically_present_nil_and_keep_original_sources() {
    let expected = [
        "b74082b0-db86-4729-8b64-856d8dd015ae",
        "00000000-0000-0000-0000-000000000000",
        "b74082b0-db86-4729-8b64-856d8dd015ae",
    ];
    for (index, case) in native::cases().into_iter().take(3).enumerate() {
        let source = FlowCarrier::from_bytes(&case.raw, native::limits()).unwrap();
        assert_eq!(source.records().len(), case.records.len(), "{}", case.name);
        let identity = source.static_settings_identity().unwrap();
        assert_eq!(identity.guid().to_string(), expected[index]);
        assert!(identity.present());
        assert_eq!(
            identity.record(),
            Some(&source.latest(435).unwrap().unwrap().id)
        );
        assert_eq!(identity.source().as_bytes(), case.raw);
        assert_identity_coverage(&identity, true);
    }
}

fn assert_identity_coverage(identity: &FlowSettingsIdentity, present: bool) {
    let coverage = identity.coverage();
    assert_eq!(
        coverage.mode,
        FlowSettingsCheckMode::ExactOptionalHeaderIdentity
    );
    assert_eq!(coverage.identity_bytes, if present { 16 } else { 0 });
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(identity.present(), present);
    assert_eq!(identity.record().is_some(), present);
    assert_eq!(
        identity.diagnostics(),
        &[FlowSettingsDiagnostic::NativeApplicationNotChecked]
    );
}

#[test]
fn missing_or_orphan_identity_defaults_differ_from_present_nil_and_use_last_occurrences() {
    for source in [
        carrier(&[]),
        carrier(&[(435, &[19; 20])]),
        carrier(&[(123, &[0; 32])]),
    ] {
        let identity = source.static_settings_identity().unwrap();
        assert_eq!(identity.guid().as_bytes(), &[0; 16]);
        assert_identity_coverage(&identity, false);
    }
    let source = carrier(&[
        (123, b"bad earlier"),
        (123, &[0; 32]),
        (435, &[19; 20]),
        (435, &[0; 20]),
    ]);
    let identity = source.static_settings_identity().unwrap();
    assert_eq!(identity.guid().as_bytes(), &[0; 16]);
    assert_eq!(identity.record().unwrap().ordinal(), 3);
    assert_identity_coverage(&identity, true);
}

#[test]
fn malformed_selected_headers_produce_errors_instead_of_fresh_nil_defaults() {
    for length in 0..32 {
        let payload = vec![0; length];
        let source = carrier(&[(123, &payload)]);
        assert_eq!(
            source.static_settings_identity().unwrap_err().class,
            FailureClass::Invalid
        );
    }
    for length in 0..20 {
        let payload = vec![0; length];
        let source = carrier(&[(123, &[0; 32]), (435, &payload)]);
        assert_eq!(
            source.static_settings_identity().unwrap_err().class,
            FailureClass::Invalid
        );
    }
}

#[test]
fn stored_identity_edits_feed_selection_without_mutating_the_cache_or_source() {
    let source = carrier(&[(123, &[0; 32]), (435, &[0; 20])]);
    let old = source.static_settings_identity().unwrap();
    let incoming = FlowGuid::from_bytes([255; 16]);
    let edit = source
        .replace_header_guid(FlowHeaderGuidField::StaticSettings, incoming)
        .unwrap();
    let next = edit.result().static_settings_identity().unwrap();
    let selected = select_flow_static_settings(next.guid(), old.guid(), 0);
    assert_eq!(selected.action(), FlowSettingsAction::InvokeLoaderSequence);
    assert_eq!(selected.cached_after_loaders_return(), incoming);
    assert!(!selected.overwrite_warning());
    assert_eq!(old.guid().as_bytes(), &[0; 16]);
    assert_eq!(
        source.static_settings_identity().unwrap().guid(),
        old.guid()
    );
}

#[test]
fn all_settings_types_are_send_sync_and_copy_isolated_across_eight_workers() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowSettingsIdentity>();
    assert_send_sync::<FlowSettingsSelection>();
    assert_send_sync::<FlowSettingsIdentityCoverage>();
    assert_send_sync::<FlowSettingsSelectionCoverage>();
    assert_send_sync::<FlowSettingsDiagnostic>();
    let incoming = FlowGuid::from_bytes([255; 16]);
    let cached = FlowGuid::from_bytes([0; 16]);
    let expected = select_flow_static_settings(incoming, cached, 0);
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| select_flow_static_settings(incoming, cached, 0)))
            .collect();
        for join in joins {
            assert_eq!(join.join().unwrap(), expected);
        }
    });
    assert_eq!(cached.as_bytes(), &[0; 16]);
}
