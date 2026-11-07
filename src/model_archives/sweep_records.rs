use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, sweep_data};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let controls = reader.scope("data", |reader| sweep_data::read(reader, map))?;
    reader.i32("raw_c04")?;
    sweep_data::bytes(reader, &["c08", "c0c", "c10", "c14", "804"])?;
    linear_dimensions(reader, map, &controls)?;
    sweep_data::bytes(reader, &["c18", "c1c"])?;
    if controls.angular {
        for label in ["angle_be0", "angle_be8"] {
            declared(reader, map, label, &["moDisplayAngularDim_c"])?;
        }
    }
    if controls.radial {
        declared(reader, map, "radius_bf0", &["moDisplayRadialDim_c"])?;
    }
    Ok(())
}

fn linear_dimensions(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    controls: &sweep_data::SweepControls,
) -> Result<()> {
    if controls.dimensions {
        declared(
            reader,
            map,
            "dimension_bd0",
            super::display_dimension::CLASSES,
        )?;
        if controls.second_dimension {
            declared(
                reader,
                map,
                "dimension_bd8",
                super::display_dimension::CLASSES,
            )?;
        }
    }
    Ok(())
}

pub(super) fn contact(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_50", ModelObjectRole::Any)?;
    reader.f64("raw_90")?;
    reader.i32("raw_98")?;
    declared(reader, map, "angle_a0", &["moDisplayAngularDim_c"])?;
    reader.f64("raw_a8")?;
    reader.i32s(&["raw_b0", "raw_b8", "raw_c8", "raw_cc"])?;
    declared(reader, map, "synch_e0", &["moLoftSynchRef_c"])?;
    reader.i32s(&["raw_40", "raw_44", "raw_48"])?;
    reader.f64("raw_c0")?;
    declared(
        reader,
        map,
        "reference_e8",
        super::model_references::WRAPPER_CLASSES,
    )
}
