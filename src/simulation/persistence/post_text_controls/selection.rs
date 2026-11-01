use super::super::SimulationPostStyleText;
use super::{
    SimulationPostTextAssignment, post_text_budget, post_text_completion, post_text_coverage,
    post_text_diagnostics, required_post_runtime_text, types::*,
};
use crate::{BessemerError, FailureClass, Result};

/// Immutable known local selections with controls, runtime parents and optional explicit assignment history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostTextPlan {
    controls: SimulationPostTextControls,
    runtime: SimulationPostTextRuntime,
    primary: SimulationPostTextSelection,
    lines: [SimulationPostTextSelection; 7],
    assignment: Option<SimulationPostTextAssignment>,
    coverage: SimulationPostTextCoverage,
}

impl SimulationPostTextControls {
    /// Select current runtime strings without reapplying stored assignment; e.g. `controls.plan_selection(&runtime, 4096)?`.
    pub fn plan_selection(
        &self,
        runtime: &SimulationPostTextRuntime,
        maximum_code_units: usize,
    ) -> Result<SimulationPostTextPlan> {
        let empty = SimulationPostStyleText::from_payload(&[], 0)?;
        let primary =
            selected_post_primary(self.primary_enabled_raw, runtime.primary.as_ref(), &empty)?;
        let lines = selected_post_lines(self.seven_lines_enabled_raw, &runtime.lines, &empty)?;
        let coverage = selected_post_coverage(self, &primary, &lines, maximum_code_units)?;
        Ok(SimulationPostTextPlan {
            controls: self.clone(),
            runtime: runtime.clone(),
            primary,
            lines,
            assignment: None,
            coverage,
        })
    }

    /// Explicitly compose known load assignment with selection and preserve both histories; e.g. `controls.plan_after_assignment(&prior_runtime, 4096)?`.
    pub fn plan_after_assignment(
        &self,
        previous_runtime: &SimulationPostTextRuntime,
        maximum_code_units: usize,
    ) -> Result<SimulationPostTextPlan> {
        let assignment =
            self.assign_primary(previous_runtime.primary.as_ref(), maximum_code_units)?;
        let mut runtime = previous_runtime.clone();
        runtime.primary = Some(assignment.primary_after().clone());
        let mut plan = self.plan_selection(&runtime, maximum_code_units)?;
        plan.assignment = Some(assignment);
        Ok(plan)
    }
}

fn selected_post_primary(
    enabled: i32,
    current: Option<&SimulationPostStyleText>,
    empty: &SimulationPostStyleText,
) -> Result<SimulationPostTextSelection> {
    if enabled == 0 {
        return Ok(SimulationPostTextSelection {
            origin: SimulationPostTextOrigin::EmptyLiteral,
            text: empty.clone(),
        });
    }
    Ok(SimulationPostTextSelection {
        origin: SimulationPostTextOrigin::RuntimePrimary,
        text: required_post_runtime_text(
            current,
            "post text selection",
            "runtime primary CString",
        )?
        .clone(),
    })
}

fn selected_post_lines(
    enabled: i32,
    runtime: &[Option<SimulationPostStyleText>; 7],
    empty: &SimulationPostStyleText,
) -> Result<[SimulationPostTextSelection; 7]> {
    let mut lines = std::array::from_fn(|_| SimulationPostTextSelection {
        origin: SimulationPostTextOrigin::EmptyLiteral,
        text: empty.clone(),
    });
    if enabled == 0 {
        return Ok(lines);
    }
    for (index, source) in runtime.iter().enumerate() {
        let text = required_post_runtime_text(
            source.as_ref(),
            "post text selection",
            &format!("runtime line[{index}] CString"),
        )?;
        lines[index] = SimulationPostTextSelection {
            origin: SimulationPostTextOrigin::RuntimeLine(index as u8),
            text: text.clone(),
        };
    }
    Ok(lines)
}

fn selected_post_coverage(
    controls: &SimulationPostTextControls,
    primary: &SimulationPostTextSelection,
    lines: &[SimulationPostTextSelection; 7],
    maximum: usize,
) -> Result<SimulationPostTextCoverage> {
    let count = selected_post_units(primary, lines, maximum)?;
    let required = usize::from(controls.primary_enabled_raw != 0)
        + 7 * usize::from(controls.seven_lines_enabled_raw != 0);
    Ok(post_text_coverage(
        controls.stored_text.character_count(),
        count,
        required,
        maximum,
    ))
}

fn selected_post_units(
    primary: &SimulationPostTextSelection,
    lines: &[SimulationPostTextSelection; 7],
    maximum: usize,
) -> Result<usize> {
    std::iter::once(primary)
        .chain(lines.iter())
        .try_fold(0, |total, selected| {
            checked_post_text_units(total, selected.text.character_count(), maximum)
        })
}

fn checked_post_text_units(current: usize, next: usize, maximum: usize) -> Result<usize> {
    let total = current.checked_add(next).ok_or_else(|| {
        BessemerError::new(
            "post text selection",
            "code-unit total",
            FailureClass::Invalid,
            format!("{current} + {next} code units; expected a representable usize total"),
        )
    })?;
    post_text_budget("post text selection", total, maximum)?;
    Ok(total)
}

impl SimulationPostTextPlan {
    /// Inspect original controls and complete stored payload; e.g. `plan.source_controls()`.
    pub fn source_controls(&self) -> &SimulationPostTextControls {
        &self.controls
    }
    /// Inspect the actual current-state parents used for selection; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> &SimulationPostTextRuntime {
        &self.runtime
    }
    /// Inspect the primary selection and source; e.g. `plan.primary_selection()`.
    pub fn primary_selection(&self) -> &SimulationPostTextSelection {
        &self.primary
    }
    /// Inspect every line position, including known empties; e.g. `plan.line_selections()`.
    pub fn line_selections(&self) -> &[SimulationPostTextSelection; 7] {
        &self.lines
    }
    /// Inspect explicitly composed assignment history, if requested; e.g. `plan.assignment()`.
    pub fn assignment(&self) -> Option<&SimulationPostTextAssignment> {
        self.assignment.as_ref()
    }
    /// Inspect exact known units and dependency coverage; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationPostTextCoverage {
        self.coverage
    }
    /// Distinguish completed empty selections from completed text; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationPostTextCompletion {
        post_text_completion(self.coverage.result_code_units)
    }
    /// Keep native, Unicode and layout exclusions with every result; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationPostTextDiagnostic; 4] {
        post_text_diagnostics()
    }
}

#[cfg(test)]
mod total_tests {
    #[test]
    fn unit_total_overflow_is_typed_without_partial_output() {
        let error = super::checked_post_text_units(usize::MAX, 1, usize::MAX).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("expected a representable usize total"));
    }
}
