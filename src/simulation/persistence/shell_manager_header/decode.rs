use super::super::{SimulationShellWireProfile, load_reader::BinaryLoadReader};
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    profile: SimulationShellWireProfile,
) -> Result<()> {
    reader.signed("wordc8")?;
    if reader.version > 800027 || profile == SimulationShellWireProfile::Writer {
        reader.signed("word10c")?;
    }
    Ok(())
}
