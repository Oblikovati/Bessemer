use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FileIndex {
    pub free_head: u64,
    pub split: u64,
    pub maximum: u64,
    pub entries: Vec<HashFileMapEntry>,
}

pub(super) struct FileReader<'a> {
    pub bytes: &'a [u8],
    pub context: HashFileContext,
    pub roles: Vec<Option<HashFilePageRole>>,
    pub remaining_records: usize,
    pub remaining_payload: usize,
    pub interpreted: usize,
}

impl FileReader<'_> {
    pub fn page(&self, identity: u64) -> Result<&[u8]> {
        let position = usize::try_from(identity)
            .ok()
            .filter(|position| *position < self.roles.len())
            .ok_or_else(|| {
                invalid(
                    identity,
                    format!("page {identity}; expected 0..{}", self.roles.len()),
                )
            })?;
        let start = position * self.context.pager_extent;
        Ok(&self.bytes[start..start + self.context.pager_extent])
    }
    pub fn claim(&mut self, identity: u64, role: HashFilePageRole) -> Result<()> {
        self.page(identity)?;
        let previous = &mut self.roles[identity as usize];
        if previous.is_some() {
            return Err(invalid(
                identity,
                format!("role {previous:?}; expected unique {role:?}"),
            ));
        }
        *previous = Some(role);
        Ok(())
    }
    pub fn charge_records(&mut self, count: usize) -> Result<()> {
        self.remaining_records = self.remaining_records.checked_sub(count).ok_or_else(|| {
            invalid(
                "records",
                format!(
                    "count {count}; expected <= remaining {}",
                    self.remaining_records
                ),
            )
        })?;
        Ok(())
    }
    pub fn charge_payload(&mut self, count: usize) -> Result<()> {
        self.remaining_payload = self.remaining_payload.checked_sub(count).ok_or_else(|| {
            invalid(
                "payload",
                format!(
                    "bytes {count}; expected <= remaining {}",
                    self.remaining_payload
                ),
            )
        })?;
        Ok(())
    }
}

pub(super) fn integer<const N: usize>(bytes: &[u8], offset: usize) -> Result<[u8; N]> {
    bytes
        .get(offset..offset.saturating_add(N))
        .and_then(|word| word.try_into().ok())
        .ok_or_else(|| {
            invalid(
                offset,
                format!(
                    "word at {offset}; expected {N} bytes within {}",
                    bytes.len()
                ),
            )
        })
}

pub(super) fn read(reader: &mut FileReader<'_>) -> Result<FileIndex> {
    let root = reader.page(1)?;
    let magic = u32::from_be_bytes(integer(root, 0)?);
    let test = u32::from_be_bytes(integer(root, 4)?);
    if magic != 0xfa782dcb || test != default_hash(b"chm@symisc") {
        return Err(invalid(
            1,
            format!("magic {magic:#x}, hash test {test:#x}; expected 0xfa782dcb/0x03f523f5"),
        ));
    }
    let mut index = FileIndex {
        free_head: u64::from_be_bytes(integer(root, 8)?),
        split: u64::from_be_bytes(integer(root, 16)?),
        maximum: u64::from_be_bytes(integer(root, 24)?),
        entries: Vec::new(),
    };
    read_maps(reader, &mut index)?;
    validate_partition(&index)?;
    Ok(index)
}

fn read_maps(reader: &mut FileReader<'_>, index: &mut FileIndex) -> Result<()> {
    let mut current = 1;
    while current != 0 {
        reader.claim(current, HashFilePageRole::Index)?;
        let (start, count, next) = map_header(reader.page(current)?, current)?;
        reader.charge_records(count)?;
        append_pairs(reader.page(current)?, start, count, &mut index.entries)?;
        reader.interpreted += start + count * 16;
        current = next;
    }
    Ok(())
}

fn map_header(page: &[u8], identity: u64) -> Result<(usize, usize, u64)> {
    let start = if identity == 1 { 44 } else { 12 };
    let count = u32::from_be_bytes(integer(page, start - 4)?) as usize;
    let next = u64::from_be_bytes(integer(page, if identity == 1 { 32 } else { 0 })?);
    if count > (page.len() - start) / 16 {
        return Err(invalid(
            identity,
            format!(
                "map count {count}; expected complete 16-byte pairs within {}",
                page.len()
            ),
        ));
    }
    Ok((start, count, next))
}

fn append_pairs(
    page: &[u8],
    start: usize,
    count: usize,
    entries: &mut Vec<HashFileMapEntry>,
) -> Result<()> {
    for offset in (start..start + count * 16).step_by(16) {
        entries.push(HashFileMapEntry {
            logical: u64::from_be_bytes(integer(page, offset)?),
            physical: u64::from_be_bytes(integer(page, offset + 8)?),
        });
    }
    Ok(())
}

fn validate_partition(index: &FileIndex) -> Result<()> {
    check_partition_domain(index)?;
    if index.entries.is_empty() && index.split == 0 && index.maximum == 1 {
        return Ok(());
    }
    let mut logical: Vec<_> = index.entries.iter().map(|entry| entry.logical).collect();
    logical.sort_unstable();
    if !complete_partition(&logical, index.maximum + index.split) {
        return Err(invalid(
            "partition",
            format!(
                "logical IDs {logical:?}; expected exactly 0..{}",
                index.maximum + index.split
            ),
        ));
    }
    Ok(())
}

fn complete_partition(logical: &[u64], length: u64) -> bool {
    logical.len() as u64 == length
        && logical
            .iter()
            .enumerate()
            .all(|(position, identity)| *identity == position as u64)
}

fn check_partition_domain(index: &FileIndex) -> Result<()> {
    if !index.maximum.is_power_of_two() || index.split >= index.maximum {
        return Err(invalid(
            "partition",
            format!(
                "split {}, maximum {}; expected power-of-two maximum > split",
                index.split, index.maximum
            ),
        ));
    }
    Ok(())
}

pub(super) fn default_hash(key: &[u8]) -> u32 {
    key.iter().take(2048).fold(5381u32, |hash, byte| {
        hash.wrapping_mul(33).wrapping_add(u32::from(*byte))
    })
}

pub(super) fn requested_bucket(hash: u32, index: &FileIndex) -> u64 {
    let candidate = u64::from(hash) & index.maximum.wrapping_mul(2).wrapping_sub(1);
    if candidate >= index.maximum.wrapping_add(index.split) {
        return u64::from(hash) & index.maximum.wrapping_sub(1);
    }
    candidate
}
