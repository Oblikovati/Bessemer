use super::{ModelDecoder, ModelObjectRole, sketch_entities, sketch_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) type DirectReader = fn(&mut ModelDecoder<'_>, &mut ObjectReader) -> Result<()>;

pub(super) fn points(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    collection(
        reader,
        map,
        ("point_count", "points"),
        Some("stored_category"),
        point,
    )
}

pub(super) fn lines(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    collection(
        reader,
        map,
        ("line_count", "lines"),
        Some("stored_category_mask"),
        line,
    )
}

pub(super) fn texts(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    collection(reader, map, ("text_count", "texts"), None, text)
}

pub(super) fn collection(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    labels: (&str, &str),
    category: Option<&str>,
    body: DirectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    let count = reader.signed_count(labels.0, if category.is_some() { 2 } else { 1 })?;
    for index in 0..count {
        reader.scope(&format!("{}[{index}]", labels.1), |reader| {
            if let Some(category) = category {
                reader.u16(category)?;
            }
            body(reader, map)
        })?;
    }
    Ok(())
}

fn coordinates(reader: &mut ModelDecoder<'_>, label: &str, dimensions: usize) -> Result<()> {
    for index in 0..dimensions {
        reader.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn point(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("entity", |reader| sketch_entities::entity(reader, map))?;
    let mask = reader.u16("mask")?;
    let dimensions = point_dimensions(reader, mask)?;
    coordinates(reader, "position", dimensions)?;
    reader.u16("point_type")?;
    reader.u16("geometric_reference_count")?;
    for index in 0..2 {
        reader.object(
            &format!("contour_handles[{index}]"),
            map,
            ModelObjectRole::Declared(sketch_references::HANDLES),
        )?;
    }
    reader.u16("center_reference_count")?;
    reader.i32("unknown_108")?;
    point_suffix(reader, dimensions)
}

fn point_dimensions(reader: &mut ModelDecoder<'_>, mask: u16) -> Result<usize> {
    if mask & 0x20 != 0 {
        return reader.checked(Err(crate::BessemerError::new(
            "model archive",
            "point mask",
            crate::FailureClass::Unsupported,
            format!("point mask {mask:#06x}; expected minimal-owner bit 0x20 clear"),
        )));
    }
    Ok(if mask & 0x10 != 0 { 2 } else { 3 })
}

fn point_suffix(reader: &mut ModelDecoder<'_>, dimensions: usize) -> Result<()> {
    coordinates(reader, "point_118", dimensions)?;
    coordinates(reader, "point_130", dimensions)?;
    let jet = reader.u16("has_jet")?;
    super::part_objects::absent_payload(reader, "point jet", i32::from(jet))?;
    reader.u16("bidirectional_external_reference_count")?;
    Ok(())
}

pub(super) fn segment(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<sketch_entities::SketchEntityFlags> {
    let flags = reader.scope("entity", |reader| sketch_entities::entity(reader, map))?;
    for name in ["end1", "end2"] {
        reader.escaped_index(name, 0xffff)?;
    }
    reader.u16("is_contour")?;
    reader.u16("is_flipped")?;
    reader.f64("normal_scale")?;
    Ok(flags)
}

fn line(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if segment(reader, map)?.flags_56 & 0x200 != 0 {
        double_vector(reader, "vector_e0")?;
    }
    reader.u16("unknown_e8")?;
    Ok(())
}

fn text(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    segment(reader, map)?;
    let record = reader.object_record(
        "dimension_text",
        map,
        ModelObjectRole::Declared(&["moDimText_c"]),
    )?;
    let curves = reader.object_integer(&record, "has_curve_data")?;
    if curves != 0 {
        text_kernel_record(reader, map)?;
    }
    reader.text("text")?;
    reader.i32s(&["long_date", "date_and_time"])
}

fn text_kernel_record(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "curve_references",
        map,
        ModelObjectRole::Declared(&["suObArray"]),
    )?;
    reader.scope("kernel_body", super::kernel_frames::read_body)?;
    Ok(())
}
