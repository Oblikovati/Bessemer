//! Complete paired Simulation framing, construction and isolated edits.
#[path = "common/simulation_failures.rs"]
mod failures;
#[path = "common/simulation_images.rs"]
mod reference;
use bessemer::simulation::*;
use reference::*;

#[test]
fn complete_owned_pairs_match_every_frozen_field_and_independent_edit_goldens() {
    let packet = owned();
    let mut cursor = Cursor(&packet);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 9);
    for _ in 0..9 {
        check_owned_pair(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn check_owned_pair(cursor: &mut Cursor<'_>) {
    let mas = cursor.span();
    let raw = cursor.span();
    let mut oracle = Cursor(cursor.span());
    let master = SimulationMasterArchive::from_bytes(mas).unwrap();
    master_oracle(&master, oracle.span());
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    gen_oracle(&image, oracle.span());
    assert!(oracle.0.is_empty());
    assert_eq!(master.to_bytes(), mas);
    assert_eq!(image.to_bytes(), raw);
    assert_eq!(
        SimulationMasterArchive::from_parts(master.parts().clone())
            .unwrap()
            .to_bytes(),
        mas
    );
    assert_eq!(
        SimulationGenArchive::from_parts(image.parts(), &master)
            .unwrap()
            .to_bytes(),
        raw
    );
    check_master_edit(&master, cursor.span());
    check_node_edit(&image, cursor.span());
    check_carrier(cursor.span(), cursor.span());
}

const LIMITS: bessemer::compound::CompoundLimits = bessemer::compound::CompoundLimits {
    maximum_file_bytes: 100_000_000,
    maximum_stream_bytes: 100_000_000,
    maximum_total_stream_bytes: 100_000_000,
    maximum_directory_entries: 100_000,
    maximum_storage_depth: 1000,
};

fn check_carrier(raw: &[u8], golden: &[u8]) {
    let study = SimulationStudy::from_bytes(raw, LIMITS).unwrap();
    assert_eq!(study.carrier().to_bytes(), raw);
    assert_eq!(study.master().revision(), study.image().master_revision());
    if study.image().layout().counts[3] == 0 {
        assert_eq!(raw, golden);
        return;
    }
    let id = study.image().node_id(0).unwrap();
    let edit = study
        .edit_node_coordinates(&id, [0x40000000, 0x40400000, 0x40800000])
        .unwrap();
    let expected = bessemer::compound::CompoundFile::from_bytes(golden, LIMITS).unwrap();
    for entry in study.carrier().entries() {
        if entry.kind() != bessemer::compound::CompoundEntryKind::Stream {
            continue;
        }
        let result = edit
            .study
            .carrier()
            .entries()
            .iter()
            .find(|other| other.id().slot() == entry.id().slot())
            .unwrap();
        let expected_entry = expected
            .entries()
            .iter()
            .find(|other| other.id().slot() == entry.id().slot())
            .unwrap();
        assert_eq!(
            edit.study.carrier().stream(result.id()).unwrap(),
            expected.stream(expected_entry.id()).unwrap()
        );
    }
    assert_eq!(study.carrier().to_bytes(), raw);
    assert_eq!(edit.diagnostics, study.diagnostics());
}

fn check_master_edit(master: &SimulationMasterArchive, golden: &[u8]) {
    let id = master.cell_id(70, 81).unwrap();
    assert_eq!((id.logical_row(), id.field_number()), (70, 81));
    assert_eq!(id.revision(), master.revision());
    let before = master.to_bytes();
    let edit = master.edit_cell(&id, 0x12345678).unwrap();
    assert_eq!(edit.archive.to_bytes(), golden);
    assert_eq!(edit.field_pattern_error_bits, 0);
    assert_eq!(edit.archive.cell(&edit.result).unwrap(), 0x12345678);
    assert!(edit.archive.cell(&id).is_err());
    assert_eq!(master.to_bytes(), before);
    assert_eq!(edit.parent, id);
    assert_eq!(edit.coverage.fields, 5670);
    let unchanged = master.edit_cell(&id, master.cell(&id).unwrap()).unwrap();
    assert_eq!(unchanged.archive.revision(), master.revision());
}

fn check_node_edit(image: &SimulationGenArchive, golden: &[u8]) {
    if image.layout().counts[3] == 0 {
        assert_eq!(image.to_bytes(), golden);
        return;
    }
    let id = image.node_id(0).unwrap();
    let before = image.to_bytes();
    let edit = image
        .edit_node_coordinates(&id, [0x40000000, 0x40400000, 0x40800000])
        .unwrap();
    assert_eq!(edit.archive.to_bytes(), golden);
    assert_eq!(edit.coordinate_pattern_error_bits, 0);
    assert_eq!(&edit.parent_revision, image.revision());
    assert_eq!(edit.parent, id);
    assert_eq!(
        edit.archive.node(&edit.result).unwrap().coordinate_patterns,
        [0x40000000, 0x40400000, 0x40800000]
    );
    assert_eq!(edit.coverage, edit.archive.coverage());
    assert_eq!(edit.archive.master_revision(), image.master_revision());
    assert!(edit.archive.node(&id).is_err());
    assert_eq!(image.to_bytes(), before);
    let node = image.node(&id).unwrap();
    assert_eq!(
        image
            .edit_node_coordinates(&id, node.coordinate_patterns)
            .unwrap()
            .archive
            .revision(),
        image.revision()
    );
}

#[test]
#[ignore = "requires separately frozen installed corpus in target/simulation-images-native.bin"]
fn complete_installed_pairs_match_all_reference_fields() {
    let packet = std::fs::read("target/simulation-images-native.bin").unwrap();
    let mut cursor = Cursor(&packet);
    assert_eq!(cursor.take(4), b"SIMN");
    assert_eq!(cursor.word(), 3);
    let mut edits = b"SIMW".to_vec();
    edits.extend_from_slice(&3_u32.to_le_bytes());
    for _ in 0..3 {
        let carrier = cursor.span();
        let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
        let raw = cursor.span();
        let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
        let mut oracle = Cursor(cursor.span());
        master_oracle(&master, oracle.span());
        gen_oracle(&image, oracle.span());
        assert!(oracle.0.is_empty());
        assert_eq!(image.to_bytes(), raw);
        let study = SimulationStudy::from_bytes(carrier, LIMITS).unwrap();
        let edit = study
            .edit_node_coordinates(
                &study.image().node_id(0).unwrap(),
                [0x40000000, 0x40400000, 0x40800000],
            )
            .unwrap();
        let encoded = edit.study.carrier().to_bytes();
        edits.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
        edits.extend_from_slice(&encoded);
    }
    assert!(cursor.0.is_empty());
    std::fs::write("target/simulation-images-native-edits.bin", edits).unwrap();
}
