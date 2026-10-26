//! Public regression for documented temporary-axis pointers in complete feature streams (#46).
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
const TEMPORARY: &[u8] = include_bytes!("fixtures/line-reference-roles/owned-temporary.bin");
const EDITED: &[u8] = include_bytes!("fixtures/line-reference-roles/owned-temporary-edited.bin");

fn owned_document(features: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
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
fn complete_revolutions_with_null_line_and_temporary_axes_reconstruct_exact_documents() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/line-reference-roles/owned-null.bin").as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/line-reference-roles/owned-line.bin").as_slice(),
            103,
        ),
        (TEMPORARY, 103),
    ] {
        let mut document = owned_document(features);
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
fn temporary_axis_edits_match_the_independent_complete_stream_and_leave_inputs_isolated() {
    let document = owned_document(TEMPORARY);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let parameter = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].revolution_suffix.axis.unknown_a8")
        .unwrap();
    parameter.value = ModelFieldValue::F64(StoredDouble::from_bits(3.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), TEMPORARY);
    assert_eq!(document.read_stream(STREAM).unwrap(), TEMPORARY);
    assert_eq!(parsed(&owned_document(EDITED)).to_bytes().unwrap(), EDITED);
}
