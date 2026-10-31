use super::*;
use std::sync::Arc;

pub(super) struct MarkBuilder {
    pub marks: Vec<Option<u32>>,
    pub writes: Vec<SimulationBodyNodeWrite>,
    pub flag: i32,
    pub resolved: bool,
    pub visited_elements: usize,
    pub selected_elements: usize,
    pub visited_references: usize,
}

impl SimulationBodyQueryMesh {
    /// Project supplied-context marks on an isolated copy, e.g. `mesh.project_body_nodes(context, None)?`.
    pub fn project_body_nodes(
        &self,
        context: SimulationBodyQueryContext,
        initial: Option<&[Option<u32>]>,
    ) -> Result<SimulationBodyQueryResult> {
        let initial_marks = capture_initial(initial, self.nodes.len() / 52)?;
        let (flag, resolved) = context.initial_flag();
        let mut builder = MarkBuilder {
            marks: copy_marks(&initial_marks)?,
            writes: Vec::new(),
            flag,
            resolved,
            visited_elements: 0,
            selected_elements: 0,
            visited_references: 0,
        };
        if context.mode == 1 {
            visits::visit_active_nodes(self, context, &mut builder)?;
        } else {
            visits::visit_elements(self, context, &mut builder)?;
        }
        Ok(publish_marks(self, context, initial_marks, builder))
    }
}

impl MarkBuilder {
    pub(super) fn write(
        &mut self,
        node_slot: usize,
        source: SimulationBodyWriteSource,
        pattern: u32,
        maximum: usize,
    ) -> Result<()> {
        reserve_write(&mut self.writes, node_slot, maximum)?;
        let previous_pattern = self.marks[node_slot];
        self.marks[node_slot] = Some(pattern);
        self.writes.push(SimulationBodyNodeWrite {
            node_slot,
            source,
            previous_pattern,
            pattern,
        });
        Ok(())
    }

    pub(super) fn resolve_family(&mut self, mode: i32, family: u8) {
        if self.resolved || !(2..=5).contains(&mode) {
            return;
        }
        self.flag = match family {
            4 => 1,
            8 | 3 => 2,
            2 => 4,
            _ => self.flag,
        };
        self.resolved = true;
    }
}

fn capture_initial(initial: Option<&[Option<u32>]>, count: usize) -> Result<Arc<[Option<u32>]>> {
    if let Some(initial) = initial {
        require(
            initial.len() == count,
            "body query initial marks",
            format!(
                "{} values; expected exactly {count} optional u32 values",
                initial.len()
            ),
        )?;
        return Ok(copy_marks(initial)?.into());
    }
    let mut marks = Vec::new();
    marks.try_reserve_exact(count).map_err(|error| invalid("body query marks allocation", format!("{count} marks: {error}; expected addressable storage within the supplied node budget")))?;
    marks.resize(count, None);
    Ok(marks.into())
}

fn publish_marks(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    initial_marks: Arc<[Option<u32>]>,
    builder: MarkBuilder,
) -> SimulationBodyQueryResult {
    let coverage = mark_coverage(context, &builder);
    SimulationBodyQueryResult {
        source: mesh.clone(),
        context,
        initial_marks,
        marks: builder.marks.into(),
        completion: if builder.writes.is_empty() {
            SimulationBodyQueryCompletion::CompletedEmpty
        } else {
            SimulationBodyQueryCompletion::CompletedWrites
        },
        writes: builder.writes.into(),
        coverage,
        native_return_value: 0,
        type_flag: builder.flag,
        type_flag_resolved: builder.resolved,
        callback_initial_i16: (context.mode != 1).then_some(context.body.pattern() as u16 as i16),
        mark_pattern_error_bits: 0,
        diagnostics: query_diagnostics(),
    }
}

fn mark_coverage(
    context: SimulationBodyQueryContext,
    builder: &MarkBuilder,
) -> SimulationBodyQueryCoverage {
    SimulationBodyQueryCoverage {
        mode: if context.mode == 1 {
            SimulationBodyQueryCheckMode::ExactAllNodeActivity
        } else {
            SimulationBodyQueryCheckMode::ExactSelectedReferencesAndNodeActivity
        },
        visited_elements: builder.visited_elements,
        selected_elements: builder.selected_elements,
        visited_references: builder.visited_references,
        write_visits: builder.writes.len(),
        unknown_slots: builder.marks.iter().filter(|value| value.is_none()).count(),
    }
}

pub(super) fn query_diagnostics() -> [SimulationBodyQueryDiagnostic; 4] {
    [
        SimulationBodyQueryDiagnostic::CadOwnershipUnassigned,
        SimulationBodyQueryDiagnostic::NativeCallbackNotExecuted,
        SimulationBodyQueryDiagnostic::NativeRuntimeNotExecuted,
        SimulationBodyQueryDiagnostic::GeometryNotValidated,
    ]
}

fn copy_marks(initial: &[Option<u32>]) -> Result<Vec<Option<u32>>> {
    let mut marks = Vec::new();
    marks.try_reserve_exact(initial.len()).map_err(|error| {
        invalid(
            "body query marks copy",
            format!(
                "{} marks: {error}; expected addressable isolated mark storage",
                initial.len()
            ),
        )
    })?;
    marks.extend_from_slice(initial);
    Ok(marks)
}

fn reserve_write(
    writes: &mut Vec<SimulationBodyNodeWrite>,
    node_slot: usize,
    maximum: usize,
) -> Result<()> {
    require(
        writes.len() < maximum,
        "body query write budget",
        format!(
            "write visit {} at node {node_slot}; expected fewer than {maximum} previous visits",
            writes.len() + 1
        ),
    )?;
    writes.try_reserve(1).map_err(|error| {
        invalid(
            "body query write allocation",
            format!(
                "{} previous writes: {error}; expected addressable ordered history",
                writes.len()
            ),
        )
    })
}
