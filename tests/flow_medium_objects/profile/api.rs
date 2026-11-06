use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/read.rs"]
mod read;
fn options() -> FlowMediumProfileOptions {
    FlowMediumProfileOptions {
        metadata: crate::api::prefix_options(),
        body: crate::api::options(),
        context: FlowMediumProfileContext::FreshBackend,
        maximum_first_objects: 1024,
        maximum_reference_objects: 1024,
        maximum_references: 100_000,
        maximum_total_wide_words: 100_000,
        maximum_total_name_units: 100_000,
    }
}
fn open(
    raw: &[u8],
    options: FlowMediumProfileOptions,
) -> bessemer::Result<FlowMediumKindOneProfile> {
    let source = crate::api::carrier(raw);
    let id = source.latest(424).unwrap().unwrap().id;
    source.record_424_kind_one_profile(&id, options)
}
fn guid(raw: &[u8]) -> FlowGuid {
    FlowGuid::from_bytes(raw.try_into().unwrap())
}
fn guid_bytes(keys: &[FlowGuid]) -> Vec<Vec<u8>> {
    keys.iter().map(|key| key.as_bytes().to_vec()).collect()
}
fn totals(case: &ProfileCase) -> [usize; 5] {
    let refs = case
        .second
        .iter()
        .map(|entry| entry.references.len())
        .sum::<usize>()
        + case.footer.keys.len();
    let words = case.first.iter().map(|entry| body_words(&entry.body)).sum();
    let units = case
        .first
        .iter()
        .map(|entry| entry.body.text.units as usize)
        .sum();
    [case.first.len(), case.second.len(), refs, words, units]
}
fn body_words(body: &BodyCase) -> usize {
    let curves = body
        .curves
        .iter()
        .map(|curve| {
            1 + curve.first.len()
                + curve.second.len()
                + curve.extra_first.len()
                + curve.extra_second.len()
        })
        .sum::<usize>();
    curves
        + body
            .first
            .iter()
            .chain(&body.second)
            .map(Vec::len)
            .sum::<usize>()
        + body.footer.scalars.len()
}
