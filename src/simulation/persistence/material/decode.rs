use super::super::load_reader::BinaryLoadReader;
use super::{groups, tails, types::MaterialStatistics};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<MaterialStatistics> {
    let properties = header(reader)?;
    let mut pairs = 0;
    for index in 0..properties {
        pairs += property(reader, &format!("property{index}"))?;
    }
    pairs += global_table(reader)?;
    let (delegates, delegated_pairs) = groups::read(reader)?;
    pairs += delegated_pairs;
    tails::read(reader)?;
    Ok(MaterialStatistics {
        properties,
        delegates,
        pairs,
        entries: reader.declared_entries(),
    })
}

fn header(reader: &mut BinaryLoadReader<'_>) -> Result<usize> {
    reader.text("text20")?;
    reader.text("text28")?;
    let count = reader.declared_count("property_count", 10000)?;
    reader.words("header_words34_38_3c", 3)?;
    for label in ["text48", "text50", "text58"] {
        reader.text(label)?;
    }
    reader.signed("word68")?;
    reader.text("text60")?;
    reader.text("text08")?;
    Ok(count)
}

fn property(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<usize> {
    reader.text(&format!("{label}.name"))?;
    reader.signed(&format!("{label}.kind"))?;
    reader.double(&format!("{label}.scalar"))?;
    reader.text(&format!("{label}.auxiliary_text"))?;
    let present = reader.signed(&format!("{label}.curve_present"))? != 0;
    reader.text(&format!("{label}.curve_name"))?;
    let count = reader.declared_count(&format!("{label}.pair_count"), 10000)?;
    reader.words(&format!("{label}.four_words"), 4)?;
    if present {
        reader.doubles(&format!("{label}.pairs"), count * 2)?;
    }
    if reader.version >= 800010 {
        reader.signed(&format!("{label}.late_word"))?;
    }
    Ok(if present { count } else { 0 })
}

fn global_table(reader: &mut BinaryLoadReader<'_>) -> Result<usize> {
    reader.text("global_curve.name")?;
    let count = reader.declared_count("global_curve.count", 10000)?;
    reader.words("global_curve.three_words", 3)?;
    reader.doubles("global_curve.pairs", count * 2)?;
    Ok(count)
}
