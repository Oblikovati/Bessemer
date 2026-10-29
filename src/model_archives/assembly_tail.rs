use super::{ModelDecoder, ModelObjectRole, configuration_io};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn lightweight(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for offset in ["1a10", "1a18"] {
        if reader.u8(&format!("presence_{offset}"))? != 0 {
            for index in 0..10 {
                reader.f64(&format!("box_{offset}[{index}]"))?;
            }
        }
    }
    reader.scope("map_1498", super::mfc_string_maps::read)?;
    reader.scope("map_14d0", super::mfc_string_maps::read)?;
    if reader.version() < 0x2b65 {
        reader.i32("discarded_compatibility_word")?;
    }
    Ok(())
}

pub(super) fn resolved(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_1558")?;
    reader.u16("word_1568")?;
    pointers(reader, map)?;
    reader.i32("word_1a20")?;
    for offset in ["18c0", "18e0"] {
        configuration_io::list(reader, map, &format!("array_{offset}"))?;
    }
    let present = reader.i32("smart_replacement_presence")?;
    configuration_io::absent(reader, "assembly smart replacement", present)?;
    if reader.i32("list_presence_1938")? != 0 {
        for offset in ["1940", "1988"] {
            configuration_io::list(reader, map, &format!("list_{offset}"))?;
        }
    }
    reader.i32("word_166c")?;
    Ok(())
}

fn pointers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (offset, classes) in [
        ("1598", &["suObArray"][..]),
        ("1600", &["moMirrorAssemblyData_c"]),
        ("1740", &["suObArray"]),
        ("1738", &["suObArray"]),
    ] {
        reader.object(
            &format!("object_{offset}"),
            map,
            ModelObjectRole::Declared(classes),
        )?;
    }
    reader.i32s(&["word_1748", "word_1590"])?;
    reader.object(
        "exploded_views",
        map,
        ModelObjectRole::Declared(&["moExplViewManager_c"]),
    )?;
    reader.object(
        "timings",
        map,
        ModelObjectRole::Declared(&["moAssemblyTimings_c"]),
    )
}

pub(super) fn routing(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_1a90")?;
    for offset in ["1a98", "1aa0", "1aa8", "1ab0"] {
        reader.text(&format!("raw_{offset}"))?;
    }
    reader.u32("raw_1ac8")?;
    reader.u32("raw_1ad0")?;
    Ok(())
}

fn timings(reader: &mut ModelDecoder<'_>, _map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&[
        "discarded_word_0",
        "discarded_word_1",
        "discarded_word_2",
        "discarded_word_3",
    ])
}

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &str,
) -> Option<Result<()>> {
    let body = match class {
        "moAssemblyTimings_c" => timings,
        "moExplViewManager_c" => super::part_tail::exploded_manager,
        "moExplId_c" => exploded_id,
        _ => return None,
    };
    Some(
        reader
            .require_ordinary_assembly()
            .and_then(|()| body(reader, map)),
    )
}

fn exploded_id(reader: &mut ModelDecoder<'_>, _map: &mut ObjectReader) -> Result<()> {
    super::part_tail::exploded_id(reader)
}
