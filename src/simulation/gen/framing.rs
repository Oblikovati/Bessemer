use super::SimulationGenLayout;
use crate::records::{dword, region};
use crate::simulation::{Result, require, unsupported, wire};

pub(super) fn decode(bytes: &[u8], version: i32) -> Result<SimulationGenLayout> {
    validate_prefix(bytes, version)?;
    let counts = wire::patterns::<14>(&bytes[404..460], "GEN prefix")?;
    validate_counts(&counts)?;
    let mut cursor = 604;
    let mut blocks = [(0, 0); 10];
    for (index, size) in [56, 144, 16, 40, 144].into_iter().enumerate() {
        blocks[index] = take(bytes, &mut cursor, size)?;
    }
    blocks[5] = records(bytes, &mut cursor, counts[3], 52)?;
    blocks[6] = records(bytes, &mut cursor, counts[4], 140)?;
    finish(bytes, version, counts, blocks, cursor)
}

fn validate_prefix(bytes: &[u8], version: i32) -> Result<()> {
    if !(800050..890104).contains(&version) {
        return Err(unsupported(
            "GEN version",
            format!("{version}; expected 800050..890104 from paired MAS"),
        ));
    }
    region(bytes, 0, 1004)?;
    let transfer = (bytes.len() - 5) / 0xa00000 + 1;
    require(
        dword(bytes, 0)? as usize == transfer,
        "GEN transfer",
        format!("count {}; expected {transfer}", dword(bytes, 0)?),
    )?;
    Ok(())
}

fn validate_counts(counts: &[u32; 14]) -> Result<()> {
    require(
        counts.iter().all(|value| *value <= i32::MAX as u32),
        "GEN counts",
        format!("{counts:?}; expected fourteen nonnegative signed words"),
    )?;
    for index in [0, 1, 2, 5, 6, 7] {
        if counts[index] != 0 {
            return Err(unsupported(
                "GEN delegated collection",
                format!(
                    "count[{index}] = {}; expected zero for complete observed profile",
                    counts[index]
                ),
            ));
        }
    }
    Ok(())
}

fn finish(
    bytes: &[u8],
    version: i32,
    counts: [u32; 14],
    mut blocks: [(usize, usize); 10],
    mut cursor: usize,
) -> Result<SimulationGenLayout> {
    let trial = region(bytes, cursor, counts[8] as usize * 288)?;
    let stride = if ranges_valid(trial, counts[4])? {
        288
    } else {
        284
    };
    blocks[7] = records(bytes, &mut cursor, counts[8], stride)?;
    blocks[8] = records(bytes, &mut cursor, counts[9], 144)?;
    let vectors = vectors(bytes, &mut cursor, blocks[7], stride)?;
    blocks[9] = take(bytes, &mut cursor, 48)?;
    validate_end(bytes, blocks[9].0, cursor, version)?;
    Ok(SimulationGenLayout {
        counts,
        auxiliary_stride: stride,
        blocks,
        vectors: vectors.into(),
    })
}

fn validate_end(bytes: &[u8], trailer: usize, cursor: usize, version: i32) -> Result<()> {
    let trailer_version = dword(bytes, trailer)? as i32;
    require(
        trailer_version == version,
        "GEN trailer",
        format!("version {trailer_version}; expected paired MAS version {version}"),
    )?;
    require(
        cursor == bytes.len(),
        "GEN tail",
        format!(
            "end {cursor}, image length {}; expected exact EOF",
            bytes.len()
        ),
    )?;
    Ok(())
}

fn ranges_valid(trial: &[u8], elements: u32) -> Result<bool> {
    let maximum = i64::from(elements) + 1000;
    for row in trial.as_chunks::<288>().0 {
        let low = i64::from(dword(row, 16)? as i32);
        let high = i64::from(dword(row, 20)? as i32);
        if low < 0
            || high < 0
            || low > maximum
            || high > maximum
            || (low == 0 && high != 0)
            || low > high
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn vectors(
    bytes: &[u8],
    cursor: &mut usize,
    auxiliary: (usize, usize),
    stride: usize,
) -> Result<Vec<Option<(usize, usize)>>> {
    let mut vectors = Vec::new();
    for (index, row) in bytes[auxiliary.0..auxiliary.0 + auxiliary.1]
        .chunks_exact(stride)
        .enumerate()
    {
        let identifier = dword(row, 0)? as i32;
        let length = dword(row, 4)? as i32;
        require(
            length >= 0,
            "GEN auxiliary vector",
            format!("row {index} length {length}; expected nonnegative length"),
        )?;
        let span = if i64::from(identifier) == index as i64 + 1 {
            Some(records(bytes, cursor, length as u32, 4)?)
        } else {
            None
        };
        vectors.push(span);
    }
    Ok(vectors)
}

fn records(bytes: &[u8], cursor: &mut usize, count: u32, stride: usize) -> Result<(usize, usize)> {
    let size = (count as usize).checked_mul(stride).ok_or_else(|| {
        crate::simulation::invalid(
            "GEN records",
            format!("count {count}, stride {stride}; expected addressable byte length"),
        )
    })?;
    take(bytes, cursor, size)
}

fn take(bytes: &[u8], cursor: &mut usize, length: usize) -> Result<(usize, usize)> {
    region(bytes, *cursor, length)?;
    let span = (*cursor, length);
    *cursor += length;
    Ok(span)
}
