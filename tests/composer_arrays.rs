//! Independent wire fixtures and immutable packed-array operations.
#[path = "common/composer_arrays.rs"]
mod reference;
use bessemer::composer::arrays::*;

#[test]
fn all_owned_vectors_and_index_width_boundaries_reconstruct_and_edit_exactly() {
    let vectors = reference::read_cases(include_bytes!("fixtures/composer-arrays/vectors.bin"));
    let indices = reference::read_cases(include_bytes!("fixtures/composer-arrays/indices.bin"));
    assert_eq!((vectors.len(), indices.len()), (54, 12));
    for case in vectors.iter().chain(&indices) {
        let changed = reference::check_case(case);
        assert!(changed.len() == case.wire.len());
    }
    for case in &vectors {
        check_vector_prefixes(case);
    }
}

fn check_vector_prefixes(case: &reference::ArrayCase) {
    let dimensions = if case.dimensions == 2 {
        ComposerVectorDimensions::Two
    } else {
        ComposerVectorDimensions::Three
    };
    let layout = ComposerVectorLayout {
        count: case.count,
        dimensions,
    };
    for end in 0..case.wire.len() {
        assert!(ComposerVectorBlock::from_bytes(&case.wire[..end], layout, 1000).is_err());
    }
    let mut extended = case.wire.clone();
    extended.push(0);
    assert!(ComposerVectorBlock::from_bytes(&extended, layout, 1000).is_err());
}

const QUANTIZED_WIRE: [u8; 28] = [
    99, 0, 1, 0, 0, 0, 0, 128, 0, 0, 128, 191, 0, 0, 128, 63, 1, 0, 128, 63, 2, 0, 128, 63, 0, 255,
    65, 127,
];
const QUANTIZED_AXES: [ComposerVectorAxis; 3] = [
    ComposerVectorAxis::Quantized {
        first: 0xbf800000,
        second: 0x3f800000,
    },
    ComposerVectorAxis::Constant(0x80000000),
    ComposerVectorAxis::Quantized {
        first: 0x3f800001,
        second: 0x3f800002,
    },
];

#[test]
fn quantized_headers_put_all_constants_before_endpoints_and_preserve_code_order() {
    let layout = ComposerVectorLayout {
        count: 2,
        dimensions: ComposerVectorDimensions::Three,
    };
    let block = ComposerVectorBlock::from_bytes(&QUANTIZED_WIRE, layout, 100).unwrap();
    assert_eq!(block.axes(), QUANTIZED_AXES);
    assert_eq!(block.codes().collect::<Vec<_>>(), [0, 255, 65, 127]);
    let packing = ComposerVectorPacking::Quantized8;
    let codes = [0, 255, 65, 127];
    let built =
        ComposerVectorBlock::from_parts(layout, packing, &QUANTIZED_AXES, &codes, 100).unwrap();
    assert_eq!(built.to_bytes(), QUANTIZED_WIRE);
}

#[test]
fn all_constant_vectors_bound_wire_work_independently_of_declared_row_count() {
    let layout = ComposerVectorLayout {
        count: u32::MAX,
        dimensions: ComposerVectorDimensions::Three,
    };
    let axes = [ComposerVectorAxis::Constant(0x7f812345); 3];
    let block =
        ComposerVectorBlock::from_parts(layout, ComposerVectorPacking::Quantized16, &axes, &[], 16)
            .unwrap();
    assert_eq!(block.coverage().stored_values, 0);
    assert_eq!(block.coverage().wire_bytes, 16);
    assert_eq!(
        block.edit_packed(&[]).unwrap().completion,
        ComposerArrayCompletion::Unchanged
    );
    let reopened = ComposerVectorBlock::from_bytes(&block.to_bytes(), layout, 16).unwrap();
    assert_eq!(reopened.axes(), axes);
}

fn direct_vector() -> ComposerVectorBlock {
    ComposerVectorBlock::from_parts(
        ComposerVectorLayout {
            count: 2,
            dimensions: ComposerVectorDimensions::Three,
        },
        ComposerVectorPacking::Direct32,
        &[
            ComposerVectorAxis::Constant(0x7f812345),
            ComposerVectorAxis::Direct,
            ComposerVectorAxis::Constant(0x80000000),
        ],
        &[0, 0, 128, 255, 69, 35, 193, 127],
        100,
    )
    .unwrap()
}

#[test]
fn independent_golden_preserves_nan_payloads_signed_zero_and_packed_row_order() {
    let block = direct_vector();
    let expected = [
        97, 1, 0, 1, 69, 35, 129, 127, 0, 0, 0, 128, 0, 0, 128, 255, 69, 35, 193, 127,
    ];
    assert_eq!(block.to_bytes(), expected);
    assert_eq!(block.codes().collect::<Vec<_>>(), [0xff800000, 0x7fc12345]);
    let edit = block
        .edit_packed(&[1, 0, 128, 255, 69, 35, 193, 127])
        .unwrap();
    let mut golden = expected;
    golden[12] = 1;
    assert_eq!(edit.block.to_bytes(), golden);
    let reverted = edit.block.edit_packed(block.packed_bytes()).unwrap();
    assert_eq!(reverted.block.to_bytes(), expected);
    assert_eq!(block.to_bytes(), expected);
}

#[test]
fn edit_contract_reports_complete_scope_fidelity_identity_and_exclusions() {
    let block = direct_vector();
    let noop = block.edit_packed(block.packed_bytes()).unwrap();
    assert_eq!(noop.completion, ComposerArrayCompletion::Unchanged);
    assert_eq!(noop.block.revision(), block.revision());
    assert_eq!(
        noop.history,
        ComposerArrayHistory::Unchanged(block.revision().clone())
    );
}

#[test]
fn changed_edits_derive_a_new_revision_from_the_complete_parent() {
    let block = direct_vector();
    let changed = block.edit_packed(&[0; 8]).unwrap();
    assert_eq!(changed.completion, ComposerArrayCompletion::Changed);
    assert_eq!(
        changed.history,
        ComposerArrayHistory::Derived {
            parent: block.revision().clone(),
            result: changed.block.revision().clone()
        }
    );
    assert_ne!(changed.block.revision(), block.revision());
}

#[test]
fn completed_edits_report_exact_stored_bits_and_wire_coverage() {
    let changed = direct_vector().edit_packed(&[0; 8]).unwrap();
    assert_eq!(changed.fidelity, ComposerArrayFidelity::ExactStoredValues);
    assert_eq!(
        changed.coverage,
        ComposerArrayCoverage {
            mode: ComposerArrayCheckMode::ExactPackedArray,
            wire_bytes: 20,
            stored_values: 2
        }
    );
}

#[test]
fn completed_edits_keep_unexecuted_geometry_and_numeric_checks_explicit() {
    let changed = direct_vector().edit_packed(&[0; 8]).unwrap();
    assert_eq!(
        changed.diagnostics,
        [
            ComposerArrayDiagnostic::FloatingReconstructionNotChecked,
            ComposerArrayDiagnostic::GeometryNotChecked,
            ComposerArrayDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn index_domains_select_storage_width_without_inventing_value_bounds() {
    for (domain, values) in [
        (0, vec![255]),
        (255, vec![255]),
        (256, vec![65535]),
        (65535, vec![65535]),
        (65536, vec![u32::MAX]),
    ] {
        check_index_identity(domain, &values);
    }
    assert!(ComposerIndexBlock::from_values(&[256], 255, 100).is_err());
    assert!(ComposerIndexBlock::from_values(&[65536], 65535, 100).is_err());
}

fn check_index_identity(domain: u32, values: &[u32]) {
    let block = ComposerIndexBlock::from_values(values, domain, 100).unwrap();
    let noop = block.edit_values(values).unwrap();
    assert_eq!(noop.block.revision(), block.revision());
    let edit = block.edit_values(&[0]).unwrap();
    assert_ne!(edit.block.revision(), block.revision());
    assert_eq!(edit.block.values().collect::<Vec<_>>(), [0]);
    assert_eq!(block.values().collect::<Vec<_>>(), values);
}

#[test]
fn source_buffers_failed_edits_and_concurrent_results_remain_isolated() {
    let block = direct_vector();
    let mut bytes = block.to_bytes();
    let parsed = ComposerVectorBlock::from_bytes(&bytes, block.layout(), 100).unwrap();
    bytes.fill(0);
    assert_eq!(parsed.to_bytes(), block.to_bytes());
    assert!(parsed.edit_packed(&[]).is_err());
    let expected = parsed.edit_packed(&[0; 8]).unwrap().block.to_bytes();
    std::thread::scope(|scope| {
        let handles = (0..6)
            .map(|_| scope.spawn(|| parsed.edit_packed(&[0; 8]).unwrap().block.to_bytes()))
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(handle.join().unwrap(), expected);
        }
    });
    assert_eq!(parsed.to_bytes(), block.to_bytes());
}

#[test]
fn all_public_array_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<ComposerVectorBlock>();
    assert_traits::<ComposerIndexBlock>();
    assert_traits::<ComposerVectorLayout>();
    assert_traits::<ComposerVectorDimensions>();
    assert_traits::<ComposerVectorPacking>();
    assert_traits::<ComposerVectorAxis>();
    assert_traits::<ComposerArrayRevision>();
    assert_traits::<ComposerArrayCheckMode>();
    assert_traits::<ComposerArrayCoverage>();
    assert_traits::<ComposerArrayFidelity>();
    assert_traits::<ComposerArrayDiagnostic>();
    assert_traits::<ComposerArrayHistory>();
    assert_traits::<ComposerArrayCompletion>();
    assert_traits::<ComposerArrayEdit<ComposerVectorBlock>>();
    assert_traits::<ComposerArrayEdit<ComposerIndexBlock>>();
}
