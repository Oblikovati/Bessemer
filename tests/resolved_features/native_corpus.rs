use super::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, PartConfigurationArchive,
};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::{ArchiveVersion, DocumentHeader};
use std::sync::Arc;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

struct ResolvedCorpusCase<'a> {
    version: ArchiveVersion,
    label: &'a str,
    header: &'a [u8],
    configuration: &'a [u8],
    manager: &'a [u8],
    features: &'a [u8],
}

fn next_case<'a>(cursor: &mut ArchiveCursor<'a>) -> ResolvedCorpusCase<'a> {
    let version = ArchiveVersion::new(cursor.u32().unwrap());
    let lengths: Vec<_> = (0..5).map(|_| cursor.u32().unwrap() as usize).collect();
    ResolvedCorpusCase {
        version,
        label: std::str::from_utf8(cursor.take(lengths[0]).unwrap()).unwrap(),
        header: cursor.take(lengths[1]).unwrap(),
        configuration: cursor.take(lengths[2]).unwrap(),
        manager: cursor.take(lengths[3]).unwrap(),
        features: cursor.take(lengths[4]).unwrap(),
    }
}

fn reconstruct(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> Result<()> {
    let configuration = ConfigurationId::from_stream_name(case.label.rsplit_once("::").unwrap().1)?;
    let header = Arc::new(DocumentHeader::from_bytes(
        case.header,
        case.version,
        LIMITS.maximum_objects,
    )?);
    let context = super::PartConfigurationContext::ordinary_external_xml(
        configuration,
        case.version,
        header,
        kernel,
    )?;
    let source = PartConfigurationArchive::from_bytes(case.configuration, context, LIMITS)?;
    let manager = super::configuration_manager::parse(case.manager, case.version, LIMITS)?;
    assert!(
        super::encode_fields(&manager.fields, LIMITS.maximum_bytes)? == case.manager,
        "complete manager reconstruction differs for {}",
        case.label
    );
    let parsed = super::resolved_features::parse(case.features, &source, &manager, LIMITS)?;
    let rebuilt = super::resolved_features::construct(parsed.fields, &source, &manager, LIMITS)?;
    let public = super::PartResolvedFeaturesArchive::from_bytes(
        case.features,
        source.clone(),
        case.manager,
        LIMITS,
    )?;
    assert!(
        public.fields() == rebuilt.fields,
        "public fields differ for {}",
        case.label
    );
    assert!(
        public.to_bytes()? == case.features,
        "public reconstruction differs for {}",
        case.label
    );
    assert!(
        super::encode_fields(&rebuilt.fields, LIMITS.maximum_bytes)? == case.features,
        "complete feature reconstruction differs for {}",
        case.label
    );
    Ok(())
}

fn reported_outcome(index: u32, result: Result<()>) -> bool {
    match result {
        Ok(()) => {
            println!("RESOLVED\t{index}\taccepted");
            true
        }
        Err(error) => {
            println!("RESOLVED\t{index}\trejected\t{error}");
            false
        }
    }
}

#[test]
fn corpus_packet_reader_requires_complete_owned_reconstruction_and_reports_failure() {
    let document =
        crate::Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let parts = [
        b"owned::Contents/Config-0".to_vec(),
        document
            .read_stream("Contents/Config-0-ModelHeader")
            .unwrap(),
        document.read_stream("Contents/Config-0").unwrap(),
        document.read_stream("Contents/CMgr").unwrap(),
        document
            .read_stream("Contents/Config-0-ResolvedFeatures")
            .unwrap(),
    ];
    let mut bytes = 19000u32.to_le_bytes().to_vec();
    for part in &parts {
        bytes.extend_from_slice(&(part.len() as u32).to_le_bytes());
    }
    for part in &parts {
        bytes.extend_from_slice(part);
    }
    let mut cursor = ArchiveCursor::new(&bytes);
    let case = next_case(&mut cursor);
    assert_eq!(cursor.remaining(), 0);
    assert!(reported_outcome(0, reconstruct(&case, None)));
    assert!(!reported_outcome(
        1,
        Err(super::invalid(
            "owned corpus failure; expected complete bytes"
        ))
    ));
}

#[test]
#[ignore = "requires prepare_resolved_corpus.py and explicit BESSEMER_KERNEL_SCHEMA"]
fn complete_native_resolved_corpus_requires_exact_typed_reconstruction() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = super::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    let count = cursor.u32().unwrap();
    assert_eq!(count, 906);
    let mut accepted = 0;
    for index in 0..count {
        accepted += u32::from(reported_outcome(
            index,
            reconstruct(&next_case(&mut cursor), kernel.clone()),
        ));
    }
    assert_eq!(cursor.remaining(), 0);
    println!("RESOLVED_TOTAL\t{accepted}\t{count}");
    assert_eq!(
        accepted, count,
        "resolved corpus incomplete; inspect RESOLVED rows"
    );
}
