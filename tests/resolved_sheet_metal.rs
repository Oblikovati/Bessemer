//! Complete sheet-metal records with persisted state and independent grid edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const POPULATED: &[u8] = include_bytes!("fixtures/sheet-metal/owned-0-1-1-2.bin");
const EDITED: &[u8] = include_bytes!("fixtures/sheet-metal/owned-0-1-1-2-edited.bin");

fn document_with(features: &[u8], state: u8) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    let manager = if state == 0 {
        include_bytes!("fixtures/sheet-metal/manager-0.bin").as_slice()
    } else {
        include_bytes!("fixtures/sheet-metal/manager-1.bin").as_slice()
    };
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document
}

fn parsed(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn sheet_and_template_records_reconstruct_documents_using_both_saved_manager_states() {
    for (features, state, slots) in [
        (
            include_bytes!("fixtures/sheet-metal/owned-0-0-0-0.bin").as_slice(),
            0,
            101,
        ),
        (
            include_bytes!("fixtures/sheet-metal/owned-0-0-1-2.bin").as_slice(),
            0,
            113,
        ),
        (
            include_bytes!("fixtures/sheet-metal/owned-1-0-1-2.bin").as_slice(),
            0,
            113,
        ),
        (
            include_bytes!("fixtures/sheet-metal/owned-1-1-1-2.bin").as_slice(),
            1,
            115,
        ),
        (
            include_bytes!("fixtures/sheet-metal/owned-1-1-1-3.bin").as_slice(),
            1,
            115,
        ),
        (POPULATED, 1, 115),
    ] {
        let mut document = document_with(features, state);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn grid_edit_preserves_raw_strided_payloads_aliases_strings_state_and_inputs() {
    let document = document_with(POPULATED, 1);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let label = fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref()
                == "root.features[0].payload.common_bend_800.allowance_08.raw_table[3]"
        })
        .unwrap();
    label.value = ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 115));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
    assert_eq!(
        parsed(&document_with(EDITED, 1)).to_bytes().unwrap(),
        EDITED
    );
}

#[test]
fn invalid_saved_state_rejects_before_publishing_a_sheet_archive() {
    let mut document = document_with(POPULATED, 1);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/sheet-metal/manager-invalid.bin").to_vec(),
        )
        .unwrap();
    let error = PartResolvedFeaturesArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(
        error.cause.contains("saved helper state 2"),
        "{}",
        error.cause
    );
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
}

#[test]
fn helper_mismatch_uses_the_persisted_state_without_a_success_based_fallback() {
    let document = document_with(POPULATED, 0);
    let result = PartResolvedFeaturesArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    );
    assert!(result.is_err());
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
}
