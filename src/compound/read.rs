use super::directory::DirectoryTree;
use super::header::{END, FREE, Header};
use super::tables::{
    Allocation, ChainExtent, SectorOwner, chain_bytes, chain_extent, stream_extent, words,
};
use super::{CompoundEntryKind, CompoundLimits, Result, budget, invalid, require};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(super) struct CompoundModel {
    pub header: Header,
    pub directory: DirectoryTree,
    pub payloads: BTreeMap<usize, Arc<[u8]>>,
    pub normal_claims: usize,
    pub mini_claims: usize,
}

pub(super) fn decode(bytes: &[u8], limits: CompoundLimits) -> Result<CompoundModel> {
    let header = Header::decode(bytes, limits)?;
    let mut allocation = Allocation::read(bytes, header)?;
    let directory = read_directory(bytes, &mut allocation, limits)?;
    let mini_stream = read_mini_stream(bytes, &mut allocation, &directory, limits)?;
    let mini_fat = read_mini_fat(bytes, &mut allocation)?;
    let mut reader = PayloadReader::new(bytes, allocation, mini_stream, mini_fat, limits);
    let payloads = reader.payloads(&directory)?;
    reader.published(directory, payloads)
}

fn read_directory(
    bytes: &[u8],
    allocation: &mut Allocation,
    limits: CompoundLimits,
) -> Result<DirectoryTree> {
    let count = directory_extent(&allocation.header);
    let chain = allocation.chain(
        allocation.header.directory_start,
        count,
        SectorOwner::Directory,
    )?;
    budget(
        "directory slots",
        chain.len() * allocation.header.sector_bytes / 128,
        limits.maximum_directory_entries,
    )?;
    DirectoryTree::parse(
        &chain_bytes(bytes, &allocation.header, &chain)?,
        allocation.header.major,
        limits,
    )
}

fn read_mini_stream(
    bytes: &[u8],
    allocation: &mut Allocation,
    directory: &DirectoryTree,
    limits: CompoundLimits,
) -> Result<Vec<u8>> {
    let root = &directory.records[0];
    budget(
        "mini-sector identifiers",
        root.size.div_ceil(64),
        0xffff_fffa,
    )?;
    budget("mini-stream bytes", root.size, limits.maximum_file_bytes)?;
    let count = root.size.div_ceil(allocation.header.sector_bytes);
    let chain = allocation.chain(
        root.start,
        stream_extent(root.size, allocation.header.sector_bytes),
        SectorOwner::MiniStream,
    )?;
    let mut mini_stream = chain_bytes(bytes, &allocation.header, &chain[..count])?;
    mini_stream.truncate(root.size);
    Ok(mini_stream)
}

fn read_mini_fat(bytes: &[u8], allocation: &mut Allocation) -> Result<Vec<u32>> {
    let chain = allocation.chain(
        allocation.header.mini_fat_start,
        ChainExtent::Exact(allocation.header.mini_fat_count),
        SectorOwner::MiniFat,
    )?;
    Ok(words(&chain_bytes(bytes, &allocation.header, &chain)?))
}

struct PayloadReader<'a> {
    bytes: &'a [u8],
    allocation: Allocation,
    mini_stream: Vec<u8>,
    mini_fat: Vec<u32>,
    mini_owners: Vec<bool>,
    total: usize,
    limits: CompoundLimits,
}

impl<'a> PayloadReader<'a> {
    fn new(
        bytes: &'a [u8],
        allocation: Allocation,
        mini_stream: Vec<u8>,
        mini_fat: Vec<u32>,
        limits: CompoundLimits,
    ) -> Self {
        let mini_owners = vec![false; mini_fat.len()];
        Self {
            bytes,
            allocation,
            mini_stream,
            mini_fat,
            mini_owners,
            total: 0,
            limits,
        }
    }

    fn published(
        self,
        directory: DirectoryTree,
        payloads: BTreeMap<usize, Arc<[u8]>>,
    ) -> Result<CompoundModel> {
        self.finish()?;
        Ok(CompoundModel {
            normal_claims: self
                .allocation
                .owners
                .iter()
                .filter(|owner| owner.is_some())
                .count(),
            mini_claims: self.mini_owners.iter().filter(|claimed| **claimed).count(),
            header: self.allocation.header,
            directory,
            payloads,
        })
    }

    fn charge_payload(&mut self, size: usize) -> Result<()> {
        budget(
            "individual stream bytes",
            size,
            self.limits.maximum_stream_bytes,
        )?;
        self.total = self.total.checked_add(size).ok_or_else(|| {
            invalid(format!(
                "aggregate stream extent {} + {size}; expected usize byte count",
                self.total
            ))
        })?;
        budget(
            "aggregate stream bytes",
            self.total,
            self.limits.maximum_total_stream_bytes,
        )
    }

    fn read_payload(&mut self, index: usize, start: u32, size: usize) -> Result<Vec<u8>> {
        self.charge_payload(size)?;
        if size < 4096 {
            return self.small_stream(index, start, size);
        }
        self.normal_stream(index, start, size)
    }

    fn mini_sector(&mut self, index: usize, next: u32) -> Result<&[u8]> {
        let slot = next as usize;
        let maximum = self.mini_fat.len();
        let claimed = self.mini_owners.get_mut(slot).ok_or_else(|| {
            invalid(format!(
                "stream {index} mini sector {slot:#010x}; expected 0..{maximum}"
            ))
        })?;
        require(
            !*claimed,
            format!(
                "stream {index} repeats mini sector {slot}; expected exclusive acyclic allocation"
            ),
        )?;
        *claimed = true;
        crate::records::region(&self.mini_stream, slot * 64, 64)
    }

    fn payloads(&mut self, directory: &DirectoryTree) -> Result<BTreeMap<usize, Arc<[u8]>>> {
        let mut payloads = BTreeMap::new();
        for (index, record) in directory.records.iter().enumerate() {
            if record.kind != CompoundEntryKind::Stream {
                continue;
            }
            payloads.insert(
                index,
                self.read_payload(index, record.start, record.size)?.into(),
            );
        }
        Ok(payloads)
    }

    fn normal_stream(&mut self, index: usize, start: u32, size: usize) -> Result<Vec<u8>> {
        let count = size.div_ceil(self.allocation.header.sector_bytes);
        let chain = self.allocation.chain(
            start,
            stream_extent(size, self.allocation.header.sector_bytes),
            SectorOwner::Stream(index),
        )?;
        let mut payload = chain_bytes(self.bytes, &self.allocation.header, &chain[..count])?;
        payload.truncate(size);
        Ok(payload)
    }

    fn small_stream(&mut self, index: usize, mut next: u32, size: usize) -> Result<Vec<u8>> {
        let extent = stream_extent(size, 64);
        let mut sectors = 0;
        let mut payload = Vec::with_capacity(size);
        while next != END {
            chain_extent(sectors, extent, false, SectorOwner::Stream(index))?;
            let take = (size - payload.len()).min(64);
            payload.extend_from_slice(&self.mini_sector(index, next)?[..take]);
            next = self.mini_fat[next as usize];
            sectors += 1;
        }
        chain_extent(sectors, extent, true, SectorOwner::Stream(index))?;
        Ok(payload)
    }

    fn finish(&self) -> Result<()> {
        self.allocation.finish()?;
        for (index, value) in self.mini_fat.iter().enumerate() {
            require(
                self.mini_owners[index] || *value == FREE,
                format!("unowned MiniFAT[{index}]={value:#010x}; expected FREESECT"),
            )?;
        }
        Ok(())
    }
}

fn directory_extent(header: &Header) -> ChainExtent {
    if header.major == 4 {
        return ChainExtent::Exact(header.directory_count);
    }
    ChainExtent::Entire
}
