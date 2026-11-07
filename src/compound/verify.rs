use super::read::CompoundModel;
use super::{CompoundEntryKind, CompoundLimits, Result, require};
use std::collections::BTreeMap;
use std::sync::Arc;

pub(super) fn reconstruct(
    model: &CompoundModel,
    payloads: &BTreeMap<usize, Arc<[u8]>>,
    limits: CompoundLimits,
) -> Result<(Vec<u8>, CompoundModel)> {
    let bytes = super::write::encode(model, payloads, limits)?;
    let reopened = super::read::decode(&bytes, limits)?;
    require(
        &reopened.payloads == payloads,
        "constructed logical payloads differ; expected every complete stream byte unchanged except the requested replacement",
    )?;
    compare_directory(model, &reopened)?;
    require(
        model.header.raw[..40] == reopened.header.raw[..40],
        "constructed header identity/version differs; expected unchanged header prefix",
    )?;
    Ok((bytes, reopened))
}

fn metadata(mut raw: [u8; 128], kind: CompoundEntryKind) -> [u8; 128] {
    if matches!(kind, CompoundEntryKind::Root | CompoundEntryKind::Stream) {
        raw[116..128].fill(0);
    }
    raw
}

fn compare_directory(original: &CompoundModel, result: &CompoundModel) -> Result<()> {
    require(
        result.directory.parents == original.directory.parents,
        "constructed parent identities differ; expected unchanged storage topology",
    )?;
    require(
        result.directory.records.len() == original.directory.records.len(),
        "constructed directory slot count differs; expected all original slots",
    )?;
    for (index, (before, after)) in original
        .directory
        .records
        .iter()
        .zip(&result.directory.records)
        .enumerate()
    {
        compare_record(index, before, after)?;
    }
    Ok(())
}

fn compare_record(
    index: usize,
    before: &super::directory::DirectoryRecord,
    after: &super::directory::DirectoryRecord,
) -> Result<()> {
    require(
        metadata(before.raw, before.kind) == metadata(after.raw, after.kind),
        format!(
            "constructed directory metadata differs at slot {index}; expected exact bytes outside stream allocation/size fields"
        ),
    )
}
