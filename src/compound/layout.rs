use super::read::CompoundModel;
use super::{CompoundLimits, Result, budget, invalid};
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug)]
pub(super) enum StreamAllocation {
    Normal(Range<usize>),
    Mini(Range<usize>),
}

#[derive(Debug)]
pub(super) struct CompoundLayout {
    pub sector_bytes: usize,
    pub directory: Range<usize>,
    pub mini_stream: Range<usize>,
    pub mini_fat: Range<usize>,
    pub fat: Range<usize>,
    pub difat: Range<usize>,
    pub streams: BTreeMap<usize, StreamAllocation>,
    pub mini_sectors: usize,
    pub file_bytes: usize,
}

impl CompoundLayout {
    pub fn stream(&self, index: usize) -> Result<&StreamAllocation> {
        self.streams.get(&index).ok_or_else(|| {
            invalid(format!(
                "stream slot {index} missing allocation; expected a complete planned stream layout"
            ))
        })
    }

    pub fn plan(
        model: &CompoundModel,
        payloads: &BTreeMap<usize, Arc<[u8]>>,
        limits: CompoundLimits,
    ) -> Result<Self> {
        let mut builder = LayoutBuilder::new(model.header.sector_bytes, limits);
        let directory = builder.normal(model.directory.records.len() * 128)?;
        let streams = builder.streams(payloads)?;
        let tail = builder.tail(model.header.major)?;
        Ok(Self {
            sector_bytes: builder.size,
            directory,
            streams,
            mini_sectors: builder.mini,
            mini_stream: tail.mini_stream,
            mini_fat: tail.mini_fat,
            fat: tail.fat,
            difat: tail.difat,
            file_bytes: tail.file_bytes,
        })
    }
}

struct LayoutBuilder {
    next: usize,
    mini: usize,
    total: usize,
    size: usize,
    limits: CompoundLimits,
}

struct LayoutTail {
    mini_stream: Range<usize>,
    mini_fat: Range<usize>,
    fat: Range<usize>,
    difat: Range<usize>,
    file_bytes: usize,
}

impl LayoutBuilder {
    fn new(size: usize, limits: CompoundLimits) -> Self {
        Self {
            next: 0,
            mini: 0,
            total: 0,
            size,
            limits,
        }
    }

    fn tail(&mut self, major: u16) -> Result<LayoutTail> {
        let mini_stream = self.normal(self.mini * 64)?;
        let mini_fat = self.normal(self.mini * 4)?;
        let (fat_count, difat_count) = table_counts(self.next, self.size / 4)?;
        let fat = self.normal(fat_count * self.size)?;
        let difat = self.normal(difat_count * self.size)?;
        let file_bytes = self.file_bytes(major)?;
        Ok(LayoutTail {
            mini_stream,
            mini_fat,
            fat,
            difat,
            file_bytes,
        })
    }

    fn file_bytes(&self, major: u16) -> Result<usize> {
        let sectors = super::address::physical_count(self.next, self.size);
        budget("physical sector identifiers", sectors, 0xffff_fffa)?;
        let file_bytes = (sectors + 1).checked_mul(self.size).ok_or_else(|| {
            invalid(format!(
                "{} sectors of {} bytes; expected representable file size",
                self.next + 1,
                self.size
            ))
        })?;
        budget(
            "constructed file bytes",
            file_bytes,
            self.limits.maximum_file_bytes,
        )?;
        if major == 3 {
            budget("constructed version-3 bytes", file_bytes, 0x8000_0000)?;
        }
        Ok(file_bytes)
    }

    fn charge(&mut self, size: usize) -> Result<()> {
        budget(
            "replacement stream bytes",
            size,
            self.limits.maximum_stream_bytes,
        )?;
        self.total = self.total.checked_add(size).ok_or_else(|| {
            invalid(format!(
                "aggregate {} plus {size}; expected representable stream bytes",
                self.total
            ))
        })?;
        budget(
            "replacement aggregate bytes",
            self.total,
            self.limits.maximum_total_stream_bytes,
        )
    }

    fn user_allocation(&mut self, size: usize) -> Result<StreamAllocation> {
        if size < 4096 {
            return self.small(size);
        }
        Ok(StreamAllocation::Normal(self.normal(size)?))
    }

    fn normal(&mut self, bytes: usize) -> Result<Range<usize>> {
        let start = self.next;
        self.next = self
            .next
            .checked_add(bytes.div_ceil(self.size))
            .ok_or_else(|| {
                invalid(format!(
                    "sector offset {start} plus {} bytes; expected representable sector range",
                    bytes
                ))
            })?;
        budget(
            "constructed sectors",
            self.next,
            self.limits.maximum_file_bytes / self.size,
        )?;
        budget("sector identifiers", self.next, 0xffff_fffa)?;
        Ok(start..self.next)
    }

    fn streams(
        &mut self,
        payloads: &BTreeMap<usize, Arc<[u8]>>,
    ) -> Result<BTreeMap<usize, StreamAllocation>> {
        let mut streams = BTreeMap::new();
        for (index, payload) in payloads {
            self.charge(payload.len())?;
            streams.insert(*index, self.user_allocation(payload.len())?);
        }
        Ok(streams)
    }

    fn small(&mut self, bytes: usize) -> Result<StreamAllocation> {
        let start = self.mini;
        self.mini = self.mini.checked_add(bytes.div_ceil(64)).ok_or_else(|| {
            invalid(format!(
                "mini sector offset {start} plus {bytes} bytes; expected representable extent"
            ))
        })?;
        budget("mini-sector identifiers", self.mini, 0xffff_fffa)?;
        budget(
            "constructed mini sectors",
            self.mini,
            self.limits.maximum_file_bytes / 64,
        )?;
        Ok(StreamAllocation::Mini(start..self.mini))
    }
}

fn table_counts(content: usize, words: usize) -> Result<(usize, usize)> {
    let (mut fat, mut difat) = (1, 0);
    loop {
        let total = content.checked_add(fat).and_then(|value| value.checked_add(difat)).ok_or_else(|| invalid(format!("content {content}, FAT {fat}, DIFAT {difat}; expected representable sector total")))?;
        let physical = super::address::physical_count(total, words * 4);
        let next_fat = physical.div_ceil(words);
        let next_difat = next_fat.saturating_sub(109).div_ceil(words - 1);
        if (next_fat, next_difat) == (fat, difat) {
            return Ok((fat, difat));
        }
        (fat, difat) = (next_fat, next_difat);
    }
}

#[cfg(test)]
#[path = "layout_tests.rs"]
mod tests;
