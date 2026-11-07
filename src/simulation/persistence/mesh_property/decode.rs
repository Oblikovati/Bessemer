use super::super::{load_base, load_reader::BinaryLoadReader};
use super::*;
use crate::{Result, simulation::SimulationPersistVersion};

pub(super) fn frame_error(mut error: crate::BessemerError) -> crate::BessemerError {
    error.operation = "decode Mesh Property";
    error
}

pub(super) fn read(
    bytes: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationMeshPropertyLimits,
) -> Result<(
    BinaryLoadReader<'_>,
    Vec<SimulationMeshPropertyRecordSpan>,
    usize,
)> {
    let mut reader = BinaryLoadReader::new(bytes, version, limits, "Mesh Property input")?;
    super::initial::read(&mut reader)?;
    if reader.version >= 800021 {
        settings(&mut reader)?;
    }
    let (records, nested) = super::direct::read(&mut reader)?;
    super::tail::read(&mut reader)?;
    reader.finish()?;
    Ok((reader, records, nested))
}

fn settings(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let texts = reader.count("mesh.text_list.count", 10000, 4)?;
    reader.texts("mesh.text_list", texts)?;
    load_base::reference_container(reader, "mesh.settings.references", "mesh.settings")?;
    if reader.version >= 800040 {
        reader.signed("mesh.word738")?;
    }
    Ok(())
}

pub(super) fn words(reader: &mut BinaryLoadReader<'_>, members: &[&str]) -> Result<()> {
    for member in members {
        reader.signed(&format!("mesh.{member}"))?;
    }
    Ok(())
}

pub(super) fn doubles(reader: &mut BinaryLoadReader<'_>, members: &[&str]) -> Result<()> {
    for member in members {
        reader.double(&format!("mesh.{member}"))?;
    }
    Ok(())
}

pub(super) fn raw_fields(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    count: usize,
    width: usize,
) -> Result<()> {
    for index in 0..count {
        reader.blob(&format!("{prefix}[{index}]"), width)?;
    }
    Ok(())
}
