use super::super::load_reader::BinaryLoadReader;
use super::SimulationInertialKind;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    label: &str,
    kind: SimulationInertialKind,
) -> Result<()> {
    match kind {
        SimulationInertialKind::Gravity => gravity(reader, label),
        SimulationInertialKind::Centrifugal => centrifugal(reader, label),
        SimulationInertialKind::BaseExcitation => excitation(reader, label),
    }
}

fn gravity(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let prefix = format!("{label}.gravity");
    reader.words(&format!("{prefix}.words"), 4)?;
    reader.doubles(&format!("{prefix}.three_doubles"), 3)?;
    reader.doubles(&format!("{prefix}.twelve_doubles"), 12)?;
    reader.texts(&format!("{prefix}.text"), 3)?;
    if reader.version >= 800037 {
        reader.words(&format!("{prefix}.late_words"), 3)?;
    }
    Ok(())
}

fn centrifugal(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let prefix = format!("{label}.centrifugal");
    reader.words(&format!("{prefix}.words"), 7)?;
    reader.texts(&format!("{prefix}.text"), 2)?;
    reader.doubles(&format!("{prefix}.three_doubles"), 3)?;
    reader.doubles(&format!("{prefix}.twelve_doubles"), 12)?;
    reader.doubles(&format!("{prefix}.final_twelve_doubles"), 12)?;
    if reader.version >= 800037 {
        reader.words(&format!("{prefix}.late_words"), 2)?;
    }
    Ok(())
}

fn excitation(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let prefix = format!("{label}.excitation");
    reader.words(&format!("{prefix}.signed_header"), 5)?;
    reader.unsigned(&format!("{prefix}.unsigned_header"))?;
    reader.doubles(&format!("{prefix}.three_doubles"), 3)?;
    reader.words(&format!("{prefix}.three_words"), 3)?;
    reader.doubles(&format!("{prefix}.twelve_doubles"), 12)?;
    reader.texts(
        &format!("{prefix}.text"),
        if reader.version >= 820012 { 4 } else { 3 },
    )?;
    reader.words(&format!("{prefix}.tail_words"), 3)?;
    if reader.version >= 820012 {
        excitation_late(reader, &prefix)?;
    }
    Ok(())
}

fn excitation_late(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    reader.signed(&format!("{prefix}.fourth_tail_word"))?;
    reader.signed(&format!("{prefix}.mixed_word0"))?;
    reader.double(&format!("{prefix}.mixed_double"))?;
    reader.signed(&format!("{prefix}.mixed_word1"))?;
    Ok(())
}
