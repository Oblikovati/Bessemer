use super::*;
use crate::composer::ComposerEdit;

impl ElectricalArchive {
    /// Replaces one occurrence on a copy; e.g. `let edit = archive.edit_member(&id, &payload)?`.
    pub fn edit_member(
        &self,
        id: &ElectricalMemberId,
        replacement: &[u8],
    ) -> Result<ElectricalArchiveEdit> {
        let edit = self
            .backend
            .edit_member(&id.0, replacement)
            .map_err(electrical_error)?;
        verify_requested_contents(&edit, id, replacement)?;
        let result = Self {
            backend: edit.archive().clone(),
            limits: self.limits,
        };
        let history = member_history(self, &result, id, !edit.is_unchanged())?;
        Ok(ElectricalArchiveEdit {
            source: self.clone(),
            result,
            completion: edit.completion(),
            history,
            ini: None,
        })
    }
    /// Replaces one lexical field value in a selected Archive.ini occurrence; e.g. `archive.edit_ini_value(&id, ordinal, b"value")?`.
    pub fn edit_ini_value(
        &self,
        id: &ElectricalMemberId,
        ordinal: usize,
        replacement: &[u8],
    ) -> Result<ElectricalArchiveEdit> {
        let document = self.archive_ini(id)?.ok_or_else(|| {
            failure(
                FailureClass::Unsupported,
                "Archive.ini",
                "non-ASCII payload; expected ASCII lexical fields",
            )
        })?;
        let ini = document.edit_value(ordinal, replacement)?;
        let mut edit = self.edit_member(id, &ini.result().to_bytes())?;
        edit.ini = Some(ini);
        Ok(edit)
    }
}

fn verify_requested_contents(
    edit: &ComposerEdit,
    id: &ElectricalMemberId,
    expected: &[u8],
) -> Result<()> {
    let result_id = edit.derived_member().map_or(&id.0, |(_, result)| result);
    let archive = edit.archive();
    let actual = archive.read_member(result_id).map_err(electrical_error)?;
    if actual == expected {
        return Ok(());
    }
    let offset = actual
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    Err(failure(
        FailureClass::Integrity,
        "replacement",
        format!(
            "reparsed byte {:?} at {offset}; expected {:?} and identical decoded bytes",
            actual.get(offset),
            expected.get(offset)
        ),
    ))
}

fn member_history(
    source: &ElectricalArchive,
    result: &ElectricalArchive,
    selected: &ElectricalMemberId,
    changed: bool,
) -> Result<Vec<ElectricalMemberHistory>> {
    check_member_count(source, result)?;
    Ok(source
        .members()
        .zip(result.members())
        .map(|(parent, result)| {
            let relation = if changed && parent.id == *selected {
                ElectricalMemberRelation::Derived
            } else {
                ElectricalMemberRelation::Copied
            };
            ElectricalMemberHistory {
                parent: parent.id,
                result: result.id,
                relation,
            }
        })
        .collect())
}

fn check_member_count(source: &ElectricalArchive, result: &ElectricalArchive) -> Result<()> {
    if source.members().len() != result.members().len() {
        return Err(failure(
            FailureClass::Integrity,
            "member history",
            format!(
                "result count {}; expected {}",
                result.members().len(),
                source.members().len()
            ),
        ));
    }
    Ok(())
}

impl ElectricalArchiveEdit {
    /// Borrows the immutable source compression checkpoint; e.g. `let original = edit.source().to_bytes()`.
    pub fn source(&self) -> &ElectricalArchive {
        &self.source
    }
    /// Borrows the complete revalidated result; e.g. `let output = edit.result().to_bytes()`.
    pub fn result(&self) -> &ElectricalArchive {
        &self.result
    }
    /// Reports completed change versus byte no-op; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> ElectricalEditCompletion {
        self.completion
    }
    /// Reports exact carrier checks and achieved error; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> ElectricalCoverage {
        self.result.coverage()
    }
    /// Borrows complete directory-order member parent/result relations; e.g. `let history = edit.history()`.
    pub fn history(&self) -> &[ElectricalMemberHistory] {
        &self.history
    }
    /// Borrows decoded-INI byte history when requested; e.g. `let lexical = edit.ini_edit()`.
    pub fn ini_edit(&self) -> Option<&ElectricalIniEdit> {
        self.ini.as_ref()
    }
    /// Borrows explicit payload/native application exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalDiagnostic] {
        self.result.diagnostics()
    }
}
