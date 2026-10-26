use super::geometric_references::LINE_REFERENCE_CLASSES;
use super::{ModelDecoder, ModelObjectRole, sketch_collections, sketch_logical};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const LINEAR_COUNTS: &[&str] = &[
    "num_x",
    "num_y",
    "user_num_x",
    "user_num_y",
    "array_position_x",
    "array_position_y",
];
const LINEAR_FLAGS: &[&str] = &[
    "had_construction_lines",
    "is_edge_for_direction1",
    "is_edge_for_direction2",
];
const CIRCULAR_COUNTS: &[&str] = &[
    "array_position",
    "word_9c",
    "instance_count",
    "user_instance_count",
];
const CONSTRAINT_POSITIONS: &[&str] = &[
    "pattern_position",
    "next_constraint_position",
    "last_constraint_position",
    "this_constraint_position",
    "pattern_multiplier",
    "auto_pattern_points",
];

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&[
        "head_position",
        "visible_entity_count",
        "count_dimension_always_visible",
    ])?;
    reader.object(
        "note_pattern",
        map,
        ModelObjectRole::Declared(&["moNotePattern_c"]),
    )
}

pub(super) fn linear(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    words(reader, LINEAR_COUNTS)?;
    double_vector(reader, "direction")?;
    construction_lines(reader, map)?;
    words(reader, LINEAR_FLAGS)?;
    linear_directions(reader, map)?;
    declared(reader, map, "count_dimension", "sgLinearPattCntDim")?;
    reader.f64("unknown_f0")?;
    reader.u16("word_d8")?;
    Ok(())
}

fn linear_directions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..2 {
        reader.object(
            &format!("edge_directions[{index}]"),
            map,
            ModelObjectRole::Declared(LINE_REFERENCE_CLASSES),
        )?;
    }
    Ok(())
}

pub(super) fn circular(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    words(reader, CIRCULAR_COUNTS)?;
    double_vector(reader, "center")?;
    construction_lines(reader, map)?;
    reader.u16("had_construction_lines")?;
    declared(reader, map, "construction_circle", "sgArcHandle")?;
    words(reader, &["had_construction_circle", "is_polygon"])?;
    declared(reader, map, "point_for_center", "sgPointHandle")?;
    words(reader, &["had_point_for_center", "is_edge_for_center"])?;
    declared(reader, map, "circle_reference", "moCircleRef_w")?;
    declared(reader, map, "count_dimension", "sgCircularPattCntDim")
}

fn declared(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    class: &str,
) -> Result<()> {
    reader.object(label, map, ModelObjectRole::Declared(&[class]))
}

fn construction_lines(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..2 {
        reader.object(
            &format!("construction_lines[{index}]"),
            map,
            ModelObjectRole::Declared(&["sgLineHandle"]),
        )?;
    }
    Ok(())
}

pub(super) fn constraint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_logical::two(reader, map)?;
    words(reader, CONSTRAINT_POSITIONS)?;
    reader.i32s(&["x_scale", "y_scale"])
}

fn words(reader: &mut ModelDecoder<'_>, labels: &[&str]) -> Result<()> {
    for label in labels {
        reader.u16(label)?;
    }
    Ok(())
}

pub(super) fn linear_collection(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    sketch_collections::direct(
        reader,
        map,
        ("linear_pattern_count", "linear_patterns"),
        linear,
    )
}

pub(super) fn following_collections(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    let groups: &[(&str, &str, super::sketch_geometry::DirectReader)] = &[
        ("linear_constraints_count", "linear_constraints", constraint),
        ("circular_patterns_count", "circular_patterns", circular),
        (
            "circular_constraints_count",
            "circular_constraints",
            constraint,
        ),
    ];
    for (count, label, body) in groups {
        sketch_collections::direct(reader, map, (count, label), *body)?;
    }
    Ok(())
}
