use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.object(
        "owner",
        map,
        ModelObjectRole::Declared(&["moAssembly_c", "moRouteAssembly_c", "moPart_c"]),
    )?;
    reader.object(
        "chain",
        map,
        ModelObjectRole::Declared(&["moReferenceChain_c"]),
    )?;
    reader.u32("raw_30")?;
    Ok(())
}

pub(super) fn context_component(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.scope("chain_component", |reader| component(reader, map))?;
    reader.object(
        "external_object_40",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.i32("raw_48")?;
    Ok(())
}

pub(super) fn unit(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("owner", map, ModelObjectRole::Any)?;
    reader.object(
        "external_object_28",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.i32("raw_30")?;
    configuration(reader, "configuration_38")
}

fn configuration(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    reader.text(&format!("{label}.name"))?;
    reader.i32(&format!("{label}.id"))?;
    Ok(())
}

pub(super) fn external_feature(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    super::external_references::shared_file(reader, map)?;
    configuration(reader, "configuration")?;
    reader.u8("raw_40")?;
    reader.i32("raw_44")?;
    reader.required_object(
        "name_handle",
        map,
        ModelObjectRole::Declared(&["moCStringHandle_c"]),
    )?;
    reader.object(
        "reference",
        map,
        ModelObjectRole::Declared(&["moReference_c", "moReferencePattern_c"]),
    )?;
    reader.i32("raw_58")?;
    Ok(())
}

pub(super) fn reference_chain(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.u16("raw_24")?;
    reader.object(
        "features",
        map,
        ModelObjectRole::Declared(&["moExtFeatureList_c"]),
    )?;
    reader.i32("raw_20")?;
    configuration(reader, "configuration")
}

pub(super) fn feature_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("items.count", 2)? {
        reader.object(
            &format!("items[{index}]"),
            map,
            ModelObjectRole::Declared(&["moExtFeature_c"]),
        )?;
    }
    Ok(())
}
