use super::{fixture, types::*};
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/read.rs"]
mod read;
#[path = "api/replay.rs"]
mod replay;

fn options(mode: bool) -> FlowLoadStateOptions {
    FlowLoadStateOptions {
        mode: if mode {
            FlowLoadMode::AssumeTrue
        } else {
            FlowLoadMode::AssumeFalse
        },
        maximum_payload_bytes: 2_000_000,
        maximum_items: 100_000,
        maximum_text_units: 1_000_000,
        maximum_array_values: 1_000_000,
        maximum_values: 2_000_000,
        maximum_fields: 1_000_000,
        maximum_scopes: 1_000_000,
    }
}
fn carrier(payload: &[u8]) -> FlowCarrier {
    let mut raw = 8u16.to_le_bytes().to_vec();
    for (id, bytes) in [(11u32, &b"decoy"[..]), (11, payload), (999, &b"opaque"[..])] {
        raw.extend_from_slice(&id.to_le_bytes());
        raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: 4_000_000,
            maximum_payload_bytes: 2_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}
fn open(raw: &[u8], options: FlowLoadStateOptions) -> bessemer::Result<FlowLoadState> {
    let source = carrier(raw);
    let id = source.latest(11).unwrap().unwrap().id;
    source.record_11_load_state(&id, options)
}
fn scope_path(scope: FlowLoadScope) -> String {
    match scope {
        FlowLoadScope::Record => "[]".into(),
        FlowLoadScope::Item(i) => format!("[\"items\",{i}]"),
        FlowLoadScope::Name(i, n) => format!("[\"items\",{i},\"names\",{n}]"),
        FlowLoadScope::Description(i) => format!("[\"items\",{i},\"description\"]"),
        FlowLoadScope::ModeTail => "[\"mode_tail\"]".into(),
    }
}
fn field_path(field: FlowLoadField<'_>) -> String {
    let mut prefix = scope_path(field.scope());
    prefix.pop();
    let separator = if prefix == "[" { "" } else { "," };
    format!("{prefix}{separator}{}]", field_suffix(field.kind()))
}
fn field_suffix(kind: FlowLoadFieldKind) -> String {
    use FlowLoadFieldKind::*;
    match kind {
        ItemWideArray(i) => format!("\"wide_arrays\",{i}"),
        SharedArray(i) => format!("\"shared_arrays\",{i}"),
        _ => format!("\"{}\"", scalar_role(kind)),
    }
}
fn scalar_role(kind: FlowLoadFieldKind) -> &'static str {
    use FlowLoadFieldKind::*;
    match kind {
        Version => "version",
        ItemCount => "count",
        Member130 => "member_130",
        ArrayWidth => "array_width",
        TextUnitCount => "code_units",
        TextUnits => "raw_units",
        MemberWords => "member_i32",
        States => "state_f0_f4",
        WideF8 => "pattern_f8",
        Member1F8 => "member_1f8",
        WideA0B0 => "patterns_a0_b0",
        ExtraCount => "extra_count",
        ExtraWords => "extra_patterns",
        FinalMembers => "final_members_128_12c",
        ModeWords => "four_patterns",
        ModeWide => "wide_patterns",
        other => panic!("Expected scalar load-state role, got {other:?}"),
    }
}
fn member_field(state: &FlowLoadState) -> FlowLoadField<'_> {
    state
        .fields()
        .find(|field| field.kind() == FlowLoadFieldKind::Member130)
        .unwrap()
}
