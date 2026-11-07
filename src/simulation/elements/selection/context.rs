use super::{Result, invalid, require};
use std::ops::Range;

/// Explicit caller body-ID bits, distinct from a stored u16 or CAD identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodySelectionId(u32);

impl SimulationBodySelectionId {
    /// Normalize full caller bits once, e.g. `SimulationBodySelectionId::from_pattern((-1i32) as u32)`.
    pub fn from_pattern(pattern: u32) -> Self {
        Self(pattern)
    }

    /// Exact full caller bits, e.g. `body.pattern()`; negative patterns do not match stored 65535.
    pub fn pattern(self) -> u32 {
        self.0
    }
}

/// Normalized supplied callback/mode/range context; no historical session is inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyQueryContext {
    pub(super) body: SimulationBodySelectionId,
    pub(super) mode: i32,
    pub(super) callback: Option<i16>,
    pub(super) range: Option<(i32, i32)>,
}

impl SimulationBodyQueryContext {
    /// Normalize the inspected branch once, e.g. `SimulationBodyQueryContext::new(body, 3, Some(-1), Some((0, 9)))?`.
    pub fn new(
        body: SimulationBodySelectionId,
        mode: i32,
        callback: Option<i16>,
        range: Option<(i32, i32)>,
    ) -> Result<Self> {
        if mode == 1 {
            return Ok(Self::active_nodes_context(body));
        }
        require(
            callback.is_some(),
            "body query callback",
            format!(
                "mode {mode}, callback {callback:?}; expected an explicit signed-i16 result outside mode 1"
            ),
        )?;
        let range = normalize_range(mode, range)?;
        Ok(Self {
            body,
            mode,
            callback,
            range,
        })
    }

    /// Original full caller ID, e.g. `context.body_id()`.
    pub fn body_id(self) -> SimulationBodySelectionId {
        self.body
    }

    /// Original signed mode, e.g. `context.mode()`; unknown modes retain fallback behavior.
    pub fn mode(self) -> i32 {
        self.mode
    }

    /// Applied supplied result, e.g. `context.callback_result()`; mode 1 bypasses it.
    pub fn callback_result(self) -> Option<i16> {
        self.callback
    }

    fn active_nodes_context(body: SimulationBodySelectionId) -> Self {
        Self {
            body,
            mode: 1,
            callback: None,
            range: None,
        }
    }

    pub(super) fn indices(self, count: usize) -> Result<Range<usize>> {
        let Some((start, end)) = self.range else {
            return Ok(0..count);
        };
        if start > end {
            return Ok(0..0);
        }
        require(
            start >= 0 && (end as usize) < count,
            "body query element range",
            format!(
                "inclusive {start}..={end} for {count} elements; expected an empty range or in-bounds nonnegative endpoints"
            ),
        )?;
        Ok(start as usize..end as usize + 1)
    }

    pub(super) fn initial_flag(self) -> (i32, bool) {
        if self.callback.is_some_and(|value| value != -1) {
            return (16, true);
        }
        match self.mode {
            0 => (1, true),
            1 | 2 | 5 => (2, true),
            4 => (4, true),
            6 => (8, true),
            _ => (-1, false),
        }
    }
}

fn normalize_range(mode: i32, range: Option<(i32, i32)>) -> Result<Option<(i32, i32)>> {
    if !(2..=5).contains(&mode) {
        return Ok(None);
    }
    let (start, end) = range.ok_or_else(|| {
        invalid(
            "body query element range",
            format!("mode {mode}, missing endpoints; expected an explicit safe inclusive range"),
        )
    })?;
    require(
        start != i32::MAX && end != i32::MAX,
        "body query element range",
        format!("inclusive {start}..={end}; expected signed endpoints other than i32::MAX"),
    )?;
    Ok(Some((start, end)))
}
