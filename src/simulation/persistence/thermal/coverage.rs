use super::*;

impl SimulationThermalCollection {
    /// Return complete exact coverage with explicit semantic exclusions; e.g. `collection.coverage().completion`.
    pub fn coverage(&self) -> SimulationThermalCoverage {
        let (embedded_archives, embedded_fields) = nested_counts(self);
        SimulationThermalCoverage {
            kind: self.kind,
            version: self.version,
            limits: self.limits,
            completion: read_completion(self.record_count),
            check_mode: SimulationThermalCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            record_count: self.record_count,
            embedded_archives,
            embedded_fields,
            diagnostics: diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}

fn nested_counts(collection: &SimulationThermalCollection) -> (usize, usize) {
    collection
        .fields
        .iter()
        .fold((0, 0), |(roots, fields), field| match &field.value {
            SimulationThermalValue::Embedded(archive) => {
                (roots + 1, fields + archive.fields().len())
            }
            _ => (roots, fields),
        })
}

fn read_completion(records: usize) -> SimulationThermalCompletion {
    if records == 0 {
        return SimulationThermalCompletion::CompletedEmptyRecords;
    }
    SimulationThermalCompletion::CompletedRecords
}

fn diagnostics() -> [SimulationThermalDiagnostic; 4] {
    [
        SimulationThermalDiagnostic::NativeApplicationNotExecuted,
        SimulationThermalDiagnostic::RuntimeVersionNotInferred,
        SimulationThermalDiagnostic::PhysicalSemanticsUnassigned,
        SimulationThermalDiagnostic::GeometryNotChecked,
    ]
}
