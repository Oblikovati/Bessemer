use bessemer::model_archives::{ConfigurationId, ModelArchiveLimits, PartResolvedFeaturesArchive};
use bessemer::{Document, header::ArchiveVersion};
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document(manager: &[u8]) -> Document {
    let mut document =
        Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document
}
fn parsed(document: &Document) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
}

/// Verify complete manager dependencies through public save and reopen.
/// Example: `contract::check_documents(managers)`.
pub fn check_documents(managers: &[&[u8]]) {
    for manager in managers {
        let mut document = document(manager);
        let before = document.to_bytes().unwrap();
        let features = parsed(&document).unwrap().to_bytes().unwrap();
        document
            .replace_stream("Contents/Config-0-ResolvedFeatures", features.clone())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), before);
        let reopened = Document::from_bytes(&before).unwrap();
        assert_eq!(reopened.read_stream("Contents/CMgr").unwrap(), *manager);
        assert_eq!(parsed(&reopened).unwrap().to_bytes().unwrap(), features);
    }
}

/// Keep a loaded result independent of later source-manager mutation.
/// Example: `contract::check_isolation(managers)`.
pub fn check_isolation(managers: &[&[u8]]) {
    for manager in managers {
        let mut document = document(manager);
        let archive = parsed(&document).unwrap();
        let before = archive.to_bytes().unwrap();
        document
            .replace_stream("Contents/CMgr", vec![0xa5])
            .unwrap();
        assert_eq!(archive.to_bytes().unwrap(), before);
        assert!(parsed(&document).is_err());
    }
}

/// Reject an incomplete manager while preserving the complete input document.
/// Example: `contract::check_incomplete(managers)`.
pub fn check_incomplete(managers: &[&[u8]]) {
    for manager in managers {
        let mut document = document(&manager[..manager.len() - 1]);
        let before = document.to_bytes().unwrap();
        assert!(parsed(&document).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        document
            .replace_stream("Contents/CMgr", manager.to_vec())
            .unwrap();
        assert!(parsed(&document).unwrap().to_bytes().is_ok());
    }
}
