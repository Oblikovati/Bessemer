use super::*;

#[test]
fn normal_chain_mapping_skips_the_reserved_slot_but_mini_chains_do_not() {
    let reserved = 524286;
    let mut table = vec![FREE; reserved + 3];
    link_normal_chain(&mut table, &(reserved - 1..reserved + 2), 4096);
    assert_eq!(table[reserved - 1], 524287);
    assert_eq!(table[reserved], FREE);
    assert_eq!(table[reserved + 1], 524288);
    assert_eq!(table[reserved + 2], END);
    assert_eq!(normal_start(&(reserved..reserved + 1), 4096), 524287);
    link_chain(&mut table, &(reserved - 1..reserved + 2));
    assert_eq!(table[reserved - 1], 524286);
    assert_eq!(table[reserved], 524287);
    assert_eq!(table[reserved + 1], END);
}

#[test]
fn no_gap_copy_and_short_output_failure_do_not_require_a_giant_buffer() {
    let mut output = [0xaa; 20];
    copy_logical(&mut output, 3, b"ordinary", 4096).unwrap();
    assert_eq!(&output[3..11], b"ordinary");
    assert_eq!(output[2], 0xaa);
    let error = copy_logical(&mut output, 2147479552, b"reserved", 4096).unwrap_err();
    assert!(error.cause.contains("2147483648"));
    assert_eq!(&output[3..11], b"ordinary");
}
