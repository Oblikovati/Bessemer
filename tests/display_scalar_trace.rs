//! Direct IEEE bit oracles keep verification independent of hardware NaN conversion behavior.
use bessemer::{display_lists::DisplayFieldValue, tessellation::StoredFloat};
#[path = "../examples/support/archive_scalar_trace.rs"]
mod archive_scalar_trace;

#[test]
fn scalar_trace_preserves_signaling_and_quiet_float32_nan_sign_and_payload() {
    let cases = [
        (0x7f800001, 0x7ff0000020000000_u64),
        (0x7f800042, 0x7ff0000840000000),
        (0xff800042, 0xfff0000840000000),
        (0x7fc00123, 0x7ff8002460000000),
        (0xffc00123, 0xfff8002460000000),
        (0x7fffffff, 0x7fffffffe0000000),
    ];
    for (bits, expected) in cases {
        let value = DisplayFieldValue::F32(StoredFloat::from_bits(bits));
        assert_eq!(
            archive_scalar_trace::scalar(&value),
            Some(format!("f:{expected:016x}"))
        );
    }
}

#[test]
fn scalar_trace_keeps_existing_finite_zero_subnormal_and_infinite_representations() {
    let cases = [
        (0, 0_u64),
        (0x80000000, 0x8000000000000000),
        (1, 0x36a0000000000000),
        (0x00800000, 0x3810000000000000),
        (0x3f800000, 0x3ff0000000000000),
        (0xbf800000, 0xbff0000000000000),
        (0x7f7fffff, 0x47efffffe0000000),
        (0x7f800000, 0x7ff0000000000000),
        (0xff800000, 0xfff0000000000000),
    ];
    for (bits, expected) in cases {
        let value = DisplayFieldValue::F32(StoredFloat::from_bits(bits));
        assert_eq!(
            archive_scalar_trace::scalar(&value),
            Some(format!("f:{expected:016x}"))
        );
    }
}
