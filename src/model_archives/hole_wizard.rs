use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, hole_parameters, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    if (0x2f46..0x2f68).contains(&reader.version()) {
        reader.i32("raw_base_compat")?;
    }
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let (pairs, enabled) = hole_parameters::read(reader, map)?;
    if reader.version() > 0x2f47 {
        reader.i32("raw_998")?;
    }
    if reader.version() > 0x332e {
        reader.i32("raw_bb0")?;
    }
    if enabled {
        slots(reader, map, pairs)?;
    }
    tail(reader, map)
}

pub(super) fn slot_axis(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["point_08", "point_20", "vector_38"] {
        double_vector(reader, label)?;
    }
    Ok(())
}

fn slots(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, pairs: i32) -> Result<()> {
    declared(reader, map, "dimension_a68", &["moDisplayDistanceDim_c"])?;
    let count = if reader.version() < 0x2b5e {
        reader.signed_count_value("slot_axes_bd8", pairs, 72)?
    } else {
        reader.signed_count("slot_axes_bd8.count", 72)?
    };
    for index in 0..count {
        reader.scope(&format!("slot_axes_bd8[{index}]"), slot_axis)?;
    }
    reader.i32s(&["raw_a70", "raw_a74", "raw_a78"])?;
    declared(
        reader,
        map,
        "dimension_a80",
        super::display_dimension::CLASSES,
    )?;
    if reader.version() < 0x42ea {
        reader.i32("raw_83c")?;
    }
    double_vector(reader, "vector_aa8")
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x3355 {
        reader.f64("raw_ba8")?;
    }
    if reader.version() > 0x3f02 {
        reader.u8("raw_bf8")?;
        reader.i32s(&["raw_bfc", "raw_c00"])?;
        model_lists::pointers(reader, map, "array_c08", ModelObjectRole::Any)?;
    }
    if reader.version() >= 0x42ea {
        reader.i32("raw_83c")?;
    }
    Ok(())
}
