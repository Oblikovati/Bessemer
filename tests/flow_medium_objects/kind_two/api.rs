use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/expected.rs"]
pub(crate) mod expected;
#[path = "api/read.rs"]
pub(crate) mod read;
fn options() -> FlowMediumKindTwoOptions {
    FlowMediumKindTwoOptions {
        body: crate::api::options(),
        maximum_word_patterns: 100_000,
    }
}
fn backend(raw: &[u8], kind: i32) -> FlowMediumBackendPrefix {
    let mut payload = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    payload.extend_from_slice(&1i32.to_le_bytes());
    payload.extend_from_slice(&kind.to_le_bytes());
    payload.extend_from_slice(&[0; 16]);
    payload.extend_from_slice(raw);
    let carrier = crate::api::carrier(&payload);
    let id = carrier.latest(424).unwrap().unwrap().id;
    carrier
        .record_424_metadata_prefix(&id, crate::api::prefix_options())
        .unwrap()
        .medium_backend_prefix()
        .unwrap()
}
fn open(raw: &[u8], options: FlowMediumKindTwoOptions) -> bessemer::Result<FlowMediumKindTwo> {
    backend(raw, 2).first_kind_two_body(options)
}

fn word_count(case: &KindTwoCase) -> usize {
    match &case.model.payload {
        ModelPayload::Table(value) => value.word_patterns.len(),
        _ => 0,
    }
}
