//! Complete manager scalar maps remain usable through public model loading and saving (#54).
use bessemer::model_archives::{ConfigurationId, ModelArchiveLimits, PartResolvedFeaturesArchive};
use bessemer::{Document, header::ArchiveVersion};
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const MANAGERS: &[&[u8]] = &[
    include_bytes!("fixtures/manager-scalars/owned-0-0.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-1.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-2.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-3.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-4.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-5.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-0.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-1.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-2.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-3.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-4.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-5.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-2-edited.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-2-edited.bin"),
];
fn document(manager: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
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
fn manager_scalar_documents_load_save_and_reopen_without_changing_maps() {
    for manager in MANAGERS {
        let mut document = document(manager);
        let before = document.to_bytes().unwrap();
        let archive = parsed(&document);
        let features = archive.to_bytes().unwrap();
        document
            .replace_stream("Contents/Config-0-ResolvedFeatures", features.clone())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), before);
        let reopened = Document::from_bytes(&before).unwrap();
        assert_eq!(reopened.read_stream("Contents/CMgr").unwrap(), *manager);
        assert_eq!(parsed(&reopened).to_bytes().unwrap(), features);
    }
}
#[test]
fn manager_mutation_after_public_load_does_not_change_the_owned_result() {
    for manager in MANAGERS {
        let mut document = document(manager);
        let archive = parsed(&document);
        let before = archive.to_bytes().unwrap();
        document
            .replace_stream("Contents/CMgr", vec![0xa5])
            .unwrap();
        assert_eq!(archive.to_bytes().unwrap(), before);
        assert!(
            PartResolvedFeaturesArchive::from_document(
                &document,
                ConfigurationId::new(0),
                ArchiveVersion::new(19000),
                None,
                LIMITS
            )
            .is_err()
        );
    }
}
#[test]
fn incomplete_manager_scalar_maps_fail_without_changing_the_document() {
    for manager in MANAGERS {
        let mut document = document(&manager[..manager.len() - 1]);
        let before = document.to_bytes().unwrap();
        assert!(
            PartResolvedFeaturesArchive::from_document(
                &document,
                ConfigurationId::new(0),
                ArchiveVersion::new(19000),
                None,
                LIMITS
            )
            .is_err()
        );
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream("Contents/CMgr", manager.to_vec())
            .unwrap();
        assert!(parsed(&document).to_bytes().is_ok());
    }
}
