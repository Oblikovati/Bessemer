use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn unsigned_words(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.count_with_minimum(&format!("{label}.count"), 4)? {
        reader.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn pointers(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<()> {
    counted_pointers(reader, map, label, role).map(|_| ())
}

pub(super) fn counted_pointers(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<usize> {
    let count = reader.object_count(&format!("{label}.count"))?;
    for index in 0..count {
        reader.object(&format!("{label}[{index}]"), map, role)?;
    }
    Ok(count)
}

pub(super) fn nodes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    pointers(reader, map, "nodes", ModelObjectRole::Any)?;
    reader.i32("unknown_tail")?;
    Ok(())
}

/// Admit complete fresh bodies only from the caller's verified local reader table.
/// Example: `scoped_pointer(reader, map, "item", &["moUnitComponent_c"], "PhotoWorks material")?`.
pub(super) fn scoped_pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    classes: &'static [&'static str],
    operation: &'static str,
) -> Result<()> {
    reader.scoped_fresh_objects(classes, operation, |reader| {
        reader.object(label, map, ModelObjectRole::Any)
    })
}

pub(super) fn require_scoped_class(
    record: &crate::archive_codec::ObjectRecord,
    label: &str,
    classes: &[&str],
    operation: &'static str,
) -> Result<()> {
    let crate::archive_codec::ObjectRecord::New { class, .. } = record else {
        return Ok(());
    };
    if classes.contains(&class.name.as_str()) {
        return Ok(());
    }
    Err(scoped_class_error(operation, label, &class.name, classes))
}

fn scoped_class_error(
    operation: &'static str,
    label: &str,
    actual: &str,
    classes: &[&str],
) -> crate::BessemerError {
    crate::BessemerError::new(
        operation,
        label,
        crate::FailureClass::Unsupported,
        format!(
            "fresh class {actual:?}; expected {}, null or an already loaded object",
            classes.join(", ")
        ),
    )
}
