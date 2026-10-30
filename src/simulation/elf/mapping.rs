use super::{
    Result, SimulationElfDirectory, SimulationElfGetterAction, SimulationElfMappedRecord,
    SimulationElfMappingCompletion, SimulationElfMappingContext, SimulationElfMappingResult,
    SimulationElfMappingWrite, SimulationElfVisitedRecord,
};
use crate::simulation::invalid;

impl SimulationElfDirectory {
    /// Apply an explicit map with native stop/skip/overwrite rules, e.g. `directory.map_records(&mapping)?`.
    pub fn map_records(
        &self,
        mapping: &SimulationElfMappingContext,
    ) -> Result<SimulationElfMappingResult> {
        let mut slots = destination_slots(mapping.destination_count)?;
        let mut writes = Vec::new();
        for source in self.records.iter() {
            if source.record.getter_action() == SimulationElfGetterAction::Stop {
                break;
            }
            if source.record.getter_action() == SimulationElfGetterAction::Skip {
                continue;
            }
            let Some(target) = mapping.destination(source.record.identifier())? else {
                continue;
            };
            copy_record(&mut slots, &mut writes, source, target)?;
        }
        Ok(publish_mapping(self, slots, writes))
    }
}

fn destination_slots(count: usize) -> Result<Vec<Option<SimulationElfMappedRecord>>> {
    let mut slots = Vec::new();
    slots.try_reserve_exact(count).map_err(|error| invalid("ELF destination allocation", format!("{count} slots: {error}; expected addressable destination storage within supplied budget")))?;
    slots.resize_with(count, || None);
    Ok(slots)
}

fn copy_record(
    slots: &mut [Option<SimulationElfMappedRecord>],
    writes: &mut Vec<SimulationElfMappingWrite>,
    source: &SimulationElfVisitedRecord,
    target: usize,
) -> Result<()> {
    let length = slots.len();
    let slot = slots.get_mut(target).ok_or_else(|| {
        invalid(
            "ELF normalized destination",
            format!("index {target}; expected less than {length} output slots"),
        )
    })?;
    let overwritten_source = slot.as_ref().map(|previous| previous.source.clone());
    *slot = Some(SimulationElfMappedRecord {
        source: source.id.clone(),
        record: source.record.clone(),
    });
    writes.push(SimulationElfMappingWrite {
        source: source.id.clone(),
        identifier: source.record.identifier(),
        destination_index: target,
        overwritten_source,
    });
    Ok(())
}

fn publish_mapping(
    directory: &SimulationElfDirectory,
    slots: Vec<Option<SimulationElfMappedRecord>>,
    writes: Vec<SimulationElfMappingWrite>,
) -> SimulationElfMappingResult {
    let coverage = mapping_coverage(directory, &slots, writes.len());
    let unassigned_indices = slots
        .iter()
        .enumerate()
        .filter_map(|(index, slot)| slot.is_none().then_some(index))
        .collect();
    SimulationElfMappingResult {
        parent_revision: directory.context.revision.clone(),
        selector: directory.selector,
        completion: mapping_completion(writes.is_empty()),
        slots: slots.into(),
        writes: writes.into(),
        unassigned_indices,
        visited_records: directory.records.len(),
        coverage,
        copied_pattern_error_bits: 0,
        diagnostics: super::initial_edit::diagnostics(),
    }
}

fn mapping_coverage(
    directory: &SimulationElfDirectory,
    slots: &[Option<SimulationElfMappedRecord>],
    copied_records: usize,
) -> super::SimulationElfMappingCoverage {
    super::SimulationElfMappingCoverage {
        mode: super::SimulationElfProjectionCheckMode::ExactSelectedRecordCopies,
        selected_source: directory.coverage(),
        copied_records,
        copied_bytes: copied_records * 244,
        destination_slots: slots.len(),
        assigned_slots: slots.iter().filter(|slot| slot.is_some()).count(),
    }
}

fn mapping_completion(empty: bool) -> SimulationElfMappingCompletion {
    if empty {
        return SimulationElfMappingCompletion::CompletedEmpty;
    }
    SimulationElfMappingCompletion::CompletedWrites
}
