use super::{AssemblyConfigurationArchive, AssemblyConfigurationContext, AssemblyRootKind};
use super::{ConfigurationId, ModelArchiveLimits};
use crate::Result;
use crate::archive_codec::ArchiveCursor;
use crate::header::{ArchiveVersion, DocumentHeader};
use std::sync::Arc;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 2000000,
    maximum_fields: 500000,
    maximum_entries: 500000,
    maximum_depth: 128,
    maximum_objects: 100000,
};

pub(super) struct AssemblyCase<'a> {
    bytes: &'a [u8],
    context: AssemblyConfigurationContext,
    slots: usize,
}

pub(super) fn next_case<'a>(cursor: &mut ArchiveCursor<'a>) -> AssemblyCase<'a> {
    let version = ArchiveVersion::new(cursor.u32().unwrap());
    let kind = match cursor.u8().unwrap() {
        0 => AssemblyRootKind::Assembly,
        1 => AssemblyRootKind::RoutingAssembly,
        value => panic!("root {value}; expected 0 or 1"),
    };
    let header_length = cursor.u32().unwrap() as usize;
    let length = cursor.u32().unwrap() as usize;
    let slots = cursor.u32().unwrap() as usize;
    let header = DocumentHeader::from_bytes(
        cursor.take(header_length).unwrap(),
        version,
        LIMITS.maximum_objects,
    )
    .unwrap();
    let context = AssemblyConfigurationContext::ordinary_external_xml(
        ConfigurationId::new(0),
        kind,
        version,
        Arc::new(header),
        None,
    )
    .unwrap();
    AssemblyCase {
        bytes: cursor.take(length).unwrap(),
        context,
        slots,
    }
}

pub(super) fn verify(case: AssemblyCase<'_>) -> Result<AssemblyConfigurationArchive> {
    let source = AssemblyConfigurationArchive::from_bytes(case.bytes, case.context, LIMITS)?;
    assert_eq!(source.parsed.object_map.slot_count(), case.slots);
    let rebuilt = AssemblyConfigurationArchive::from_fields(
        source.fields().to_vec(),
        source.context().clone(),
        LIMITS,
    )?;
    assert_eq!(rebuilt.to_bytes()?, case.bytes);
    assert_eq!(source.to_bytes()?, case.bytes);
    Ok(source)
}

#[test]
#[ignore = "requires exported installed native assembly corpus; run verify_reference_corpus.py --export"]
fn complete_native_assembly_corpus_requires_exact_typed_construction() {
    let raw = std::fs::read("target/assembly-native-probe.bin").unwrap();
    let mut cursor = ArchiveCursor::new(&raw);
    assert_eq!(cursor.take(4).unwrap(), b"ACFG");
    let count = cursor.u32().unwrap();
    assert_eq!(count, 180);
    let mut accepted = 0;
    for index in 0..count {
        match verify(next_case(&mut cursor)) {
            Ok(_) => {
                accepted += 1;
                println!("ASSEMBLY\t{index}\taccepted");
            }
            Err(error) => println!("ASSEMBLY\t{index}\t{error}"),
        }
    }
    cursor.finish().unwrap();
    println!("ASSEMBLY_TOTAL\t{accepted}\t{count}");
    assert_eq!(accepted, count);
}
