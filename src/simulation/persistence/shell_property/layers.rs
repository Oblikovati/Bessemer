use super::super::{load_reader::BinaryLoadReader, material};
use super::{SimulationShellPropertyContext, presence, types::ShellPropertyStatistics};
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    context: &SimulationShellPropertyContext,
    statistics: &mut ShellPropertyStatistics,
) -> Result<()> {
    let count = reader.count("shell.layer_count", 10000, 16)?;
    reader.words("shell.words370_374", 2)?;
    let external = reader.signed("shell.word378")?;
    let known = presence::selected(&context.layer_presence, count, reader.version)?;
    statistics.layers = count;
    for index in 0..count {
        let prefix = format!("layer{index}");
        reader.doubles(&format!("{prefix}.doubles"), 2)?;
        inline_material(reader, &prefix, external, statistics)?;
        if presence::lookup(known, index)? == Some(true) {
            descriptors(reader, &prefix, statistics)?;
        }
    }
    Ok(())
}

fn inline_material(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    external: i32,
    statistics: &mut ShellPropertyStatistics,
) -> Result<()> {
    if external != 0 {
        return Ok(());
    }
    if reader.signed(&format!("{prefix}.material_presence"))? == 0 {
        return Ok(());
    }
    material::read_inline(reader, &format!("{prefix}.material"))?;
    statistics.materials += 1;
    Ok(())
}

fn descriptors(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    statistics: &mut ShellPropertyStatistics,
) -> Result<()> {
    let flags = reader.signed(&format!("{prefix}.flags"))?;
    for bit in [1, 2] {
        if flags & bit != 0 {
            reader.double(&format!("{prefix}.descriptor{bit}.double"))?;
            reader.text(&format!("{prefix}.descriptor{bit}.text"))?;
            statistics.descriptors += 1;
        }
    }
    Ok(())
}
