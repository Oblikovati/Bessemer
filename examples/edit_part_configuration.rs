//! Renames the owned ordinary Config-0/version-19000 part baseline; framed kernels need explicit schema context.
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartConfigurationArchive,
    PartConfigurationEdit, TextEncoding,
};
use bessemer::{BessemerError, Document, FailureClass, Result, header::ArchiveVersion};

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
        return Err(format!("arguments {arguments:?}; expected INPUT OUTPUT NEW_NAME for ordinary Config-0/version-19000").into());
    };
    let document = Document::from_bytes(&std::fs::read(input)?)?;
    let edit = rename(&document, name)?;
    let mut changed = document.clone();
    changed.replace_stream(
        &ConfigurationId::new(0).stream_name(),
        edit.archive().to_bytes()?,
    )?;
    std::fs::write(output, changed.to_bytes()?)?;
    println!(
        "{:?}: {} checked stream bytes; exact stored fields; geometric checks excluded",
        edit.completion(),
        edit.coverage().wire_bytes
    );
    Ok(())
}

fn rename(document: &Document, name: &str) -> Result<PartConfigurationEdit> {
    let archive = PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )?;
    let mut fields = archive.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.node.name.name")
        .ok_or_else(|| BessemerError {
            operation: "rename part",
            entity: "Contents/Config-0".into(),
            class: FailureClass::Missing,
            cause: "root.node.name.name absent; expected one root name field".into(),
        })?;
    field.value = ModelFieldValue::Text(ArchiveString::new(name, TextEncoding::Utf16)?);
    archive.edit_fields(fields)
}

#[cfg(test)]
mod tests {
    #[test]
    fn name_edit_matches_the_independent_complete_configuration() {
        let document =
            bessemer::Document::from_bytes(include_bytes!("../tests/fixtures/blank-part.SLDPRT"))
                .unwrap();
        let edit = super::rename(&document, "Bessemer Ω").unwrap();
        assert_eq!(
            edit.archive().to_bytes().unwrap(),
            include_bytes!("../tests/fixtures/model-archives/native-part-name-edited.bin")
        );
    }
}
