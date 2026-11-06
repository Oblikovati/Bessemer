use super::*;
use bessemer::flow::*;
#[path = "bounds.rs"]
mod bounds;
#[path = "edits.rs"]
mod edits;

fn options(case: &LinkCase) -> FlowLinkRecordOptions {
    FlowLinkRecordOptions {
        compression: if case.compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        global_pointer_count: case
            .pointer_count
            .map(|v| FlowGlobalPointerCount::try_from(v as u32).unwrap()),
        maximum_decoded_bytes: 1_000_000,
        maximum_entries: 10_000,
        maximum_array_elements: 100_000,
    }
}

fn carrier(payload: &[u8], maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (identifier, bytes) in [
        (402_u32, &b"decoy"[..]),
        (402, payload),
        (999, &b"opaque\0"[..]),
    ] {
        raw.extend_from_slice(&identifier.to_le_bytes());
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

fn links(case: &LinkCase) -> FlowLinkRecords {
    let source = carrier(&case.stored, 1_000_000);
    source
        .record_402_links(&source.latest(402).unwrap().unwrap().id, options(case))
        .unwrap()
}

pub fn check_case(case: &LinkCase) {
    let actual = links(case);
    check_checkpoint(&actual, case);
    assert_eq!(actual.entries().len(), case.entries.len());
    assert!(actual.entry(case.entries.len()).is_none() && actual.entry(usize::MAX).is_none());
    for (entry, frozen) in actual.entries().zip(&case.entries) {
        check_entry(entry, frozen);
        check_entry(actual.entry(frozen.ordinal as usize).unwrap(), frozen);
    }
    check_tail(actual.tail(), &case.tail, case.decoded.len());
    check_coverage(&actual, case);
}

fn check_entry(actual: FlowLinkEntry<'_>, frozen: &LinkEntry) {
    assert_eq!(actual.ordinal(), frozen.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (frozen.start as usize, frozen.end as usize)
    );
    assert_eq!(actual.reference_indices(), frozen.references);
    check_wide(actual.wide_words(), &frozen.wide, frozen.start as usize + 8);
    assert_eq!(actual.code_byte(), frozen.code);
    assert_eq!(actual.native_code_byte(), frozen.native_code);
    check_integers(
        actual.variable_integers(),
        &frozen.variable,
        frozen.start as usize + 66,
    );
}

fn check_wide(actual: FlowTableWideArray<'_>, values: &[u64], start: usize) {
    assert_eq!(actual.word_count(), values.len());
    assert_eq!(actual.payload_interval(), (start, start + values.len() * 8));
    assert_eq!(
        actual.as_bytes(),
        values
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<_>>()
    );
    for (index, &value) in values.iter().enumerate() {
        assert_eq!(actual.raw_bits(index), Some(value));
    }
    assert!(actual.raw_bits(values.len()).is_none() && actual.raw_bits(usize::MAX).is_none());
}

fn check_integers(actual: FlowLinkIntegers<'_>, values: &[i32], start: usize) {
    assert_eq!(actual.word_count(), values.len());
    assert_eq!(actual.payload_interval(), (start, start + values.len() * 4));
    assert_eq!(
        actual.as_bytes(),
        values
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<_>>()
    );
    assert_eq!(actual.values().collect::<Vec<_>>(), values);
    for (index, &value) in values.iter().enumerate() {
        assert_eq!(actual.value(index), Some(value));
    }
    assert!(actual.value(values.len()).is_none() && actual.value(usize::MAX).is_none());
}

fn check_tail(actual: FlowLinkTail<'_>, frozen: &LinkTail, length: usize) {
    check_tail_prefix(actual, frozen, length);
    assert_eq!(actual.numeric().is_some(), frozen.numeric.is_some());
    if let Some(numeric) = &frozen.numeric {
        check_numeric(actual.numeric().unwrap(), numeric);
    }
    assert_eq!(actual.final_array().is_some(), frozen.final_tail.is_some());
    if let Some(final_tail) = &frozen.final_tail {
        check_final(actual.final_array().unwrap(), final_tail);
    }
}

fn check_numeric(actual: FlowLinkNumericTail<'_>, frozen: &NumericTail) {
    assert_eq!(
        actual.payload_interval(),
        (frozen.start as usize, frozen.end as usize)
    );
    check_wide(actual.wide_words(), &frozen.wide, frozen.start as usize + 4);
    assert_eq!(actual.runtime_default_index(), frozen.runtime_default);
    assert_eq!(actual.stored_indices().is_some(), frozen.indices.is_some());
    if let Some(indices) = &frozen.indices {
        check_integers(
            actual.stored_indices().unwrap(),
            indices,
            frozen.start as usize + 4 + frozen.wide.len() * 8,
        );
    }
}

fn check_final(actual: FlowLinkFinalTail<'_>, frozen: &FinalTail) {
    assert_eq!(
        actual.payload_interval(),
        (frozen.start as usize, frozen.end as usize)
    );
    assert_eq!(actual.element_bytes(), frozen.width as usize);
    assert_eq!(actual.wide_words().is_some(), frozen.width == 8);
    assert_eq!(actual.signed_words().is_some(), frozen.width == 4);
    if frozen.width == 8 {
        check_wide(
            actual.wide_words().unwrap(),
            &frozen.values,
            frozen.start as usize + 4,
        );
    } else {
        check_integers(
            actual.signed_words().unwrap(),
            &frozen.values.iter().map(|v| *v as i32).collect::<Vec<_>>(),
            frozen.start as usize + 4,
        );
    }
}

fn array_elements(case: &LinkCase) -> usize {
    let numeric = case
        .tail
        .numeric
        .as_ref()
        .map_or(0, |n| n.wide.len() + n.indices.as_ref().map_or(0, Vec::len));
    case.variable_count as usize
        + case.tail.conditional.len()
        + numeric
        + case.tail.final_tail.as_ref().map_or(0, |t| t.values.len())
}

fn check_coverage(actual: &FlowLinkRecords, case: &LinkCase) {
    let coverage = actual.coverage();
    check_coverage_counts(coverage, case);
    assert_eq!(
        coverage.pointer_range,
        actual.options().global_pointer_count
    );
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(
        coverage.mode,
        if case.compressed {
            FlowLegacyTableCheckMode::ExactZlibTable
        } else {
            FlowLegacyTableCheckMode::ExactRawTable
        }
    );
    check_diagnostics(actual);
}

fn check_checkpoint(actual: &FlowLinkRecords, case: &LinkCase) {
    assert_eq!(actual.decoded_bytes(), case.decoded);
    assert_eq!(actual.version(), case.version);
    assert_eq!(actual.record().ordinal(), 1);
    assert_eq!(
        actual.source().latest(402).unwrap().unwrap().id,
        *actual.record()
    );
    assert_eq!(
        actual
            .options()
            .global_pointer_count
            .map(|v| v.value() as i32),
        case.pointer_count
    );
}

fn check_tail_prefix(actual: FlowLinkTail<'_>, frozen: &LinkTail, length: usize) {
    assert_eq!(actual.payload_interval(), (frozen.start as usize, length));
    assert_eq!(actual.members(), frozen.members);
    check_integers(
        actual.discarded_words(),
        &frozen.discarded,
        frozen.start as usize + 44,
    );
    assert_eq!(actual.control(), frozen.control);
    check_integers(
        actual.conditional_integers(),
        &frozen.conditional,
        frozen.conditional_start as usize,
    );
    assert_eq!(
        actual.conditional_integers().payload_interval().1,
        frozen.conditional_end as usize
    );
}

fn check_coverage_counts(coverage: FlowLinkRecordCoverage, case: &LinkCase) {
    assert_eq!(
        (coverage.stored_bytes, coverage.decoded_bytes),
        (case.stored.len(), case.decoded.len())
    );
    assert_eq!(
        (
            coverage.entries,
            coverage.variable_integers,
            coverage.array_elements
        ),
        (
            case.entries.len(),
            case.variable_count as usize,
            array_elements(case)
        )
    );
}

fn check_diagnostics(actual: &FlowLinkRecords) {
    assert_eq!(
        actual.diagnostics(),
        &[
            FlowLinkRecordDiagnostic::NativeRuntimeNotReproduced,
            FlowLinkRecordDiagnostic::EngineeringRolesNotResolved,
            FlowLinkRecordDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowLinkRecordDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
