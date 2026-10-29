use super::{AssemblyConfigurationArchive, AssemblyRootKind, ConfigurationId, ModelArchiveLimits};
use crate::{Document, header::ArchiveVersion};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

#[test]
fn complete_assembly_and_routing_fields_match_every_independent_primitive_and_map_slot() {
    let original =
        Document::from_bytes(include_bytes!("../fixtures/blank-assembly.SLDASM")).unwrap();
    let ordinary = original.read_stream("Contents/Config-0").unwrap();
    for (kind, bytes, oracle, reads) in [
        (
            AssemblyRootKind::Assembly,
            ordinary.as_slice(),
            include_str!("../fixtures/assembly-configurations/owned-assembly.txt"),
            5056,
        ),
        (
            AssemblyRootKind::RoutingAssembly,
            include_bytes!("../fixtures/assembly-configurations/owned-routing.bin").as_slice(),
            include_str!("../fixtures/assembly-configurations/owned-routing.txt"),
            5075,
        ),
    ] {
        let mut document = original.clone();
        document
            .replace_stream("Contents/Config-0", bytes.to_vec())
            .unwrap();
        let archive = AssemblyConfigurationArchive::from_document(
            &document,
            ConfigurationId::new(0),
            kind,
            ArchiveVersion::new(19000),
            None,
            LIMITS,
        )
        .unwrap();
        assert_eq!(archive.parsed.object_map.slot_count(), 87);
        super::manager_oracle_tests::assert_reference_fields(
            archive.fields(),
            bytes,
            oracle,
            reads,
        );
    }
}

#[test]
fn native_assembly_dependencies_match_all_reference_primitives_and_force_complete_reconstruction() {
    let raw = include_bytes!("../fixtures/assembly-configurations/native-records.bin");
    let mut cursor = crate::archive_codec::ArchiveCursor::new(raw);
    assert_eq!(cursor.take(4).unwrap(), b"AFIX");
    assert_eq!(cursor.u32().unwrap(), 9);
    for _ in 0..9 {
        let case = super::assembly_corpus_tests::next_case(&mut cursor);
        let length = cursor.u32().unwrap() as usize;
        let oracle = std::str::from_utf8(cursor.take(length).unwrap()).unwrap();
        let archive = super::assembly_corpus_tests::verify(case).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            archive.fields(),
            &archive.to_bytes().unwrap(),
            oracle,
            oracle.lines().count(),
        );
    }
    cursor.finish().unwrap();
}
