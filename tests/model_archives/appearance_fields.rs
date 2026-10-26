use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, encode_fields};
use crate::archive_codec::{ArchiveString, MaterialRead, StoredDouble, StoredFloat};
use crate::header::{ArchiveVersion, TextEncoding};
use crate::{Document, FailureClass, Result};

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 4096,
    maximum_fields: 512,
    maximum_entries: 128,
    maximum_depth: 16,
    maximum_objects: 128,
};
const PART: &[u8] = include_bytes!("../fixtures/blank-part.SLDPRT");
const PRIMITIVE_BYTES: &[u8] =
    b"\xff\x00\x80\x00\x00\x00\x80\xff\xff\xff\xff\x23\x01\x80\x7f\x00\x00\x00\x00\x00\x00\x00\x80";

fn native_appearance_bytes() -> Vec<u8> {
    let document = Document::from_bytes(PART).unwrap();
    let configuration = document.read_stream("Contents/Config-0").unwrap();
    configuration[108..953].to_vec()
}

fn read_appearance(input: &[u8], limits: ModelFieldLimits) -> Result<Vec<ModelField>> {
    let mut decoder = ModelDecoder::new(input, ArchiveVersion::new(19000), limits)?;
    crate::archive_codec::material::visual(&mut decoder)?;
    decoder.finish()
}

fn replace_user_name(fields: &mut [ModelField], text: &str) {
    let field = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.material.user_name")
        .unwrap();
    let value = ArchiveString::new(text, TextEncoding::Utf16).unwrap();
    field.value = ModelFieldValue::Text(value);
}

fn expected_name_change(original: &[u8]) -> Vec<u8> {
    let name: &[u8] = &[0xff, 0xfe, 0xff, 2, 0xa9, 3, b'x', 0];
    [&original[..12], name, &original[16..]].concat()
}

fn reduced_appearance_limits(exact: ModelFieldLimits) -> [ModelFieldLimits; 3] {
    [
        ModelFieldLimits {
            maximum_bytes: 844,
            ..exact
        },
        ModelFieldLimits {
            maximum_fields: exact.maximum_fields - 1,
            ..exact
        },
        ModelFieldLimits {
            maximum_depth: 0,
            ..exact
        },
    ]
}

fn expected_primitive_values() -> [ModelFieldValue; 6] {
    [
        ModelFieldValue::U8(255),
        ModelFieldValue::I16(-32768),
        ModelFieldValue::I32(i32::MIN),
        ModelFieldValue::U32(u32::MAX),
        ModelFieldValue::F32(StoredFloat::from_bits(0x7f800123)),
        ModelFieldValue::F64(StoredDouble::from_bits(0x8000000000000000)),
    ]
}

#[test]
fn native_model_appearance_uses_shared_layout_and_forced_typed_construction() {
    let bytes = native_appearance_bytes();
    let fields = read_appearance(&bytes, LIMITS).unwrap();
    assert_eq!(fields[0].path.as_ref(), "root.color");
    assert_eq!(fields[0].value, ModelFieldValue::U32(0x00e4e4e4));
    assert_eq!(encode_fields(&fields, 845).unwrap(), bytes);
    assert_eq!(
        read_appearance(&encode_fields(&fields, 845).unwrap(), LIMITS).unwrap(),
        fields
    );
    assert!(encode_fields(&fields, 844).is_err());
}

#[test]
fn independent_variable_name_bytes_reopen_and_revert_without_source_mutation() {
    let bytes = native_appearance_bytes();
    assert_eq!(&bytes[12..16], &[0xff, 0xfe, 0xff, 0]);
    let original = read_appearance(&bytes, LIMITS).unwrap();
    let mut changed = original.clone();
    replace_user_name(&mut changed, "Ωx");
    let expected = expected_name_change(&bytes);
    let encoded = encode_fields(&changed, LIMITS.maximum_bytes).unwrap();
    assert_eq!(encoded, expected);
    assert_eq!(read_appearance(&encoded, LIMITS).unwrap(), changed);
    replace_user_name(&mut changed, "");
    let reverted = encode_fields(&changed, LIMITS.maximum_bytes).unwrap();
    let retained = encode_fields(&original, LIMITS.maximum_bytes).unwrap();
    assert_eq!(reverted, bytes);
    assert_eq!(retained, bytes);
}

#[test]
fn every_native_prefix_and_trailing_byte_rejects_complete_field_publication() {
    let bytes = native_appearance_bytes();
    for end in 0..bytes.len() {
        assert!(
            read_appearance(&bytes[..end], LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut extended = bytes.clone();
    extended.push(0);
    assert!(read_appearance(&extended, LIMITS).is_err());
}

#[test]
fn exact_model_limits_reject_one_less_byte_field_and_scope() {
    let bytes = native_appearance_bytes();
    let count = read_appearance(&bytes, LIMITS).unwrap().len();
    let exact = ModelFieldLimits {
        maximum_bytes: 845,
        maximum_fields: count,
        ..LIMITS
    };
    assert!(read_appearance(&bytes, exact).is_ok());
    for limits in reduced_appearance_limits(exact) {
        let error = read_appearance(&bytes, limits).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert_eq!(error.operation, "model archive");
        assert!(error.cause.contains("expected"));
    }
}

#[test]
fn ignored_read_scope_and_aggregate_budget_failures_poison_the_builder() {
    let mut decoder = ModelDecoder::new(&[], ArchiveVersion::new(19000), LIMITS).unwrap();
    let error = decoder.u32("missing").unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
    let mut decoder = ModelDecoder::new(&[], ArchiveVersion::new(19000), LIMITS).unwrap();
    decoder.entries(128, "first").unwrap();
    let error = decoder.entries(1, "overflow").unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
    let mut decoder = ModelDecoder::new(&[], ArchiveVersion::new(19000), LIMITS).unwrap();
    let error = decoder
        .scope("failure", |_| {
            Err::<(), _>(super::invalid("sentinel 1; expected 0"))
        })
        .unwrap_err();
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn primitive_values_preserve_all_widths_nan_payloads_and_signed_zero() {
    let mut decoder =
        ModelDecoder::new(PRIMITIVE_BYTES, ArchiveVersion::new(19000), LIMITS).unwrap();
    decoder.u8("byte").unwrap();
    decoder.i16("word").unwrap();
    decoder.i32("signed").unwrap();
    decoder.u32("unsigned").unwrap();
    decoder.f32("nan").unwrap();
    decoder.f64("zero").unwrap();
    let actual = decoder.finish().unwrap();
    let values: Vec<_> = actual.iter().map(|field| field.value.clone()).collect();
    assert_eq!(values, expected_primitive_values());
    assert_eq!(encode_fields(&actual, 23).unwrap(), PRIMITIVE_BYTES);
}

#[test]
fn exhausted_field_budget_cannot_publish_after_consuming_the_last_byte() {
    let limits = ModelFieldLimits {
        maximum_fields: 0,
        ..LIMITS
    };
    let mut decoder = ModelDecoder::new(&[1], ArchiveVersion::new(19000), limits).unwrap();
    let error = decoder.u8("last").unwrap_err();
    assert_eq!(decoder.remaining(), 0);
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn aggregate_overflow_keeps_the_first_failure_and_rejects_further_reads() {
    let limits = ModelFieldLimits {
        maximum_entries: usize::MAX,
        ..LIMITS
    };
    let mut decoder = ModelDecoder::new(&[1], ArchiveVersion::new(19000), limits).unwrap();
    decoder.entries(usize::MAX, "first").unwrap();
    let error = decoder.entries(1, "overflow").unwrap_err();
    assert_eq!(decoder.u8("after_failure").unwrap_err(), error);
    assert_eq!(decoder.remaining(), 1);
    assert_eq!(decoder.finish().unwrap_err(), error);
}

#[test]
fn owned_fields_survive_source_changes_and_parallel_encoding_matches() {
    let expected = native_appearance_bytes();
    let mut input = expected.clone();
    let fields = read_appearance(&input, LIMITS).unwrap();
    input.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 845).unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
