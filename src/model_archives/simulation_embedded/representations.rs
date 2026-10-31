use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{
    ModelDecoder, ModelObjectRole,
    entity_representations::{self, RepresentationLayout},
};

const HISTORICAL: &[&str] = &[
    "moFromSktEntSurfIdRep_c",
    "moEndFaceSurfIdRep_c",
    "moFaceBlendSurfIdRep_c",
    "moPLineSurfIdRep_c",
    "moIndirectSurfIdRep_c",
    "moTopEdgeIdRep_c",
    "moSegmEdgeIdRep_c",
    "moSimpleSurfIdRep_c",
    "moSegmRightTrimEdgeIdRep_c",
    "moPLineProjIdRep_c",
    "moBottomEdgeIdRep_c",
    "moStartEdgeIdRep_c",
    "moDerivedEdgeIdRep_c",
    "moFilletSurfIdRep_c",
    "moShellSurfIdRep_c",
    "moDerivedSurfIdRep_c",
    "moMirPatternSurfIdRep_c",
    "moThinClosingSurfIdRep_c",
    "moFromSktEnt3IntSurfIdRep_c",
    "moRadiateEdgeIdRep_c",
];
const MODERN: &[&str] = &[
    "moEndEdgeIdRep_c",
    "moEndFace3IntSurfIdRep_c",
    "moSweepSideSurfIdRep_c",
    "moWzdHoleSurfIdRep_c",
    "moSideFace3IntSurfIdRep_c",
    "moCirPatternSurfIdRep_c",
    "moLPatternSurfIdRep_c",
    "moDerivedSurfIdRep2_c",
    "moSimpleDoubleEdgeIdRep_c",
    "moSimpleDoubleSurfIdRep_c",
    "moThinOffsetSurfIdRep_c",
    "moPLineInterSurfIdRep_c",
    "moPttnBodyEdgeIdRep_c",
    "moPLineSilhIdRep_c",
    "moDerivedEdgeIdRep2_c",
];

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    if name == "moFR_c" {
        return Some(feature_reference(reader, map));
    }
    let minimum = if HISTORICAL.contains(&name) {
        4281
    } else if MODERN.contains(&name) {
        4307
    } else {
        return None;
    };
    Some(read_representation(reader, map, name, minimum))
}

fn layout(name: &str) -> Result<RepresentationLayout> {
    use RepresentationLayout::{Local, Source};
    match name {
        "moSimpleDoubleEdgeIdRep_c" | "moSimpleDoubleSurfIdRep_c" => Ok(Local(&["38", "3c"])),
        "moDerivedSurfIdRep2_c" | "moDerivedEdgeIdRep2_c" => Ok(Source(&["48", "4c"])),
        _ => entity_representations::layout(name).ok_or_else(|| {
            super::unsupported(
                name,
                format!("representation {name:?}; expected a declared embedded suffix layout"),
            )
        }),
    }
}

fn read_representation(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
    minimum: u32,
) -> Result<()> {
    if reader.archive_version().value() < minimum {
        return Err(super::unsupported(
            name,
            format!(
                "version {}; expected {name} version >= {minimum}",
                reader.archive_version().value()
            ),
        ));
    }
    let layout = layout(name)?;
    reader.embedded_representation(|reader| body(reader, map, name, layout))
}

fn body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
    layout: RepresentationLayout,
) -> Result<()> {
    reader.object("feature", map, ModelObjectRole::Any)?;
    reader.object("external_reference", map, ModelObjectRole::Any)?;
    let offsets = match layout {
        RepresentationLayout::Keeper => &[][..],
        RepresentationLayout::Local(offsets) => offsets,
        RepresentationLayout::Source(offsets) => {
            reader.object("source_entity.representation", map, ModelObjectRole::Any)?;
            offsets
        }
    };
    for offset in offsets {
        reader.i32(&format!("unknown_{offset}"))?;
    }
    if name == "moCirPatternSurfIdRep_c"
        && (0x23ec..0x23ef).contains(&reader.archive_version().value())
    {
        reader.i32("legacy_word")?;
    }
    Ok(())
}

fn feature_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "external_model",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.i32("unknown_10")?;
    reader.u32("time")?;
    Ok(())
}
