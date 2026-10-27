use super::{ModelDecoder, ModelObjectRole, boundary_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RepresentationLayout {
    Keeper,
    Local(&'static [&'static str]),
    Source(&'static [&'static str]),
}

use RepresentationLayout::{Keeper, Local, Source};

// #46: these finite variants are the declared representation family in 11.18.2;
// unrelated runtime classes still fail the model pointer-role check.
const LAYOUTS: &[(&str, RepresentationLayout)] = &[
    ("moEntityIdRep_c", Keeper),
    ("moSurfaceIdRep_c", Keeper),
    ("moFromSktEntSurfIdRep_c", Local(&["38"])),
    ("moFilletSurfIdRep_c", Local(&["38"])),
    ("moFaceBlendSurfIdRep_c", Local(&["38"])),
    // 11.225 / C-253 proves both primary dispatches, ancestry and the one-word suffix.
    ("moDraftSurfIdRep_c", Local(&["38"])),
    ("moBackDraftSurfIdRep_c", Local(&["38"])),
    // 11.184 / C-212 binds the common representation helper and one signed suffix.
    ("moWireEdgeIdRep_c", Local(&["38"])),
    ("moSimpleSurfIdRep_c", Local(&["38"])),
    ("moSimpleEdgeIdRep_c", Local(&["38"])),
    // #54 / 11.232 / C-274 binds the exact native dispatch and entity ancestry.
    ("moEdgeRefIdRep_c", Local(&["38"])),
    ("moSegmEdgeIdRep_c", Local(&["38"])),
    ("moEndEdgeIdRep_c", Local(&["38"])),
    ("moTopEdgeIdRep_c", Local(&["38"])),
    ("moThinOffsetSurfIdRep_c", Local(&["38"])),
    ("moBottomEdgeIdRep_c", Local(&["38"])),
    ("moStartEdgeIdRep_c", Local(&["38"])),
    ("moEndFaceSurfIdRep_c", Local(&["38", "3c"])),
    // #54 / 11.350 / C-392 binds the exact native serializer and edge ancestry.
    ("moTrimWireExtEdgeIdRep_c", Local(&["38", "3c"])),
    ("moRadiateSurfIdRep_c", Local(&["38", "3c"])),
    // C-243 / 11.215 proves the common helper and two-word suffix, including edge ancestry.
    ("moRadiateEdgeIdRep_c", Local(&["38", "3c"])),
    ("moNetSideSurfIdRep_c", Local(&["38", "3c"])),
    ("moSweepSideSurfIdRep_c", Local(&["38", "3c"])),
    ("moWzdHoleSurfIdRep_c", Local(&["38", "3c"])),
    ("moUnBendSurfIdRep_c", Local(&["38", "3c"])),
    ("moThinClosingSurfIdRep_c", Local(&["38", "3c"])),
    ("moBendEndSurfIdRep_c", Local(&["38", "3c"])),
    ("moSegmLeftTrimEdgeIdRep_c", Local(&["38", "3c"])),
    ("moSegmRightTrimEdgeIdRep_c", Local(&["38", "3c"])),
    ("moEndFace3IntSurfIdRep_c", Local(&["38", "3c", "40"])),
    ("moFromSktEnt3IntSurfIdRep_c", Local(&["38", "3c", "40"])),
    ("moSideFace3IntSurfIdRep_c", Local(&["38", "3c", "40"])),
    ("moDerivedSurfIdRep_c", Source(&["48"])),
    ("moPLineSurfIdRep_c", Source(&["48"])),
    ("moPunchSurfIdRep_c", Source(&["48"])),
    ("moCirPatternSurfIdRep_c", Source(&["48"])),
    ("moCopySurfIdRep_c", Source(&["48"])),
    ("moPLineProjIdRep_c", Source(&["48"])),
    ("moDerivedEdgeIdRep_c", Source(&["48"])),
    ("moCopyEdgeIdRep_c", Source(&["48"])),
    ("moTrimExtEdgeIdRep_c", Source(&["48"])),
    // #54 / C-252 proves both primary dispatches and the direct moEntityId_c member.
    ("moLeftTrimEdgeIdRep_c", Source(&["48"])),
    ("moRightTrimEdgeIdRep_c", Source(&["48"])),
    ("moSurfExtNeighborIdRep_c", Source(&["48"])),
    ("moMirPatternSurfIdRep_c", Source(&[])),
    ("moIndirectSurfIdRep_c", Source(&[])),
    ("moShellSurfIdRep_c", Source(&[])),
    ("moLPatternSurfIdRep_c", Source(&["48", "4c"])),
    // #54 / C-392 proves the direct entity-ID member without an outer map slot.
    ("moPttnBodyEdgeIdRep_c", Source(&["48", "4c"])),
    // #54 / 11.192 / C-220 proves direct moEntityId_c +38 and both signed suffix words.
    ("moPLineSilhIdRep_c", Source(&["48", "4c"])),
    ("moDimPatternSurfIdRep_c", Source(&["48", "4c"])),
    ("moPLineInterSurfIdRep_c", Source(&["48", "4c"])),
    ("moMacroFeatRefSurfIdRep_c", Source(&["48", "4c", "50"])),
];

pub(super) fn layout(name: &str) -> Option<RepresentationLayout> {
    LAYOUTS.iter().find_map(
        |(class, layout)| {
            if *class == name { Some(*layout) } else { None }
        },
    )
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    layout: RepresentationLayout,
) -> Result<()> {
    boundary_references::entity_representation(reader, map)?;
    let offsets = match layout {
        Keeper => return Ok(()),
        Local(offsets) => offsets,
        Source(offsets) => {
            reader.scope("source_entity", |reader| {
                boundary_references::entity_id(reader, map)
            })?;
            offsets
        }
    };
    for offset in offsets {
        reader.i32(&format!("unknown_{offset}"))?;
    }
    Ok(())
}

pub(super) fn feature_reference(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object(
        "external_model",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.i32("unknown_10")?;
    reader.u32("time")?;
    Ok(())
}
