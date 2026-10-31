use super::super::load_reader::BinaryLoadReader;
use super::{SimulationThermalKind, base};
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    label: &str,
    kind: SimulationThermalKind,
) -> Result<()> {
    match kind {
        SimulationThermalKind::Temperature => temperature(reader, label),
        SimulationThermalKind::Convection => convection(reader, label),
        SimulationThermalKind::HeatFlux | SimulationThermalKind::HeatPower => {
            heat(reader, label, kind == SimulationThermalKind::HeatPower)
        }
        SimulationThermalKind::Radiation => radiation(reader, label),
    }
}

fn temperature(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    reader.words(&format!("{label}.thermal.words"), 2)?;
    reader.double(&format!("{label}.thermal.double"))?;
    reader.text(&format!("{label}.thermal.text"))?;
    reader.signed(&format!("{label}.thermal.word24"))?;
    if reader.version >= 890501 {
        pairs(reader, label)?;
    }
    Ok(())
}

fn convection(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    reader.words(&format!("{label}.thermal.words"), 2)?;
    reader.doubles(&format!("{label}.thermal.doubles"), 2)?;
    reader.texts(&format!("{label}.thermal.text"), 2)?;
    if reader.version > 800001 {
        base::optional_slot(reader, &format!("{label}.thermal.optional"))?;
    }
    if reader.version >= 890501 {
        reader.signed(&format!("{label}.thermal.word2f8"))?;
    }
    Ok(())
}

fn heat(reader: &mut BinaryLoadReader<'_>, label: &str, power: bool) -> Result<()> {
    reader.words(&format!("{label}.thermal.words"), 2)?;
    reader.double(&format!("{label}.thermal.double0"))?;
    reader.signed(&format!("{label}.thermal.word0"))?;
    reader.doubles(&format!("{label}.thermal.doubles"), 2)?;
    reader.signed(&format!("{label}.thermal.word1"))?;
    reader.texts(&format!("{label}.thermal.text"), 3)?;
    if power {
        reader.signed(&format!("{label}.thermal.word24"))?;
    }
    if reader.version >= 800037 {
        reader.signed(&format!("{label}.thermal.late_word"))?;
    }
    if power && reader.version >= 830027 {
        reader.signed(&format!("{label}.thermal.word344"))?;
    }
    if reader.version >= 890501 {
        heat_late(reader, label, power)?;
    }
    Ok(())
}

fn heat_late(reader: &mut BinaryLoadReader<'_>, label: &str, power: bool) -> Result<()> {
    if power {
        return pairs(reader, label);
    }
    reader.words(&format!("{label}.thermal.two_late_words"), 2)
}

fn radiation(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    reader.words(&format!("{label}.thermal.words"), 4)?;
    reader.doubles(&format!("{label}.thermal.doubles"), 4)?;
    reader.texts(&format!("{label}.thermal.text"), 3)?;
    if reader.version >= 890501 {
        reader.signed(&format!("{label}.thermal.word300"))?;
    }
    Ok(())
}

fn pairs(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    if reader.signed(&format!("{label}.thermal.selector"))? != 1 {
        return Ok(());
    }
    let count = reader.count(&format!("{label}.thermal.pair_count"), 10000, 8)?;
    reader.words(&format!("{label}.thermal.pairs"), count * 2)
}
