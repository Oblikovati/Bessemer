use super::*;
use crate::records::{region, word};

pub(crate) struct ComposerMemberWire<'a> {
    pub flags: u16,
    pub local_offset: usize,
    pub extra: &'a [u8],
    pub comment: &'a [u8],
}

impl ComposerArchive {
    /// Borrows checked metadata for another ZIP carrier; e.g. `self.member_wire(id)?`.
    pub(crate) fn member_wire(&self, id: &ComposerMemberId) -> Result<ComposerMemberWire<'_>> {
        let record = &self.directory.members[self.member_index(id)?];
        let header = &self.original[record.central.clone()];
        let extra_start = 46 + usize::from(word(header, 28)?);
        let extra_length = usize::from(word(header, 30)?);
        Ok(ComposerMemberWire {
            flags: word(header, 8)?,
            local_offset: record.local.start - 3,
            extra: region(header, extra_start, extra_length)?,
            comment: &header[extra_start + extra_length..],
        })
    }
}
