use super::reader::ThermalReader;
use crate::{Result, simulation::invalid};

pub(super) fn record(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    references(reader, label)?;
    reader.signed(&format!("{label}.base.integer"))?;
    reader.texts(&format!("{label}.base.text"), 4)?;
    reader.words(&format!("{label}.base.two_integers"), 2)?;
    scalar_block(reader, label)?;
    if reader.version > 800027 {
        reader.signed(&format!("{label}.base.id"))?;
    }
    reader.doubles(&format!("{label}.base.six_doubles"), 6)?;
    reader.texts(&format!("{label}.base.array_text"), 6)?;
    if reader.version > 800001 {
        optional_slot(reader, &format!("{label}.optional0"))?;
        optional_slot(reader, &format!("{label}.optional1"))?;
    }
    gated_words(reader, label)?;
    late_base(reader, label)
}

fn scalar_block(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    let prefix = format!("{label}.base.scalar_block");
    reader.double(&format!("{prefix}.f8"))?;
    reader.signed(&format!("{prefix}.ec"))?;
    reader.double(&format!("{prefix}.e0"))?;
    reader.unsigned(&format!("{prefix}.e8"))?;
    reader.words(&format!("{prefix}.words"), 3)
}

fn references(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    let count = reader.count(&format!("{label}.references.count"), 10000, 1)?;
    for index in 0..count {
        reference(reader, &format!("{label}.reference{index}"))?;
    }
    reader.words(&format!("{label}.references.tail"), 2)
}

fn reference(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    if reader.version < 840002 {
        reader.words(&format!("{label}.legacy_header"), 7)?;
        reader.text(&format!("{label}.legacy_name"))?;
        reference_blob(reader, label)?;
        if reader.version >= 800075 {
            reader.words(&format!("{label}.tail"), 2)?;
        }
        return Ok(());
    }
    reader.words(&format!("{label}.id_header"), 2)?;
    reference_blob(reader, label)?;
    reader.embedded(&format!("{label}.archive0"))?;
    reader.embedded(&format!("{label}.archive1"))?;
    reader.words(&format!("{label}.header"), 5)?;
    reader.text(&format!("{label}.name"))?;
    reader.words(&format!("{label}.tail"), 2)?;
    if reader.version >= 890021 {
        reader.words(&format!("{label}.late_tail"), 3)?;
    }
    Ok(())
}

fn reference_blob(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    let declared = reader.signed(&format!("{label}.blob_count"))?;
    let count = reader.available_count(declared, label, 1000000, 1)?;
    reader.blob(&format!("{label}.blob"), count)
}

pub(super) fn optional_slot(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    reader.signed(&format!("{label}.integer"))?;
    if reader.signed(&format!("{label}.present"))? != 0 {
        optional_body(reader, label)?;
    }
    Ok(())
}

fn optional_body(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    reader.text(&format!("{label}.name"))?;
    reader.signed(&format!("{label}.header[0]"))?;
    let selector = reader.signed(&format!("{label}.header[1]"))?;
    reader.words(&format!("{label}.header_middle"), 2)?;
    let count = reader.signed(&format!("{label}.header[4]"))?;
    if selector == 4 {
        reader.doubles(&format!("{label}.nine_doubles"), 9)?;
    }
    if reader.version >= 820011 {
        reader.signed(&format!("{label}.version_integer"))?;
    }
    let count = reader.bounded(count, label, 100000, 16)?;
    reader.doubles(&format!("{label}.pairs"), count * 2)
}

fn gated_words(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    for (enabled, suffix) in [
        (reader.version > 800002, "ac"),
        (reader.version > 800037, "48"),
        (reader.version >= 800066, "230"),
    ] {
        if enabled {
            reader.signed(&format!("{label}.base.{suffix}"))?;
        }
    }
    Ok(())
}

fn late_base(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    if reader.version <= 810001 {
        return Ok(());
    }
    for (threshold, suffix) in [(870006, "c8"), (870010, "170")] {
        if reader.version >= threshold {
            reader.signed(&format!("{label}.base.{suffix}"))?;
        }
    }
    if reader.version >= 880008 {
        reader.double(&format!("{label}.base.108"))?;
    }
    if reader.version < 890009 {
        return Ok(());
    }
    late_texts(reader, label)
}

fn late_texts(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    reader.words(&format!("{label}.base.three_integers"), 3)?;
    reader.text(&format!("{label}.base.158"))?;
    reader.text(&format!("{label}.base.160"))?;
    if reader.version >= 890011 {
        terminal_table(reader, label)?;
    }
    Ok(())
}

fn terminal_table(reader: &mut ThermalReader<'_>, label: &str) -> Result<()> {
    let mode = reader.signed(&format!("{label}.table.mode"))?;
    if !(-100..=100).contains(&mode) {
        return Err(invalid(
            label,
            format!("table mode {mode}; expected -100..=100"),
        ));
    }
    let count = reader.count(&format!("{label}.table.count"), 50, 184)?;
    for index in 0..count {
        let prefix = format!("{label}.table{index}");
        reader.text(&format!("{prefix}.name"))?;
        for pair in 0..15 {
            reader.double(&format!("{prefix}.pairs[{pair}].double"))?;
            reader.signed(&format!("{prefix}.pairs[{pair}].word"))?;
        }
    }
    Ok(())
}
