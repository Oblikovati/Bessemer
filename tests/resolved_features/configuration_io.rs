use super::configuration_io as io;
use super::{ModelArchiveLimits, ModelDecoder, encode_fields};
use crate::{FailureClass, archive_codec::ObjectReader, header::ArchiveVersion};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 1000,
    maximum_fields: 100,
    maximum_entries: 20,
    maximum_depth: 8,
    maximum_objects: 8,
};

fn reader(bytes: &[u8]) -> ModelDecoder<'_> {
    ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS).unwrap()
}

#[test]
fn positive_only_native_counts_keep_negative_empty_values_and_bound_positive_entries() {
    for raw in [i32::MIN, -1, 0] {
        let bytes = raw.to_le_bytes();
        let mut decoder = reader(&bytes);
        assert_eq!(io::positive_count(&mut decoder, "count", 2).unwrap(), 0);
        assert_eq!(
            encode_fields(&decoder.finish().unwrap(), 1000).unwrap(),
            bytes
        );
    }
    let mut decoder = reader(&[1, 0, 0, 0, 0, 0]);
    assert_eq!(io::positive_count(&mut decoder, "count", 2).unwrap(), 1);
    decoder.u16("item").unwrap();
    assert!(decoder.finish().is_ok());
    assert!(io::positive_count(&mut reader(&[2, 0, 0, 0, 0, 0]), "count", 2).is_err());
    assert!(io::empty_signed(&mut reader(&[1, 0, 0, 0, 0, 0]), "count", 2).is_err());
}

#[test]
fn short_dword_and_qword_empty_map_counts_reconstruct_exactly_and_reject_payloads() {
    for bytes in [
        vec![0, 0],
        vec![255, 255, 0, 0, 0, 0],
        vec![255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0],
    ] {
        for end in 0..bytes.len() {
            assert!(io::empty_string_int_map(&mut reader(&bytes[..end]), "map").is_err());
        }
        let mut decoder = reader(&bytes);
        io::empty_string_int_map(&mut decoder, "map").unwrap();
        assert_eq!(
            encode_fields(&decoder.finish().unwrap(), 1000).unwrap(),
            bytes
        );
    }
    let mut decoder = reader(&[1, 0]);
    assert_eq!(
        io::empty_string_int_map(&mut decoder, "map")
            .unwrap_err()
            .class,
        FailureClass::Invalid
    );
    assert!(decoder.finish().is_err());
    let mut feasible = reader(&[1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        io::empty_string_int_map(&mut feasible, "map")
            .unwrap_err()
            .class,
        FailureClass::Unsupported
    );
    let mut maximum = reader(&[255; 14]);
    assert_eq!(
        io::empty_string_int_map(&mut maximum, "map")
            .unwrap_err()
            .class,
        FailureClass::Invalid
    );
}

#[test]
fn direct_lists_signed_arrays_and_strings_keep_their_individual_count_framing() {
    let mut map = ObjectReader::new();
    let mut list = reader(&[1, 0, 0, 0]);
    io::list(&mut list, &mut map, "list").unwrap();
    assert_eq!(
        encode_fields(&list.finish().unwrap(), 1000).unwrap(),
        [1, 0, 0, 0]
    );
    let mut array = reader(&[1, 0, 0, 0, 0, 0]);
    io::pointers(&mut array, &mut map, "array", super::ModelObjectRole::Any).unwrap();
    assert_eq!(
        encode_fields(&array.finish().unwrap(), 1000).unwrap(),
        [1, 0, 0, 0, 0, 0]
    );
    let bytes = [2, 0, 1, b'a', 1, b'b'];
    let mut strings = reader(&bytes);
    io::strings(&mut strings, "strings").unwrap();
    assert_eq!(
        encode_fields(&strings.finish().unwrap(), 1000).unwrap(),
        bytes
    );
}

#[test]
fn integer_maps_preserve_signed_keys_unsigned_bits_and_each_physical_pair() {
    let bytes = [1, 0, 0, 0, 254, 255, 255, 255, 255, 255, 255, 255];
    for unsigned in [false, true] {
        let mut decoder = reader(&bytes);
        io::integer_pairs(&mut decoder, "map", unsigned).unwrap();
        assert_eq!(
            encode_fields(&decoder.finish().unwrap(), 1000).unwrap(),
            bytes
        );
    }
}

#[test]
fn unsupported_signed_presence_diagnostics_keep_the_original_value() {
    io::absent(&mut reader(&[]), "print_settings", 0).unwrap();
    for presence in [i32::MIN, -1, 1, i32::MAX] {
        let error = io::absent(&mut reader(&[]), "print_settings", presence).unwrap_err();
        assert!(error.cause.contains(&format!("presence {presence};")));
        assert!(error.cause.contains("expected 0"));
    }
}
