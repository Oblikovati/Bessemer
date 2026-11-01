use super::{
    SimulationShellGroup, SimulationShellGroupCheckMode, SimulationShellGroupCompletion,
    SimulationShellGroupCoverage, SimulationShellGroupFieldId, SimulationShellGroupHistory,
    SimulationShellGroupValue, fields,
};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Complete isolated edit retaining original/replacement/result and every ordered field parent.
#[derive(Debug, Clone)]
pub struct SimulationShellGroupEdit {
    source: SimulationShellGroup,
    archive: SimulationShellGroup,
    parent: SimulationShellGroupFieldId,
    result: SimulationShellGroupFieldId,
    replacement: SimulationShellGroupValue,
    coverage: SimulationShellGroupCoverage,
    history: Arc<[SimulationShellGroupHistory]>,
}

impl SimulationShellGroup {
    /// Replace one exact typed wire value, preserving all surrounding bytes; e.g. `group.edit_field(&id, replacement)?`.
    pub fn edit_field(
        &self,
        id: &SimulationShellGroupFieldId,
        replacement: SimulationShellGroupValue,
    ) -> Result<SimulationShellGroupEdit> {
        let original = self.field(id)?;
        require_kind(&original.value, &replacement, &original.path)?;
        let archive = replaced(self, id.ordinal(), &replacement)?;
        let changed = archive.revision != self.revision;
        let result = archive.field_id(id.ordinal())?;
        let history = compose_history(self, &archive, id.ordinal(), changed)?;
        let coverage = edited_coverage(&archive, changed);
        Ok(SimulationShellGroupEdit {
            source: self.clone(),
            archive,
            parent: id.clone(),
            result,
            replacement,
            coverage,
            history,
        })
    }
}

fn kind_name(value: &SimulationShellGroupValue) -> &'static str {
    match value {
        SimulationShellGroupValue::Signed(_) => "Signed",
        SimulationShellGroupValue::DoubleBits(_) => "DoubleBits",
        SimulationShellGroupValue::Text(_) => "MFC CString",
    }
}

fn require_kind(
    original: &SimulationShellGroupValue,
    replacement: &SimulationShellGroupValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!(
            "replacement kind {}; expected {}",
            kind_name(replacement),
            kind_name(original)
        ),
    ))
}

fn replaced(
    source: &SimulationShellGroup,
    ordinal: usize,
    replacement: &SimulationShellGroupValue,
) -> Result<SimulationShellGroup> {
    let (start, end) = field_span(source, ordinal)?;
    let bytes = fields::value_bytes(replacement, source.limits.maximum_bytes)?;
    // CString equality omits prefix width; exact wire equality controls no-ops (#137).
    if source.parent[start..end] == bytes {
        return Ok(source.clone());
    }
    let mut fields = source.fields.to_vec();
    let count = fields.len();
    let field = fields.get_mut(ordinal).ok_or_else(|| {
        invalid(
            "ShellManagerGroup replacement",
            format!("ordinal {ordinal}; expected < {count}"),
        )
    })?;
    field.value = replacement.clone();
    let result =
        SimulationShellGroup::from_fields(fields, source.version, source.profile, source.limits)?;
    verify_preservation(source, &result, ordinal)?;
    Ok(result)
}

fn field_span(group: &SimulationShellGroup, ordinal: usize) -> Result<(usize, usize)> {
    group.offsets.get(ordinal).copied().ok_or_else(|| {
        invalid(
            "ShellManagerGroup field span",
            format!("ordinal {ordinal}; expected < {}", group.offsets.len()),
        )
    })
}

fn verify_preservation(
    source: &SimulationShellGroup,
    result: &SimulationShellGroup,
    ordinal: usize,
) -> Result<()> {
    let (start, end) = field_span(source, ordinal)?;
    let (next_start, next_end) = field_span(result, ordinal)?;
    if start == next_start
        && source.parent[..start] == result.parent[..next_start]
        && source.parent[end..] == result.parent[next_end..]
    {
        return Ok(());
    }
    Err(invalid(
        "ShellManagerGroup edit preservation",
        format!(
            "source {start}..{end}, result {next_start}..{next_end}; expected identical prefix and suffix"
        ),
    ))
}

fn compose_history(
    source: &SimulationShellGroup,
    result: &SimulationShellGroup,
    ordinal: usize,
    changed: bool,
) -> Result<Arc<[SimulationShellGroupHistory]>> {
    let mut rows = Vec::new();
    for index in 0..source.fields.len() {
        rows.push(super::super::load_identity::outer_field_relation(
            source.field_id(index)?,
            result.field_id(index)?,
            changed && ordinal == index,
        ));
    }
    Ok(rows.into())
}

fn edited_coverage(group: &SimulationShellGroup, changed: bool) -> SimulationShellGroupCoverage {
    let mut coverage = group.coverage();
    coverage.check_mode = SimulationShellGroupCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationShellGroupCompletion::Changed
    } else {
        SimulationShellGroupCompletion::Unchanged
    };
    coverage
}

impl SimulationShellGroupEdit {
    /// Borrow the complete retained original; e.g. `edit.source().to_bytes()`.
    pub fn source(&self) -> &SimulationShellGroup {
        &self.source
    }
    /// Borrow the complete revalidated result; e.g. `edit.archive().fields()`.
    pub fn archive(&self) -> &SimulationShellGroup {
        &self.archive
    }
    /// Return the exact requested original identity; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationShellGroupFieldId {
        self.parent.clone()
    }
    /// Return the exact resulting identity, preserving no-op revisions; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationShellGroupFieldId {
        self.result.clone()
    }
    /// Borrow the complete caller replacement; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationShellGroupValue {
        &self.replacement
    }
    /// Return exact completion, scope, diagnostics and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationShellGroupCoverage {
        self.coverage
    }
    /// Borrow all ordered source/replacement field relations; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationShellGroupHistory] {
        &self.history
    }
}
