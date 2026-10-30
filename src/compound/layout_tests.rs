use super::*;

#[test]
fn table_self_coverage_includes_the_reserved_hole_at_independent_boundary_counts() {
    assert_eq!(table_counts(523773, 1024).unwrap(), (512, 1));
    assert_eq!(table_counts(523774, 1024).unwrap(), (512, 1));
    assert_eq!(table_counts(523775, 1024).unwrap(), (513, 1));
    assert_eq!((523773 + 512 + 1 + 1) * 4096_usize, 2147479552);
    assert_eq!((523774 + 512 + 1 + 2) * 4096_usize, 2147487744);
    assert!(table_counts(usize::MAX, 1024).is_err());
}

#[test]
fn mini_sector_identifier_limit_rejects_wraparound_before_serialization() {
    let limits = CompoundLimits {
        maximum_file_bytes: usize::MAX,
        maximum_stream_bytes: usize::MAX,
        maximum_total_stream_bytes: usize::MAX,
        maximum_directory_entries: usize::MAX,
        maximum_storage_depth: usize::MAX,
    };
    let mut boundary = LayoutBuilder::new(4096, limits);
    boundary.mini = 0xffff_fff9;
    assert!(boundary.small(64).is_ok());
    assert_eq!(boundary.mini, 0xffff_fffa);
    assert!(
        boundary
            .small(1)
            .unwrap_err()
            .cause
            .contains("mini-sector identifiers")
    );
}
