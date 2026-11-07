use super::{SimulationLegacyPlot, SimulationLegacyPlotValue};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Original fixed-buffer member, not its packed wire offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegacyPlotWideMember {
    /// Member +30, 128 code units.
    Buffer30,
    /// Member +13C, 128 code units.
    Buffer13C,
    /// Member +23C, 128 code units.
    Buffer23C,
    /// Member +33C, 128 code units.
    Buffer33C,
    /// Member +454, 64 code units.
    Buffer454,
}

impl SimulationLegacyPlotWideMember {
    fn path(self) -> &'static str {
        match self {
            Self::Buffer30 => "legacy_plot.member_30",
            Self::Buffer13C => "legacy_plot.member_13c",
            Self::Buffer23C => "legacy_plot.member_23c",
            Self::Buffer33C => "legacy_plot.member_33c",
            Self::Buffer454 => "legacy_plot.member_454",
        }
    }
    /// Return the native bounded scan limit; e.g. `member.code_unit_limit()`.
    pub fn code_unit_limit(self) -> usize {
        if self == Self::Buffer454 { 64 } else { 128 }
    }
}

/// Owned bounded raw-prefix view; no native CString trimming or Unicode normalization is modeled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegacyPlotWideView {
    member: SimulationLegacyPlotWideMember,
    raw: Arc<[u8]>,
    start: usize,
    prefix_units: usize,
}

impl SimulationLegacyPlot {
    /// Own one complete buffer and its pre-NUL prefix only from version700004; e.g. `plot.fixed_wide_view(member)?`.
    pub fn fixed_wide_view(
        &self,
        member: SimulationLegacyPlotWideMember,
    ) -> Result<Option<SimulationLegacyPlotWideView>> {
        if self.version.value() < 700004 {
            return Ok(None);
        }
        let (raw, start) = self.fixed_wide_field(member)?;
        let prefix_units = raw
            .as_chunks::<2>()
            .0
            .iter()
            .position(|unit| *unit == [0, 0])
            .unwrap_or(member.code_unit_limit());
        Ok(Some(SimulationLegacyPlotWideView {
            member,
            raw,
            start,
            prefix_units,
        }))
    }

    fn fixed_wide_field(
        &self,
        member: SimulationLegacyPlotWideMember,
    ) -> Result<(Arc<[u8]>, usize)> {
        let ordinal = self
            .fields
            .iter()
            .position(|field| field.path.as_ref() == member.path())
            .ok_or_else(|| {
                invalid(
                    member.path(),
                    "absent field; expected the complete fixed historical buffer",
                )
            })?;
        let SimulationLegacyPlotValue::RawBytes(raw) = &self.fields[ordinal].value else {
            return Err(invalid(
                member.path(),
                "non-buffer field; expected complete fixed raw bytes",
            ));
        };
        Ok((raw.clone(), self.offsets[ordinal].0))
    }
}

impl SimulationLegacyPlotWideView {
    /// Return the original fixed member identity; e.g. `view.member()`.
    pub fn member(&self) -> SimulationLegacyPlotWideMember {
        self.member
    }
    /// Borrow every original buffer byte, including its suffix; e.g. `view.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw
    }
    /// Return the original packed span without counting member gaps; e.g. `view.source_offsets()`.
    pub fn source_offsets(&self) -> (usize, usize) {
        (self.start, self.start + self.raw.len())
    }
    /// Return the bounded pre-NUL unit count; e.g. `view.prefix_code_unit_count()`.
    pub fn prefix_code_unit_count(&self) -> usize {
        self.prefix_units
    }
    /// Distinguish a bounded unterminated buffer from a found zero unit; e.g. `view.terminator_present()`.
    pub fn terminator_present(&self) -> bool {
        self.prefix_units < self.member.code_unit_limit()
    }
    /// Borrow raw code units before the first zero, without trimming; e.g. `view.prefix_bytes()`.
    pub fn prefix_bytes(&self) -> &[u8] {
        &self.raw[..self.prefix_units * 2]
    }
    /// Borrow the entire suffix beginning at the first zero, or empty if absent; e.g. `view.remainder_bytes()`.
    pub fn remainder_bytes(&self) -> &[u8] {
        &self.raw[self.prefix_units * 2..]
    }
}
