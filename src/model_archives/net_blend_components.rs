use super::display_records::declared;
use super::raw_fields::{
    RawScalarKind::{Double, Signed, Unsigned},
    fields,
};
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn contact(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("reference_08", map, ModelObjectRole::Any)?;
    reader.u32("raw_10")?;
    Ok(())
}

pub(super) fn group(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("component_reference", |reader| {
        super::component_references::base(reader, map)
    })?;
    reader.i32("raw_e8")?;
    for label in ["list_f0", "list_138"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn multi_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("component_reference", |reader| {
        super::component_references::base(reader, map)
    })?;
    edges(reader, map)?;
    multi_tail(reader, map)
}

fn edges(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = reader.u32("edges_08.count")?;
    for index in 0..reader.counted_entries("multi-edge references", i64::from(count), 2)? {
        declared(
            reader,
            map,
            &format!("edges_08[{index}]"),
            &["moCompEdge_c"],
        )?;
    }
    Ok(())
}

fn multi_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["reference_38", "reference_40"] {
        reader.object(label, map, ModelObjectRole::Any)?;
    }
    fields(reader, Signed, "impl_30 impl_34")?;
    fields(reader, Double, "impl_48 impl_50")?;
    for label in ["point_58", "point_70"] {
        crate::archive_codec::vectors::double_vector(reader, label)?;
    }
    fields(reader, Unsigned, "impl_88")
}
