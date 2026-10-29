use super::{ModelDecoder, display_dimension_tail as tail, display_placement as placement};
use crate::Result;
use crate::archive_codec::ObjectReader;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DisplayDimensionKind {
    Base,
    Distance,
    Radial,
    Angular,
    Revolve,
    Order,
}

pub(super) const CLASSES: &[&str] = &[
    "moDisplayDim_c",
    // #54 / C-402 proves the base display callback and no scalar suffix.
    "moDisplayScalarDim_c",
    // #54 / C-400 proves all three optional-reference predicates return zero.
    "moDisplayLinearPattCntDim_c",
    "moDisplayCircularPattCntDim_c",
    "moDisplayDistanceDim_c",
    "moDisplayRadialDim_c",
    "moDisplayAngularDim_c",
    "moDisplayRevolveDim_c",
    "moDisplayOrdDim_c",
];

impl DisplayDimensionKind {
    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "moDisplayDim_c"
            | "moDisplayScalarDim_c"
            | "moDisplayLinearPattCntDim_c"
            | "moDisplayCircularPattCntDim_c" => Some(Self::Base),
            "moDisplayDistanceDim_c" => Some(Self::Distance),
            "moDisplayRadialDim_c" => Some(Self::Radial),
            "moDisplayAngularDim_c" => Some(Self::Angular),
            "moDisplayRevolveDim_c" => Some(Self::Revolve),
            "moDisplayOrdDim_c" => Some(Self::Order),
            _ => None,
        }
    }
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: DisplayDimensionKind,
) -> Result<()> {
    super::display_object::read(reader, map)?;
    reader.scope("item", |reader| super::display_item::read(reader, map))?;
    reader.scope("dimension_text", |reader| {
        super::sketch_text::read(reader, map)
    })?;
    reader.scope("dimension", |reader| shared_fields(reader, map, kind))?;
    if kind == DisplayDimensionKind::Base {
        return Ok(());
    }
    reader.scope("suffix", |reader| suffix(reader, map, kind))
}

fn suffix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: DisplayDimensionKind,
) -> Result<()> {
    use super::display_suffixes as suffix;
    match kind {
        DisplayDimensionKind::Base => Ok(()),
        DisplayDimensionKind::Distance => suffix::distance(reader, map),
        DisplayDimensionKind::Radial => suffix::radial(reader, map),
        DisplayDimensionKind::Angular => suffix::angular(reader),
        DisplayDimensionKind::Revolve => suffix::revolve(reader, map),
        DisplayDimensionKind::Order => super::alignment_display::suffix(reader, map),
    }
}

fn shared_fields(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: DisplayDimensionKind,
) -> Result<()> {
    placement::prefix(reader, map)?;
    placement::placement(reader)?;
    placement::references(reader, map, kind)?;
    tail::units(reader, map)?;
    tail::favorite(reader, map)?;
    tail::strings(reader)?;
    tail::attachments(reader, map)?;
    tail::jogs(reader, map)?;
    tail::late_fields(reader, map)
}
