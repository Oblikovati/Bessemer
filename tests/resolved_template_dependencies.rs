//! Complete template aliases with saved manager state and independently expected edits (#54).
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, StoredDouble,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
use bessemer::{Document, header::ArchiveVersion};
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const CASES: &[(&[u8], bool, usize)] = &[
    (
        include_bytes!("fixtures/template-dependencies/owned-0-0.bin"),
        false,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-1.bin"),
        false,
        113,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-2.bin"),
        false,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-3.bin"),
        false,
        113,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-4.bin"),
        false,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-5.bin"),
        false,
        113,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-0.bin"),
        true,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-1.bin"),
        true,
        115,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-2.bin"),
        true,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-3.bin"),
        true,
        115,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-4.bin"),
        true,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-5.bin"),
        true,
        115,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-0.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-1.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-2.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-3.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-4.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-5.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-0.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-1.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-2.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-3.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-4.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-5.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-0-2-edited.bin"),
        false,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-1-2-edited.bin"),
        true,
        101,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-2-2-edited.bin"),
        false,
        99,
    ),
    (
        include_bytes!("fixtures/template-dependencies/owned-3-2-edited.bin"),
        false,
        99,
    ),
];
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

fn document(features: &[u8], extended: bool) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let manager = if extended {
        include_bytes!("fixtures/sheet-metal/manager-1.bin").as_slice()
    } else {
        include_bytes!("fixtures/sheet-metal/manager-0.bin").as_slice()
    };
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
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
fn complete_template_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, extended, slots) in CASES {
        let mut document = document(bytes, *extended);
        let before = document.to_bytes().unwrap();
        let archive = parsed(&document, true).unwrap();
        assert_eq!(archive.object_slots(), (97, *slots));
        assert_eq!(archive.to_bytes().unwrap(), *bytes);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), before);
        let reopened = Document::from_bytes(&before).unwrap();
        assert_eq!(parsed(&reopened, true).unwrap().to_bytes().unwrap(), *bytes);
        document.replace_stream(STREAM, vec![0xa5]).unwrap();
        assert_eq!(archive.to_bytes().unwrap(), *bytes);
    }
}
#[test]
fn independent_template_and_id_edits_match_complete_owned_goldens() {
    {
        let (bytes, extended, _) = CASES[2];
        let document = document(bytes, extended);
        let source = parsed(&document, true).unwrap();
        let mut fields = source.fields().to_vec();
        fields
            .iter_mut()
            .find(|f| f.path.as_ref() == "root.features[0].payload.box_918.data_10.raw_48")
            .unwrap()
            .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5_f64.to_bits()));
        let changed = source.edit_fields(fields).unwrap();
        assert_eq!(changed.archive().to_bytes().unwrap(), CASES[24].0);
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
    {
        let (bytes, extended, _) = CASES[8];
        let document = document(bytes, extended);
        let source = parsed(&document, true).unwrap();
        let mut fields = source.fields().to_vec();
        fields
            .iter_mut()
            .find(|f| f.path.as_ref() == "root.features[0].payload.box_918.data_10.raw_48")
            .unwrap()
            .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5_f64.to_bits()));
        let changed = source.edit_fields(fields).unwrap();
        assert_eq!(changed.archive().to_bytes().unwrap(), CASES[25].0);
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
    {
        let (bytes, extended, _) = CASES[14];
        let document = document(bytes, extended);
        let source = parsed(&document, true).unwrap();
        let mut fields = source.fields().to_vec();
        fields
            .iter_mut()
            .find(|f| f.path.as_ref() == "root.features[0].unknown_38")
            .unwrap()
            .value = ModelFieldValue::I32(12345);
        let changed = source.edit_fields(fields).unwrap();
        assert_eq!(changed.archive().to_bytes().unwrap(), CASES[26].0);
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
    {
        let (bytes, extended, _) = CASES[20];
        let document = document(bytes, extended);
        let source = parsed(&document, true).unwrap();
        let mut fields = source.fields().to_vec();
        fields
            .iter_mut()
            .find(|f| f.path.as_ref() == "root.features[0].unknown_38")
            .unwrap()
            .value = ModelFieldValue::I32(12345);
        let changed = source.edit_fields(fields).unwrap();
        assert_eq!(changed.archive().to_bytes().unwrap(), CASES[27].0);
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
}
#[test]
fn template_raw_kernel_bodies_require_explicit_schema_without_mutating_inputs() {
    for index in [2, 8] {
        let (bytes, extended, _) = CASES[index];
        let document = document(bytes, extended);
        let before = document.to_bytes().unwrap();
        assert!(parsed(&document, false).is_err());
        assert_eq!(document.to_bytes().unwrap(), before);
        assert_eq!(parsed(&document, true).unwrap().to_bytes().unwrap(), bytes);
    }
}
