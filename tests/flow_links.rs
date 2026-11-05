//! Reference-checked ordered candidate lookup with immutable source checkpoints.
#[path = "flow_links/fixture.rs"]
mod fixture;
use bessemer::flow::*;

#[test]
fn all_1260_queries_match_source_selection_and_checked_prefixes() {
    for case in fixture::cases() {
        for request in &case.requests {
            let lookup =
                find_flow_carrier_by_guid(&case.candidates, request.kind, request.query).unwrap();
            assert_eq!(lookup.result_type(), request.kind);
            assert_eq!(lookup.query(), request.query);
            assert_eq!(lookup.selected_index(), request.selected);
            check_coverage(&lookup, request);
            check_inputs(&lookup, &case.candidates);
        }
    }
}

fn check_coverage(lookup: &FlowGuidLookup, request: &fixture::LinkRequest) {
    let coverage = lookup.coverage();
    assert_eq!(
        coverage.mode,
        FlowGuidLookupCheckMode::ExactOrderedCandidatePrefix
    );
    assert_eq!(coverage.visited_candidates, request.visited);
    assert_eq!(coverage.checked_eligible_candidates, request.eligible);
    assert_eq!(coverage.supplied_candidates, lookup.candidates().len());
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(lookup.diagnostics().len(), 3);
}

fn check_inputs(lookup: &FlowGuidLookup, candidates: &[FlowLinkCandidate]) {
    for (retained, supplied) in lookup.candidates().iter().zip(candidates) {
        assert_eq!(retained.path(), supplied.path());
        assert_eq!(retained.result_type(), supplied.result_type());
        assert_eq!(retained.carrier().as_bytes(), supplied.carrier().as_bytes());
    }
    let selected = lookup
        .selected_index()
        .map(|index| candidates[index].path());
    assert_eq!(lookup.selected().map(FlowLinkCandidate::path), selected);
}

#[test]
fn all_21_related_groups_preserve_seed_and_conditional_lookup_provenance() {
    for case in fixture::cases() {
        let related = case.seed.related_carriers(&case.candidates).unwrap();
        assert_eq!(related.seed().as_bytes(), case.seed.as_bytes());
        assert_eq!(related.cpt_lookup().selected_index(), case.cpt);
        assert_eq!(related.settings_stage(), case.stage);
        let settings = related
            .settings_lookup()
            .and_then(FlowGuidLookup::selected_index);
        assert_eq!(settings, case.settings);
        assert_eq!(
            related.settings_lookup().is_some(),
            case.stage == FlowRelatedSettingsStage::Searched
        );
        check_inputs(related.cpt_lookup(), &case.candidates);
    }
}

#[test]
fn result_type_boundary_and_native_extensions_are_explicit() {
    for value in 0..=u32::from(u16::MAX) {
        let parsed = FlowResultType::try_from(value);
        assert_eq!(parsed.is_ok(), matches!(value, 1 | 2 | 4));
        if let Ok(kind) = parsed {
            assert_eq!(kind.native_value(), value);
        }
    }
    assert!(FlowResultType::try_from(u32::MAX).is_err());
    assert_eq!(FlowResultType::Cpt.extension(), "cpt");
    assert_eq!(FlowResultType::Fld.extension(), "fld");
    assert_eq!(FlowResultType::StaticSettings.extension(), "cfld");
}

fn bad_header() -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(123_u32.to_le_bytes());
    wire.extend(0_u64.to_le_bytes());
    FlowCarrier::from_bytes(&wire, fixture::limits()).unwrap()
}

#[test]
fn ineligible_and_post_match_malformed_headers_are_not_eagerly_loaded() {
    let case = fixture::cases().remove(0);
    let good = case.candidates[0].clone();
    let query = good
        .carrier()
        .lookup_guid(FlowResultType::Cpt)
        .unwrap()
        .unwrap();
    let bad_fld = FlowLinkCandidate::new(FlowResultType::Fld, "bad.fld", bad_header());
    let bad_cpt = FlowLinkCandidate::new(FlowResultType::Cpt, "bad.cpt", bad_header());
    let candidates = [bad_fld, good.clone(), bad_cpt.clone()];
    let lookup = find_flow_carrier_by_guid(&candidates, FlowResultType::Cpt, query).unwrap();
    assert_eq!(lookup.selected_index(), Some(1));
    assert_eq!(lookup.coverage().visited_candidates, 2);
    assert_eq!(lookup.coverage().checked_eligible_candidates, 1);
    assert!(find_flow_carrier_by_guid(&[bad_cpt, good], FlowResultType::Cpt, query).is_err());
}

#[test]
fn empty_cpt_path_stops_settings_lookup_and_literal_paths_remain_unchanged() {
    let case = fixture::cases().remove(1);
    let empty_cpt = FlowLinkCandidate::new(
        FlowResultType::Cpt,
        "",
        case.candidates[0].carrier().clone(),
    );
    let candidates = [empty_cpt, case.candidates[2].clone()];
    let related = case.seed.related_carriers(&candidates).unwrap();
    assert_eq!(related.cpt_lookup().selected_index(), Some(0));
    assert_eq!(
        related.settings_stage(),
        FlowRelatedSettingsStage::EmptyCptPath
    );
    assert!(related.settings_lookup().is_none());
    assert_eq!(related.cpt_lookup().selected().unwrap().path(), "");
}

#[test]
fn retained_candidates_survive_caller_mutation_and_parallel_selection() {
    let mut case = fixture::cases().remove(1);
    let related = case.seed.related_carriers(&case.candidates).unwrap();
    case.candidates.clear();
    let retained = related.cpt_lookup().candidates();
    assert_eq!(retained.len(), 3);
    check_parallel(related.seed(), retained);
    let selected = related.settings_lookup().unwrap().selected().unwrap();
    assert_eq!(selected.path(), "1.cfld");
}

#[test]
fn lookup_contract_types_are_send_and_sync() {
    assert_traits::<FlowResultType>();
    assert_traits::<FlowLinkCandidate>();
    assert_traits::<FlowGuidLookup>();
    assert_traits::<FlowGuidLookupCheckMode>();
    assert_traits::<FlowGuidLookupCoverage>();
    assert_traits::<FlowGuidLookupDiagnostic>();
    assert_traits::<FlowRelatedSettingsStage>();
    assert_traits::<FlowRelatedCarriers>();
}

fn assert_traits<T: Send + Sync>() {}

#[test]
fn executed_empty_settings_lookup_retains_literal_cpt_path_and_complete_absence() {
    let case = fixture::cases().remove(1);
    let literal = "../Shared\\unchanged.cpt";
    let cpt = FlowLinkCandidate::new(
        FlowResultType::Cpt,
        literal,
        case.candidates[0].carrier().clone(),
    );
    let related = case.seed.related_carriers(&[cpt]).unwrap();
    assert_eq!(related.settings_stage(), FlowRelatedSettingsStage::Searched);
    assert_eq!(related.cpt_lookup().selected().unwrap().path(), literal);
    let searched = related.settings_lookup().unwrap();
    assert!(searched.selected().is_none());
    assert_eq!(searched.coverage().visited_candidates, 1);
    assert_eq!(searched.coverage().checked_eligible_candidates, 1);
}

fn check_parallel(seed: &FlowCarrier, retained: &[FlowLinkCandidate]) {
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || {
                let related = seed.related_carriers(retained).unwrap();
                assert_eq!(related.cpt_lookup().selected_index(), Some(0));
            });
        }
    });
}

#[test]
fn nil_and_defaulted_candidate_keys_do_not_match_formatted_nil_queries() {
    let mut nil_wire = 8_u16.to_le_bytes().to_vec();
    nil_wire.extend(123_u32.to_le_bytes());
    nil_wire.extend(32_u64.to_le_bytes());
    nil_wire.extend([0; 32]);
    let nil = FlowCarrier::from_bytes(&nil_wire, fixture::limits()).unwrap();
    let missing = FlowCarrier::from_bytes(&8_u16.to_le_bytes(), fixture::limits()).unwrap();
    for source in [nil, missing] {
        for kind in [
            FlowResultType::Cpt,
            FlowResultType::Fld,
            FlowResultType::StaticSettings,
        ] {
            assert_eq!(source.lookup_guid(kind).unwrap(), None);
            let candidate = FlowLinkCandidate::new(kind, "nil", source.clone());
            let lookup =
                find_flow_carrier_by_guid(&[candidate], kind, FlowGuid::from_bytes([0; 16]))
                    .unwrap();
            assert!(lookup.selected().is_none());
        }
    }
}
