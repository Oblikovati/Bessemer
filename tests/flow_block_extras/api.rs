use super::*;
use bessemer::flow::*;
#[path = "oracle.rs"]
mod oracle;
use oracle::{ExpectedExtraField, ExpectedExtraFrame};
#[path = "bounds.rs"]
mod bounds;
#[path = "concurrency.rs"]
mod concurrency;
#[path = "edits.rs"]
mod edits;

fn options(case: &ExtraCase) -> FlowBlockExtraOptions {
    FlowBlockExtraOptions {
        compression: if case.compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        enabled: FlowBlockExtrasEnabled::AssumeEnabled,
        leaf_count: FlowLeafCount::try_from(case.leaves).unwrap(),
        maximum_decoded_bytes: 5_000_000,
        maximum_blocks: 10_000,
        maximum_entities: 200_000,
        maximum_fields: 1_000_000,
        maximum_values: 5_000_000,
    }
}

fn carrier(payload: &[u8], maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (identifier, bytes) in [
        (408_u32, &b"decoy"[..]),
        (408, payload),
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

fn extras(case: &ExtraCase) -> FlowBlockExtras {
    let source = carrier(&case.stored, 5_000_000);
    source
        .record_408_extras(&source.latest(408).unwrap().unwrap().id, options(case))
        .unwrap()
}

#[test]
fn every_field_scope_stride_raw_value_and_context_matches_the_independent_oracle() {
    let cases = cases();
    let frames = oracle::frames();
    assert_eq!(cases.len(), frames.len());
    for (case, expected) in cases.iter().zip(&frames) {
        assert_eq!(case.name, expected.name);
        let actual = extras(case);
        check_checkpoint(&actual, case);
        check_scopes(&actual, expected);
        assert_eq!(actual.fields().len(), expected.fields.len());
        assert!(
            actual.field_at(expected.fields.len()).is_none()
                && actual.field_at(usize::MAX).is_none()
        );
        for (field, frozen) in actual.fields().zip(&expected.fields) {
            check_field(field, frozen);
        }
        check_coverage(&actual, case, expected);
    }
}

fn check_checkpoint(actual: &FlowBlockExtras, case: &ExtraCase) {
    assert_eq!(actual.version(), case.version);
    assert_eq!(actual.decoded_bytes(), case.decoded);
    assert_eq!(actual.record().ordinal(), 1);
    assert_eq!(
        actual.source().latest(408).unwrap().unwrap().id,
        *actual.record()
    );
    assert_eq!(actual.options().leaf_count.value(), case.leaves);
    assert_eq!(
        actual.options().enabled,
        FlowBlockExtrasEnabled::AssumeEnabled
    );
}

fn check_scopes(actual: &FlowBlockExtras, expected: &ExpectedExtraFrame) {
    assert_eq!(actual.scopes().len(), expected.scopes.len());
    for (scope, frozen) in actual.scopes().zip(&expected.scopes) {
        assert_eq!(scope.entity(), frozen.entity);
        assert_eq!(scope.payload_interval(), frozen.interval);
        assert_eq!(scope.native_index(), frozen.native_index);
        assert_eq!(scope.index_is_stored(), frozen.stored);
    }
}

fn check_field(actual: FlowBlockExtraField<'_>, expected: &ExpectedExtraField) {
    assert_eq!(actual.entity(), expected.entity);
    assert_eq!(actual.kind(), expected.kind);
    assert_eq!(actual.encoding(), expected.encoding);
    assert_eq!(actual.layout(), expected.layout);
    assert_eq!(actual.is_editable(), expected.editable);
    assert_eq!(actual.kind().is_structural(), !expected.editable);
    assert_eq!(
        actual.encoding().element_bytes(),
        expected.layout.element_bytes
    );
    check_elements(actual, expected);
    check_field_identity(actual);
}

fn check_elements(actual: FlowBlockExtraField<'_>, expected: &ExpectedExtraField) {
    let layout = expected.layout;
    assert_eq!(
        expected.packed.len(),
        layout.element_count * layout.element_bytes
    );
    for (index, raw) in expected
        .packed
        .chunks_exact(layout.element_bytes)
        .enumerate()
    {
        check_element(actual, expected, index, raw);
    }
    check_element_bounds(actual, expected);
}

fn expected_value(encoding: FlowBlockExtraEncoding, raw: &[u8]) -> FlowBlockExtraValue {
    match encoding {
        FlowBlockExtraEncoding::Byte => FlowBlockExtraValue::Byte(raw[0]),
        FlowBlockExtraEncoding::Signed32 => {
            FlowBlockExtraValue::Signed32(i32::from_le_bytes(raw.try_into().unwrap()))
        }
        FlowBlockExtraEncoding::WideBits => {
            FlowBlockExtraValue::WideBits(u64::from_le_bytes(raw.try_into().unwrap()))
        }
    }
}

fn check_field_identity(actual: FlowBlockExtraField<'_>) {
    let id = actual.id();
    assert_eq!(id.ordinal(), actual.ordinal());
    assert!(id.leaf_count().value() <= i32::MAX as u32);
    assert_eq!(id.record().ordinal(), 1);
}

fn check_coverage(actual: &FlowBlockExtras, case: &ExtraCase, expected: &ExpectedExtraFrame) {
    let coverage = actual.coverage();
    assert_eq!(
        (coverage.stored_bytes, coverage.decoded_bytes),
        (case.stored.len(), case.decoded.len())
    );
    assert_eq!(
        (coverage.entities, coverage.fields),
        (expected.scopes.len(), expected.fields.len())
    );
    assert_eq!(
        coverage.values,
        expected.fields.iter().map(|f| f.layout.element_count).sum()
    );
    assert_eq!(coverage.leaf_count.value(), case.leaves);
    assert_eq!(coverage.enabled, FlowBlockExtrasEnabled::AssumeEnabled);
    assert_eq!(coverage.achieved_error, 0);
    check_mode(actual, case, coverage);
}

fn check_element_bounds(actual: FlowBlockExtraField<'_>, expected: &ExpectedExtraField) {
    let layout = expected.layout;
    for index in [layout.element_count, usize::MAX] {
        assert!(
            actual.value(index).is_none()
                && actual.element_bytes(index).is_none()
                && actual.element_offset(index).is_none()
        );
    }
    let packed =
        (layout.stride_bytes == layout.element_bytes).then_some(expected.packed.as_slice());
    assert_eq!(actual.contiguous_bytes(), packed);
}

fn check_mode(actual: &FlowBlockExtras, case: &ExtraCase, coverage: FlowBlockExtraCoverage) {
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

fn check_element(
    actual: FlowBlockExtraField<'_>,
    expected: &ExpectedExtraField,
    index: usize,
    raw: &[u8],
) {
    let layout = expected.layout;
    assert_eq!(
        actual.element_offset(index),
        Some(layout.first_byte + index * layout.stride_bytes)
    );
    assert_eq!(actual.element_bytes(index), Some(raw));
    assert_eq!(
        actual.value(index),
        Some(expected_value(expected.encoding, raw))
    );
}

fn check_diagnostics(actual: &FlowBlockExtras) {
    assert_eq!(
        actual.diagnostics(),
        &[
            FlowBlockExtraDiagnostic::EnabledContextNotReconstructed,
            FlowBlockExtraDiagnostic::TreeOrderNotAssigned,
            FlowBlockExtraDiagnostic::NativeObjectReferencesNotResolved,
            FlowBlockExtraDiagnostic::EngineeringRolesNotResolved,
            FlowBlockExtraDiagnostic::NativeRuntimeNotReproduced,
            FlowBlockExtraDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
