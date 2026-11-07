use super::display_records::{declared, optional_point};
use super::{ModelDecoder, ModelObjectRole, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn units(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("alignment_name")?;
    declared(reader, map, "length_units", &["moLengthUserUnits_c"])?;
    declared(reader, map, "angle_units", &["moAngleUserUnits_c"])?;
    reader.i32("word_8f8")?;
    reader.f64("unknown_ad0")?;
    reader.scope("parameters_5b0", super::display_placement::parameters)?;
    reader.text("text_680")
}

pub(super) fn favorite(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("favorite", map, ModelObjectRole::Any)?;
    reader.f64("unknown_b40")?;
    reader.u16("word_b68")?;
    reader.u16("word_b6c")?;
    reader.u32("word_b60")?;
    sketch_arrays::doubles(reader, "values_830", 4)?;
    reader.i32("word_af4")?;
    reader.f64("unknown_8b8")?;
    reader.u16("word_8fc")?;
    reader.i32("word_900")?;
    reader.scope("transform_5c0", serialized_transform)
}

pub(super) fn favorite_handle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["word_8", "word_c"])?;
    reader.u32("word_10")?;
    Ok(())
}

fn serialized_transform(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let mut flags = 0;
    for index in 0..30 {
        let word = reader.u32(&format!("words[{index}]"))?;
        if index == 28 {
            flags = word;
        }
    }
    if flags & 0xf0 == 0 {
        return Ok(());
    }
    reader.checked(Err(crate::BessemerError::new("model archive", "display serialized transform", crate::FailureClass::Unsupported,
        format!("serialized transform flags {flags:#x}; expected bits 0xf0 clear until its perspective body is documented"))))
}

pub(super) fn strings(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64("unknown_648")?;
    for label in ["text_b70", "text_b78", "text_b80", "text_b88"] {
        reader.text(label)?;
    }
    Ok(())
}

pub(super) fn attachments(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_lists::pointers(reader, map, "attach_points", ModelObjectRole::Any)?;
    reader.scope("items_908", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.i32("word_b9c")?;
    reader.u32("word_e40")?;
    Ok(())
}

pub(super) fn jogs(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..2 {
        declared(reader, map, &format!("jogs[{index}]"), &["moDimJog_c"])?;
    }
    declared(
        reader,
        map,
        "related_dimension",
        super::display_dimension::CLASSES,
    )?;
    reader.u32("flags_b64")?;
    view_tags(reader, map)?;
    optional_point(reader, "point_ac0")?;
    optional_point(reader, "point_ac8")?;
    reader.i32("word_a90")?;
    Ok(())
}

pub(super) fn late_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("backup", map, ModelObjectRole::Any)?;
    reader.i32s(&["word_b4c", "word_b50", "word_d78", "word_d7c", "word_f00"])?;
    reader.f64("unknown_8c8")?;
    reader.i32s(&[
        "word_8d0", "word_f08", "word_f0c", "word_f10", "word_8d8", "word_8dc",
    ])?;
    for label in ["text_808", "text_810"] {
        reader.text(label)?;
    }
    reader.i32s(&["word_d80", "word_f84", "word_f54"])?;
    reader.text("text_f60")?;
    reader.i32s(&["word_650", "word_f80"])?;
    double_vector(reader, "point_f68")?;
    reader.i32s(&["word_f58", "word_8e0"])?;
    if reader.version() >= 0x27e0 {
        reader.text("text_f88")?;
    }
    Ok(())
}

fn view_tags(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..2 {
        declared(
            reader,
            map,
            &format!("view_tags[{index}]"),
            &["moDrViewTag_c"],
        )?;
    }
    Ok(())
}
