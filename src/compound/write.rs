use super::address::{copy_segments, lock_sector, physical_sector};
use super::header::{DIF, END, FAT, FREE};
use super::layout::{CompoundLayout, StreamAllocation};
use super::read::CompoundModel;
use super::{CompoundEntryKind, CompoundLimits, Result, invalid};
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;

pub(super) fn encode(
    model: &CompoundModel,
    payloads: &BTreeMap<usize, Arc<[u8]>>,
    limits: CompoundLimits,
) -> Result<Vec<u8>> {
    let layout = CompoundLayout::plan(model, payloads, limits)?;
    let mut output = vec![0; layout.file_bytes];
    write_header(&mut output, model, &layout)?;
    write_directory(&mut output, model, payloads, &layout)?;
    write_streams(&mut output, payloads, &layout)?;
    write_tables(&mut output, &layout)?;
    Ok(output)
}

fn write_header(output: &mut [u8], model: &CompoundModel, layout: &CompoundLayout) -> Result<()> {
    copy_at(output, 0, &model.header.raw)?;
    write_header_counts(output, model.header.major, layout)?;
    for ordinal in 0..109 {
        let value = layout.fat.clone().nth(ordinal).map_or(FREE, |sector| {
            physical_sector(sector, layout.sector_bytes) as u32
        });
        put_word(output, 76 + ordinal * 4, value)?;
    }
    Ok(())
}

fn write_directory(
    output: &mut [u8],
    model: &CompoundModel,
    payloads: &BTreeMap<usize, Arc<[u8]>>,
    layout: &CompoundLayout,
) -> Result<()> {
    let base = sector_offset(layout.directory.start, layout.sector_bytes);
    for (index, record) in model.directory.records.iter().enumerate() {
        let raw = allocated_record(record, index, payloads, layout)?;
        copy_logical(output, base + index * 128, &raw, layout.sector_bytes)?;
    }
    Ok(())
}

fn allocation_fields(raw: &mut [u8], first: u32, size: usize) -> Result<()> {
    put_word(raw, 116, first)?;
    copy_at(raw, 120, &(size as u64).to_le_bytes())
}

fn write_streams(
    output: &mut [u8],
    payloads: &BTreeMap<usize, Arc<[u8]>>,
    layout: &CompoundLayout,
) -> Result<()> {
    for (index, payload) in payloads {
        let offset = match layout.stream(*index)? {
            StreamAllocation::Normal(chain) => sector_offset(chain.start, layout.sector_bytes),
            StreamAllocation::Mini(chain) => {
                sector_offset(layout.mini_stream.start, layout.sector_bytes) + chain.start * 64
            }
        };
        if !payload.is_empty() {
            copy_logical(output, offset, payload, layout.sector_bytes)?;
        }
    }
    Ok(())
}

fn write_tables(output: &mut [u8], layout: &CompoundLayout) -> Result<()> {
    let (fat, mini_fat) = allocation_words(layout);
    write_words(
        output,
        sector_offset(layout.fat.start, layout.sector_bytes),
        &fat,
        layout.sector_bytes,
    )?;
    write_words(
        output,
        sector_offset(layout.mini_fat.start, layout.sector_bytes),
        &mini_fat,
        layout.sector_bytes,
    )?;
    write_difat(output, layout)
}

fn write_difat(output: &mut [u8], layout: &CompoundLayout) -> Result<()> {
    for (ordinal, sector) in layout.difat.clone().enumerate() {
        let values = difat_words(layout, ordinal, sector);
        write_words(
            output,
            sector_offset(sector, layout.sector_bytes),
            &values,
            layout.sector_bytes,
        )?;
    }
    Ok(())
}

fn link_chain(table: &mut [u32], chain: &Range<usize>) {
    for sector in chain.clone() {
        table[sector] = if sector + 1 < chain.end {
            (sector + 1) as u32
        } else {
            END
        };
    }
}

fn write_words(output: &mut [u8], offset: usize, words: &[u32], size: usize) -> Result<()> {
    for (index, value) in words.iter().enumerate() {
        copy_logical(output, offset + index * 4, &value.to_le_bytes(), size)?;
    }
    Ok(())
}

fn put_word(output: &mut [u8], offset: usize, value: u32) -> Result<()> {
    copy_at(output, offset, &value.to_le_bytes())
}

fn copy_at(output: &mut [u8], offset: usize, bytes: &[u8]) -> Result<()> {
    let maximum = output.len();
    let end = offset.checked_add(bytes.len()).ok_or_else(|| {
        invalid(format!(
            "output offset {offset} plus {}; expected within {maximum} bytes",
            bytes.len()
        ))
    })?;
    let target = output.get_mut(offset..end).ok_or_else(|| {
        invalid(format!(
            "output range {offset}..{end}; expected within {maximum} bytes"
        ))
    })?;
    target.copy_from_slice(bytes);
    Ok(())
}

fn start(chain: &Range<usize>) -> u32 {
    if chain.is_empty() {
        END
    } else {
        chain.start as u32
    }
}

fn stream_start(allocation: &StreamAllocation, size: usize) -> u32 {
    match allocation {
        StreamAllocation::Normal(chain) => normal_start(chain, size),
        StreamAllocation::Mini(chain) => start(chain),
    }
}

fn normal_start(chain: &Range<usize>, size: usize) -> u32 {
    if chain.is_empty() {
        return END;
    }
    physical_sector(chain.start, size) as u32
}

fn sector_offset(sector: usize, size: usize) -> usize {
    (sector + 1) * size
}

fn write_header_counts(output: &mut [u8], major: u16, layout: &CompoundLayout) -> Result<()> {
    let directory_count = if major == 3 {
        0
    } else {
        layout.directory.len() as u32
    };
    let counts = [
        (40, directory_count),
        (44, layout.fat.len() as u32),
        (48, normal_start(&layout.directory, layout.sector_bytes)),
        (52, 0),
        (60, normal_start(&layout.mini_fat, layout.sector_bytes)),
        (64, layout.mini_fat.len() as u32),
        (68, normal_start(&layout.difat, layout.sector_bytes)),
        (72, layout.difat.len() as u32),
    ];
    for (offset, value) in counts {
        put_word(output, offset, value)?;
    }
    Ok(())
}

fn allocated_record(
    record: &super::directory::DirectoryRecord,
    index: usize,
    payloads: &BTreeMap<usize, Arc<[u8]>>,
    layout: &CompoundLayout,
) -> Result<[u8; 128]> {
    let mut raw = record.raw;
    match record.kind {
        CompoundEntryKind::Root => allocation_fields(
            &mut raw,
            normal_start(&layout.mini_stream, layout.sector_bytes),
            layout.mini_sectors * 64,
        )?,
        CompoundEntryKind::Stream => {
            let size = payload_size(payloads, index)?;
            allocation_fields(
                &mut raw,
                stream_start(layout.stream(index)?, layout.sector_bytes),
                size,
            )?;
        }
        _ => {}
    }
    Ok(raw)
}

fn allocation_words(layout: &CompoundLayout) -> (Vec<u32>, Vec<u32>) {
    let words = layout.sector_bytes / 4;
    let mut fat = vec![FREE; layout.fat.len() * words];
    let mut mini_fat = vec![FREE; layout.mini_fat.len() * words];
    for chain in [&layout.directory, &layout.mini_stream, &layout.mini_fat] {
        link_normal_chain(&mut fat, chain, layout.sector_bytes);
    }
    for allocation in layout.streams.values() {
        match allocation {
            StreamAllocation::Normal(chain) => {
                link_normal_chain(&mut fat, chain, layout.sector_bytes)
            }
            StreamAllocation::Mini(chain) => link_chain(&mut mini_fat, chain),
        }
    }
    mark_table_sectors(&mut fat, layout);
    (fat, mini_fat)
}

fn link_normal_chain(table: &mut [u32], chain: &Range<usize>, size: usize) {
    for logical in chain.clone() {
        let next = if logical + 1 < chain.end {
            physical_sector(logical + 1, size) as u32
        } else {
            END
        };
        table[physical_sector(logical, size)] = next;
    }
}

fn copy_logical(output: &mut [u8], offset: usize, bytes: &[u8], size: usize) -> Result<()> {
    for segment in copy_segments(offset, bytes.len(), size) {
        if segment.source.is_empty() {
            continue;
        }
        copy_at(output, segment.output, &bytes[segment.source])?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "write_tests.rs"]
mod tests;

fn payload_size(payloads: &BTreeMap<usize, Arc<[u8]>>, index: usize) -> Result<usize> {
    payloads
        .get(&index)
        .map(|payload| payload.len())
        .ok_or_else(|| {
            invalid(format!(
                "stream slot {index} lacks a payload; expected complete logical bytes"
            ))
        })
}

fn difat_words(layout: &CompoundLayout, ordinal: usize, sector: usize) -> Vec<u32> {
    let slots = layout.sector_bytes / 4 - 1;
    let mut values = vec![FREE; slots + 1];
    for (slot, value) in values[..slots].iter_mut().enumerate() {
        *value = layout
            .fat
            .clone()
            .nth(109 + ordinal * slots + slot)
            .map_or(FREE, |id| physical_sector(id, layout.sector_bytes) as u32);
    }
    values[slots] = if sector + 1 < layout.difat.end {
        physical_sector(sector + 1, layout.sector_bytes) as u32
    } else {
        END
    };
    values
}

fn mark_table_sectors(fat: &mut [u32], layout: &CompoundLayout) {
    for sector in layout.fat.clone() {
        fat[physical_sector(sector, layout.sector_bytes)] = FAT;
    }
    for sector in layout.difat.clone() {
        fat[physical_sector(sector, layout.sector_bytes)] = DIF;
    }
    if let Some(reserved) = lock_sector(layout.sector_bytes)
        .filter(|slot| (slot + 1) * layout.sector_bytes < layout.file_bytes)
    {
        fat[reserved] = END;
    }
}
