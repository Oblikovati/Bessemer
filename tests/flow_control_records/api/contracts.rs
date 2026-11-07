use super::*;
use std::io::Write;
fn stored(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
fn encoded_index(raw: &[u8], width: FlowControlLengthWidth) -> Vec<u8> {
    wrap_index(&stored(raw), width)
}
fn wrap_index(compressed: &[u8], width: FlowControlLengthWidth) -> Vec<u8> {
    let mut payload = vec![0; 8];
    payload.extend_from_slice(compressed);
    let distance = (payload.len() + width.byte_width() + 4) as u64;
    payload.extend_from_slice(&distance.to_le_bytes()[..width.byte_width()]);
    payload.extend_from_slice(&INDEX.to_le_bytes());
    payload
}
pub(super) fn sample(identifier: u32, width: u8) -> ControlCase {
    cases()
        .into_iter()
        .find(|r| r.identifier == identifier && r.width == width && r.reference)
        .unwrap()
}
fn accepts(row: &ControlCase, raw: &[u8], options: FlowControlOptions) -> bool {
    if row.identifier == SAVED {
        return open_saved(raw, options).is_ok();
    }
    open_index(raw, width(row), options).is_ok()
}
#[test]
fn exact_and_one_below_complete_decode_and_entry_limits_are_checked_for_every_case() {
    for row in cases() {
        check_limits(&row);
    }
    for (identifier, width) in [(SAVED, 0), (INDEX, 4), (INDEX, 8)] {
        let row = sample(identifier, width);
        let minimum = if identifier == SAVED { 16 } else { 4 };
        for maximum in [0, 1, minimum - 1] {
            assert!(!accepts(
                &row,
                &row.stored,
                FlowControlOptions {
                    maximum_decoded_bytes: maximum,
                    ..options()
                }
            ));
        }
    }
}
fn check_limits(row: &ControlCase) {
    let mut limits = FlowControlOptions {
        maximum_decoded_bytes: row.decoded.len(),
        maximum_entries: row.count as usize,
    };
    assert!(accepts(row, &row.stored, limits));
    limits.maximum_decoded_bytes -= 1;
    assert!(!accepts(row, &row.stored, limits));
    limits.maximum_decoded_bytes += 1;
    if limits.maximum_entries > 0 {
        limits.maximum_entries -= 1;
        assert!(!accepts(row, &row.stored, limits));
    }
}
#[test]
fn empty_tables_keep_required_headers_and_zero_entry_bounds() {
    for row in cases().into_iter().filter(|r| r.count == 0) {
        let limits = FlowControlOptions {
            maximum_entries: 0,
            ..options()
        };
        assert!(accepts(&row, &row.stored, limits));
        assert_eq!(
            row.decoded.len(),
            if row.identifier == SAVED { 16 } else { 4 }
        );
    }
}
#[test]
fn every_small_proper_decoded_prefix_and_suffix_and_impossible_count_is_rejected() {
    for (identifier, width) in [(SAVED, 0), (INDEX, 4), (INDEX, 8)] {
        let row = sample(identifier, width);
        for length in 0..row.decoded.len() {
            reject_decoded(&row, &row.decoded[..length]);
        }
        let mut extended = row.decoded.clone();
        extended.push(0);
        reject_decoded(&row, &extended);
        reject_count(&row);
    }
}
fn reject_count(row: &ControlCase) {
    let mut raw = row.decoded.clone();
    if row.identifier == SAVED {
        raw[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
        reject_decoded(row, &raw);
        return;
    }
    for count in [i32::MIN, -1, i32::MAX] {
        raw[..4].copy_from_slice(&count.to_le_bytes());
        reject_decoded(row, &raw);
    }
}
fn reject_decoded(row: &ControlCase, raw: &[u8]) {
    let payload = if row.identifier == SAVED {
        stored(raw)
    } else {
        encoded_index(raw, width(row))
    };
    assert!(!accepts(
        row,
        &payload,
        FlowControlOptions {
            maximum_entries: usize::MAX,
            ..options()
        }
    ));
}
#[test]
fn zero_markers_and_every_footer_bit_and_declared_width_are_strict() {
    let saved = sample(SAVED, 0);
    for bit in 0..64 {
        let mut raw = saved.decoded.clone();
        raw[..8].copy_from_slice(&(1_u64 << bit).to_le_bytes());
        reject_decoded(&saved, &raw);
    }
    for width_bytes in [4, 8] {
        let row = sample(INDEX, width_bytes);
        let footer = row.footer.as_ref().unwrap();
        for offset in (0..8).chain(footer.start as usize..row.stored.len()) {
            reject_bits(&row, offset);
        }
        let other = if width_bytes == 4 {
            FlowControlLengthWidth::EightBytes
        } else {
            FlowControlLengthWidth::FourBytes
        };
        assert!(open_index(&row.stored, other, options()).is_err());
    }
}
fn reject_bits(row: &ControlCase, offset: usize) {
    for bit in 0..8 {
        let mut raw = row.stored.clone();
        raw[offset] ^= 1 << bit;
        assert!(!accepts(row, &raw, options()));
    }
}
#[test]
fn checksum_truncation_concatenation_junk_and_non_zlib_wrappers_are_rejected() {
    for (identifier, width) in [(SAVED, 0), (INDEX, 4), (INDEX, 8)] {
        let row = sample(identifier, width);
        for length in 0..row.stored.len() {
            assert!(!accepts(&row, &row.stored[..length], options()));
        }
        check_bad_streams(&row);
    }
}
fn check_bad_streams(row: &ControlCase) {
    let member = stored(&row.decoded);
    let mut corrupt = member.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    let concatenated = [member.as_slice(), member.as_slice()].concat();
    let junk = [member.as_slice(), b"x"].concat();
    for stream in [
        corrupt,
        concatenated,
        junk,
        wrapper(&row.decoded, false),
        wrapper(&row.decoded, true),
    ] {
        let payload = if row.identifier == SAVED {
            stream
        } else {
            wrap_index(&stream, width(row))
        };
        assert!(!accepts(row, &payload, options()));
    }
}
fn wrapper(raw: &[u8], gzip: bool) -> Vec<u8> {
    if gzip {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(raw).unwrap();
        return encoder.finish().unwrap();
    }
    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
#[test]
fn direct_control_reads_preserve_ordinary_and_solver_selector_policies() {
    let saved = check_saved(&sample(SAVED, 0));
    assert!(saved.source().latest(SAVED).unwrap().is_some());
    assert!(saved.source().solver_latest(SAVED).is_err());
    let index = check_index(&sample(INDEX, 8));
    assert!(index.source().latest(INDEX).is_err());
    assert!(index.source().solver_latest(INDEX).is_err());
    assert_eq!(saved.source().records().len(), 3);
    assert_eq!(index.source().records().len(), 3);
}
#[test]
fn invalid_indices_foreign_wrong_and_stale_occurrences_fail_without_mutating_sources() {
    let saved = check_saved(&sample(SAVED, 0));
    for ordinal in [saved.stored_count(), usize::MAX] {
        assert!(saved.entry(ordinal).is_err());
        assert!(saved.replace_key(ordinal, 0).is_err());
        assert!(saved.replace_value(ordinal, 0).is_err());
    }
    let index = check_index(&sample(INDEX, 8));
    for ordinal in [index.stored_count(), usize::MAX] {
        assert!(index.entry(ordinal).is_err());
        assert!(index.replace_identifier(ordinal, 0).is_err());
        assert!(index.replace_record_start(ordinal, 0).is_err());
        assert!(index.replace_record_span(ordinal, 0).is_err());
    }
    check_saved_occurrences(&saved);
    check_index_occurrences(&index, saved.record());
}
fn check_saved_occurrences(saved: &FlowSavedSpans) {
    let foreign = open_saved(saved.source().records().nth(1).unwrap().payload, options()).unwrap();
    reject_saved_id(saved.source(), foreign.record());
    reject_saved_id(saved.source(), &saved.source().records().nth(2).unwrap().id);
    let changed = saved.replace_key(0, u64::MAX).unwrap();
    reject_saved_id(changed.result().source(), saved.record());
}
fn reject_saved_id(source: &FlowCarrier, id: &FlowRecordId) {
    assert!(source.saved_span_writer(id, options()).is_err());
}
fn check_index_occurrences(index: &FlowTerminalIndex, foreign_id: &FlowRecordId) {
    let width = index.length_width();
    let foreign = open_index(
        index.source().records().nth(1).unwrap().payload,
        width,
        options(),
    )
    .unwrap();
    reject_index_id(index.source(), foreign.record(), width);
    reject_index_id(index.source(), foreign_id, width);
    reject_index_id(
        index.source(),
        &index.source().records().nth(2).unwrap().id,
        width,
    );
    let changed = index.replace_record_start(0, u64::MAX).unwrap();
    reject_index_id(changed.result().source(), index.record(), width);
}
fn reject_index_id(source: &FlowCarrier, id: &FlowRecordId, width: FlowControlLengthWidth) {
    assert!(source.terminal_index_writer(id, width, options()).is_err());
}
#[test]
fn both_index_widths_recompute_changed_compressed_lengths_and_footer_distances() {
    for width_bytes in [4, 8] {
        let row = cases()
            .into_iter()
            .find(|r| r.width == width_bytes && r.name.ends_with("-level-0"))
            .unwrap();
        let view = check_index(&row);
        let changed = view.replace_record_start(0, u64::MAX).unwrap();
        edit::check_recomputed_footer(changed.result());
        assert_ne!(
            changed.result().compressed_interval().1,
            view.compressed_interval().1
        );
        assert_ne!(
            changed.result().backward_distance(),
            view.backward_distance()
        );
    }
}
#[test]
fn all_new_and_reused_public_types_are_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<FlowControlOptions>();
    traits::<FlowControlLengthWidth>();
    traits::<FlowSavedSpans>();
    traits::<FlowSavedSpan<'static>>();
    traits::<FlowTerminalIndex>();
    traits::<FlowTerminalIndexEntry<'static>>();
    traits::<FlowControlCheckMode>();
    traits::<FlowControlCoverage>();
    traits::<FlowControlDiagnostic>();
    traits::<FlowSimpleTableEdit<FlowSavedSpans>>();
    traits::<FlowSimpleTableEdit<FlowTerminalIndex>>();
    traits::<FlowCarrierEdit>();
}
