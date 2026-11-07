use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const AXIS_CLASSES: &[&str] = &[
    "moCSysAxisData_c",
    "moLineCSysAxisData_c",
    "moPntCSysAxisData_c",
];

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    match name {
        "moCoordSys_c" => Some(coordinates(reader, map)),
        name if AXIS_CLASSES.contains(&name) => Some(axis(reader, map)),
        _ => None,
    }
}

fn coordinates(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::reference_planes::reference_geometry(reader, map)?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(
        reader,
        map,
        "origin_6f0",
        &["moPointRef_w", "moEndPointRef_w", "moLoftSynchRef_c"],
    )?;
    selected_axes(reader, map)?;
    for label in ["raw_710", "raw_714", "raw_718"] {
        reader.u8(label)?;
    }
    double_vector(reader, "point_728")?;
    tail(reader)
}

fn selected_axes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let first = reader.u16("selector_71c")?;
    let second = reader.u16("selector_720")?;
    // C-198: the native predicate compares equality; duplicate selectors select one occurrence.
    for (number, member) in [(1, "6f8"), (2, "700"), (3, "708")] {
        if first == number || second == number {
            declared(reader, map, &format!("axis_{member}"), AXIS_CLASSES)?;
        }
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() <= 0x371a {
        return Ok(());
    }
    reader.i32s(&["raw_744", "raw_748"])?;
    for label in ["raw_750", "raw_758", "raw_760"] {
        reader.f64(label)?;
    }
    Ok(())
}

fn axis(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "references_08", ModelObjectRole::Any)?;
    double_vector(reader, "vector_28")
}
