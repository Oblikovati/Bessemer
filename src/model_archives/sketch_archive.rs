use super::{
    ModelDecoder, sketch_constraints, sketch_curves, sketch_dimensions, sketch_geometry,
    sketch_logical, sketch_patterns, sketch_tail,
};
use crate::Result;
use crate::archive_codec::ObjectReader;

const STAGES: &[sketch_geometry::DirectReader] = &[
    sketch_geometry::points,
    sketch_geometry::lines,
    sketch_geometry::texts,
    sketch_curves::hatches,
    sketch_curves::arcs,
    sketch_curves::ellipses,
    sketch_curves::conics,
    super::sketch_splines::read,
    super::sketch_surfaces::read,
    sketch_dimensions::valued_collections,
    sketch_logical::collections,
    sketch_logical::isoparam_collection,
    sketch_patterns::linear_collection,
    sketch_patterns::following_collections,
    after_patterns,
];

pub(super) fn ordinary(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for stage in STAGES {
        stage(reader, map)?;
    }
    Ok(())
}

pub(super) fn after_patterns(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object(
        "display_dimensions",
        map,
        super::ModelObjectRole::Declared(&["suObList"]),
    )?;
    sketch_constraints::collections(reader, map)?;
    for (name, stage) in TAIL_STAGES {
        reader.scope(name, |reader| stage(reader, map))?;
    }
    Ok(())
}

const TAIL_STAGES: &[(&str, sketch_geometry::DirectReader)] = &[
    ("tail_offsets", super::sketch_offsets::collections),
    (
        "styled_intersections",
        super::sketch_intersections::collections,
    ),
    ("state", state),
    ("visual", sketch_tail::visual),
    ("local_relations", sketch_tail::local_relations),
    ("coincidence_regions", sketch_tail::coincidences),
    ("silhouettes", silhouettes),
    ("block_manager", super::sketch_managers::blocks),
    ("loop_manager", super::sketch_managers::loops),
    ("slot_manager", super::sketch_managers::slots),
    ("final_fields", final_ordinary),
];

fn state(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    sketch_tail::state(reader)
}

fn silhouettes(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    sketch_tail::silhouettes(reader)
}

fn final_ordinary(reader: &mut ModelDecoder<'_>, _: &mut ObjectReader) -> Result<()> {
    sketch_tail::final_ordinary(reader)
}
