use super::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelEditCheckMode, ModelFieldValue,
    ModelObjectRecord, PartConfigurationArchive,
};

pub(super) const DIAGNOSTICS: &[ModelArchiveDiagnostic] = &[
    ModelArchiveDiagnostic::GeometryOutsideScope,
    ModelArchiveDiagnostic::ExternalDocumentsNotChecked,
];

impl PartConfigurationArchive {
    /// Returns exact stored-schema coverage for the immutable published snapshot.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) {
    /// let checked_bytes = archive.coverage().wire_bytes;
    /// # }
    /// ```
    pub fn coverage(&self) -> ModelArchiveCoverage {
        let mut coverage = ModelArchiveCoverage {
            mode: ModelEditCheckMode::ExactStoredArchive,
            wire_bytes: self.parsed.wire_bytes(),
            typed_fields: self.fields().len(),
            new_objects: 0,
            object_aliases: 0,
            imported_slots: self.parsed.lightweight().imports.slots().len(),
        };
        for field in self.fields() {
            charge_object(&mut coverage, &field.value);
        }
        coverage
    }

    /// Borrows explicit exclusions; successful wire checks do not imply valid geometry.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) {
    /// let excluded_checks = archive.diagnostics();
    /// # }
    /// ```
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        DIAGNOSTICS
    }
}

pub(super) fn charge_object(coverage: &mut ModelArchiveCoverage, value: &ModelFieldValue) {
    let ModelFieldValue::ObjectTag(tag) = value else {
        return;
    };
    match tag.record {
        ModelObjectRecord::New { .. } => coverage.new_objects += 1,
        ModelObjectRecord::Reference { .. } => coverage.object_aliases += 1,
        ModelObjectRecord::Null => {}
    }
}
