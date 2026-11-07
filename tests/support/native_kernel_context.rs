use bessemer::model_archives::CompressedKernelContext;

/// Loads the explicitly selected installed schema for native tests; e.g. `let kernel = native_test_kernel_context();`.
pub(crate) fn native_test_kernel_context() -> CompressedKernelContext {
    let path = std::env::var_os("BESSEMER_KERNEL_SCHEMA").unwrap();
    let bytes = std::fs::read(path).unwrap();
    CompressedKernelContext {
        base: bessemer::parasolid::BaseSchema::from_bytes(&bytes).unwrap(),
        maximum_expanded: 64 * 1024 * 1024,
        transmit_limits: bessemer::parasolid::TransmitLimits {
            maximum_records: 1_000_000,
            maximum_field_elements: 10_000_000,
        },
    }
}
