use super::{
    ModelDecoder, ModelObjectRole, sketch_arrays, sketch_collections, sketch_dimensions,
    sketch_logical,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const COLLECTIONS: &[(&str, &str, super::sketch_geometry::DirectReader)] = &[
    (
        "equal_distance_points_count",
        "equal_distance_points",
        points,
    ),
    (
        "equal_distance_constraints_count",
        "equal_distance_constraints",
        constraint,
    ),
];

pub(super) fn points(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for label in ["current_constraint", "processed", "array_position"] {
        reader.u16(label)?;
    }
    sketch_dimensions::handles(reader, map, &["main_entity"])
}

pub(super) fn constraint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_logical::one(reader, map)?;
    reader.u16("owner_position")?;
    sketch_dimensions::handles(reader, map, &["along_entity"])?;
    for label in ["first_point", "mid_point", "last_point"] {
        reader.object(label, map, ModelObjectRole::Declared(&["sgPointHandle"]))?;
    }
    Ok(())
}

pub(super) fn logical_group(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() <= 0x46cb {
        return reader.checked(Err(super::invalid(format!(
            "logical group version {}; expected > {}",
            reader.version(),
            0x46cb
        ))));
    }
    sketch_logical::base(reader, map)?;
    let count = sketch_collections::word_count(reader, "relation_indices.count")?;
    sketch_arrays::integers(reader, "relation_indices", count)
}

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (count, label, body) in COLLECTIONS {
        sketch_collections::direct(reader, map, (count, label), *body)?;
    }
    if reader.version() > 0x46cb {
        sketch_collections::direct(
            reader,
            map,
            ("logical_groups_count", "logical_groups"),
            logical_group,
        )?;
    }
    Ok(())
}
