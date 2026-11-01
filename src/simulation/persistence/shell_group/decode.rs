use super::{SimulationShellGroupLimits, reader::ShellGroupReader};
use crate::{
    Result,
    simulation::{SimulationPersistVersion, SimulationShellWireProfile},
};

pub(super) fn read(
    input: &[u8],
    version: SimulationPersistVersion,
    profile: SimulationShellWireProfile,
    limits: SimulationShellGroupLimits,
) -> Result<ShellGroupReader<'_>> {
    let mut reader = ShellGroupReader::new(input, limits)?;
    let count = reader.record_count()?;
    for index in 0..count {
        record(
            &mut reader,
            index,
            version.value() >= 890016 || profile == SimulationShellWireProfile::Writer,
        )?;
    }
    reader.finish()?;
    Ok(reader)
}

fn record(reader: &mut ShellGroupReader<'_>, index: usize, late: bool) -> Result<()> {
    let prefix = format!("record{index}");
    for string in 0..4 {
        reader.text(&format!("{prefix}.string{string}"))?;
    }
    reader.signed(&format!("{prefix}.word28"))?;
    reader.signed(&format!("{prefix}.word2c"))?;
    reader.double(&format!("{prefix}.double30"))?;
    for word in ["70", "74", "78"] {
        reader.signed(&format!("{prefix}.word{word}"))?;
    }
    if late {
        reader.signed(&format!("{prefix}.word84"))?;
    }
    Ok(())
}
