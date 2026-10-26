use super::{ModelDecoder, ModelObjectRole};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ModelNodePrefix {
    pub flags_28: u32,
    pub id: i32,
    pub unknown_2c: Option<u32>,
}

pub(super) fn lightweight(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<ModelNodePrefix> {
    require_node_version(reader)?;
    reader.object("name", map, ModelObjectRole::Declared(&["moNodeName_c"]))?;
    reader.i32("unknown_0c")?;
    let flags_28 = reader.u32("flags_28")?;
    let id = reader.i32("id")?;
    let unknown_2c = late_node(reader)?;
    Ok(ModelNodePrefix {
        flags_28,
        id,
        unknown_2c,
    })
}

fn late_node(reader: &mut ModelDecoder<'_>) -> Result<Option<u32>> {
    let flags = if reader.version() > 0x143a {
        Some(reader.u32("unknown_2c")?)
    } else {
        None
    };
    if reader.version() > 0x23cd {
        reader.text("unknown_20")?;
    }
    Ok(flags)
}

fn require_node_version(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0xc7c {
        return Ok(());
    }
    let error = BessemerError::new(
        "model archive",
        "node lightweight",
        FailureClass::Unsupported,
        format!("node version {}; expected > 0xC7C", reader.version()),
    );
    reader.checked(Err(error))
}
