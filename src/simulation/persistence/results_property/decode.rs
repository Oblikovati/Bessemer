use super::super::load_reader::BinaryLoadReader;
use super::SimulationResultsContext;
use crate::Result;

pub(super) fn read(
    reader: &mut BinaryLoadReader<'_>,
    context: SimulationResultsContext,
) -> Result<()> {
    match context {
        SimulationResultsContext::RootReader(_) | SimulationResultsContext::RootCurrentWriter => {
            root(reader)
        }
        SimulationResultsContext::CategoryReader(_)
        | SimulationResultsContext::CategoryCurrentWriter => category(reader),
        SimulationResultsContext::PlotReader(_) => plot(reader),
    }
}

fn root(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("member58")?;
    reader.text("text60")?;
    reader.unsigned("member70")?;
    if reader.version >= 840013 {
        reader.signed("member74")?;
    }
    if reader.version >= 890402 {
        reader.signed("member08")?;
    }
    if reader.version >= 890605 {
        reader.text("text68")?;
    }
    Ok(())
}

fn category(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("word10")?;
    if reader.version >= 810010 {
        return reader.unsigned("unsigneda8");
    }
    if reader.version < 800044 {
        return Ok(());
    }
    let count = reader.count("text_list.count", 100000, 4)?;
    for index in 0..count {
        reader.text(&format!("text_list.{index}"))?;
    }
    Ok(())
}

fn plot(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.words("temporary_word_and_member28", 2)?;
    if reader.version >= 830035 {
        reader.text("text08")?;
    }
    Ok(())
}
