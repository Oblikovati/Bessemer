use super::{
    ChunkType, PngChunk, PngContents, PngRaster, invalid, order::recognized, samples, zlib,
};
use crate::Result;

pub(super) fn replace_samples(contents: &PngContents, raster: &PngRaster) -> Result<PngContents> {
    let image = PngChunk {
        kind: ChunkType::new(*b"IDAT")?,
        payload: zlib::deflate(&samples::encode_samples(raster)?)?,
    };
    let mut chunks = Vec::new();
    let mut inserted = false;
    for chunk in &contents.chunks {
        if chunk.kind.bytes() == *b"IDAT" {
            if !inserted {
                chunks.push(image.clone());
                inserted = true;
            }
            continue;
        }
        if retain_after_image_edit(chunk) {
            chunks.push(chunk.clone());
        }
    }
    Ok(PngContents { chunks })
}

fn retain_after_image_edit(chunk: &PngChunk) -> bool {
    let kind = chunk.kind.bytes();
    if [*b"hIST", *b"tIME"].contains(&kind) {
        return false;
    }
    !chunk.kind.ancillary() || recognized(kind) || chunk.kind.safe_to_copy()
}

type UnknownKey<'a> = ([u8; 4], &'a [u8]);
type UnknownPositions<'a> = std::collections::BTreeMap<UnknownKey<'a>, Vec<usize>>;

pub(super) fn require_copy_policy(old: &PngContents, new: &PngContents) -> Result<()> {
    let changed = old
        .chunks
        .iter()
        .filter(|chunk| !chunk.kind.ancillary())
        .ne(new.chunks.iter().filter(|chunk| !chunk.kind.ancillary()));
    let previous = unknown_positions(old);
    for (key, positions) in unknown_positions(new) {
        if changed && !key.0[3].is_ascii_lowercase() {
            return Err(invalid(format!(
                "retained unsafe unknown chunk {:?} after critical edit; expected explicit removal",
                key.0
            )));
        }
        if let Some(old_positions) = previous.get(&key) {
            require_regions(key.0, old_positions, &positions)?;
        }
    }
    Ok(())
}

fn unknown_positions(contents: &PngContents) -> UnknownPositions<'_> {
    let mut positions = UnknownPositions::new();
    let mut critical_count = 0;
    let mut image_started = false;
    for chunk in &contents.chunks {
        if chunk.kind.ancillary() && !recognized(chunk.kind.bytes()) {
            let region = if chunk.kind.safe_to_copy() {
                usize::from(image_started)
            } else {
                critical_count
            };
            positions
                .entry((chunk.kind.bytes(), &chunk.payload))
                .or_default()
                .push(region);
        }
        critical_count += usize::from(!chunk.kind.ancillary());
        image_started |= chunk.kind.bytes() == *b"IDAT";
    }
    positions
}

fn require_regions(kind: [u8; 4], previous: &[usize], current: &[usize]) -> Result<()> {
    for (old, new) in previous.iter().zip(current) {
        if old != new {
            return Err(invalid(format!(
                "moved unknown chunk {kind:?} from region {old} to {new}; expected unchanged critical boundaries"
            )));
        }
    }
    Ok(())
}
