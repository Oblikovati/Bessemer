use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn distance(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("dd_mask")?;
    sketch_arrays::doubles(reader, "bent_parameters", 2)?;
    double_vector(reader, "bent_point")?;
    reader.i32("selected_first")?;
    declared(
        reader,
        map,
        "parent_radial_dimension",
        &["moDisplayRadialDim_c", "moDisplayRevolveDim_c"],
    )?;
    if reader.version() > 0x3747 {
        reader.i32("unknown_1010")?;
    }
    Ok(())
}

pub(super) fn radial(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("flags_fd8")?;
    sketch_arrays::doubles(reader, "bent_parameters", 6)?;
    sketch_arrays::vectors(reader, "end_points", 2)?;
    let count = super::display_records::bounded_count(reader, "callout_strings", 4)?;
    for index in 0..count {
        reader.text(&format!("callout_strings[{index}]"))?;
    }
    declared(reader, map, "callout_data", &["moHwCalloutData_c"])?;
    reader.text("text_1048")?;
    declared(reader, map, "list_1068", &["suObList"])?;
    if reader.i32("has_transform_1060")? != 0 {
        reader.transform("transform_1060")?;
    }
    radial_helpers(reader, map)
}

fn radial_helpers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "mirror_helper", &["moMirrorHWHelper_c"])?;
    reader.i32("word_fd4")?;
    if reader.version() > 0x333d {
        declared(reader, map, "hole_wizard_info", &["moHoleWizardInfo_c"])?;
    }
    if reader.version() > 0x3b21 {
        reader.i32("word_10a4")?;
    }
    Ok(())
}

pub(super) fn angular(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u16("word_100c")?;
    reader.u16("word_1010")?;
    reader.u8("byte_1008")?;
    sketch_arrays::vectors(reader, "vectors", 2)?;
    sketch_arrays::doubles(reader, "parameters", 4)?;
    reader.i32("word_1018")?;
    Ok(())
}

pub(super) fn revolve(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    radial(reader, map)?;
    reader.i32("is_diameter")?;
    sketch_arrays::vectors(reader, "text_directions", 2)?;
    sketch_arrays::doubles(reader, "revolve_parameters", 4)?;
    reader.transform("dimension_plane")?;
    reader.i32("is_plane_set")?;
    Ok(())
}

pub(super) fn callout_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "hole_wizard", &["moCompHoleWzd_c"])?;
    model_lists::pointers(reader, map, "items_10", ModelObjectRole::Any)?;
    reader.i32s(&["word_34", "word_38", "word_50", "word_54"])?;
    reader.text("text_40")?;
    reader.text("text_48")?;
    model_lists::pointers(reader, map, "items_58", ModelObjectRole::Any)?;
    reader.i32("word_3c")?;
    Ok(())
}
