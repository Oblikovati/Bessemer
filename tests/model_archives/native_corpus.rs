use std::sync::Arc;

use super::kernel_frames::CompressedKernelContext;
use super::{
    ConfigurationId, ModelFieldLimits, PartConfigurationArchive, PartConfigurationContext,
};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::{ArchiveVersion, DocumentHeader};

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

fn reconstruct(
    version: u32,
    header: &[u8],
    bytes: &[u8],
    kernel: Option<CompressedKernelContext>,
    configuration: ConfigurationId,
) -> Result<Vec<u8>> {
    let version = ArchiveVersion::new(version);
    let header = Arc::new(DocumentHeader::from_bytes(
        header,
        version,
        LIMITS.maximum_objects,
    )?);
    let context =
        PartConfigurationContext::ordinary_external_xml(configuration, version, header, kernel)?;
    let parsed = PartConfigurationArchive::from_bytes(bytes, context, LIMITS)?;
    PartConfigurationArchive::from_fields(
        parsed.fields().to_vec(),
        parsed.context().clone(),
        LIMITS,
    )?
    .to_bytes()
}

fn check_case(
    cursor: &mut ArchiveCursor<'_>,
    index: u32,
    kernel: &Option<CompressedKernelContext>,
) -> bool {
    let label_length = cursor.u32().unwrap() as usize;
    let version = cursor.u32().unwrap();
    let header_length = cursor.u32().unwrap() as usize;
    let stream_length = cursor.u32().unwrap() as usize;
    let label = std::str::from_utf8(cursor.take(label_length).unwrap()).unwrap();
    let header = cursor.take(header_length).unwrap();
    let stream = cursor.take(stream_length).unwrap();
    let configuration = label
        .rsplit_once("::")
        .ok_or_else(|| {
            super::invalid(format!(
                "corpus label {label:?}; expected file::Contents/Config-N"
            ))
        })
        .and_then(|(_, stream)| ConfigurationId::from_stream_name(stream));
    reported_outcome(
        index,
        label,
        stream,
        configuration.and_then(|configuration| {
            reconstruct(version, header, stream, kernel.clone(), configuration)
        }),
    )
}

fn reported_outcome(index: u32, label: &str, stream: &[u8], result: Result<Vec<u8>>) -> bool {
    match result {
        Ok(bytes) if bytes == stream => {
            println!("CORPUS\t{index}\taccepted\t{label}");
            true
        }
        Ok(_) => {
            println!("CORPUS\t{index}\tmismatch\t{label}");
            false
        }
        Err(error) => {
            println!("CORPUS\t{index}\trejected\t{label}\t{error}");
            false
        }
    }
}

#[test]
fn corpus_outcomes_require_matching_bytes_and_successful_completion() {
    assert!(reported_outcome(0, "owned case", &[1, 2], Ok(vec![1, 2])));
    assert!(!reported_outcome(0, "owned case", &[1, 2], Ok(vec![1, 3])));
    let error = super::invalid("fixture error; expected a complete corpus record");
    assert!(!reported_outcome(0, "owned case", &[1, 2], Err(error)));
}

#[test]
#[ignore = "requires target/model-native-corpus.bin from prepare_native_corpus.py"]
fn complete_native_part_corpus_requires_exact_typed_reconstruction() {
    let kernel = corpus_kernel_context();
    let bytes = std::fs::read("target/model-native-corpus.bin").unwrap();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BMC1");
    let count = cursor.u32().unwrap();
    assert_eq!(count, 906);
    let mut accepted = 0;
    for index in 0..count {
        accepted += u32::from(check_case(&mut cursor, index, &kernel));
    }
    assert_eq!(cursor.remaining(), 0);
    println!("CORPUS_TOTAL\t{accepted}\t{count}");
    assert_eq!(
        accepted, count,
        "native corpus remains incomplete; inspect CORPUS rows"
    );
}

pub(super) fn corpus_kernel_context() -> Option<CompressedKernelContext> {
    let path = std::env::var_os("BESSEMER_KERNEL_SCHEMA")?;
    let bytes = std::fs::read(path).unwrap();
    let base = crate::parasolid::BaseSchema::from_bytes(&bytes).unwrap();
    Some(CompressedKernelContext {
        base,
        maximum_expanded: 64 * 1024 * 1024,
        transmit_limits: crate::parasolid::TransmitLimits {
            maximum_records: 1_000_000,
            maximum_field_elements: 10_000_000,
        },
    })
}
