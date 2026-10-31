use super::{Result, require, wire};

/// Native selected-getter action of the signed stored identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfGetterAction {
    /// Negative identifier: stop after reading this complete record.
    Stop,
    /// Zero identifier: skip this complete record.
    Skip,
    /// Positive identifier: require an explicit external source-to-destination map.
    RequiresExternalMapping,
}

/// Complete immutable 244-byte record; scalar component meanings remain unassigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfScalarRecord {
    identifier: i32,
    patterns: [u32; 60],
}

impl SimulationElfScalarRecord {
    /// Read the identifier and every exact scalar pattern, e.g. `SimulationElfScalarRecord::from_bytes(record)?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let words = wire::patterns::<61>(bytes, "ELF scalar record")?;
        let mut patterns = [0; 60];
        patterns.copy_from_slice(&words[1..]);
        Ok(Self {
            identifier: words[0] as i32,
            patterns,
        })
    }

    /// Reproduce resolved store-stage groups with positive-zero omitted cells, e.g. `SimulationElfScalarRecord::from_store_groups(7, &[&[bits]])?`.
    pub fn from_store_groups(identifier: i32, groups: &[&[u32]]) -> Result<Self> {
        validate_group_count(groups.len())?;
        let mut patterns = [0; 60];
        for (index, group) in groups.iter().enumerate() {
            require(
                group.len() <= 6,
                "ELF store group",
                format!(
                    "group {index} has {} patterns; expected at most six",
                    group.len()
                ),
            )?;
            patterns[index * 6..index * 6 + group.len()].copy_from_slice(group);
        }
        Ok(Self {
            identifier,
            patterns,
        })
    }

    /// Complete exact stored record, e.g. `record.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(244);
        wire::append_patterns(&mut bytes, &[self.identifier as u32]);
        wire::append_patterns(&mut bytes, &self.patterns);
        bytes
    }

    /// Exact signed identifier without external-map interpretation, e.g. `record.identifier()`.
    pub fn identifier(&self) -> i32 {
        self.identifier
    }

    /// All sixty binary32 patterns, including signed zero and NaNs, e.g. `record.patterns()`.
    pub fn patterns(&self) -> &[u32; 60] {
        &self.patterns
    }

    /// Ten physical six-slot groups, e.g. `record.groups()[0][0]`.
    pub fn groups(&self) -> [[u32; 6]; 10] {
        std::array::from_fn(|group| {
            std::array::from_fn(|component| self.patterns[group * 6 + component])
        })
    }

    /// Native stop/skip/map branch, e.g. `record.getter_action()`.
    pub fn getter_action(&self) -> SimulationElfGetterAction {
        if self.identifier < 0 {
            return SimulationElfGetterAction::Stop;
        }
        if self.identifier == 0 {
            return SimulationElfGetterAction::Skip;
        }
        SimulationElfGetterAction::RequiresExternalMapping
    }
}

fn validate_group_count(count: usize) -> Result<()> {
    require(
        count <= 10,
        "ELF store groups",
        format!(
            "{} groups; expected at most ten groups of at most six patterns",
            count
        ),
    )
}
