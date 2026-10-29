use super::{XmlLimits, encoding, reader};
use crate::{BessemerError, FailureClass, Result};

pub(crate) fn validate(bytes: &[u8], limits: XmlLimits) -> Result<()> {
    encoding::require_size(
        bytes.len(),
        limits.maximum_bytes,
        "Inspection UTF-8 XML bytes",
    )?;
    let body = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(bytes);
    let text = std::str::from_utf8(body)
        .map_err(|error| invalid(format!("XML encoding error {error}; expected strict UTF-8")))?;
    let model = reader::parse_document(text, limits)?;
    let root = model.elements.first().map(|element| &element.name);
    if root.is_some_and(|root| {
        root.local.as_ref() == "SOLIDWORKSINSPECTION"
            && root.namespace.as_deref() == Some("http://www.extensiblecad.com")
    }) {
        return Ok(());
    }
    Err(invalid(format!(
        "XML root {root:?}; expected {{http://www.extensiblecad.com}}SOLIDWORKSINSPECTION"
    )))
}

fn invalid(cause: String) -> BessemerError {
    BessemerError::new(
        "Inspection XML",
        "standalone project",
        FailureClass::Invalid,
        cause,
    )
}
