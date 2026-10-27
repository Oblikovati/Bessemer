use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, features, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

// #59: planar surfaces supply their concrete keeper through the inherited nonnull base pointer.
const ID_KEEPERS: &[&str] = &[
    "moIdKeeper_c",
    "moPlanarSurfIdKeeper_c",
    // #54 / C-181: 26 exact-EOF native surface roots supply this concrete inherited keeper.
    "moExtruRefSurfIdKeeper_c",
];

pub(super) fn header(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| model_feature(reader, map))?;
    for label in ["array_360", "array_380"] {
        reader.scope(label, |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    if reader.u16("has_id_keeper")? != 0 {
        reader.required_object("id_keeper", map, ModelObjectRole::Declared(ID_KEEPERS))?;
    }
    Ok(())
}

pub(super) fn model_feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| features::read(reader, map))?;
    reader.i32("word_354")?;
    Ok(())
}

pub(super) fn atom_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (label, body) in ATOM_STAGES {
        reader.scope(label, |reader| body(reader, map))?;
    }
    Ok(())
}

const ATOM_STAGES: &[(&str, super::sketch_geometry::DirectReader)] = &[
    ("atom_references", references),
    ("atom_choosers", choosers),
    ("atom_maps", maps),
    ("atom_late", late),
];

pub(super) fn nullable_array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("presence")? != 0 {
        reader.scope("array", |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["array_520", "array_528"] {
        reader.scope(label, |reader| nullable_array(reader, map))?;
    }
    declared(reader, map, "atom", &["moAtom_c"])?;
    let count = reader.signed_count("indices_490.count", 4)?;
    sketch_arrays::integers(reader, "indices_490", count)?;
    reader.i32("word_538")?;
    reader.scope("array_540", |reader| nullable_array(reader, map))?;
    reader.i32("word_53c")?;
    Ok(())
}

pub(super) fn chooser(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.u16("presence")? == 0 {
        return Ok(());
    }
    reader.scope("body", |reader| {
        reader.scope("items", |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
        declared(reader, map, "owner", CLASSES)
    })
}

fn choosers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("chooser_4a0", |reader| chooser(reader, map))?;
    reader.u16("word_498")?;
    reader.scope("chooser_4a8", |reader| chooser(reader, map))?;
    reader.i32("word_4b0")?;
    for label in ["chooser_4b8", "chooser_4c0"] {
        reader.scope(label, |reader| chooser(reader, map))?;
    }
    reader.i32("word_570")?;
    Ok(())
}

fn maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("array_548", |reader| nullable_array(reader, map))?;
    if reader.i32("partition_presence")? != 0 {
        declared(reader, map, "partition_record", &["moPMarkRecord_c"])?;
        sketch_arrays::integers(reader, "partition_words", 3)?;
    }
    reader.i32("word_5c8")?;
    for (label, edge) in [("map_670", false), ("map_5d0", true), ("map_3f8", false)] {
        atom_map(reader, map, label, edge)?;
    }
    Ok(())
}

fn atom_map(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    edge: bool,
) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), if edge { 6 } else { 8 })? {
        let key = format!("{label}[{index}][0]");
        if edge {
            declared(reader, map, &key, &["moCompEdge_c"])?;
        } else {
            reader.u32(&key)?;
        }
        reader.u32(&format!("{label}[{index}][1]"))?;
    }
    Ok(())
}

fn late(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_488")?;
    for index in 0..reader.signed_count("appearances.count", 6)? {
        reader.i32(&format!("appearances[{index}][0]"))?;
        declared(
            reader,
            map,
            &format!("appearances[{index}][1]"),
            &["moVisualProperties_c"],
        )?;
    }
    if reader.i32("data_6a0_presence")? != 0 {
        reader.scope("data_6a0", surface_ids)?;
    }
    names(reader, map, "face_names", true)?;
    names(reader, map, "index_names", false)?;
    reader.i32("word_6b4")?;
    Ok(())
}

fn names(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    face: bool,
) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), if face { 3 } else { 5 })? {
        let key = format!("{label}[{index}][0]");
        if face {
            declared(reader, map, &key, &["moCompFace_c"])?;
        } else {
            reader.i32(&key)?;
        }
        reader.text(&format!("{label}[{index}][1]"))?;
    }
    Ok(())
}

fn surface_ids(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("entries.count", 8)? {
        reader.scope(&format!("entries[{index}]"), |reader| {
            reader.u32("key")?;
            if reader.i32("value")? < 0 && reader.i32("children_presence")? != 0 {
                let count = reader.signed_count("children.count", 4)?;
                sketch_arrays::integers(reader, "children", count)?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) const CLASSES: &[&str] = &[
    "moAtomFeature_c",
    "moBodyFeature_c",
    "moExtrusion_c",
    "moBoss_c",
    "moCut_c",
    "moICE_c",
    "moRevolution_c",
    "moRevCut_c",
    "moRib_c",
    "moRefGeom_c",
    "moRefPlane_c",
    // #46 / C-174: the axis constructor inherits moRefGeom_c and its atom-owner contract.
    "moRefAxis_c",
    // #54 / C-191: reference points retain the same inherited atom-owner contract.
    "moRefPoint_c",
    // #54 / C-193: both routing features inherit the common reference-geometry owner layer.
    "moConnectionPoint_c",
    "moRoutePoint_c",
    // #54 / C-198: coordinate systems call the same inherited reference-geometry serializer.
    "moCoordSys_c",
    "Fillet_c",
    "VarFillet_c",
    "moLPattern_c",
    // #54: shared pattern choosers can reference their enclosing mirror object.
    "moMirrorPattern_c",
    "moShell_c",
    "moCirPattern_c",
    "moHoleWzd_c",
    "moImported_c",
    "moBaseBody_c",
    "moExtruRefSurface_c",
    "moSweep_c",
    "moSweepCut_c",
    "moDome_c",
    // #54 / C-182 and C-296: both sheet-metal serializers inherit the body-feature owner layer.
    "moSheetMetal_c",
    "moTemplateSheetMetal_c",
    // #54 / C-183: the bend-group constructor retains the body-feature owner contract.
    "moSMBaseFlange_c",
    // #54 / C-194: flat patterns use the complete bend-group/body-feature layer.
    "moFlatPattern_c",
    // #54 / C-186: both shared blend serializers call the full body-feature base.
    "moBlend_c",
    "moBlendCut_c",
    // #54 / C-199: the structural-member serializer starts with the full body-feature base.
    "moWeldMemberFeat_c",
    // #60: both complete readers use this inherited atom-feature owner role.
    "moPlanarSurface_c",
    "Chamfer_c",
    "moPLine_c",
    "moScale_c",
];

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: BodyFeatureKind,
) -> Result<()> {
    base(reader, map)?;
    match kind {
        BodyFeatureKind::Extrusion => reader.scope("extrusion_suffix", |reader| {
            super::body_end_specs::extrusion(reader, map)
        }),
        BodyFeatureKind::Revolution => reader.scope("revolution_suffix", |reader| {
            super::body_end_specs::revolution(reader, map)
        }),
        BodyFeatureKind::Rib => reader.scope("rib_suffix", |reader| {
            super::body_end_specs::rib(reader, map)
        }),
    }
}

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    header(reader, map)?;
    atom_fields(reader, map)?;
    reader.scope("body_suffix", |reader| suffix(reader, map))
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "assembly_feature_data", &["moAsmFeatData_c"])?;
    declared(
        reader,
        map,
        "component_feature",
        super::component_references::FEATURE_CLASSES,
    )?;
    reader.scope("array_778", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.i32("word_798")?;
    Ok(())
}

#[derive(Clone, Copy)]
pub(super) enum BodyFeatureKind {
    Extrusion,
    Revolution,
    Rib,
}

impl BodyFeatureKind {
    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "moExtrusion_c" | "moBoss_c" | "moCut_c" | "moICE_c" => Some(Self::Extrusion),
            "moRevolution_c" | "moRevCut_c" => Some(Self::Revolution),
            "moRib_c" => Some(Self::Rib),
            _ => None,
        }
    }
}
