use super::super::load_reader::BinaryLoadReader;
use super::SimulationMeshPropertyRecordSpan;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
) -> Result<(Vec<SimulationMeshPropertyRecordSpan>, usize)> {
    if reader.version < 800074 {
        return Ok((Vec::new(), 0));
    }
    let count = reader.count("mesh.direct_records.count", 10000, 36)?;
    let mut records = Vec::new();
    let mut nested = 0;
    for index in 0..count {
        let start = reader.position();
        nested += record(reader, index)?;
        records.push(SimulationMeshPropertyRecordSpan {
            index,
            start,
            end: reader.position(),
        });
    }
    Ok((records, nested))
}

fn record(reader: &mut BinaryLoadReader<'_>, index: usize) -> Result<usize> {
    let prefix = format!("mesh.record{index}");
    reader.words(&format!("{prefix}.five_words"), 5)?;
    if reader.version >= 870018 {
        reader.words(&format!("{prefix}.words14_18"), 2)?;
    }
    reader.words(&format!("{prefix}.four_words1c_28"), 4)?;
    let mut nested = 0;
    if reader.version >= 850002 {
        nested += extension(reader, &format!("{prefix}.extension2c"))?;
    }
    if reader.version >= 850030 {
        nested += extension(reader, &format!("{prefix}.extension38"))?;
    }
    Ok(nested)
}

fn extension(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<usize> {
    if reader.signed(&format!("{prefix}.mode"))? != 1 {
        return Ok(0);
    }
    let count = reader.count(&format!("{prefix}.count"), 10000, 268)?;
    for index in 0..count {
        super::nested::read(reader, &format!("{prefix}.nested{index}"))?;
    }
    Ok(count)
}
