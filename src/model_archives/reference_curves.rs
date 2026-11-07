use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body_feature", |reader| {
        super::body_features::base(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn reference_array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<ObjectRecord> {
    reader.object_record(
        "references_860",
        map,
        ModelObjectRole::Declared(&["suObArray"]),
    )
}

fn dwords(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    reader.scope(label, |reader| model_lists::unsigned_words(reader, "items"))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let references = reference_array(reader, map)?;
    reader.scope("wire_820", kernel_frames::read_body)?;
    model_lists::pointers(reader, map, "attributes_948", ModelObjectRole::Any)?;
    reader.i32s(&["raw_828", "raw_87c", "raw_880"])?;
    let flags = reader.i32("raw_878")?;
    dwords(reader, "dwords_888")?;
    array_branch(reader, map, flags, &references)?;
    for label in ["dwords_8f0", "dwords_910"] {
        dwords(reader, label)?;
    }
    reader.i32("raw_930")?;
    reader.u16("raw_858")?;
    reader.i32("raw_938")?;
    suffix(reader, map)
}

fn array_branch(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    flags: i32,
    references: &ObjectRecord,
) -> Result<()> {
    if flags & 2 != 0 {
        return dwords(reader, "dwords_8a8");
    }
    let populated = !matches!(references, ObjectRecord::Null)
        && reader.referenced_count(references, "items.count")? > 0;
    if populated {
        return model_lists::pointers(reader, map, "objects_8c8", ModelObjectRole::Any);
    }
    if reader.version() > 0x2b94 {
        declared(reader, map, "plane_8e8", &["moCompRefPlane_c"])?;
    }
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2b87 {
        extra_bodies(reader, map)?;
        reader.i32("raw_934")?;
        declared(reader, map, "plane_868", &["moCompRefPlane_c"])?;
        declared(reader, map, "direction_850", &["moDirectionSpec_c"])?;
    }
    if reader.version() > 0x3efb {
        declared(
            reader,
            map,
            "curve_870",
            super::reference_point_data::CURVES,
        )?;
    }
    Ok(())
}

fn extra_bodies(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = reader.u16("body_count_840")?;
    for index in 0..reader.counted_entries("reference-curve extra bodies", i64::from(count), 3)? {
        reader.scope(&format!("bodies[{index}]"), |reader| {
            reader.scope("body", kernel_frames::read_body)?;
            model_lists::pointers(reader, map, "attributes", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}
