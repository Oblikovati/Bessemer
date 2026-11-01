use super::super::{load_persist_id, load_reader::BinaryLoadReader};
use super::SimulationShellCompositeProfile;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    profile: SimulationShellCompositeProfile,
) -> Result<()> {
    match profile {
        SimulationShellCompositeProfile::FlipTurnReader
        | SimulationShellCompositeProfile::FlipTurnWriter => flip_turn(reader),
        _ => extra(reader, profile),
    }
}

fn flip_turn(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let count = reader.count("count", 100000, 16)?;
    for record in 0..count {
        for word in 0..4 {
            reader.signed(&format!("record{record}.word{word}"))?;
        }
    }
    Ok(())
}

fn extra(
    reader: &mut BinaryLoadReader<'_>,
    profile: SimulationShellCompositeProfile,
) -> Result<()> {
    reader.signed("member37c")?;
    reader.signed("member3f0")?;
    let selector = reader.signed("member3f4")?;
    let writer = profile == SimulationShellCompositeProfile::ExtraWriter;
    if selector != 1 && !writer {
        return Ok(());
    }
    let mode = reader.signed("member3f8")?;
    reader.doubles("double400_428", 6)?;
    if mode != 4 || writer && reader.position() == reader.bytes.len() {
        return Ok(());
    }
    load_persist_id::read(reader)
}
