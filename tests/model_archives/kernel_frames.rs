use super::annotation_view_tests::LIMITS;
use super::kernel_frames::{CompressedKernelContext, KernelFrame, read_body};
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, encode_fields};
use crate::block_stream::{BlockStream, ChunkFraming};
use crate::header::ArchiveVersion;
use crate::parasolid::{BaseSchema, FieldValues, TransmitBody, TransmitLimits};
use crate::{FailureClass, Result};

pub(super) const FRAME: &[u8] = include_bytes!("../fixtures/model-archives/kernel-frame-one.bin");

pub(super) fn context() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("../fixtures/parasolid/base-schema.s_t"))
            .unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

pub(super) fn decode(
    bytes: &[u8],
    kernel: Option<CompressedKernelContext>,
    count: usize,
) -> Result<Vec<ModelField>> {
    decode_with_limits(bytes, kernel, count, LIMITS)
}

fn decode_with_limits(
    bytes: &[u8],
    kernel: Option<CompressedKernelContext>,
    count: usize,
    limits: super::ModelFieldLimits,
) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_kernel(kernel);
    for index in 0..count {
        reader.scope(&format!("bodies[{index}]"), read_body)?;
    }
    reader.finish()
}

pub(super) fn kernel(fields: &[ModelField]) -> &KernelFrame {
    fields
        .iter()
        .find_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some(frame.as_ref()),
            _ => None,
        })
        .unwrap()
}

#[test]
fn compressed_kernel_is_a_complete_typed_partition_and_reconstructs_exactly() {
    let fields = decode(FRAME, Some(context()), 1).unwrap();
    let frame = kernel(&fields);
    assert_eq!(frame.record_count(), 1);
    assert_eq!(
        frame.expanded_bytes(),
        include_bytes!("../fixtures/model-archives/kernel-transmit-one.bin").len()
    );
    let TransmitBody::Partition(list) = &frame.contents().body else {
        panic!("expected partition")
    };
    assert_eq!(list.records[0].node_type, 60000);
    assert_eq!(
        list.records[0].fields,
        vec![FieldValues::Integers(vec![-7])]
    );
    assert_eq!(
        frame.with_contents(frame.contents().clone()).unwrap(),
        *frame
    );
    assert_eq!(encode_fields(&fields, 30000).unwrap(), FRAME);
}

#[test]
fn typed_kernel_edit_rebuilds_both_lengths_and_reopens_independent_expected_transmit() {
    let fields = decode(FRAME, Some(context()), 1).unwrap();
    let original = kernel(&fields);
    let mut contents = original.contents().clone();
    let TransmitBody::Partition(list) = &mut contents.body else {
        panic!("expected partition")
    };
    list.records[0].fields[0] = FieldValues::Integers(vec![9]);
    let changed = original.with_contents(contents).unwrap();
    let encoded = changed.to_bytes().unwrap();
    let carrier =
        BlockStream::from_bytes(&encoded[5..], ChunkFraming::LengthPrefixed, 10000).unwrap();
    assert_eq!(
        carrier.chunks()[0].payload(),
        include_bytes!("../fixtures/model-archives/kernel-transmit-edited.bin")
    );
    assert_eq!(
        i32::from_le_bytes(encoded[..4].try_into().unwrap()) as usize,
        encoded.len() - 4
    );
    let reparsed = decode(&[&[0][..], &encoded].concat(), Some(context()), 1).unwrap();
    assert_eq!(kernel(&reparsed), &changed);
    assert_eq!(original.to_bytes().unwrap(), FRAME[1..]);
}

#[test]
fn kernel_context_and_aggregate_expansion_and_record_budgets_are_required() {
    assert_eq!(
        decode(FRAME, None, 1).unwrap_err().class,
        FailureClass::Missing
    );
    let bytes = [FRAME, FRAME].concat();
    let mut limited = context();
    limited.maximum_expanded =
        include_bytes!("../fixtures/model-archives/kernel-transmit-one.bin").len();
    assert!(decode(&bytes, Some(limited), 2).is_err());
    let mut limited = context();
    limited.transmit_limits.maximum_records = 0;
    assert!(decode(FRAME, Some(limited), 1).is_err());
    let mut entry_limits = LIMITS;
    entry_limits.maximum_entries = 1;
    assert!(decode_with_limits(&bytes, Some(context()), 2, entry_limits).is_err());
    assert_eq!(
        encode_fields(&decode(&[195], None, 1).unwrap(), 1).unwrap(),
        [195]
    );
}

#[test]
fn kernel_prefixes_bad_type_compression_and_trailing_bytes_never_publish() {
    for length in 0..FRAME.len() {
        assert!(
            decode(&FRAME[..length], Some(context()), 1).is_err(),
            "prefix {length}"
        );
    }
    for offset in [1, 5, 10, 30] {
        let mut malformed = FRAME.to_vec();
        malformed[offset] ^= 0xff;
        assert!(
            decode(&malformed, Some(context()), 1).is_err(),
            "offset {offset}"
        );
    }
    let mut trailing = FRAME.to_vec();
    trailing.push(0);
    assert!(decode(&trailing, Some(context()), 1).is_err());
}

#[test]
fn kernel_partition_kind_and_single_body_contract_reject_invalid_edits() {
    let fields = decode(FRAME, Some(context()), 1).unwrap();
    let frame = kernel(&fields);
    let mut contents = frame.contents().clone();
    let TransmitBody::Partition(list) = &mut contents.body else {
        panic!("expected partition")
    };
    list.records.clear();
    contents.layouts.clear();
    assert!(frame.with_contents(contents).is_err());
    let mut contents = frame.contents().clone();
    contents.format = crate::parasolid::TransmitFormat::Bare;
    assert!(frame.with_contents(contents).is_err());
    assert_eq!(frame.to_bytes().unwrap(), FRAME[1..]);
}

#[test]
fn kernel_results_own_source_and_support_deterministic_parallel_writes() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<KernelFrame>();
    let mut source = FRAME.to_vec();
    let fields = decode(&source, Some(context()), 1).unwrap();
    source.fill(0xa5);
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), FRAME);
        }
    });
}

#[test]
fn length_changing_kernel_edit_reports_the_new_expansion_before_reparse() {
    let fields = decode(FRAME, Some(context()), 1).unwrap();
    let original = kernel(&fields);
    let mut contents = original.contents().clone();
    let suffix = " length-changing edit";
    contents.title.push_str(suffix);
    let changed = original.with_contents(contents).unwrap();
    assert_eq!(
        changed.expanded_bytes(),
        original.expanded_bytes() + suffix.len()
    );
    let bytes = changed.to_bytes().unwrap();
    let reparsed = KernelFrame::from_payload(&bytes[5..], &context()).unwrap();
    assert_eq!(changed.expanded_bytes(), reparsed.expanded_bytes());
    assert_eq!(original.to_bytes().unwrap(), FRAME[1..]);
}
