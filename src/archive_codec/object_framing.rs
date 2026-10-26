use super::ObjectFraming;
use crate::{BessemerError, Result};

#[derive(Clone, Copy)]
pub(crate) struct ObjectClassView<'a> {
    pub name: &'a str,
    pub schema: u16,
}

pub(crate) fn write_object_framing(
    framing: &ObjectFraming,
    class: Option<ObjectClassView<'_>>,
    mut emit: impl FnMut(&[u8]) -> Result<()>,
    invalid: impl Fn(String) -> BessemerError,
) -> Result<()> {
    require_framing(framing, class.is_some(), &invalid)?;
    emit(&framing.tag.to_le_bytes())?;
    if let Some(value) = framing.extended {
        emit(&value.to_le_bytes())?;
    }
    if let Some(class) = class {
        write_descriptor(class, &mut emit, &invalid)?;
    }
    Ok(())
}

fn require_framing(
    framing: &ObjectFraming,
    has_class: bool,
    invalid: &impl Fn(String) -> BessemerError,
) -> Result<()> {
    if framing.extended.is_some() != (framing.tag == 0x7fff) || has_class != (framing.tag == 0xffff)
    {
        return Err(invalid(format!(
            "MFC tag {:#06x} with extension/class presence {}/{}; expected framing selected by 0x7FFF/0xFFFF",
            framing.tag,
            framing.extended.is_some(),
            has_class
        )));
    }
    Ok(())
}

fn write_descriptor(
    class: ObjectClassView<'_>,
    emit: &mut impl FnMut(&[u8]) -> Result<()>,
    invalid: &impl Fn(String) -> BessemerError,
) -> Result<()> {
    let length = descriptor_length(class.name, invalid)?;
    emit(&class.schema.to_le_bytes())?;
    emit(&length.to_le_bytes())?;
    emit(class.name.as_bytes())
}

fn descriptor_length(name: &str, invalid: &impl Fn(String) -> BessemerError) -> Result<u16> {
    if name.is_empty() || !name.is_ascii() {
        return Err(invalid(format!(
            "class name {:?}; expected nonempty ASCII",
            name
        )));
    }
    u16::try_from(name.len())
        .map_err(|_| invalid(format!("class name length {}; expected u16", name.len())))
}
