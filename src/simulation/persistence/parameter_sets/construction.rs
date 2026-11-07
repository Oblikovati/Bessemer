use super::layout::{
    ParameterGates, bounded_count, checked_extent, construction_count, require_presence,
};
use super::*;

pub(super) struct ParameterConstruction {
    pub size: usize,
    pub record_count: i32,
}

pub(super) fn validate(
    parts: &SimulationParameterParts,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<ParameterConstruction> {
    let record_count = validated_counts(parts, gates, limits)?;
    let mut size = base_size(parts.table.len(), gates)?;
    for record in &parts.records {
        size = add_size(size, record_size(record, parts.table.len(), gates, limits)?)?;
    }
    Ok(ParameterConstruction { size, record_count })
}

fn validated_counts(
    parts: &SimulationParameterParts,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<i32> {
    let record_count = construction_count(
        parts.records.len(),
        limits.maximum_records,
        "ParameterSets construction records",
    )?;
    construction_count(
        parts.table.len(),
        limits.maximum_table_items,
        "ParameterSets construction table",
    )?;
    require_table(parts, gates, limits)?;
    require_header(&parts.header, gates)?;
    Ok(record_count)
}

fn base_size(table_count: usize, gates: ParameterGates) -> Result<usize> {
    let size = 28
        + usize::from(gates.early_word) * 4
        + usize::from(gates.second_early) * 4
        + usize::from(gates.late_word) * 4
        + usize::from(gates.second_late) * 4;
    add_size(
        size,
        checked_extent(table_count, 4, "ParameterSets construction table bytes")?,
    )
}

fn require_table(
    parts: &SimulationParameterParts,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<()> {
    let expected = if gates.modern_table {
        bounded_count(
            parts.header.words[0],
            limits.maximum_table_items,
            "ParameterSets construction header count",
        )?
    } else {
        100
    };
    require(
        parts.table.len() == expected,
        "ParameterSets construction table shape",
        format!(
            "{} entries, first header word {}; expected exactly {expected} entries for the selected version",
            parts.table.len(),
            parts.header.words[0]
        ),
    )
}

fn require_header(header: &SimulationParameterHeader, gates: ParameterGates) -> Result<()> {
    require_presence(
        header.gate700013.is_some(),
        gates.early_word,
        "ParameterSets gate700013",
    )?;
    require_presence(
        header.gate700018.is_some(),
        gates.second_early,
        "ParameterSets gate700018",
    )?;
    require_presence(
        header.gate820015.is_some(),
        gates.late_word,
        "ParameterSets gate820015",
    )?;
    require_presence(
        header.gate830009.is_some(),
        gates.second_late,
        "ParameterSets gate830009",
    )
}

fn record_size(
    record: &SimulationParameterRecordParts,
    table_count: usize,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<usize> {
    require_record_shape(record, table_count, gates, limits)?;
    let fixed = 16 + usize::from(gates.record_word) * 4 + usize::from(gates.early_word) * 8;
    let size = add_size(fixed, record.text.payload().len())?;
    add_size(
        size,
        checked_extent(table_count, 8, "ParameterSets construction value bytes")?,
    )
}

fn require_record_shape(
    record: &SimulationParameterRecordParts,
    table_count: usize,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
) -> Result<()> {
    record.text.require_limit(limits.maximum_text_bytes)?;
    require_presence(
        record.numbers.conditional_word.is_some(),
        gates.record_word,
        "ParameterSets record conditional word",
    )?;
    require_presence(
        record.numbers.conditional_double_bits.is_some(),
        gates.early_word,
        "ParameterSets record conditional double",
    )?;
    require(
        record.numbers.values_bits.len() == table_count,
        "ParameterSets record values shape",
        format!(
            "{} complete double patterns; expected exactly {table_count} values",
            record.numbers.values_bits.len()
        ),
    )
}

fn add_size(size: usize, extra: usize) -> Result<usize> {
    size.checked_add(extra).ok_or_else(|| {
        invalid(
            "ParameterSets construction extent",
            format!("{size} plus {extra} bytes; expected a representable complete image"),
        )
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn complete_parameter_extent_checks_addition_overflow() {
        assert!(super::add_size(usize::MAX, 1).is_err());
        assert_eq!(super::add_size(120000, 12).unwrap(), 120012);
    }
}
