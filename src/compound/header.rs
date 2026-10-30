use super::{CompoundLimits, Result, budget, invalid, require};
use crate::records::{dword, region, word};
use std::sync::Arc;

pub(super) const FREE: u32 = 0xffff_ffff;
pub(super) const END: u32 = 0xffff_fffe;
pub(super) const FAT: u32 = 0xffff_fffd;
pub(super) const DIF: u32 = 0xffff_fffc;

#[derive(Debug, Clone)]
pub(super) struct Header {
    pub raw: Arc<[u8]>,
    pub major: u16,
    pub sector_bytes: usize,
    pub sectors: usize,
    pub directory_start: u32,
    pub directory_count: usize,
    pub fat_count: usize,
    pub mini_fat_start: u32,
    pub mini_fat_count: usize,
    pub difat_start: u32,
    pub difat_count: usize,
}

impl Header {
    fn count_fields(&self) -> [(&'static str, usize); 4] {
        [
            ("FAT sectors", self.fat_count),
            ("DIFAT sectors", self.difat_count),
            ("MiniFAT sectors", self.mini_fat_count),
            ("directory sectors", self.directory_count),
        ]
    }

    fn fields(bytes: &[u8], major: u16, sector_bytes: usize) -> Result<Self> {
        Ok(Self {
            raw: region(bytes, 0, sector_bytes)?.into(),
            major,
            sector_bytes,
            sectors: bytes.len() / sector_bytes - 1,
            directory_start: dword(bytes, 48)?,
            directory_count: dword(bytes, 40)? as usize,
            fat_count: dword(bytes, 44)? as usize,
            mini_fat_start: dword(bytes, 60)?,
            mini_fat_count: dword(bytes, 64)? as usize,
            difat_start: dword(bytes, 68)?,
            difat_count: dword(bytes, 72)? as usize,
        })
    }

    pub fn decode(bytes: &[u8], limits: CompoundLimits) -> Result<Self> {
        budget("file bytes", bytes.len(), limits.maximum_file_bytes)?;
        let signature = region(bytes, 0, 8)?;
        require(
            signature == [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
            format!("signature {signature:02x?}; expected MS-CFB D0CF11E0A1B11AE1"),
        )?;
        let major = word(bytes, 26)?;
        let sector_bytes = sector_size(major, word(bytes, 30)?)?;
        validate_header(bytes, major, sector_bytes)?;
        let header = Self::fields(bytes, major, sector_bytes)?;
        header.check_counts()?;
        Ok(header)
    }

    fn check_counts(&self) -> Result<()> {
        budget("physical sector identifiers", self.sectors, 0xffff_fffa)?;
        require(
            self.fat_count > 0,
            "FAT sector count 0; expected at least one FAT sector",
        )?;
        for (label, count) in self.count_fields() {
            budget(label, count, self.sectors)?;
        }
        require(
            self.major != 3 || self.directory_count == 0,
            format!(
                "version-3 directory count {}; expected 0",
                self.directory_count
            ),
        )?;
        Ok(())
    }

    pub fn sector<'a>(&self, bytes: &'a [u8], id: u32) -> Result<&'a [u8]> {
        require(
            (id as usize) < self.sectors,
            format!("sector {id:#010x}; expected 0..{}", self.sectors),
        )?;
        region(
            bytes,
            (id as usize + 1) * self.sector_bytes,
            self.sector_bytes,
        )
    }
}

fn sector_size(major: u16, shift: u16) -> Result<usize> {
    match (major, shift) {
        (3, 9) => Ok(512),
        (4, 12) => Ok(4096),
        _ => Err(invalid(format!(
            "major {major}, sector shift {shift}; expected (3,9) or (4,12)"
        ))),
    }
}

fn validate_header(bytes: &[u8], major: u16, sector_bytes: usize) -> Result<()> {
    validate_sector_geometry(bytes, major, sector_bytes)?;
    validate_reserved_header(bytes)?;
    require(
        word(bytes, 28)? == 0xfffe && word(bytes, 32)? == 6,
        format!(
            "byte order {:#06x}, mini shift {}; expected 0xFFFE and 6",
            word(bytes, 28)?,
            word(bytes, 32)?
        ),
    )?;
    require(
        dword(bytes, 56)? == 4096,
        format!("mini cutoff {}; expected 4096", dword(bytes, 56)?),
    )
}

fn validate_sector_geometry(bytes: &[u8], major: u16, sector_bytes: usize) -> Result<()> {
    require(
        bytes.len() >= 3 * sector_bytes && bytes.len().is_multiple_of(sector_bytes),
        format!(
            "file length {}; expected at least three whole {sector_bytes}-byte sectors",
            bytes.len()
        ),
    )?;
    zero_region(bytes, 512, sector_bytes - 512, "header-sector padding")?;
    if major == 3 {
        budget("version-3 file bytes", bytes.len(), 0x8000_0000)?;
    }
    Ok(())
}

fn validate_reserved_header(bytes: &[u8]) -> Result<()> {
    zero_region(bytes, 8, 16, "header CLSID")?;
    zero_region(bytes, 34, 6, "reserved header bytes")
}

fn zero_region(bytes: &[u8], start: usize, count: usize, label: &str) -> Result<()> {
    let region = region(bytes, start, count)?;
    if let Some((index, value)) = region.iter().enumerate().find(|(_, byte)| **byte != 0) {
        return Err(invalid(format!(
            "{label} at byte {} is {value:#04x}; expected zero in {start}..{}",
            start + index,
            start + count
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_identifier_bound_is_checked_without_allocating_a_giant_file() {
        let mut bytes = vec![0; 4096];
        bytes[44..48].copy_from_slice(&1_u32.to_le_bytes());
        let mut header = Header::fields(&bytes, 4, 4096).unwrap();
        header.sectors = 0xffff_fffa;
        header.check_counts().unwrap();
        header.sectors += 1;
        assert!(
            header
                .check_counts()
                .unwrap_err()
                .cause
                .contains("physical sector identifiers")
        );
    }
}
