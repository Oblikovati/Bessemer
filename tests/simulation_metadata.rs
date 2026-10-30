//! Bare Simulation metadata scalars, caller projections and complete existing-stream edits.
#[path = "common/simulation_metadata_failures.rs"]
mod failures;
#[path = "common/simulation_guid_reference.rs"]
mod guid_reference;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
#[path = "common/simulation_metadata.rs"]
mod reference;
use bessemer::simulation::*;
use packet::Cursor;
use reference::*;

#[test]
fn all_owned_bare_scalar_and_helper_patterns_match_actual_reference_fields() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 7);
    for _ in 0..7 {
        owned_scalar_carrier(&mut cursor);
    }
    guid_reference::writer_cases(&mut cursor);
    scalar_cases(&mut cursor);
    helper_cases(&mut cursor);
    assert!(cursor.0.is_empty());
}

fn owned_scalar_carrier(cursor: &mut Cursor<'_>) {
    let guid_raw = cursor.span();
    let guid = guid_reference::check_guid(guid_raw, cursor.span());
    guid_reference::check_edits(&guid, cursor);
    let original = cursor.span();
    let guid_golden = cursor.span();
    let scalar_golden = cursor.span();
    let helper_golden = cursor.span();
    let archive = SimulationMetadataArchive::from_bytes(original, LIMITS).unwrap();
    assert_eq!(archive.guids().unwrap().to_bytes(), guid_raw);
    let foreign = SimulationMetadataArchive::from_bytes(original, LIMITS).unwrap();
    assert!(
        archive
            .edit_scalar(
                foreign
                    .scalar(SimulationScalarKind::TimeStamp)
                    .unwrap()
                    .id(),
                3
            )
            .is_err()
    );
    check_guid_carrier(&archive, guid_golden);
    let id = archive
        .scalar(SimulationScalarKind::TimeStamp)
        .unwrap()
        .id();
    let edit = archive.edit_scalar(id, 0x87654321).unwrap();
    assert_carrier_expected(edit.archive.carrier(), scalar_golden);
    assert_eq!(
        edit.archive.guids().unwrap().revision(),
        archive.guids().unwrap().revision()
    );
    let helper = archive.apply_helper_stamp(0x87654321_u32 as i32).unwrap();
    assert_carrier_expected(helper.archive.carrier(), helper_golden);
    assert_eq!(
        helper.archive.guids().unwrap().revision(),
        archive.guids().unwrap().revision()
    );
    check_helper_history(&archive, &helper);
    assert_eq!(archive.carrier().to_bytes(), original);
    let unchanged = archive
        .edit_scalar(
            id,
            archive
                .scalar(SimulationScalarKind::TimeStamp)
                .unwrap()
                .pattern(),
        )
        .unwrap();
    assert_eq!(unchanged.archive.carrier().to_bytes(), original);
}

fn check_guid_carrier(archive: &SimulationMetadataArchive, golden: &[u8]) {
    let guid = archive.guids().unwrap();
    let id = &guid.fields()[0].id;
    if !guid.tail().is_empty() {
        assert!(archive.edit_guid_field(id, Some(&[65])).is_err());
        return;
    }
    let foreign =
        SimulationGuidArchive::from_bytes(&guid.to_bytes(), LIMITS.maximum_guid_stream_bytes)
            .unwrap();
    assert!(
        archive
            .edit_guid_field(&foreign.fields()[0].id, None)
            .is_err()
    );
    let edit = archive
        .edit_guid_field(id, Some(&[0x52, 0xd800, 0, 0x5a]))
        .unwrap();
    assert_carrier_expected(edit.archive.carrier(), golden);
    let unchanged = archive
        .edit_guid_field(id, guid.fields()[0].stored_units())
        .unwrap();
    assert_eq!(
        unchanged.archive.carrier().to_bytes(),
        archive.carrier().to_bytes()
    );
    for kind in [
        SimulationScalarKind::TimeStamp,
        SimulationScalarKind::RunState,
    ] {
        assert_eq!(
            edit.archive.scalar(kind).unwrap().id(),
            archive.scalar(kind).unwrap().id()
        );
    }
    let encoded = edit.archive.carrier().to_bytes();
    assert_eq!(
        SimulationMetadataArchive::from_bytes(&encoded, LIMITS)
            .unwrap()
            .guids()
            .unwrap()
            .to_bytes(),
        edit.guid_edit.archive.to_bytes()
    );
}

fn check_helper_history(parent: &SimulationMetadataArchive, edit: &SimulationMetadataHelperEdit) {
    assert_eq!(&edit.history.parent, parent.carrier().revision());
    assert_eq!(&edit.history.result, edit.archive.carrier().revision());
    assert_eq!(edit.history.entries.len(), parent.carrier().entries().len());
    for relation in edit.history.entries.iter() {
        assert_eq!(relation.parent.slot(), relation.result.slot());
        parent.carrier().entry(&relation.parent).unwrap();
        edit.archive.carrier().entry(&relation.result).unwrap();
    }
    assert!(!edit.native_io_executed);
    assert_eq!(
        edit.scalar_edits[0].archive.kind(),
        SimulationScalarKind::RunState
    );
    assert_eq!(
        edit.scalar_edits[1].archive.kind(),
        SimulationScalarKind::TimeStamp
    );
}

#[test]
#[ignore = "requires separately frozen installed metadata corpus in target/simulation-metadata-native.bin"]
fn all_native_scalar_payloads_edit_reopen_and_preserve_every_unrelated_stream() {
    let raw = std::fs::read("target/simulation-metadata-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SIMN");
    assert_eq!(cursor.word(), 3);
    let mut edits = b"SIMU".to_vec();
    edits.extend_from_slice(&4_u32.to_le_bytes());
    for _ in 0..3 {
        native_carrier(&mut cursor, &mut edits);
    }
    assert!(cursor.0.is_empty());
    std::fs::write("target/simulation-metadata-native-edits.bin", edits).unwrap();
}

fn native_carrier(cursor: &mut Cursor<'_>, edits: &mut Vec<u8>) {
    let original = cursor.span();
    let archive = SimulationMetadataArchive::from_bytes(original, LIMITS).unwrap();
    assert!(archive.guids().is_none());
    for tag in 0..2 {
        let kind = kind(tag);
        let present = cursor.word() != 0;
        assert_eq!(archive.scalar(kind).is_some(), present);
        if !present {
            continue;
        }
        let scalar = archive.scalar(kind).unwrap();
        assert_eq!(scalar.to_bytes(), cursor.span());
        scalar_oracle(scalar, cursor.span());
        let edit = archive.edit_scalar(scalar.id(), 0x87654321).unwrap();
        verify_native_edit(&archive, &edit, kind);
        let encoded = edit.archive.carrier().to_bytes();
        edits.extend_from_slice(&(original.len() as u32).to_le_bytes());
        edits.extend_from_slice(original);
        edits.extend_from_slice(&tag.to_le_bytes());
        edits.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
        edits.extend_from_slice(&encoded);
    }
    if archive.scalar(SimulationScalarKind::RunState).is_none() {
        assert!(archive.apply_helper_stamp(3).is_err());
    } else {
        let unchanged = archive
            .apply_helper_stamp(
                archive
                    .scalar(SimulationScalarKind::TimeStamp)
                    .unwrap()
                    .signed_value(),
            )
            .unwrap();
        assert_eq!(unchanged.archive.carrier().to_bytes(), original);
        assert!(
            unchanged
                .history
                .entries
                .iter()
                .all(|entry| !entry.replaced_stream)
        );
    }
    assert_eq!(archive.carrier().to_bytes(), original);
}

fn verify_native_edit(
    parent: &SimulationMetadataArchive,
    edit: &SimulationMetadataScalarEdit,
    kind: SimulationScalarKind,
) {
    let encoded = edit.archive.carrier().to_bytes();
    let reopened = SimulationMetadataArchive::from_bytes(&encoded, LIMITS).unwrap();
    assert_eq!(reopened.scalar(kind).unwrap().pattern(), 0x87654321);
    for entry in parent.carrier().entries() {
        if entry.kind() != bessemer::compound::CompoundEntryKind::Stream {
            continue;
        }
        let relation = edit
            .carrier_edit
            .history
            .entries
            .iter()
            .find(|relation| relation.parent == *entry.id())
            .unwrap();
        let actual = edit.archive.carrier().stream(&relation.result).unwrap();
        let expected = if entry
            .name_units()
            .iter()
            .copied()
            .eq(kind.stream_name().encode_utf16())
        {
            0x87654321_u32.to_le_bytes().to_vec()
        } else {
            parent.carrier().stream(entry.id()).unwrap().to_vec()
        };
        assert_eq!(actual, expected);
    }
}

#[test]
#[ignore = "exports independent owned GUID carrier edits for frozen-reader verification"]
fn owned_complete_guid_carrier_edits_export_without_publishing_unknown_tails() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 7);
    let mut bundle = b"SIMG".to_vec();
    bundle.extend_from_slice(&6_u32.to_le_bytes());
    for index in 0_u32..7 {
        export_guid_carrier(&mut cursor, &mut bundle, index);
    }
    std::fs::write("target/simulation-metadata-guid-edits.bin", bundle).unwrap();
}

fn export_guid_carrier(cursor: &mut Cursor<'_>, bundle: &mut Vec<u8>, index: u32) {
    for _ in 0..6 {
        cursor.span();
    }
    let original = cursor.span();
    for _ in 0..3 {
        cursor.span();
    }
    let archive = SimulationMetadataArchive::from_bytes(original, LIMITS).unwrap();
    let guids = archive.guids().unwrap();
    if !guids.tail().is_empty() {
        assert!(
            archive
                .edit_guid_field(&guids.fields()[0].id, None)
                .is_err()
        );
        return;
    }
    let edit = archive
        .edit_guid_field(&guids.fields()[0].id, Some(&[0x52, 0xd800, 0, 0x5a]))
        .unwrap();
    let encoded = edit.archive.carrier().to_bytes();
    bundle.extend_from_slice(&index.to_le_bytes());
    bundle.extend_from_slice(&(original.len() as u32).to_le_bytes());
    bundle.extend_from_slice(original);
    bundle.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
    bundle.extend_from_slice(&encoded);
}
