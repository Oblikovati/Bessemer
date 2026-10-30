use super::*;
use crate::records::dword;

pub(super) struct DecodedLegacyMap {
    pub offsets: [usize; 3],
    pub groups: [Arc<[[i32; 4]]>; 3],
}

pub(super) fn decode(bytes: &[u8], maximum: usize) -> Result<DecodedLegacyMap> {
    let mut cursor = 0;
    let mut offsets = [0; 3];
    let mut groups = Vec::with_capacity(3);
    for (index, offset) in offsets.iter_mut().enumerate() {
        *offset = cursor;
        let (records, end) = read_group(bytes, cursor, maximum, index)?;
        groups.push(records);
        cursor = end;
    }
    require_endpoint(cursor, bytes.len())?;
    Ok(DecodedLegacyMap {
        offsets,
        groups: three_groups(groups)?,
    })
}

fn require_endpoint(cursor: usize, length: usize) -> Result<()> {
    require(
        cursor == length,
        "legacy map endpoint",
        format!("end {cursor}, {length} bytes; expected exactly three groups and EOF"),
    )
}

fn three_groups(groups: Vec<Arc<[[i32; 4]]>>) -> Result<[Arc<[[i32; 4]]>; 3]> {
    groups.try_into().map_err(|groups: Vec<Arc<[[i32; 4]]>>| {
        invalid(
            "legacy map groups",
            format!("{}; expected exactly three groups", groups.len()),
        )
    })
}

fn read_group(
    bytes: &[u8],
    offset: usize,
    maximum: usize,
    group: usize,
) -> Result<(Arc<[[i32; 4]]>, usize)> {
    let count = dword(bytes, offset)? as i32;
    let start = offset + 4;
    require(
        count >= 0 && count as usize <= maximum && count as usize <= (bytes.len() - start) / 16,
        "legacy map record count",
        format!(
            "group {group}, {count} records with {} remaining bytes; expected a nonnegative signed-i32 count at most {maximum} and sixteen bytes per record",
            bytes.len() - start
        ),
    )?;
    let count = count as usize;
    let mut records = Vec::new();
    records.try_reserve_exact(count).map_err(|error| invalid("legacy map record allocation", format!("group {group}, {count} records, {error}; expected available complete record storage")))?;
    for index in 0..count {
        records.push(read_record(bytes, start + index * 16)?);
    }
    Ok((records.into(), start + count * 16))
}

fn read_record(bytes: &[u8], offset: usize) -> Result<[i32; 4]> {
    Ok([
        dword(bytes, offset)? as i32,
        dword(bytes, offset + 4)? as i32,
        dword(bytes, offset + 8)? as i32,
        dword(bytes, offset + 12)? as i32,
    ])
}

pub(super) fn encode(groups: &[Vec<[i32; 4]>; 3], maximum: usize) -> Result<Vec<u8>> {
    let size = construction_size(groups, maximum)?;
    let mut bytes = super::super::reserve_bytes(size)?;
    for group in groups {
        let count = i32::try_from(group.len()).map_err(|_| {
            invalid(
                "legacy map construction count",
                format!("{}; expected a signed-i32 count", group.len()),
            )
        })?;
        bytes.extend_from_slice(&count.to_le_bytes());
        bytes.extend(group.iter().flatten().flat_map(|word| word.to_le_bytes()));
    }
    Ok(bytes)
}

fn construction_size(groups: &[Vec<[i32; 4]>; 3], maximum: usize) -> Result<usize> {
    let mut size = 12usize;
    for (index, group) in groups.iter().enumerate() {
        require(
            group.len() <= maximum && i32::try_from(group.len()).is_ok(),
            "legacy map construction count",
            format!(
                "group {index}, {} records; expected a nonnegative signed-i32 count at most {maximum}",
                group.len()
            ),
        )?;
        size = group
            .len()
            .checked_mul(16)
            .and_then(|payload| size.checked_add(payload))
            .ok_or_else(|| {
                invalid(
                    "legacy map construction extent",
                    format!(
                        "{size} bytes plus {} records; expected representable complete image",
                        group.len()
                    ),
                )
            })?;
    }
    Ok(size)
}
