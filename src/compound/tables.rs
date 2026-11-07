use super::header::{DIF, END, FAT, FREE, Header};
use super::{Result, invalid, require};
use crate::records::dword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SectorOwner {
    RangeLock(bool),
    Fat,
    Difat,
    Directory,
    MiniFat,
    MiniStream,
    Stream(usize),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ChainExtent {
    Entire,
    Exact(usize),
    AtLeast(usize),
}

pub(super) struct Allocation {
    pub header: Header,
    pub fat: Vec<u32>,
    pub owners: Vec<Option<SectorOwner>>,
}

impl Allocation {
    fn reserve_range_lock(&mut self) -> Result<()> {
        let Some(index) = super::address::lock_sector(self.header.sector_bytes) else {
            return Ok(());
        };
        if index >= self.header.sectors {
            return Ok(());
        }
        let file_bytes = (self.header.sectors + 1) * self.header.sector_bytes;
        let allocated = super::address::lock_state(self.fat[index], file_bytes)?;
        self.claim(index as u32, SectorOwner::RangeLock(allocated))
    }

    fn load_fat(&mut self, bytes: &[u8], ids: Vec<u32>) -> Result<()> {
        for id in ids {
            self.claim(id, SectorOwner::Fat)?;
            self.fat.extend(words(self.header.sector(bytes, id)?));
        }
        require(
            self.fat.len() >= self.header.sectors,
            format!(
                "FAT has {} slots; expected at least {} sectors",
                self.fat.len(),
                self.header.sectors
            ),
        )
    }

    fn extend_difat(&mut self, bytes: &[u8], ids: &mut Vec<u32>) -> Result<()> {
        let mut next = self.header.difat_start;
        for _ in 0..self.header.difat_count {
            self.claim(next, SectorOwner::Difat)?;
            let block = self.header.sector(bytes, next)?;
            let end = block.len() - 4;
            ids.extend(words(&block[..end]));
            next = dword(block, end)?;
        }
        require(
            next == END,
            format!("DIFAT terminal {next:#010x}; expected ENDOFCHAIN"),
        )
    }

    pub fn read(bytes: &[u8], header: Header) -> Result<Self> {
        let mut allocation = Self {
            owners: vec![None; header.sectors],
            header,
            fat: Vec::new(),
        };
        let ids = allocation.fat_ids(bytes)?;
        allocation.load_fat(bytes, ids)?;
        allocation.reserve_range_lock()?;
        Ok(allocation)
    }

    fn fat_ids(&mut self, bytes: &[u8]) -> Result<Vec<u32>> {
        let mut ids = words(&self.header.raw[76..512]);
        self.extend_difat(bytes, &mut ids)?;
        let expected = self.header.fat_count;
        require(
            ids.len() >= expected,
            format!(
                "DIFAT capacity {}; expected {expected} FAT slots",
                ids.len()
            ),
        )?;
        free_padding(&ids, expected, "DIFAT")?;
        ids.truncate(expected);
        Ok(ids)
    }

    pub fn claim(&mut self, sector: u32, owner: SectorOwner) -> Result<()> {
        let maximum = self.owners.len();
        let slot = self.owners.get_mut(sector as usize).ok_or_else(|| {
            invalid(format!(
                "{owner:?} sector {sector:#010x}; expected 0..{maximum}"
            ))
        })?;
        require(
            slot.is_none(),
            format!(
                "sector {sector} owned by {slot:?} and {owner:?}; expected exclusive allocation"
            ),
        )?;
        *slot = Some(owner);
        Ok(())
    }

    pub fn chain(
        &mut self,
        start: u32,
        count: ChainExtent,
        owner: SectorOwner,
    ) -> Result<Vec<u32>> {
        let mut sectors = Vec::new();
        let mut next = start;
        while next != END {
            chain_extent(sectors.len(), count, false, owner)?;
            self.claim(next, owner)?;
            sectors.push(next);
            next = self.fat[next as usize];
        }
        chain_extent(sectors.len(), count, true, owner)?;
        Ok(sectors)
    }

    pub fn finish(&self) -> Result<()> {
        for (index, owner) in self.owners.iter().enumerate() {
            check_marker(index, *owner, self.fat[index], self.header.sectors)?;
        }
        free_padding(&self.fat, self.header.sectors, "FAT")
    }
}

pub(super) fn words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|value| u32::from_le_bytes(*value))
        .collect()
}

pub(super) fn chain_bytes(bytes: &[u8], header: &Header, chain: &[u32]) -> Result<Vec<u8>> {
    let mut output = Vec::with_capacity(chain.len() * header.sector_bytes);
    for sector in chain {
        output.extend_from_slice(header.sector(bytes, *sector)?);
    }
    Ok(output)
}

pub(super) fn chain_extent(
    actual: usize,
    expected: ChainExtent,
    complete: bool,
    owner: SectorOwner,
) -> Result<()> {
    let valid = match expected {
        ChainExtent::Entire => true,
        ChainExtent::Exact(count) => {
            if complete {
                actual == count
            } else {
                actual < count
            }
        }
        ChainExtent::AtLeast(count) => !complete || actual >= count,
    };
    require(
        valid,
        format!("{owner:?} chain length {actual}, complete {complete}; expected {expected:?}"),
    )
}

pub(super) fn stream_extent(bytes: usize, sector_bytes: usize) -> ChainExtent {
    // #109: logical length is a minimum; allocated tail capacity still has an owner.
    if bytes == 0 {
        return ChainExtent::Exact(0);
    }
    ChainExtent::AtLeast(bytes.div_ceil(sector_bytes))
}

fn check_marker(
    index: usize,
    owner: Option<SectorOwner>,
    actual: u32,
    sectors: usize,
) -> Result<()> {
    let valid = match owner {
        None => actual == FREE,
        Some(SectorOwner::RangeLock(allocated)) => actual == if allocated { END } else { FREE },
        Some(SectorOwner::Fat) => actual == FAT,
        Some(SectorOwner::Difat) => actual == DIF,
        Some(_) => actual == END || (actual as usize) < sectors,
    };
    require(
        valid,
        format!(
            "FAT[{index}]={actual:#010x}, owner {owner:?}; expected matching allocation marker or chain link"
        ),
    )
}

fn free_padding(words: &[u32], start: usize, label: &str) -> Result<()> {
    if let Some((index, value)) = words
        .iter()
        .enumerate()
        .skip(start)
        .find(|(_, word)| **word != FREE)
    {
        return Err(invalid(format!(
            "{label} padding slot {index} is {value:#010x}; expected FREESECT after {start} slots"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "table_tests.rs"]
mod tests;
