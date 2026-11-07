use super::{PngChunk, PngColor, PngContents, PngHeader, PngLimits, invalid, unsupported};
use crate::Result;
use std::collections::BTreeSet;

#[derive(Default)]
struct ChunkOrder {
    seen: BTreeSet<[u8; 4]>,
    image_started: bool,
    image_ended: bool,
}

pub(super) fn validate_order(
    contents: &PngContents,
    header: PngHeader,
    limits: PngLimits,
) -> Result<()> {
    if contents.chunks.len() > limits.maximum_chunks {
        return Err(invalid(format!(
            "PNG chunk count {}; expected at most {}",
            contents.chunks.len(),
            limits.maximum_chunks
        )));
    }
    let mut order = ChunkOrder::default();
    for (index, chunk) in contents.chunks.iter().enumerate() {
        order.visit(chunk, index, contents.chunks.len(), header)?;
    }
    if !order.seen.contains(b"IEND") || !order.image_started {
        return Err(invalid(
            "PNG required chunks absent; expected IDAT and final IEND",
        ));
    }
    Ok(())
}

impl ChunkOrder {
    fn visit(
        &mut self,
        chunk: &PngChunk,
        index: usize,
        count: usize,
        header: PngHeader,
    ) -> Result<()> {
        let kind = chunk.kind.bytes();
        if chunk.payload.len() > i32::MAX as usize {
            return Err(invalid(format!(
                "chunk {kind:?} length {}; expected at most 2147483647",
                chunk.payload.len()
            )));
        }
        require_supported(chunk)?;
        if singleton(kind) && self.seen.contains(&kind) {
            return Err(invalid(format!(
                "duplicate chunk {kind:?}; expected at most one"
            )));
        }
        self.critical_order(chunk, index, count, header)?;
        self.ancillary_order(kind)?;
        self.seen.insert(kind);
        Ok(())
    }

    fn critical_order(
        &mut self,
        chunk: &PngChunk,
        index: usize,
        count: usize,
        header: PngHeader,
    ) -> Result<()> {
        match &chunk.kind.bytes() {
            b"IHDR" if index != 0 => {
                return Err(invalid(format!("IHDR at index {index}; expected index 0")));
            }
            b"PLTE" => self.palette_order(header)?,
            b"IDAT" => self.image_order(header)?,
            b"IEND" if index + 1 != count || !chunk.payload.is_empty() => {
                return Err(invalid(format!(
                    "IEND at index {index} length {}; expected final empty chunk",
                    chunk.payload.len()
                )));
            }
            _ => (),
        }
        if self.image_started && chunk.kind.bytes() != *b"IDAT" {
            self.image_ended = true;
        }
        Ok(())
    }

    fn palette_order(&self, header: PngHeader) -> Result<()> {
        if self.image_started
            || matches!(header.color, PngColor::Grayscale | PngColor::GrayscaleAlpha)
        {
            return Err(invalid(format!(
                "PLTE for {:?} after image={}; expected optional RGB or required indexed palette before IDAT",
                header.color, self.image_started
            )));
        }
        if [*b"bKGD", *b"hIST", *b"tRNS"]
            .iter()
            .any(|kind| self.seen.contains(kind))
        {
            return Err(invalid(
                "PLTE after dependent metadata; expected PLTE before bKGD/hIST/tRNS",
            ));
        }
        Ok(())
    }

    fn image_order(&mut self, header: PngHeader) -> Result<()> {
        if self.image_ended {
            return Err(invalid(
                "nonconsecutive IDAT; expected one contiguous IDAT sequence",
            ));
        }
        if header.color == PngColor::Indexed && !self.seen.contains(b"PLTE") {
            return Err(invalid(
                "indexed IDAT without PLTE; expected palette before image",
            ));
        }
        self.image_started = true;
        Ok(())
    }

    fn ancillary_order(&self, kind: [u8; 4]) -> Result<()> {
        let early = [*b"cHRM", *b"gAMA", *b"iCCP", *b"sBIT", *b"sRGB"].contains(&kind);
        let before_image =
            early || [*b"bKGD", *b"hIST", *b"tRNS", *b"pHYs", *b"sPLT"].contains(&kind);
        if (early && self.seen.contains(b"PLTE")) || (before_image && self.image_started) {
            return Err(invalid(format!(
                "late chunk {kind:?}; expected before {}",
                if early { "PLTE and IDAT" } else { "IDAT" }
            )));
        }
        if (kind == *b"sRGB" && self.seen.contains(b"iCCP"))
            || (kind == *b"iCCP" && self.seen.contains(b"sRGB"))
        {
            return Err(invalid(
                "both sRGB and iCCP present; expected at most one color-space declaration",
            ));
        }
        Ok(())
    }
}

fn require_supported(chunk: &PngChunk) -> Result<()> {
    let kind = chunk.kind.bytes();
    if !chunk.kind.ancillary() && ![*b"IHDR", *b"PLTE", *b"IDAT", *b"IEND"].contains(&kind) {
        return Err(unsupported(format!(
            "unknown critical chunk {kind:?}; expected IHDR, PLTE, IDAT or IEND"
        )));
    }
    Ok(())
}

fn singleton(kind: [u8; 4]) -> bool {
    [
        *b"IHDR", *b"PLTE", *b"IEND", *b"cHRM", *b"gAMA", *b"iCCP", *b"sBIT", *b"sRGB", *b"bKGD",
        *b"hIST", *b"tRNS", *b"pHYs", *b"tIME",
    ]
    .contains(&kind)
}

pub(super) fn recognized(kind: [u8; 4]) -> bool {
    singleton(kind) || [*b"IDAT", *b"tEXt", *b"zTXt", *b"iTXt", *b"sPLT"].contains(&kind)
}
