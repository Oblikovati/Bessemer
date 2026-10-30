use super::*;

#[test]
fn independent_reserved_offset_and_sector_counts_cover_both_sides_of_two_gib() {
    let reserved = 524286;
    assert_eq!(lock_sector(4096), Some(reserved));
    assert_eq!((reserved + 1) * 4096, 2147479552);
    assert_eq!(physical_sector(reserved - 1, 4096), reserved - 1);
    assert_eq!(physical_sector(reserved, 4096), reserved + 1);
    assert_eq!(physical_sector(reserved + 1, 4096), reserved + 2);
    assert_eq!(physical_count(reserved, 4096), reserved);
    assert_eq!(physical_count(reserved + 1, 4096), reserved + 2);
    assert_eq!(physical_count(reserved + 2, 4096), reserved + 3);
    assert_eq!(lock_sector(512), None);
    assert_eq!(physical_sector(reserved, 512), reserved);
    assert_eq!(physical_count(reserved + 1, 512), reserved + 1);
}

#[test]
fn independent_copy_ranges_exclude_every_reserved_byte_without_skipping_content() {
    let boundary = 2147479552;
    let before =
        copy_segments(boundary - 5, 10, 4096).map(|segment| (segment.output, segment.source));
    assert_eq!(before, [(boundary - 5, 0..5), (boundary + 4096, 5..10)]);
    let at = copy_segments(boundary, 10, 4096);
    assert_eq!(
        (at[1].output, at[1].source.clone()),
        (boundary + 4096, 0..10)
    );
    let after = copy_segments(boundary + 17, 10, 4096);
    assert_eq!(
        (after[1].output, after[1].source.clone()),
        (boundary + 4113, 0..10)
    );
}

#[test]
fn allocated_and_free_boundary_states_match_the_declared_size_rule() {
    assert!(lock_state(END, 2147483648).unwrap());
    assert!(!lock_state(FREE, 2147483648).unwrap());
    assert!(lock_state(END, 2147487744).unwrap());
    assert!(lock_state(FREE, 2147487744).is_err());
    assert!(lock_state(0, 2147483648).is_err());
    assert!(lock_state(super::super::header::FAT, 2147487744).is_err());
}

#[test]
fn copy_ending_before_the_reservation_omits_an_empty_suffix() {
    let boundary = 2147479552;
    let ending = copy_segments(boundary - 5, 5, 4096);
    assert_eq!(
        (ending[0].output, ending[0].source.clone()),
        (boundary - 5, 0..5)
    );
    assert!(ending[1].source.is_empty());
    let legacy = copy_segments(boundary - 5, 10, 512);
    assert_eq!(
        (legacy[0].output, legacy[0].source.clone()),
        (boundary - 5, 0..10)
    );
}
