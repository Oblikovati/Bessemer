// #251: empty directories have no first record from which to recover their signature set.
const REGISTERED: &[u8; 16_000] = include_bytes!("container_signatures.bin");

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContainerSignatures {
    pub(crate) local: [u8; 4],
    pub(crate) central: [u8; 4],
    pub(crate) end: [u8; 4],
}

pub(crate) fn for_key(key: u32) -> Option<ContainerSignatures> {
    let rows = REGISTERED.as_chunks::<16>().0;
    let index = rows
        .binary_search_by_key(&key, |row| {
            u32::from_be_bytes([row[0], row[1], row[2], row[3]])
        })
        .ok()?;
    let row = rows.get(index)?;
    Some(ContainerSignatures {
        local: [row[4], row[5], row[6], row[7]],
        central: [row[8], row[9], row[10], row[11]],
        end: [row[12], row[13], row[14], row[15]],
    })
}
