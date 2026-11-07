use super::super::load_reader::BinaryLoadReader;
use super::SimulationSolidChildKind;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    kind: SimulationSolidChildKind,
) -> Result<()> {
    match kind {
        SimulationSolidChildKind::Component => component(reader),
        SimulationSolidChildKind::Folder => folder(reader),
    }
}

fn component(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.text("component.text68")?;
    for word in ["word70", "word74", "word78"] {
        reader.signed(&format!("component.{word}"))?;
    }
    if reader.version >= 800066 {
        reader.signed("component.word7c")?;
    }
    if reader.version >= 840013 {
        reader.signed("component.wordc0")?;
    }
    Ok(())
}

fn folder(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("folder.word58")?;
    reader.text("folder.text60")?;
    // C-730 supplies no fixed folder cap; use the signed wire domain and explicit caller budgets (#149).
    let count = reader.count("folder.list68.count", 2147483647, 4)?;
    for index in 0..count {
        reader.text(&format!("folder.list68.{index}"))?;
    }
    if reader.version >= 840014 {
        reader.signed("folder.worda0")?;
    }
    if reader.version >= 870017 {
        reader.signed("folder.wordac")?;
    }
    Ok(())
}
