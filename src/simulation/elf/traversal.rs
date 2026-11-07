use super::{
    Result, SimulationElfDirectory, SimulationElfDirectoryCompletion, SimulationElfGetterAction,
    SimulationElfIndexContext, SimulationElfRecordId, SimulationElfSelector,
    SimulationElfVisitedRecord, require,
};
use crate::records::dword;

pub(super) fn read(
    context: &SimulationElfIndexContext,
    selector: SimulationElfSelector,
) -> Result<SimulationElfDirectory> {
    let (first, count) = selected_pair(context, selector)?;
    let mut cursor = super::index::record_start(first, count)?;
    let records = visits(context, count, &mut cursor)?;
    let completion = completion(count, &records);
    Ok(SimulationElfDirectory {
        context: context.clone(),
        selector,
        first,
        declared_count: count,
        records: records.into(),
        cursor,
        completion,
    })
}

fn visits(
    context: &SimulationElfIndexContext,
    count: u32,
    cursor: &mut Option<usize>,
) -> Result<Vec<SimulationElfVisitedRecord>> {
    let mut visited = Vec::new();
    for index in 0..count {
        validate_visit_budget(index, context.limits.maximum_visited_records)?;
        let offset = (*cursor).ok_or_else(|| crate::simulation::invalid("ELF visit cursor", format!("missing cursor at visit {index} of {count}; expected a normalized selected data-record address")))?;
        let id = SimulationElfRecordId {
            revision: context.revision.clone(),
            body_offset: offset,
        };
        let record = context.record(&id)?;
        let stop = record.getter_action() == SimulationElfGetterAction::Stop;
        visited.push(SimulationElfVisitedRecord { id, record });
        *cursor = Some(offset + 244);
        if stop {
            break;
        }
    }
    Ok(visited)
}

fn completion(
    count: u32,
    records: &[SimulationElfVisitedRecord],
) -> SimulationElfDirectoryCompletion {
    if count == 0 {
        return SimulationElfDirectoryCompletion::Empty;
    }
    if records
        .last()
        .is_some_and(|row| row.record.getter_action() == SimulationElfGetterAction::Stop)
    {
        return SimulationElfDirectoryCompletion::StoppedOnNegative;
    }
    SimulationElfDirectoryCompletion::DeclaredCountExhausted
}

fn selected_pair(
    context: &SimulationElfIndexContext,
    selector: SimulationElfSelector,
) -> Result<(i32, u32)> {
    let pair = selector.pair_body_offset() + 4;
    let first = dword(&context.bytes, pair)? as i32;
    let count = dword(&context.bytes, pair + 4)? as i32;
    require(
        count >= 0,
        "ELF selected directory count",
        format!(
            "{count} at stream offset {}; expected nonnegative signed count",
            pair + 4
        ),
    )?;
    Ok((first, count as u32))
}

fn validate_visit_budget(index: u32, maximum: usize) -> Result<()> {
    require(
        (index as usize) < maximum,
        "ELF visit budget",
        format!(
            "visit {index}; expected at most {} actually visited records",
            maximum
        ),
    )
}
