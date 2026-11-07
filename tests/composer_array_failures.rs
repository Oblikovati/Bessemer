//! Strict packed-array rejection and resource boundaries.
use bessemer::composer::arrays::*;

const DIRECT_WIRE: [u8; 20] = [
    97, 1, 0, 1, 69, 35, 129, 127, 0, 0, 0, 128, 0, 0, 128, 255, 69, 35, 193, 127,
];
const DIRECT_LAYOUT: ComposerVectorLayout = ComposerVectorLayout {
    count: 2,
    dimensions: ComposerVectorDimensions::Three,
};
const EMPTY_LAYOUT: ComposerVectorLayout = ComposerVectorLayout {
    count: 0,
    dimensions: ComposerVectorDimensions::Two,
};

#[test]
fn every_vector_prefix_and_extension_is_rejected() {
    for end in 0..DIRECT_WIRE.len() {
        assert!(ComposerVectorBlock::from_bytes(&DIRECT_WIRE[..end], DIRECT_LAYOUT, 100).is_err());
    }
    let mut changed = DIRECT_WIRE.to_vec();
    changed.push(0);
    assert!(ComposerVectorBlock::from_bytes(&changed, DIRECT_LAYOUT, 100).is_err());
}

#[test]
fn unknown_modes_and_noncanonical_flags_fail_explicitly() {
    let mut changed = DIRECT_WIRE;
    changed[0] = 0;
    let error = ComposerVectorBlock::from_bytes(&changed, DIRECT_LAYOUT, 100).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert_eq!(error.operation, "composer packed array");
    changed = DIRECT_WIRE;
    changed[1] = 2;
    assert!(ComposerVectorBlock::from_bytes(&changed, DIRECT_LAYOUT, 100).is_err());
}

#[test]
fn vector_limits_and_declared_counts_are_checked_before_allocation() {
    assert!(ComposerVectorBlock::from_bytes(&DIRECT_WIRE, DIRECT_LAYOUT, 19).is_err());
    let wrong_count = ComposerVectorLayout {
        count: 3,
        ..DIRECT_LAYOUT
    };
    assert!(ComposerVectorBlock::from_bytes(&DIRECT_WIRE, wrong_count, 100).is_err());
    let oversized = ComposerVectorLayout {
        count: u32::MAX,
        ..EMPTY_LAYOUT
    };
    assert!(ComposerVectorBlock::from_bytes(&[97], oversized, 100).is_err());
}

#[test]
fn typed_axis_counts_and_incompatible_lane_modes_are_rejected() {
    let quantized = ComposerVectorAxis::Quantized {
        first: 0,
        second: 0,
    };
    for axes in [
        vec![],
        vec![ComposerVectorAxis::Direct; 3],
        vec![ComposerVectorAxis::Constant(0); 2],
        vec![quantized; 2],
    ] {
        let packing = ComposerVectorPacking::Direct32;
        assert!(ComposerVectorBlock::from_parts(EMPTY_LAYOUT, packing, &axes, &[], 100).is_err());
    }
    let packing = ComposerVectorPacking::Quantized8;
    let axes = [ComposerVectorAxis::Direct; 2];
    assert!(ComposerVectorBlock::from_parts(EMPTY_LAYOUT, packing, &axes, &[], 100).is_err());
}

#[test]
fn zero_rows_still_require_all_quantized_header_bytes() {
    let wire = [98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let empty = ComposerVectorBlock::from_bytes(&wire, EMPTY_LAYOUT, 17).unwrap();
    assert_eq!(empty.coverage().stored_values, 0);
    for end in 0..wire.len() {
        assert!(ComposerVectorBlock::from_bytes(&wire[..end], EMPTY_LAYOUT, 100).is_err());
    }
    let packing = ComposerVectorPacking::Quantized16;
    assert!(ComposerVectorBlock::from_parts(EMPTY_LAYOUT, packing, empty.axes(), &[], 16).is_err());
}

#[test]
fn index_empty_complete_spans_limits_and_failed_edits_are_explicit() {
    for domain in [0, 255, 256, 65535, 65536, u32::MAX] {
        let empty = ComposerIndexBlock::from_bytes(&[], 0, domain, 0).unwrap();
        assert_eq!(empty.coverage().wire_bytes, 0);
        assert!(ComposerIndexBlock::from_bytes(&[0], 0, domain, 100).is_err());
        let block = ComposerIndexBlock::from_values(&[0, 1, 2], domain, 100).unwrap();
        let bytes = block.to_bytes();
        for end in 0..bytes.len() {
            assert!(ComposerIndexBlock::from_bytes(&bytes[..end], 3, domain, 100).is_err());
        }
        assert!(ComposerIndexBlock::from_bytes(&bytes, 3, domain, bytes.len() - 1).is_err());
        assert!(block.edit_values(&[]).is_err());
        assert_eq!(block.to_bytes(), bytes);
    }
    assert!(ComposerIndexBlock::from_bytes(&[], u32::MAX, u32::MAX, 100).is_err());
}
