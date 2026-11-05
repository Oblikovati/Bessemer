use super::property_domains_3086_tests::{assert_property_splice, assert_signed_property_domain};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::records_with_count;
use super::{ModelField, ModelFieldValue};

fn named_payload(path: &str, labels: &[&str]) -> bool {
    let Some((prefix, label)) = path.rsplit_once('.') else {
        return false;
    };
    prefix.ends_with(".payload") && labels.contains(&label)
}

pub(super) fn integer_family(wire: &'static [u8], tag: &[u8], total: u32, keys: &[&str]) -> usize {
    let mut occurrences = 0;
    let mut covered = std::collections::BTreeSet::new();
    for (_, version, raw, _, _) in records_with_count(wire, tag, total) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| named_payload(&field.path, keys))
        {
            assert_signed_property_domain(version, &fields, &field.path);
            covered.insert(field.path.rsplit('.').next().unwrap().to_owned());
            occurrences += 1;
        }
    }
    assert_eq!(covered.len(), keys.len());
    assert!(occurrences > 0);
    occurrences
}

pub(super) fn byte_domain(version: u32, fields: &[ModelField], path: &str) {
    for byte in 0..=u8::MAX {
        assert_property_splice(version, fields, path, &[byte], ModelFieldValue::U8(byte));
    }
}

fn byte_family(packet: &'static [u8], magic: &[u8], count: u32, label: &str) -> usize {
    let mut occurrences = 0;
    for (_, version, raw, _, _) in records_with_count(packet, magic, count) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| named_payload(&field.path, &[label]))
        {
            byte_domain(version, &fields, &field.path);
            occurrences += 1;
        }
    }
    assert!(occurrences > 0);
    occurrences
}

#[test]
fn dome_integer_properties_preserve_upper_bits_and_full_signed_wire_domains() {
    let packet = include_bytes!("../fixtures/domes/cases.bin");
    let labels = ["raw_858", "raw_85c", "raw_860"];
    let occurrences = integer_family(packet, b"DOF1", 128, &labels);
    eprintln!("DOME_INTEGER_SPLICES\t{}", occurrences * 6);
}

#[test]
fn fill_options_and_status_preserve_noncanonical_packed_words_and_every_byte() {
    let packet = include_bytes!("../fixtures/filled-surfaces/cases.bin");
    let words = integer_family(packet, b"FLS1", 264, &["raw_70", "raw_78", "raw_74"]);
    let bytes = byte_family(packet, b"FLS1", 264, "raw_10c");
    eprintln!(
        "FILL_WORD_SPLICES\t{}\nFILL_BYTE_SPLICES\t{}",
        words * 6,
        bytes * 256
    );
}

#[test]
fn holewizard_named_integers_and_existing_sketch_byte_retain_raw_domains() {
    let packet = include_bytes!("../fixtures/hole-wizard/cases.bin");
    let labels = [
        "raw_80c", "raw_840", "raw_848", "raw_898", "raw_8c8", "raw_914", "raw_980", "raw_998",
        "raw_a58", "raw_a5c", "raw_a60", "raw_a70", "raw_bb0", "raw_bfc", "raw_c00", "raw_83c",
    ];
    let words = integer_family(packet, b"HWF1", 74, &labels);
    let bytes = byte_family(packet, b"HWF1", 74, "raw_bf8");
    eprintln!(
        "HOLE_INTEGER_SPLICES\t{}\nHOLE_BYTE_SPLICES\t{}",
        words * 6,
        bytes * 256
    );
}
