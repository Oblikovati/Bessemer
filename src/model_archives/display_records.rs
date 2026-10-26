use super::{ModelDecoder, ModelObjectRole, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn declared(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    names: &[&str],
) -> Result<()> {
    reader.object(label, map, ModelObjectRole::Declared(names))
}

pub(super) fn optional_array(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    if reader.i32(&format!("has_{label}"))? != 0 {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn optional_point(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    if reader.i32(&format!("has_{label}"))? != 0 {
        double_vector(reader, label)?;
    }
    Ok(())
}

pub(super) fn indexed_objects(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    member: &str,
    class: &str,
) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 6)? {
        reader.scope(&format!("{label}[{index}]"), |reader| {
            reader.i32("index")?;
            declared(reader, map, member, &[class])
        })?;
    }
    Ok(())
}

pub(super) fn bounded_vectors(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    maximum: usize,
) -> Result<()> {
    let count = bounded_count(reader, label, maximum)?;
    sketch_arrays::vectors(reader, label, count)
}

pub(super) fn bounded_count(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    maximum: usize,
) -> Result<usize> {
    let count = reader.signed_count(&format!("{label}.count"), 1)?;
    if count <= maximum {
        return Ok(count);
    }
    reader.checked(Err(super::invalid(format!(
        "{label} count {count}; expected 0..={maximum}"
    ))))
}
