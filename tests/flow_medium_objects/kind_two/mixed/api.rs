use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/read.rs"]
mod read;
fn options() -> FlowMediumMixedOptions {
    FlowMediumMixedOptions {
        profile: FlowMediumProfileOptions {
            metadata: crate::api::prefix_options(),
            body: crate::api::options(),
            context: FlowMediumProfileContext::FreshBackend,
            maximum_first_objects: 1024,
            maximum_reference_objects: 1024,
            maximum_references: 100_000,
            maximum_total_wide_words: 1_000_000,
            maximum_total_name_units: 1_000_000,
        },
        maximum_body_word_patterns: 100_000,
        maximum_total_word_patterns: 1_000_000,
    }
}
fn open(raw: &[u8], options: FlowMediumMixedOptions) -> bessemer::Result<FlowMediumMixedProfile> {
    let carrier = crate::api::carrier(raw);
    let id = carrier.latest(424)?.unwrap().id;
    carrier.record_424_mixed_profile(&id, options)
}
fn guid(raw: &[u8]) -> FlowGuid {
    FlowGuid::from_bytes(raw.try_into().unwrap())
}
fn body_totals(body: &MixedBody) -> (usize, usize, usize) {
    match body {
        MixedBody::One(value) => (
            crate::api::fields(value)
                .iter()
                .map(|(_, _, words)| words.len())
                .sum(),
            0,
            value.prefix.text.units as usize,
        ),
        MixedBody::Two(value) => kind_two_totals(value),
    }
}
fn totals(case: &MixedCase) -> (usize, usize, usize, usize) {
    let (mut wide, mut words, mut names) = (0, 0, 0);
    for entry in &case.first {
        let (a, b, c) = body_totals(&entry.body);
        wide += a;
        words += b;
        names += c;
    }
    (
        wide,
        words,
        names,
        case.second
            .iter()
            .map(|entry| entry.references.len())
            .sum::<usize>()
            + case.footer.keys.len(),
    )
}

fn kind_two_totals(value: &KindTwoCase) -> (usize, usize, usize) {
    let words = match &value.model.payload {
        ModelPayload::Table(table) => table.word_patterns.len(),
        _ => 0,
    };
    (
        super::super::api::expected::fields(value)
            .iter()
            .map(|(_, _, words)| words.len())
            .sum(),
        words,
        value.prefix.text.units as usize,
    )
}
