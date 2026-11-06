use super::{fixture, types::*};
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/read.rs"]
pub(super) mod read;
pub(super) fn options() -> FlowMediumBodyOptions {
    FlowMediumBodyOptions {
        maximum_body_bytes: 1_000_000,
        maximum_wide_words: 100_000,
        maximum_name_units: 100_000,
    }
}
fn backend(raw: &[u8], kind: i32) -> FlowMediumBackendPrefix {
    let mut payload = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    payload.extend_from_slice(&1i32.to_le_bytes());
    payload.extend_from_slice(&kind.to_le_bytes());
    if kind == 1 {
        payload.extend_from_slice(&[0; 16]);
    }
    payload.extend_from_slice(raw);
    let source = carrier(&payload);
    let id = source.latest(424).unwrap().unwrap().id;
    source
        .record_424_metadata_prefix(&id, prefix_options())
        .unwrap()
        .medium_backend_prefix()
        .unwrap()
}
pub(super) fn carrier(payload: &[u8]) -> FlowCarrier {
    let mut wire = 8u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (424u32, &b"decoy"[..]),
        (424, payload),
        (999, &b"opaque"[..]),
    ] {
        wire.extend_from_slice(&id.to_le_bytes());
        wire.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        wire.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 2_000_000,
            maximum_payload_bytes: 1_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}
pub(super) fn prefix_options() -> FlowMetadataPrefixOptions {
    FlowMetadataPrefixOptions {
        string_mode: FlowMetadataStringMode::NativeUtf16,
        maximum_payload_bytes: 1_000_000,
        maximum_vectors: 1024,
        maximum_objects: 1024,
        maximum_values: 100_000,
        maximum_string_units: 100_000,
    }
}
fn open(raw: &[u8], options: FlowMediumBodyOptions) -> bessemer::Result<FlowMediumKindOne> {
    backend(raw, 1).first_kind_one_body(options)
}
type ExpectedWide = (FlowMediumWideField, usize, Vec<u64>);
fn fields(case: &BodyCase) -> Vec<ExpectedWide> {
    let mut result = Vec::new();
    for (ordinal, curve) in case.prefix.curves.iter().enumerate() {
        curve_fields(&mut result, ordinal, curve);
    }
    let offset = shared_fields(&mut result, case);
    push_wide(
        &mut result,
        FlowMediumWideField::FinalScalars,
        offset,
        &case.footer.scalars,
    );
    result
}
fn push_wide(
    result: &mut Vec<ExpectedWide>,
    field: FlowMediumWideField,
    offset: usize,
    words: &[u64],
) {
    result.push((field, offset, words.to_vec()));
}
fn curve_fields(result: &mut Vec<ExpectedWide>, ordinal: usize, curve: &BodyCurve) {
    let field = FlowMediumWideField::CurveScalar { curve: ordinal };
    push_wide(result, field, curve.start as usize + 1, &[curve.scalar]);
    let mut offset = curve.start as usize + 13;
    for (index, (array, words)) in curve_arrays(curve).into_iter().enumerate() {
        if index == 2 {
            offset += 1;
        }
        if index < 2 || curve.flag != 0 {
            let field = FlowMediumWideField::CurveArray {
                curve: ordinal,
                array,
            };
            push_wide(result, field, offset, words);
        }
        offset += words.len() * 8;
    }
    assert_eq!(offset, curve.end as usize);
}
fn curve_arrays(curve: &BodyCurve) -> [(FlowMediumCurveArray, &[u64]); 4] {
    [
        (FlowMediumCurveArray::First, &curve.first),
        (FlowMediumCurveArray::Second, &curve.second),
        (FlowMediumCurveArray::ExtraFirst, &curve.extra_first),
        (FlowMediumCurveArray::ExtraSecond, &curve.extra_second),
    ]
}

fn shared_fields(result: &mut Vec<ExpectedWide>, case: &BodyCase) -> usize {
    let mut offset = case.prefix.curves.last().unwrap().end as usize + 4;
    for (array, words) in case
        .prefix
        .first
        .iter()
        .chain(&case.prefix.second)
        .enumerate()
    {
        if array == 2 {
            offset += 4;
        }
        push_wide(
            result,
            FlowMediumWideField::SharedArray { array },
            offset,
            words,
        );
        offset += words.len() * 8;
    }
    offset
}
