use super::{
    DisplayArchive, DisplayArchiveContents, DisplayArchiveEdit, DisplayArchiveLimits,
    DisplayEditCheckMode, DisplayEditCompletion, DisplayEditCoverage, DisplayEditDiagnostic,
    DisplayEditFidelity, DisplayFieldChange, DisplayFieldValue, DisplayObjectPointer,
    archive_edit_history::edit_history,
};
use crate::Result;

impl DisplayArchive {
    /// Validates a whole stored-archive edit and returns its exact coverage and structural history.
    /// Geometry and external-block checks are explicitly excluded from this operation.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::display_lists::DisplayArchive, limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
    /// let outcome = source.edit_with_history(source.contents().clone(), limits)?;
    /// assert!(outcome.changes().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_with_history(
        &self,
        contents: DisplayArchiveContents,
        limits: DisplayArchiveLimits,
    ) -> Result<DisplayArchiveEdit> {
        let archive = self.with_contents(contents, limits)?;
        let coverage = edit_coverage(&archive, archive.to_bytes()?.len());
        let (changes, unchanged_fields) = edit_history(self, &archive);
        let completion = edit_completion(self, &archive);
        let diagnostics = edit_diagnostics(coverage);
        Ok(DisplayArchiveEdit {
            archive,
            completion,
            coverage,
            diagnostics,
            changes,
            unchanged_fields,
        })
    }
}

fn edit_coverage(archive: &DisplayArchive, wire_bytes: usize) -> DisplayEditCoverage {
    let mut result = DisplayEditCoverage {
        mode: DisplayEditCheckMode::ExactStoredArchive,
        wire_bytes,
        typed_fields: archive.contents().fields.len(),
        loaded_objects: 0,
        object_aliases: 0,
        unchecked_external_faces: 0,
    };
    for field in &archive.contents().fields {
        charge_edit_field(&mut result, &field.value);
    }
    result
}

fn edit_completion(source: &DisplayArchive, result: &DisplayArchive) -> DisplayEditCompletion {
    if source.contents() == result.contents() {
        return DisplayEditCompletion::Unchanged;
    }
    DisplayEditCompletion::Changed
}

fn charge_edit_field(result: &mut DisplayEditCoverage, value: &DisplayFieldValue) {
    match value {
        DisplayFieldValue::ObjectTag(tag) => match tag.target {
            DisplayObjectPointer::New(_) => result.loaded_objects += 1,
            DisplayObjectPointer::Reference(_) => result.object_aliases += 1,
            DisplayObjectPointer::Null => {}
        },
        DisplayFieldValue::ExternalFacet { .. } => result.unchecked_external_faces += 1,
        _ => {}
    }
}

fn edit_diagnostics(coverage: DisplayEditCoverage) -> Vec<DisplayEditDiagnostic> {
    let mut diagnostics = vec![DisplayEditDiagnostic::GeometryOutsideScope];
    if coverage.unchecked_external_faces != 0 {
        diagnostics.push(DisplayEditDiagnostic::ExternalFaceLinksNotChecked);
    }
    diagnostics
}

impl DisplayArchiveEdit {
    /// Borrows the fully validated result; failed edits never publish this report.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) -> bessemer::Result<()> {
    /// let bytes = edit.archive().to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn archive(&self) -> &DisplayArchive {
        &self.archive
    }

    /// Distinguishes a completed unchanged edit from a completed changed edit.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// let completion = edit.completion();
    /// # }
    /// ```
    pub fn completion(&self) -> DisplayEditCompletion {
        self.completion
    }

    /// Returns exact whole-stream schema and shared-object validation counts.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// let checked_fields = edit.coverage().typed_fields;
    /// # }
    /// ```
    pub fn coverage(&self) -> DisplayEditCoverage {
        self.coverage
    }

    /// Returns achieved stored-field equality, without assigning a geometric error bound.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// let fidelity = edit.fidelity();
    /// # }
    /// ```
    pub fn fidelity(&self) -> DisplayEditFidelity {
        DisplayEditFidelity::ExactStoredFields
    }

    /// Borrows explicit exclusions from this operation's validation scope.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// let exclusions = edit.diagnostics();
    /// # }
    /// ```
    pub fn diagnostics(&self) -> &[DisplayEditDiagnostic] {
        &self.diagnostics
    }

    /// Borrows changed fields in source-path order followed by created result-path order.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// for change in edit.changes() { let path = change.path(); }
    /// # }
    /// ```
    pub fn changes(&self) -> &[DisplayFieldChange] {
        &self.changes
    }

    /// Counts fields equal at the same structural path; those fields have no change relation.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::display_lists::DisplayArchiveEdit) {
    /// let retained = edit.unchanged_fields();
    /// # }
    /// ```
    pub fn unchanged_fields(&self) -> usize {
        self.unchanged_fields
    }
}
