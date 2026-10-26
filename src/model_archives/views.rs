use super::{ModelDecoder, ModelObjectRole};
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.entries(10, "model view slots")?;
    for index in 0..10 {
        reader.object(
            &format!("views[{index}]"),
            map,
            ModelObjectRole::Declared(&["moView_c"]),
        )?;
    }
    Ok(())
}

pub(super) fn view(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name")?;
    reader.transform("transform_360")?;
    reader.transform("transform_368")?;
    reader.u16("raw_380")?;
    reader.i32("raw_384")?;
    reader.f64("raw_370")?;
    for index in 0..reader.signed_count("section_data_count", 2)? {
        reader.object(
            &format!("section_data[{index}]"),
            map,
            ModelObjectRole::Declared(&["moSectionModelData_c"]),
        )?;
    }
    reader.i32s(&["view_index", "raw_38c"])?;
    view_lists(reader, map)
}

fn view_lists(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("list_390_presence")? != 0 {
        reader.scope("list_390", |reader| nested_list(reader, map))?;
    }
    reader.i32("raw_3a0")?;
    reader.f64("raw_3b0")?;
    if reader.i32("list_3a8_presence")? != 0 {
        reader.scope("list_3a8", |reader| nested_list(reader, map))?;
    }
    reader.i32("raw_3b8")?;
    if reader.version() >= 0x3764 {
        reader.i32("raw_3c0")?;
    }
    Ok(())
}

fn nested_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.object_count("count")? {
        let label = format!("items[{index}]");
        let record = reader.object_tag(&label, map, ModelObjectRole::Any)?;
        reject_transient_new(reader, &label, record)?;
    }
    Ok(())
}

fn reject_transient_new(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    record: ObjectRecord,
) -> Result<()> {
    if !matches!(record, ObjectRecord::New { .. }) {
        return Ok(());
    }
    let error = BessemerError::new(
        "model archive",
        label,
        FailureClass::Unsupported,
        format!(
            "transient view-list record {record:?}; expected null or an existing reference until helper-state body layouts are verified"
        ),
    );
    reader.checked(Err(error))
}
