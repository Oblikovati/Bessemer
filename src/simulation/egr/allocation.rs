use super::{EgrHeader, Result, SimulationEgrFreeRecord, require};
use std::collections::BTreeSet;

pub(super) fn validate_partition(
    bytes: &[u8],
    header: &EgrHeader,
    directory: &[i32],
) -> Result<Vec<SimulationEgrFreeRecord>> {
    let mut active = BTreeSet::new();
    for (index, number) in directory.iter().copied().enumerate() {
        accept_active(number, index + 1, header.allocated, &mut active)?;
    }
    validate_live_summary(header, directory, active.len())?;
    walk_free(bytes, header, &active)
}

fn accept_active(
    number: i32,
    logical: usize,
    allocated: usize,
    active: &mut BTreeSet<i32>,
) -> Result<()> {
    if number == 0 {
        return Ok(());
    }
    require(
        number > 0 && number as usize <= allocated && active.insert(number),
        "EGR live mapping",
        format!(
            "logical ordinal {logical}, physical number {number}; expected a unique live physical number in 1..={allocated}, or a zero hole"
        ),
    )
}

fn walk_free(
    bytes: &[u8],
    header: &EgrHeader,
    active: &BTreeSet<i32>,
) -> Result<Vec<SimulationEgrFreeRecord>> {
    let mut records = Vec::new();
    let mut seen = BTreeSet::new();
    let mut number = header.words[6];
    while number != 0 {
        accept_deleted(number, header, active, &mut seen, records.len())?;
        let link = deleted_link(bytes, header, number);
        number = link.next_record_number;
        records.push(link);
    }
    validate_complete_partition(header, active.len(), seen.len())?;
    Ok(records)
}

fn accept_deleted(
    number: i32,
    header: &EgrHeader,
    active: &BTreeSet<i32>,
    seen: &mut BTreeSet<i32>,
    visited: usize,
) -> Result<()> {
    require(
        number > 0
            && number as usize <= header.allocated
            && !active.contains(&number)
            && visited < header.free_count
            && seen.insert(number),
        "EGR deleted-record link",
        format!(
            "physical number {number} after {visited} deleted visits; expected a non-live, unvisited number in 1..={} within {} linked records",
            header.allocated, header.free_count
        ),
    )
}

fn validate_live_summary(header: &EgrHeader, directory: &[i32], live: usize) -> Result<()> {
    require(
        live == header.live_count,
        "EGR live directory",
        format!("{live} live mappings; expected {}", header.live_count),
    )?;
    let high = directory
        .iter()
        .rposition(|number| *number != 0)
        .map_or(0, |index| index + 1);
    require(
        header.words[8] >= 0 && header.words[8] as usize == high,
        "EGR high-water word",
        format!(
            "{}; expected highest live logical ordinal {high}",
            header.words[8]
        ),
    )
}

fn deleted_link(bytes: &[u8], header: &EgrHeader, number: i32) -> SimulationEgrFreeRecord {
    let start = header.rows_start + (number as usize - 1) * 128;
    let next = i32::from_le_bytes([
        bytes[start],
        bytes[start + 1],
        bytes[start + 2],
        bytes[start + 3],
    ]);
    SimulationEgrFreeRecord {
        record_number: number as u32,
        next_record_number: next,
    }
}

fn validate_complete_partition(header: &EgrHeader, live: usize, deleted: usize) -> Result<()> {
    require(
        deleted == header.free_count && live + deleted == header.allocated,
        "EGR physical partition",
        format!(
            "{live} live and {deleted} linked deleted records; expected {} live, {} deleted and {} total allocated records",
            header.live_count, header.free_count, header.allocated
        ),
    )
}
