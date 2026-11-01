use super::super::{load_base, load_reader::BinaryLoadReader};
use super::{SimulationShellPropertyContext, types::ShellPropertyStatistics};
use crate::{Result, simulation::invalid};

pub(super) fn frame_error(mut error: crate::BessemerError) -> crate::BessemerError {
    error.operation = "decode shell Property";
    error
}

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    context: &SimulationShellPropertyContext,
) -> Result<ShellPropertyStatistics> {
    load_base::record_with_terminal(reader, "property", shell_terminal_table)?;
    let base_end = reader.position();
    let subtype = derived_prefix(reader)?;
    let mut statistics = ShellPropertyStatistics {
        base_end,
        subtype,
        layers: 0,
        materials: 0,
        descriptors: 0,
    };
    if subtype == 2 && reader.version >= 830010 {
        super::layers::read(reader, context, &mut statistics)?;
    }
    if reader.version >= 870022 {
        reader.signed("shell.word480")?;
    }
    if reader.version >= 890006 {
        reader.text("shell.text488")?;
    }
    Ok(statistics)
}

fn shell_terminal_table(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let mode = reader.signed(&format!("{label}.table.mode"))?;
    let count = reader.count(&format!("{label}.table.count"), 50, 184)?;
    if count != 0 && !(-100..=100).contains(&mode) {
        return Err(invalid(
            label,
            format!(
                "mode {mode} with count {count}; expected -100..=100 for nonempty shell curves"
            ),
        ));
    }
    load_base::terminal_records(reader, label, count)
}

fn derived_prefix(reader: &mut BinaryLoadReader<'_>) -> Result<i32> {
    reader.double("shell.double328")?;
    reader.signed("shell.word330")?;
    let subtype = reader.signed("shell.subtype334")?;
    reader.signed("shell.word340")?;
    reader.text("shell.text300")?;
    reader.text("shell.text348")?;
    unsigned_pair(reader, "shell.words338_33c")?;
    if reader.version >= 820017 {
        unsigned_pair(reader, "shell.words350_354")?;
    }
    derived_extension(reader)?;
    Ok(subtype)
}

fn unsigned_pair(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    for index in 0..2 {
        reader.unsigned(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn derived_extension(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version <= 830008 {
        return Ok(());
    }
    reader.signed("shell.word358")?;
    if reader.version >= 840005 {
        reader.signed("shell.word368")?;
        reader.double("shell.double360")?;
    }
    Ok(())
}
