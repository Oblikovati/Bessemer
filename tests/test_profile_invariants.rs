//! Test build optimization must retain diagnostic and checked-arithmetic behavior (#53).

fn runtime_sum(left: u32, right: u32) -> u32 {
    left + right
}

#[test]
fn test_builds_keep_debug_assertions_and_integer_overflow_checks() {
    assert!(std::hint::black_box(cfg!(debug_assertions)));
    let left = std::hint::black_box(u32::MAX);
    let right = std::hint::black_box(1);
    assert!(std::panic::catch_unwind(|| runtime_sum(left, right)).is_err());
}
