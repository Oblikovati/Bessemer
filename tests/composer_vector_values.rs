//! Exact-rational binary32 profile oracles and immutable numerical results.
#[path = "common/composer_vector_values.rs"]
mod reference;
use bessemer::composer::arrays::*;

const PROFILE: ComposerVectorProfile = ComposerVectorProfile::NearestEvenGradual79V1;
const PAIR_LAYOUT: ComposerVectorLayout = ComposerVectorLayout {
    count: 1,
    dimensions: ComposerVectorDimensions::Two,
};

fn quantized_pair(count: u32, endpoints: (u32, u32)) -> ComposerVectorBlock {
    let axes = [ComposerVectorAxis::Quantized {
        first: endpoints.0,
        second: endpoints.1,
    }; 2];
    let codes = vec![65; count as usize * 2];
    ComposerVectorBlock::from_parts(
        ComposerVectorLayout {
            count,
            ..PAIR_LAYOUT
        },
        ComposerVectorPacking::Quantized8,
        &axes,
        &codes,
        100,
    )
    .unwrap()
}

#[test]
fn all_weights_endpoint_degeneracies_and_generated_pairs_match_exact_ratio_oracles() {
    let cases = reference::value_cases(include_bytes!("fixtures/composer-vectors/values.bin"));
    assert_eq!(cases.len(), 8224);
    for case in cases {
        reference::verify_case(&case);
    }
}

#[test]
fn separate_binary32_steps_match_the_native_leaf_counterexample() {
    let block = quantized_pair(1, (0xbf800000, 0x3f800000));
    let values = block.reconstruct(PROFILE, 2).unwrap();
    assert_eq!(values.patterns()[0].bits(), 0xbefafaf9);
    assert_ne!(values.patterns()[0].bits(), 0xbefafafb);
    assert_eq!(
        values.report().completion,
        ComposerVectorValueCompletion::Completed
    );
    assert_eq!(
        values.report().fidelity,
        ComposerVectorValueFidelity::ExactProfilePatterns
    );
    assert!(!values.report().coverage.raw_copy_only);
}

#[test]
fn nonfinite_varying_endpoints_fail_only_when_arithmetic_is_requested() {
    for word in [0x7f800000, 0xff800000, 0x7f812345, 0x7fc12345] {
        let block = quantized_pair(1, (word, 0));
        let original = block.to_bytes();
        let error = block.reconstruct(PROFILE, 2).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert_eq!(error.operation, "reconstruct Composer vectors");
        assert_eq!(block.to_bytes(), original);
        let empty = quantized_pair(0, (word, word))
            .reconstruct(PROFILE, 0)
            .unwrap();
        assert!(empty.patterns().is_empty());
        assert_eq!(empty.report().coverage.components, 0);
    }
}

const COPIED_AXES: [ComposerVectorAxis; 3] = [
    ComposerVectorAxis::Constant(0x7f812345),
    ComposerVectorAxis::Constant(0xff800000),
    ComposerVectorAxis::Constant(0x80000000),
];
const MIXED_AXES: [ComposerVectorAxis; 3] = [
    ComposerVectorAxis::Constant(0x80000000),
    ComposerVectorAxis::Direct,
    ComposerVectorAxis::Direct,
];
const XYZ_LAYOUT: ComposerVectorLayout = ComposerVectorLayout {
    count: 1,
    dimensions: ComposerVectorDimensions::Three,
};

fn value_bits(values: &ComposerVectorValues) -> Vec<u32> {
    values.patterns().iter().map(|value| value.bits()).collect()
}

#[test]
fn constant_lanes_copy_nonfinite_patterns_and_signed_zero_without_arithmetic() {
    let layout = ComposerVectorLayout {
        count: 2,
        ..XYZ_LAYOUT
    };
    let packing = ComposerVectorPacking::Quantized8;
    let block = ComposerVectorBlock::from_parts(layout, packing, &COPIED_AXES, &[], 16).unwrap();
    let values = block.reconstruct(PROFILE, 6).unwrap();
    let expected = [
        0x7f812345, 0xff800000, 0x80000000, 0x7f812345, 0xff800000, 0x80000000,
    ];
    assert_eq!(value_bits(&values), expected);
    assert!(values.report().coverage.raw_copy_only);
}

#[test]
fn direct_lanes_preserve_nonfinite_words_and_mix_with_constants_in_row_order() {
    let codes = [0x45, 0x23, 0x81, 0x7f, 0, 0, 0x80, 0xff];
    let packing = ComposerVectorPacking::Direct32;
    let block =
        ComposerVectorBlock::from_parts(XYZ_LAYOUT, packing, &MIXED_AXES, &codes, 100).unwrap();
    let values = block.reconstruct(PROFILE, 3).unwrap();
    assert_eq!(value_bits(&values), [0x80000000, 0x7f812345, 0xff800000]);
    assert!(values.report().coverage.raw_copy_only);
}

#[test]
fn output_budget_prevents_constant_expansion_and_keeps_the_source_valid() {
    let axes = [ComposerVectorAxis::Constant(0); 3];
    let layout = ComposerVectorLayout {
        count: u32::MAX,
        dimensions: ComposerVectorDimensions::Three,
    };
    let block =
        ComposerVectorBlock::from_parts(layout, ComposerVectorPacking::Direct32, &axes, &[], 16)
            .unwrap();
    let original = block.to_bytes();
    assert!(block.reconstruct(PROFILE, 100).is_err());
    assert_eq!(block.to_bytes(), original);
    assert!(quantized_pair(1, (0, 0)).reconstruct(PROFILE, 1).is_err());
}

#[test]
fn published_values_survive_source_edits_drops_and_concurrent_reconstruction() {
    let block = quantized_pair(1, (0xbf800000, 0x3f800000));
    let values = block.reconstruct(PROFILE, 2).unwrap();
    let edit = block.edit_packed(&[0, 255]).unwrap();
    assert_ne!(
        edit.block.reconstruct(PROFILE, 2).unwrap().patterns(),
        values.patterns()
    );
    std::thread::scope(|scope| {
        let handles = (0..6)
            .map(|_| scope.spawn(|| block.reconstruct(PROFILE, 2).unwrap()))
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(handle.join().unwrap().patterns(), values.patterns());
        }
    });
    drop(block);
    assert_eq!(values.patterns()[0].bits(), 0xbefafaf9);
}

#[test]
fn reconstruction_history_and_diagnostics_preserve_the_control_state_boundary() {
    let block = quantized_pair(1, (0, 0x3f800000));
    let values = block.reconstruct(PROFILE, 2).unwrap();
    assert_eq!(values.report().profile, PROFILE);
    assert_eq!(values.report().coverage.source, block.coverage());
    let ComposerArrayHistory::Derived { parent, result } = &values.report().history else {
        panic!("expected derived values")
    };
    assert_eq!(parent, block.revision());
    assert_ne!(parent, result);
    assert_eq!(
        values.report().diagnostics,
        [
            ComposerVectorValueDiagnostic::NativeControlStateNotChecked,
            ComposerVectorValueDiagnostic::GeometryNotChecked,
            ComposerVectorValueDiagnostic::QuantizationErrorNotEstablished,
            ComposerVectorValueDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn all_numeric_result_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<ComposerVectorProfile>();
    assert_traits::<ComposerVectorValues>();
    assert_traits::<ComposerVectorValueCompletion>();
    assert_traits::<ComposerVectorValueFidelity>();
    assert_traits::<ComposerVectorValueCheckMode>();
    assert_traits::<ComposerVectorValueDiagnostic>();
    assert_traits::<ComposerVectorValueCoverage>();
    assert_traits::<ComposerVectorReconstructionReport>();
}
