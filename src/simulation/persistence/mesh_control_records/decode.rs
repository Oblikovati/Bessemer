use super::super::{load_base, load_reader::BinaryLoadReader};
use super::{SimulationMeshControlLimits, SimulationMeshControlRecordSpan};
use crate::{Result, simulation::SimulationPersistVersion};

pub(super) fn frame_error(mut error: crate::BessemerError) -> crate::BessemerError {
    error.operation = "decode MeshControl records";
    error
}

pub(super) fn read(
    bytes: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationMeshControlLimits,
) -> Result<(BinaryLoadReader<'_>, Vec<SimulationMeshControlRecordSpan>)> {
    let mut reader =
        BinaryLoadReader::new(bytes, version, limits, "MeshControl counted records input")?;
    let count = reader.count("mesh_control.count", 10000, 188)?;
    let mut records = Vec::new();
    for index in 0..count {
        records.push(read_record(&mut reader, index)?);
    }
    reader.finish()?;
    Ok((reader, records))
}

fn read_record(
    reader: &mut BinaryLoadReader<'_>,
    index: usize,
) -> Result<SimulationMeshControlRecordSpan> {
    let label = format!("record{index}");
    let start = reader.position();
    load_base::record_with_terminal(reader, &label, mesh_curves)?;
    let base_end = reader.position();
    derived_prefix(reader, &format!("{label}.mesh"))?;
    Ok(SimulationMeshControlRecordSpan {
        index,
        start,
        base_end,
        end: reader.position(),
    })
}

fn mesh_curves(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    load_base::terminal_table_with_empty_mode(reader, label, "MeshControl curves")
}

fn mesh_signed(reader: &mut BinaryLoadReader<'_>, prefix: &str, members: &[&str]) -> Result<()> {
    for member in members {
        reader.signed(&format!("{prefix}.{member}"))?;
    }
    Ok(())
}

fn mesh_doubles(reader: &mut BinaryLoadReader<'_>, prefix: &str, members: &[&str]) -> Result<()> {
    for member in members {
        reader.double(&format!("{prefix}.{member}"))?;
    }
    Ok(())
}

fn derived_prefix(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    mesh_doubles(reader, prefix, &["double370", "double378"])?;
    mesh_signed(reader, prefix, &["word380", "word388", "word24"])?;
    for member in ["text2e8", "text2f0", "text310"] {
        reader.text(&format!("{prefix}.{member}"))?;
    }
    if reader.version < 800032 {
        return Ok(());
    }
    reader.signed(&format!("{prefix}.word320"))?;
    reader.double(&format!("{prefix}.double328"))?;
    if reader.version < 830000 {
        return Ok(());
    }
    if reader.version >= 850015 {
        reader.text(&format!("{prefix}.text318"))?;
    }
    late_members(reader, prefix)
}

fn late_members(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    if reader.version < 850016 {
        return Ok(());
    }
    mesh_signed(reader, prefix, &["word330", "word334"])?;
    if reader.version >= 850018 {
        reader.signed(&format!("{prefix}.word384"))?;
    }
    if reader.version >= 870016 {
        mesh_signed(reader, prefix, &["word3c8", "word3cc"])?;
    }
    if reader.version < 890111 {
        return Ok(());
    }
    mesh_doubles(reader, prefix, &["double390", "double398"])?;
    mesh_signed(
        reader,
        prefix,
        &["word3a0", "word3a4", "word3a8", "word368", "word36c"],
    )
}
