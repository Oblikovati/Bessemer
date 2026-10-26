//! Reconstructs an ordinary part's feature stream with an explicit model version and configuration.
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{ConfigurationId, ModelArchiveLimits, PartResolvedFeaturesArchive};
use bessemer::{Document, Result};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [input, version, configuration, output] = arguments.as_slice() else {
        return Err(format!(
            "arguments {arguments:?}; expected INPUT MODEL_VERSION CONFIGURATION_NUMBER OUTPUT"
        )
        .into());
    };
    let document = Document::from_bytes(&std::fs::read(input)?)?;
    let version = ArchiveVersion::new(version.parse()?);
    let configuration = ConfigurationId::new(configuration.parse()?);
    let rebuilt = reconstruct(&document, configuration, version)?;
    std::fs::write(output, rebuilt.to_bytes()?)?;
    Ok(())
}

fn reconstruct(
    document: &Document,
    configuration: ConfigurationId,
    version: ArchiveVersion,
) -> Result<Document> {
    let archive =
        PartResolvedFeaturesArchive::from_document(document, configuration, version, None, LIMITS)?;
    let mut rebuilt = document.clone();
    rebuilt.replace_stream(
        &format!("{}-ResolvedFeatures", configuration.stream_name()),
        archive.to_bytes()?,
    )?;
    Ok(rebuilt)
}

#[cfg(test)]
mod tests {
    #[test]
    fn complete_owned_document_reconstruction_keeps_every_stream_and_encoding() {
        let bytes = include_bytes!("../tests/fixtures/blank-part.SLDPRT");
        let document = bessemer::Document::from_bytes(bytes).unwrap();
        let rebuilt = super::reconstruct(
            &document,
            super::ConfigurationId::new(0),
            super::ArchiveVersion::new(19000),
        )
        .unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
        assert_eq!(document.to_bytes().unwrap(), bytes);
    }
}
