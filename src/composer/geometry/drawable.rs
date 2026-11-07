use super::{reader::SceneReader, *};
use crate::composer::arrays::ComposerVectorDimensions::{Three, Two};

pub(super) fn drawable(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    let kind = reader.byte(&format!("{path}.kind"))?;
    match kind {
        b'a' => triangles(reader, path),
        b'b' => indexed_triangles(reader, path),
        b'c' | b'd' => strips(reader, path, kind),
        b'e' => lines(reader, path),
        b'f' => points(reader, path),
        _ => Err(unsupported(format!(
            "drawable kind {kind}; expected a through f"
        ))),
    }
}

fn triangles(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    let triangles = reader.word(&format!("{path}.triangle_count"))?;
    let count = triangles.checked_mul(3).ok_or_else(|| {
        invalid(format!(
            "triangle count {triangles}; expected at most {}",
            u32::MAX / 3
        ))
    })?;
    reader.vector(&format!("{path}.coordinates"), count, Three)?;
    let binding = reader.word(&format!("{path}.normal_binding"))?;
    if binding != 0 {
        reader.vector(&format!("{path}.normals"), count, Three)?;
    }
    reader.word(&format!("{path}.trailing_word"))?;
    Ok(())
}

fn indexed_triangles(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.word(&format!("{path}.triangle_count"))?;
    let coords = reader.word(&format!("{path}.coordinate_count"))?;
    let indices = reader.word(&format!("{path}.index_count"))?;
    reader.vector(&format!("{path}.coordinates"), coords, Three)?;
    reader.indices(&format!("{path}.coordinate_indices"), indices, coords)?;
    separate_normals(reader, path, indices)?;
    let layers = reader.word(&format!("{path}.texture_layer_count"))?;
    if layers == 0 {
        return Ok(());
    }
    indexed_textures(reader, path, layers, indices)
}

fn separate_normals(reader: &mut SceneReader<'_, '_>, path: &str, indices: u32) -> Result<()> {
    let binding = reader.word(&format!("{path}.normal_binding"))?;
    if binding == 0 {
        return Ok(());
    }
    let normals = reader.word(&format!("{path}.normal_count"))?;
    reader.vector(&format!("{path}.normals"), normals, Three)?;
    reader.indices(&format!("{path}.normal_indices"), indices, normals)
}

fn strips(reader: &mut SceneReader<'_, '_>, path: &str, kind: u8) -> Result<()> {
    let strips = reader.word(&format!("{path}.strip_count"))?;
    let coords = reader.word(&format!("{path}.coordinate_count"))?;
    let indices = reader.word(&format!("{path}.index_count"))?;
    let maximum = reader.word(&format!("{path}.length_domain"))?;
    reader.vector(&format!("{path}.coordinates"), coords, Three)?;
    strip_normals(reader, path, kind, coords, indices)?;
    reader.indices(&format!("{path}.strip_lengths"), strips, maximum)?;
    let layers = reader.word(&format!("{path}.texture_layer_count"))?;
    if !(1..=4).contains(&layers) {
        return Ok(());
    }
    if kind == b'd' {
        return textures(reader, path, layers, coords);
    }
    if !reader.context.texture_import_enabled {
        return Ok(());
    }
    indexed_textures(reader, path, layers, indices)
}

fn strip_normals(
    reader: &mut SceneReader<'_, '_>,
    path: &str,
    kind: u8,
    coords: u32,
    indices: u32,
) -> Result<()> {
    if kind == b'c' {
        reader.indices(&format!("{path}.coordinate_indices"), indices, coords)?;
        return separate_normals(reader, path, indices);
    }
    let binding = reader.word(&format!("{path}.normal_binding"))?;
    if binding != 0 {
        reader.vector(&format!("{path}.normals"), coords, Three)?;
    }
    reader.indices(&format!("{path}.coordinate_indices"), indices, coords)
}

fn indexed_textures(
    reader: &mut SceneReader<'_, '_>,
    path: &str,
    layers: u32,
    indices: u32,
) -> Result<()> {
    let coords = reader.word(&format!("{path}.texture_coordinate_count"))?;
    textures(reader, path, layers, coords)?;
    reader.indices(&format!("{path}.texture_indices"), indices, coords)
}

fn textures(reader: &mut SceneReader<'_, '_>, path: &str, layers: u32, coords: u32) -> Result<()> {
    reader.count_fits(layers, 4, "texture layers")?;
    for index in 0..layers {
        let prefix = format!("{path}.textures[{index}]");
        let binding = reader.word(&format!("{prefix}.binding"))?;
        if binding == 3 {
            reader.vector(&format!("{prefix}.coordinates"), coords, Two)?;
        }
    }
    Ok(())
}

fn lines(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.word(&format!("{path}.line_width_pattern"))?;
    let strips = reader.word(&format!("{path}.strip_count"))?;
    let coords = reader.word(&format!("{path}.coordinate_count"))?;
    let maximum = reader.word(&format!("{path}.length_domain"))?;
    reader.vector(&format!("{path}.coordinates"), coords, Three)?;
    reader.indices(&format!("{path}.strip_lengths"), strips, maximum)
}

fn points(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.word(&format!("{path}.point_size_pattern"))?;
    let coords = reader.word(&format!("{path}.coordinate_count"))?;
    reader.vector(&format!("{path}.coordinates"), coords, Three)
}
