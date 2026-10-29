use super::raw_fields::RawScalarKind::{Signed, Text, Unsigned, Word};
use super::{ModelDecoder, ModelObjectRole, features, raw_fields};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &str,
) -> Option<Result<()>> {
    let body = match class {
        "moReference_c" => reference,
        "moReferencePattern_c" => pattern,
        "moSmartPartData_c" => smart_part,
        "moCirPatternItemId_c" => circular_item,
        _ => return None,
    };
    Some(
        reader
            .require_ordinary_assembly()
            .and_then(|()| body(reader, map)),
    )
}

fn circular_item(reader: &mut ModelDecoder<'_>, _map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_18", "raw_1c"])?;
    reader.u8("raw_08")?;
    reader.f64("raw_10")
}

fn smart_part(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("raw_10")?;
    for index in 0..reader.signed_count("relations.count", 2)? {
        pointer(
            reader,
            map,
            &format!("relations[{index}]"),
            &["moSmartRelation_c"],
        )?;
    }
    reader.i32s(&["raw_64", "raw_68"])
}

fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::read(reader, map)?;
    optional_transform(reader, "transform_358")?;
    if reader.u8("box_presence")? != 0 {
        for index in 0..10 {
            reader.f64(&format!("box_368[{index}]"))?;
        }
    }
    pointer(reader, map, "external_object", &["moExtObject_c"])?;
    raw_fields::groups(reader, &[(Word, "3f8 400"), (Unsigned, "3d8")])?;
    reader.text("configuration.name")?;
    reader.i32("configuration.id")?;
    optional_transform(reader, "discarded_transform")?;
    transform_array(reader)?;
    smart_fields(reader, map)?;
    final_fields(reader, map)
}

fn optional_transform(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    if reader.u8(&format!("{label}_presence"))? != 0 {
        reader.transform(label)?;
    }
    Ok(())
}

fn transform_array(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.u8("transform_array_presence")? == 0 {
        return Ok(());
    }
    for index in 0..reader.signed_count("transforms.count", 1)? {
        optional_transform(reader, &format!("transforms[{index}]"))?;
    }
    Ok(())
}

fn smart_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_448")?;
    reader.u32("raw_3e8")?;
    pointer(reader, map, "smart_part", &["moSmartPartData_c"])?;
    reader.i32("raw_450")?;
    pointer(reader, map, "smc_instance", &["moSMCInstanceData_c"])?;
    pointer(
        reader,
        map,
        "smart_insert",
        &["moSmartInsertComponentData_c"],
    )?;
    reader.i32("raw_4a8")?;
    pointer(reader, map, "toolbox_specs", &["moToolboxPartSpecArray_c"])?;
    reader.i32s(&["raw_4ac", "raw_44c"])?;
    pointer(reader, map, "xml_rules", &["moXmlRulesModelData_c"])?;
    reader.i32s(&["raw_4d0", "raw_410", "raw_414", "raw_4b8", "raw_4d4"])?;
    reader.u32("time")?;
    reader.i32("raw_4e8")?;
    pointer(reader, map, "external_feature", &["moExtFeature_c"])
}

fn final_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Signed, "548"),
            (Text, "4f8 500"),
            (Signed, "51c 528 380"),
            (Unsigned, "554"),
        ],
    )?;
    if reader.version() > 0x2b6c {
        reader.u32("raw_3ec")?;
        reader.u32("raw_3f0")?;
    }
    if reader.version() > 0x2f53 {
        pointer(reader, map, "reference_wrapper", &["moRefWrapper_c"])?;
    }
    if reader.version() > 0x2f7d {
        pointer(reader, map, "stack_part", &["moStackPartData_c"])?;
        reader.i32s(&["raw_494", "raw_498", "raw_49c"])?;
    }
    if reader.version() > 0x42d2 {
        reader.i32("raw_3d4")?;
    }
    Ok(())
}

fn pattern(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference(reader, map)?;
    reader.i32("raw_560")?;
    reader.required_object(
        "pattern_item",
        map,
        ModelObjectRole::Declared(&[
            "moPatternItemId_c",
            "moLPatternItemId_c",
            "moCirPatternItemId_c",
        ]),
    )?;
    pointer(
        reader,
        map,
        "seed_reference",
        &["moReference_c", "moReferencePattern_c"],
    )?;
    reader.i32("raw_578")?;
    if reader.version() > 0x332d {
        reader.i32("raw_57c")?;
    }
    if reader.version() > 0x42d0 {
        reader.i32("raw_580")?;
    }
    if reader.version() >= 0x42e2 {
        reader.text("raw_558")?;
    }
    Ok(())
}

fn pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    classes: &[&str],
) -> Result<()> {
    reader.object(label, map, ModelObjectRole::Declared(classes))
}
