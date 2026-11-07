//! Edits the owned Config-0 plane's stored token; application naming remains a separate policy.
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, PartResolvedFeaturesEdit, TextEncoding,
};
use bessemer::{BessemerError, Document, FailureClass, Result};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const PLANE: &str = "root.features[13].model_feature.feature.node.name.name";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [input, output, name] = arguments.as_slice() else {
        return Err(format!("arguments {arguments:?}; expected INPUT OUTPUT NEW_TOKEN for the owned Config-0/version-19000 baseline").into());
    };
    let mut document = Document::from_bytes(&std::fs::read(input)?)?;
    let edit = edit_plane_token(&document, name)?;
    document.replace_stream(STREAM, edit.archive().to_bytes()?)?;
    std::fs::write(output, document.to_bytes()?)?;
    println!(
        "{:?}: {} checked bytes, {} field relations; exact stored fields",
        edit.completion(),
        edit.coverage().wire_bytes,
        edit.changes().len()
    );
    Ok(())
}

fn edit_plane_token(document: &Document, name: &str) -> Result<PartResolvedFeaturesEdit> {
    let source = PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )?;
    let mut fields = source.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == PLANE)
        .ok_or_else(|| BessemerError {
            operation: "edit stored plane token",
            entity: STREAM.into(),
            class: FailureClass::Missing,
            cause: format!("field {PLANE:?} absent; expected the owned baseline's plane name"),
        })?;
    field.value = ModelFieldValue::Text(ArchiveString::new(name, TextEncoding::Utf16)?);
    source.edit_fields(fields)
}

#[cfg(test)]
mod tests {
    #[test]
    fn public_plane_token_edit_matches_the_independent_feature_stream() {
        let document =
            bessemer::Document::from_bytes(include_bytes!("../tests/fixtures/blank-part.SLDPRT"))
                .unwrap();
        let edit = super::edit_plane_token(&document, "Bessemer Plane Ω").unwrap();
        assert_eq!(
            edit.archive().to_bytes().unwrap(),
            include_bytes!("../tests/fixtures/resolved-features/owned-features-plane-edited.bin")
        );
        assert_eq!(edit.changes().len(), 1);
    }
}
