use super::{ComposerCompression, ComposerLimits, decode::ComposerRecord, failure};
use crate::{FailureClass, Result, compression};
use flate2::{Decompress, FlushDecompress, Status};
use std::sync::Arc;

pub(super) fn expand_members(
    bytes: &[u8],
    members: &[ComposerRecord],
    limits: ComposerLimits,
) -> Result<Vec<Arc<[u8]>>> {
    let mut total = 0usize;
    let mut expanded = Vec::with_capacity(members.len());
    for member in members {
        let size = member.expanded as usize;
        total = total.checked_add(size).ok_or_else(|| {
            failure(
                FailureClass::Invalid,
                &member.name,
                "decoded total overflow; expected bounded bytes",
            )
        })?;
        require_budget(&member.name, size, limits.maximum_member_bytes)?;
        require_budget(
            "aggregate decoded members",
            total,
            limits.maximum_expanded_bytes,
        )?;
        expanded.push(expand_member(&bytes[member.stored.clone()], member)?.into());
    }
    Ok(expanded)
}

fn expand_member(stored: &[u8], member: &ComposerRecord) -> Result<Vec<u8>> {
    let expanded = match member.method {
        ComposerCompression::Stored if stored.len() == member.expanded as usize => stored.to_vec(),
        ComposerCompression::Stored => {
            return Err(failure(
                FailureClass::Integrity,
                &member.name,
                format!(
                    "stored bytes {}; expected {} before copying",
                    stored.len(),
                    member.expanded
                ),
            ));
        }
        ComposerCompression::Deflated => {
            inflate_exact(stored, member.expanded as usize, &member.name)?
        }
    };
    let crc = compression::checksum(&expanded);
    if expanded.len() != member.expanded as usize || crc != member.crc {
        return Err(failure(
            FailureClass::Integrity,
            &member.name,
            format!(
                "decoded size {}, CRC {crc:08x}; expected {}, CRC {:08x}",
                expanded.len(),
                member.expanded,
                member.crc
            ),
        ));
    }
    Ok(expanded)
}

pub(super) fn inflate_exact(stored: &[u8], expected: usize, name: &str) -> Result<Vec<u8>> {
    let capacity = expected.checked_add(1).ok_or_else(|| {
        failure(
            FailureClass::Invalid,
            name,
            format!("decoded size {expected}; expected room for termination guard"),
        )
    })?;
    let mut decoder = Decompress::new(false);
    let mut expanded = vec![0; capacity];
    let status = decoder
        .decompress(stored, &mut expanded, FlushDecompress::Finish)
        .map_err(|error| {
            failure(
                FailureClass::Integrity,
                name,
                format!("raw deflate {error}; expected a complete stream"),
            )
        })?;
    if status != Status::StreamEnd
        || decoder.total_in() != stored.len() as u64
        || decoder.total_out() != expected as u64
    {
        return Err(failure(
            FailureClass::Integrity,
            name,
            format!(
                "deflate {status:?}, input {}, output {}; expected StreamEnd, input {}, output {expected}",
                decoder.total_in(),
                decoder.total_out(),
                stored.len()
            ),
        ));
    }
    expanded.truncate(expected);
    Ok(expanded)
}

pub(super) fn require_budget(name: &str, actual: usize, maximum: usize) -> Result<()> {
    if actual <= maximum {
        return Ok(());
    }
    Err(failure(
        FailureClass::Invalid,
        name,
        format!("size/count {actual}; expected <= {maximum}"),
    ))
}
