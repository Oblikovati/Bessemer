use super::super::load_reader::BinaryLoadReader;
use super::SimulationMeshControlHeaderProfile;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    profile: SimulationMeshControlHeaderProfile,
) -> Result<()> {
    reader.signed("word08")?;
    if reader.version > 800027 || profile == SimulationMeshControlHeaderProfile::Writer {
        reader.signed("word50")?;
    }
    if reader.version >= 840013 || profile == SimulationMeshControlHeaderProfile::Writer {
        reader.signed("word60")?;
    }
    Ok(())
}
