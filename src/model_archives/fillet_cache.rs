use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("presence")? == 0 {
        return Ok(());
    }
    reader.i32("word_72c")?;
    declared(reader, map, "edge_8b0", &["moCompEdge_c"])?;
    for index in 0..reader.signed_count("bodies.count", 1)? {
        reader.scope(&format!("bodies[{index}]"), kernel_frames::read_body)?;
    }
    face_edges(reader, map)?;
    reader.scope("list_9e0", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    face_indices(reader, map)?;
    reader.i32s(&["word_af8", "word_8c0", "word_8c4", "word_8c8"])
}

fn face_edges(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("face_edge_map.count", 4)? {
        declared(
            reader,
            map,
            &format!("face_edge_map[{index}][0]"),
            &["moCompFace_c"],
        )?;
        declared(
            reader,
            map,
            &format!("face_edge_map[{index}][1]"),
            &["moCompEdge_c"],
        )?;
    }
    Ok(())
}

fn face_indices(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("face_indices.count", 6)? {
        declared(
            reader,
            map,
            &format!("face_indices[{index}][0]"),
            &["moCompFace_c"],
        )?;
        let label = format!("face_indices[{index}][1]");
        let count = reader.signed_count(&format!("{label}.count"), 4)?;
        sketch_arrays::integers(reader, &label, count)?;
    }
    Ok(())
}
