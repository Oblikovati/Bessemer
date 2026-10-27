//! Complete flat-pattern reconstruction and independent direct-box edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const RICH: &[u8] = include_bytes!("fixtures/flat-patterns/owned-rich-1.bin");
const EDITED: &[u8] = include_bytes!("fixtures/flat-patterns/owned-rich-1-edited.bin");

fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

fn document_with(features: &[u8], extended: bool) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    let manager = if extended {
        include_bytes!("fixtures/sheet-metal/manager-1.bin").as_slice()
    } else {
        include_bytes!("fixtures/sheet-metal/manager-0.bin").as_slice()
    };
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document
}

fn parsed(document: &Document, with_kernel: bool) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        with_kernel.then(kernel),
        LIMITS,
    )
}

#[test]
fn complete_flat_patterns_reconstruct_documents_with_both_saved_states_and_raw_kernel_forms() {
    for (features, extended, slots) in [
        (
            include_bytes!("fixtures/flat-patterns/owned-empty.bin").as_slice(),
            false,
            101,
        ),
        (
            include_bytes!("fixtures/flat-patterns/owned-rich-0.bin").as_slice(),
            false,
            113,
        ),
        (RICH, true, 115),
        (EDITED, true, 115),
    ] {
        let mut document = document_with(features, extended);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document, true).unwrap();
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn direct_box_edit_matches_independent_bytes_and_preserves_typed_kernel_and_inputs() {
    let document = document_with(RICH, true);
    let archive = parsed(&document, true).unwrap();
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.box_918.data_10.raw_48")
        .unwrap()
        .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 115));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn present_raw_kernel_requires_explicit_schema_and_failure_preserves_the_document() {
    let document = document_with(RICH, true);
    assert_eq!(
        parsed(&document, false).unwrap_err().class,
        bessemer::FailureClass::Missing
    );
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
    let absent = document_with(
        include_bytes!("fixtures/flat-patterns/owned-empty.bin"),
        false,
    );
    assert!(parsed(&absent, false).is_ok());
}

#[test]
fn negative_pair_count_edit_is_rejected_without_changing_shared_records() {
    let document = document_with(RICH, true);
    let archive = parsed(&document, true).unwrap();
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.pairs_ac0.count")
        .unwrap()
        .value = ModelFieldValue::I32(-1);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}
