use super::*;
use bessemer::flow::*;
#[path = "api/bounds.rs"]
mod bounds;
#[path = "api/edits.rs"]
mod edits;
#[path = "api/growth.rs"]
mod growth;
#[path = "api/read.rs"]
mod read;
#[path = "api/widths.rs"]
mod widths;

fn count(value: u32) -> FlowFieldCount {
    FlowFieldCount::try_from(i64::from(value)).unwrap()
}

fn layout(context: &FieldContext) -> FlowFieldLayout {
    match context {
        FieldContext::Legacy {
            table_count,
            object_widths,
            expected,
        } => {
            let widths = object_widths.iter().copied().map(count).collect();
            let expected = expected
                .as_ref()
                .map(|entries| entries.iter().map(|entry| entry.map(count)).collect());
            FlowFieldLayout::Legacy(
                FlowLegacyFieldLayout::new(count(*table_count), widths, expected).unwrap(),
            )
        }
        FieldContext::Regions {
            rows,
            widths,
            active,
        } => region_layout(rows, widths, active),
    }
}

fn region_layout(rows: &[u32], widths: &[u32], active: &[bool]) -> FlowFieldLayout {
    assert_eq!(rows.len(), widths.len());
    assert_eq!(rows.len(), active.len());
    let regions = rows
        .iter()
        .zip(widths)
        .zip(active)
        .map(|((rows, width), active)| FlowFieldRegionContext {
            row_count: count(*rows),
            width: count(*width),
            active: *active,
        })
        .collect();
    FlowFieldLayout::Regions(FlowRegionFieldLayout::new(regions).unwrap())
}

fn options(frame: &FieldFrame) -> FlowFieldValueOptions {
    FlowFieldValueOptions {
        body_compression: if frame.compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        compatibility_restriction: frame.restriction,
        layout: layout(&frame.context),
        maximum_decoded_body_bytes: 2_000_000,
        maximum_items: 10_000,
        maximum_table_words: 10_000,
        maximum_wide_words: 1_000_000,
    }
}

fn carrier(payload: &[u8], identifier: u32, maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (identifier, &b"decoy"[..]),
        (identifier, payload),
        (999, &b"opaque\0"[..]),
    ] {
        raw.extend_from_slice(&id.to_le_bytes());
        raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: maximum + 100,
            maximum_payload_bytes: maximum,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn open(payload: &[u8], options: FlowFieldValueOptions) -> bessemer::Result<FlowFieldValues> {
    let source = carrier(payload, 108, 2_000_000);
    let id = source.latest(108).unwrap().unwrap().id;
    source.record_108_values(&id, options)
}

fn read_case(case: &FieldCase) -> FlowFieldValues {
    open(&case.frame.stored, options(&case.frame)).unwrap()
}

fn assert_invalid(result: bessemer::Result<FlowFieldValues>) {
    let error = result.unwrap_err();
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("record 108"));
    assert!(!error.cause.is_empty());
}
