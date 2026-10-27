use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::raw_fields::{
    RawScalarKind::{Double, Octet, Signed, Unsigned},
    fields, groups,
};
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    let (thin, mode) = reader.scope("data", |reader| data(reader, map))?;
    if thin == 0 {
        return Ok(());
    }
    declared(reader, map, "dimension_10f8", DIMENSIONS)?;
    if mode == 3 {
        declared(reader, map, "dimension_1100", DIMENSIONS)?;
    }
    Ok(())
}

fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<(u8, i32)> {
    for label in ["curves_50", "curves_2a8"] {
        reader.scope(label, |reader| curve_set(reader, map, false))?;
    }
    reader.scope("centerline_500", |reader| curve_set(reader, map, true))?;
    let controls = reader.scope("body_options_848", body_options)?;
    reader.i32("raw_8d0")?;
    if reader.version() > 0x2f4b {
        reader.scope("surface_degree_8d8", |reader| {
            fields(reader, Signed, "flag_10 14 18")
        })?;
    }
    Ok(controls)
}

fn curve_set(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    centerline: bool,
) -> Result<()> {
    let count = model_lists::counted_pointers(reader, map, "contacts_08", ModelObjectRole::Any)?;
    dependent_arrays(reader, map, count)?;
    for label in ["dwords_1c8", "dwords_1e8", "dwords_208"] {
        model_lists::unsigned_words(reader, label)?;
    }
    fields(reader, Unsigned, "228 22c 230 234")?;
    reader.scope("data_238", |reader| curve_options(reader, centerline))
}

fn dependent_arrays(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    count: usize,
) -> Result<()> {
    contact_arrays(reader, map, count)?;
    points(reader, "points_48", count as i64)?;
    // #54 / 11.226 / C-268: the load loop preserves negative M and consumes no points.
    let second = reader.i32("points_50_count")?;
    if second > 0 {
        points(reader, "points_50", i64::from(second))?;
    }
    Ok(())
}

fn contact_arrays(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    count: usize,
) -> Result<()> {
    for index in 0..reader.counted_entries("net-blend contact arrays", count as i64, 2)? {
        model_lists::pointers(
            reader,
            map,
            &format!("contact_arrays_28[{index}]"),
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

fn points(reader: &mut ModelDecoder<'_>, label: &str, count: i64) -> Result<()> {
    for index in 0..reader.counted_entries(label, count, 24)? {
        crate::archive_codec::vectors::double_vector(reader, &format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn curve_options(reader: &mut ModelDecoder<'_>, centerline: bool) -> Result<()> {
    if !centerline {
        fields(reader, Octet, "08 0c")?;
        return fields(reader, Signed, "discarded 10 14");
    }
    groups(
        reader,
        &[
            (Signed, "0c 28"),
            (Octet, "44 48 4c 08"),
            (Double, "38 18 20"),
            (Octet, "10 2c"),
            (Double, "30"),
            (Signed, "40"),
        ],
    )
}

fn body_options(reader: &mut ModelDecoder<'_>) -> Result<(u8, i32)> {
    fields(reader, Octet, "08 0c 10")?;
    reader.i32("raw_14")?;
    let thin = reader.u8("want_thin_solid_18")?;
    fields(reader, Double, "20 28")?;
    let mode = reader.i32("raw_30")?;
    Ok((thin, mode))
}
