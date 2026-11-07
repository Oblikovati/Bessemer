use super::{ModelDecoder, ModelFieldLimits};
use crate::archive_codec::MaterialRead;
use crate::header::ArchiveVersion;

fn reader(bytes: &[u8], maximum_fields: usize) -> ModelDecoder<'_> {
    let limits = ModelFieldLimits {
        maximum_bytes: 64,
        maximum_fields,
        maximum_entries: 8,
        maximum_depth: 4,
        maximum_objects: 4,
    };
    ModelDecoder::new(bytes, ArchiveVersion::new(19000), limits).unwrap()
}

#[test]
fn bounded_sections_restore_the_outer_cursor_and_keep_one_field_budget() {
    let mut decoder = reader(&[1, 2, 3], 2);
    decoder.bounded(1, |reader| reader.u8("first")).unwrap();
    decoder.bounded(1, |reader| reader.u8("second")).unwrap();
    assert_eq!(decoder.remaining(), 1);
    let error = decoder.u8("third").unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn a_bounded_overread_cannot_take_bytes_from_the_following_section() {
    let mut decoder = reader(&[1, 2, 3], 8);
    let error = decoder.bounded(1, |reader| reader.u16("word")).unwrap_err();
    assert_eq!(decoder.remaining(), 2);
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn a_bounded_underread_cannot_publish_after_the_outer_cursor_is_restored() {
    let mut decoder = reader(&[1, 2, 3], 8);
    let error = decoder.bounded(2, |reader| reader.u8("byte")).unwrap_err();
    assert_eq!(decoder.remaining(), 1);
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn nested_and_empty_sections_do_not_reset_state_or_invent_fields() {
    let mut decoder = reader(&[1, 2], 2);
    decoder
        .bounded(2, |reader| {
            reader.bounded(0, |_| Ok(()))?;
            reader.bounded(1, |reader| reader.u8("first"))?;
            reader.u8("second")
        })
        .unwrap();
    assert_eq!(decoder.finish().unwrap().len(), 2);
}

#[test]
fn impossible_section_size_and_ignored_inner_errors_remain_first_failures() {
    let mut decoder = reader(&[1], 8);
    let error = decoder.bounded(usize::MAX, |_| Ok(())).unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
    let mut decoder = reader(&[1], 8);
    let error = decoder
        .bounded(1, |reader| {
            let _ = reader.u16("impossible");
            Ok(())
        })
        .unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn signed_section_lengths_reject_negative_and_out_of_stream_values() {
    for size in [-1_i32, 1, i32::MAX] {
        let bytes = size.to_le_bytes();
        let mut decoder = reader(&bytes, 8);
        let error = decoder.section_size("resolved_size").unwrap_err();
        assert_eq!(decoder.finish().unwrap_err(), error);
    }
    let mut decoder = reader(&[0, 0, 0, 0], 8);
    assert_eq!(decoder.section_size("resolved_size").unwrap(), 0);
    decoder.bounded(0, |_| Ok(())).unwrap();
    assert_eq!(decoder.finish().unwrap().len(), 1);
}
