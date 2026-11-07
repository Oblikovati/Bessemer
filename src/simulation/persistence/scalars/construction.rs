use super::*;

impl SimulationPersistScalarArchive {
    /// Construct exact ordered i32 framing, e.g. `SimulationPersistScalarArchive::from_integer_values(&[-1, 0], 10000)?`.
    pub fn from_integer_values(values: &[i32], maximum_entries: usize) -> Result<Self> {
        let count = construction_count(values.len(), maximum_entries)?;
        let mut bytes = super::super::reserve_bytes(codec::frame_extent(4, values.len(), 4)?)?;
        bytes.extend_from_slice(&count.to_le_bytes());
        bytes.extend(values.iter().flat_map(|value| value.to_le_bytes()));
        Self::from_bytes(
            &bytes,
            SimulationPersistScalarProfile::CountedI32,
            maximum_entries,
        )
    }

    /// Construct exact ordered keys with metadata-only capacity, e.g. `SimulationPersistScalarArchive::from_unique_keys(&[-1, 1 << 40], 1000, 10000)?`.
    pub fn from_unique_keys(
        keys: &[i64],
        allocation_capacity: i32,
        maximum_entries: usize,
    ) -> Result<Self> {
        let count = construction_count(keys.len(), maximum_entries)?;
        require(
            allocation_capacity >= count,
            "UniqueIds construction capacity",
            format!(
                "{allocation_capacity} for {count} used keys; expected signed-i32 capacity at least {count}"
            ),
        )?;
        let mut bytes = super::super::reserve_bytes(codec::frame_extent(8, keys.len(), 8)?)?;
        bytes.extend_from_slice(&count.to_le_bytes());
        bytes.extend_from_slice(&allocation_capacity.to_le_bytes());
        bytes.extend(keys.iter().flat_map(|key| key.to_le_bytes()));
        Self::from_bytes(
            &bytes,
            SimulationPersistScalarProfile::UniqueIdsI64,
            maximum_entries,
        )
    }
}

fn construction_count(count: usize, maximum: usize) -> Result<i32> {
    require(
        count <= maximum,
        "scalar construction count",
        format!("{count}; expected at most {maximum} entries"),
    )?;
    i32::try_from(count).map_err(|_| {
        invalid(
            "scalar construction count",
            format!("{count}; expected a nonnegative signed-i32 entry count"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::construction_count;

    #[test]
    fn scalar_constructor_count_checks_limit_and_signed_header_width_before_allocation() {
        assert_eq!(construction_count(0, 0).unwrap(), 0);
        assert_eq!(construction_count(3, 3).unwrap(), 3);
        assert!(construction_count(3, 2).is_err());
        assert!(construction_count(i32::MAX as usize + 1, usize::MAX).is_err());
    }
}
