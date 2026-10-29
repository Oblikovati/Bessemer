use super::{reader::SceneReader, *};

pub(super) fn decode_scene(
    bytes: &[u8],
    context: ComposerSceneContext,
    limits: ComposerSceneLimits,
) -> Result<SceneLedger> {
    budget("decoded bytes", bytes.len(), limits.maximum_decoded_bytes)?;
    let mut ledger = SceneLedger::default();
    let mut reader = SceneReader::new(bytes, &mut ledger, context, limits);
    metadata::information(&mut reader)?;
    let mut index = 0;
    while reader.remaining() != 0 {
        entry(&mut reader, &format!("entries[{index}]"))?;
        index += 1;
    }
    Ok(ledger)
}

fn entry(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    let kind = reader.byte(&format!("{path}.tag"))?;
    match kind {
        b'u' => geode(reader, path),
        b'v' => face_body(reader, path),
        _ => Err(unsupported(format!(
            "scene record tag {kind}; expected u (117) or v (118)"
        ))),
    }
}

fn geode(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.record(true)?;
    let flag = reader.byte(&format!("{path}.matrix_presence"))?;
    if flag != 0 {
        reader.patterns(&format!("{path}.matrix"), 16)?;
    }
    reader.text(&format!("{path}.name"))?;
    let lods = metadata::userdata(reader, &format!("{path}.userdata"))?;
    reader.count_fits(lods, 4, "LOD count")?;
    for index in 0..lods {
        lod(reader, &format!("{path}.lods[{index}]"))?;
    }
    Ok(())
}

fn lod(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    let count = reader.word(&format!("{path}.geometry_count"))?;
    reader.count_fits(count, 1, "drawable count")?;
    for index in 0..count {
        let prefix = format!("{path}.faces[{index}]");
        let tag = reader.byte(&format!("{prefix}.tag"))?;
        if tag != b'v' {
            return Err(invalid(format!("drawable tag {tag}; expected v (118)")));
        }
        face_body(reader, &prefix)?;
    }
    Ok(())
}

fn face_body(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.record(false)?;
    metadata::attributes(reader, &format!("{path}.attributes"))?;
    drawable::drawable(reader, &format!("{path}.geometry"))
}
