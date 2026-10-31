use crate::Result;
use crate::model_archives::ModelDecoder;

pub(super) fn signed_count(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    maximum: usize,
    minimum: usize,
) -> Result<usize> {
    let count = reader.signed_count(label, minimum)?;
    if count > maximum {
        return Err(crate::model_archives::invalid(format!(
            "{label} count {count}; expected 0..={maximum}",
        )));
    }
    Ok(count)
}
