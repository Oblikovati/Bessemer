use super::*;

#[derive(Clone, Copy)]
pub(super) struct ParameterGates {
    pub record_word: bool,
    pub early_word: bool,
    pub second_early: bool,
    pub modern_table: bool,
    pub late_word: bool,
    pub second_late: bool,
}

impl ParameterGates {
    pub fn new(version: SimulationPersistVersion) -> Self {
        let value = version.value();
        Self {
            record_word: value > 700012,
            early_word: value > 700013,
            second_early: value > 700018,
            modern_table: value >= 810026,
            late_word: value > 820015,
            second_late: value > 830009,
        }
    }
}

pub(super) fn bounded_count(value: i32, maximum: usize, label: &str) -> Result<usize> {
    require(
        value >= 0 && value as usize <= maximum,
        label,
        format!("{value}; expected a nonnegative signed-i32 count at most {maximum}"),
    )?;
    Ok(value as usize)
}

pub(super) fn construction_count(value: usize, maximum: usize, label: &str) -> Result<i32> {
    require(
        value <= maximum,
        label,
        format!("{value}; expected a count at most {maximum}"),
    )?;
    i32::try_from(value).map_err(|_| {
        invalid(
            label,
            format!("{value}; expected a nonnegative signed-i32 count"),
        )
    })
}

pub(super) fn checked_extent(count: usize, width: usize, label: &str) -> Result<usize> {
    count.checked_mul(width).ok_or_else(|| {
        invalid(
            label,
            format!("{count} items of {width} bytes; expected a representable complete extent"),
        )
    })
}

pub(super) fn reserve_items<T>(count: usize, label: &str) -> Result<Vec<T>> {
    let mut items = Vec::new();
    items.try_reserve_exact(count).map_err(|error| {
        invalid(
            label,
            format!("{count} complete items, {error}; expected available bounded storage"),
        )
    })?;
    Ok(items)
}

pub(super) fn minimum_record_size(table_count: usize, gates: ParameterGates) -> Result<usize> {
    let fixed = 16 + usize::from(gates.record_word) * 4 + usize::from(gates.early_word) * 8;
    checked_extent(table_count, 8, "ParameterSets record values extent")?.checked_add(fixed).ok_or_else(|| invalid("ParameterSets record extent", format!("{table_count} values plus {fixed} bytes; expected a representable minimum record size")))
}

pub(super) fn require_presence(actual: bool, expected: bool, label: &str) -> Result<()> {
    require(
        actual == expected,
        label,
        format!("present {actual}; expected present {expected} for the explicit version"),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn parameter_extents_reject_overflow_before_allocation() {
        assert!(super::checked_extent(usize::MAX, 4, "test extent").is_err());
        let gates =
            super::ParameterGates::new(super::SimulationPersistVersion::new(830010).unwrap());
        assert!(super::minimum_record_size(usize::MAX, gates).is_err());
        assert!(super::construction_count(usize::MAX, usize::MAX, "test count").is_err());
        assert_eq!(
            super::checked_extent(10001, 8, "test extent").unwrap(),
            80008
        );
    }
}
