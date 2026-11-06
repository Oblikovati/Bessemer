use super::*;
use bessemer::flow::*;
#[path = "bounds.rs"]
mod bounds;
#[path = "concurrency.rs"]
mod concurrency;
#[path = "edits.rs"]
mod edits;
#[path = "growth.rs"]
mod growth;

#[derive(Clone, Debug)]
enum SimpleView {
    Parallel(FlowParallelTable),
    LeafPairs(FlowLeafPairs),
    Five(FlowFiveWordTable),
    Bytes(FlowRawByteTable),
}

impl SimpleView {
    fn source(&self) -> &FlowCarrier {
        match self {
            Self::Parallel(table) => table.source(),
            Self::LeafPairs(table) => table.source(),
            Self::Five(table) => table.source(),
            Self::Bytes(table) => table.source(),
        }
    }
    fn record(&self) -> &FlowRecordId {
        match self {
            Self::Parallel(table) => table.record(),
            Self::LeafPairs(table) => table.record(),
            Self::Five(table) => table.record(),
            Self::Bytes(table) => table.record(),
        }
    }
    fn decoded(&self) -> &[u8] {
        match self {
            Self::Parallel(table) => table.decoded_bytes(),
            Self::LeafPairs(table) => table.decoded_bytes(),
            Self::Five(table) => table.decoded_bytes(),
            Self::Bytes(table) => table.decoded_bytes(),
        }
    }
}

fn carrier(frame: &SimpleFrame, maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (frame.identifier, &b"decoy"[..]),
        (frame.identifier, frame.stored.as_slice()),
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

fn compression(frame: &SimpleFrame) -> FlowLegacyTableTransport {
    if frame.compressed {
        FlowLegacyTableTransport::Zlib
    } else {
        FlowLegacyTableTransport::Raw
    }
}

fn parallel_options(frame: &SimpleFrame) -> FlowParallelTableOptions {
    FlowParallelTableOptions {
        compression: compression(frame),
        maximum_decoded_bytes: 2_000_000,
        maximum_wide_words: 50_000,
    }
}

fn leaf_options(frame: &SimpleFrame) -> FlowLeafPairOptions {
    FlowLeafPairOptions {
        compression: compression(frame),
        leaf_count: FlowLeafCount::try_from(frame.leaf.unwrap()).unwrap(),
        maximum_decoded_bytes: 2_000_000,
        maximum_wire_rows: 10_000,
    }
}

fn legacy_count(frame: &SimpleFrame) -> Option<FlowSimpleLegacyCount> {
    frame
        .legacy
        .map(|count| FlowSimpleLegacyCount::try_from(count).unwrap())
}

fn five_options(frame: &SimpleFrame) -> FlowFiveWordTableOptions {
    FlowFiveWordTableOptions {
        compression: compression(frame),
        legacy_count: legacy_count(frame),
        maximum_decoded_bytes: 2_000_000,
        maximum_wire_rows: 10_000,
    }
}

fn byte_options(frame: &SimpleFrame) -> FlowRawByteTableOptions {
    FlowRawByteTableOptions {
        compression: compression(frame),
        legacy_count: legacy_count(frame),
        maximum_decoded_bytes: 2_000_000,
        maximum_raw_bytes: 10_000,
    }
}

fn read_case(case: &SimpleCase) -> SimpleView {
    let source = carrier(&case.frame, 2_000_000);
    let id = source.latest(case.frame.identifier).unwrap().unwrap().id;
    read_at(&source, &id, &case.frame).unwrap()
}

fn read_at(
    source: &FlowCarrier,
    id: &FlowRecordId,
    frame: &SimpleFrame,
) -> bessemer::Result<SimpleView> {
    match frame.identifier {
        407 => source
            .record_407_arrays(id, parallel_options(frame))
            .map(SimpleView::Parallel),
        437 => source
            .record_437_leaf_pairs(id, leaf_options(frame))
            .map(SimpleView::LeafPairs),
        411 => source
            .record_411_rows(id, five_options(frame))
            .map(SimpleView::Five),
        413 => source
            .record_413_bytes(id, byte_options(frame))
            .map(SimpleView::Bytes),
        id => panic!("identifier {id}; expected simple table"),
    }
}

#[test]
fn every_118_view_matches_complete_fields_count_origins_intervals_and_checkpoints() {
    for case in cases() {
        read::check_case(&case);
    }
}

#[path = "api/read.rs"]
mod read;
