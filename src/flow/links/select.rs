use super::*;

pub(super) fn find(
    candidates: Arc<[FlowLinkCandidate]>,
    kind: FlowResultType,
    query: FlowGuid,
) -> Result<FlowGuidLookup> {
    let (selected, visited, eligible) = scan_candidates(&candidates, kind, query)?;
    Ok(FlowGuidLookup {
        candidates,
        kind,
        query,
        selected,
        visited,
        eligible,
    })
}

fn scan_candidates(
    candidates: &[FlowLinkCandidate],
    kind: FlowResultType,
    query: FlowGuid,
) -> Result<(Option<usize>, usize, usize)> {
    let mut eligible = 0;
    for (index, candidate) in candidates.iter().enumerate() {
        if !kind.accepts(candidate.kind) {
            continue;
        }
        eligible += 1;
        if candidate.carrier.lookup_guid(kind)? == Some(query) {
            return Ok((Some(index), index + 1, eligible));
        }
    }
    Ok((None, candidates.len(), eligible))
}

pub(super) fn related(
    seed: &FlowCarrier,
    candidates: Arc<[FlowLinkCandidate]>,
) -> Result<FlowRelatedCarriers> {
    let keys = lookup_keys(seed)?;
    let cpt = find(
        candidates.clone(),
        FlowResultType::Cpt,
        keys[0].unwrap_or(FlowGuid::from_bytes([0; 16])),
    )?;
    let stage = settings_stage(&cpt, keys[2]);
    let settings = settings_lookup(candidates, keys[2], stage)?;
    Ok(FlowRelatedCarriers {
        seed: seed.clone(),
        cpt,
        settings,
        stage,
    })
}

fn settings_stage(cpt: &FlowGuidLookup, settings: Option<FlowGuid>) -> FlowRelatedSettingsStage {
    let Some(index) = cpt.selected else {
        return FlowRelatedSettingsStage::NoRelatedCpt;
    };
    if cpt.candidates[index].path.is_empty() {
        return FlowRelatedSettingsStage::EmptyCptPath;
    }
    if settings.is_none() {
        return FlowRelatedSettingsStage::EmptySettingsKey;
    }
    FlowRelatedSettingsStage::Searched
}

fn settings_lookup(
    candidates: Arc<[FlowLinkCandidate]>,
    key: Option<FlowGuid>,
    stage: FlowRelatedSettingsStage,
) -> Result<Option<FlowGuidLookup>> {
    if stage != FlowRelatedSettingsStage::Searched {
        return Ok(None);
    }
    key.map(|guid| find(candidates, FlowResultType::StaticSettings, guid))
        .transpose()
}
