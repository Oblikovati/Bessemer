use super::raw_fields::RawScalarKind::{Double, Signed, Text, Unsigned};
use super::{ModelDecoder, ModelObjectRole, part_objects, raw_fields};
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord, vectors::double_vector};
use crate::{BessemerError, FailureClass, Result};

const PREFIX: &[(raw_fields::RawScalarKind, &str)] = &[
    (Signed, "20"),
    (Unsigned, "28 2c 30"),
    (Double, "38"),
    (Signed, "40 48 4c"),
    (Double, "68 70 78 80 88 90"),
    (Signed, "1c0 1c4"),
    (Double, "50 58"),
    (Signed, "60 98 9c"),
    (Double, "b0 b8"),
    (Signed, "a0 a4 a8"),
    (Text, "e0 e8"),
    (Signed, "f8"),
    (Double, "100 108 110 118 120 128"),
    (Unsigned, "130"),
    (Double, "138 140 150 158 168 170 178 180 188 190 198 1a0"),
    (Signed, "1a8"),
];
const SUFFIX: &[(raw_fields::RawScalarKind, &str)] = &[
    (Double, "c0 c8"),
    (Signed, "1ac 220"),
    (Double, "228 230 238 240"),
    (Signed, "248"),
    (Double, "250 258 260 268 270 278"),
    (Text, "280"),
    (Signed, "d8 1b0 1b4 1b8 1bc 288 2ac"),
    (Double, "2a0"),
    (Signed, "2a8 298"),
    (Double, "1c8"),
    (Signed, "2c0"),
    (Double, "2c8"),
    (Signed, "2d0"),
    (Text, "290"),
    (Signed, "2d4 2e0"),
    (Text, "2f8"),
    (Double, "300 308"),
    (Signed, "310 314"),
    (Double, "318"),
];

/// Retain the exact presence byte and conditionally read the direct manager.
/// Example: `photoworks_materials::optional(reader, map)?` at configuration +608.
pub(super) fn optional(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.u8("has_data_608")? == 0 {
        return Ok(());
    }
    reader.scope("optional_data_608", |reader| manager(reader, map))
}

/// Preserve the direct manager and its complete, nonnull material objects.
/// Example: `photoworks_materials::manager(reader, map)?` after +608 presence.
pub(super) fn manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_10", "raw_14"])?;
    for index in 0..reader.object_count("materials.count")? {
        reader.required_object(
            &format!("materials[{index}]"),
            map,
            ModelObjectRole::Declared(&["moPWMaterialProperties_c"]),
        )?;
    }
    Ok(())
}

/// Retain the property-data wire order without applying native path/default rewrites.
/// Example: `photoworks_materials::properties(reader, map)?` for a material object.
pub(super) fn properties(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    raw_fields::groups(reader, PREFIX)?;
    double_vector(reader, "vector_1d8")?;
    double_vector(reader, "vector_1f0")?;
    reader.scope("list_08", |reader| mapped_list(reader, map))?;
    double_vector(reader, "point_208")?;
    reader.scope("list_20", |reader| mapped_list(reader, map))?;
    raw_fields::groups(reader, SUFFIX)
}

fn mapped_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.object_count("count")? {
        mapped_pointer(reader, map, &format!("items[{index}]"))?;
    }
    Ok(())
}

fn mapped_pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    let record = reader.object_tag(label, map, ModelObjectRole::Any)?;
    let ObjectRecord::New { class, .. } = record else {
        return Ok(());
    };
    if class.name != "moUnitComponent_c" {
        return reader.checked(Err(BessemerError::new(
            "PhotoWorks material",
            label,
            FailureClass::Unsupported,
            format!(
                "fresh class {:?}; expected moUnitComponent_c, null or an already loaded object",
                class.name
            ),
        )));
    }
    reader.scope(label, |reader| part_objects::unit_component(reader, map))
}
