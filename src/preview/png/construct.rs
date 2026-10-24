use super::{
    ChunkType, PngChunk, PngContents, PngLimits, PngRaster, header, invalid, metadata, order,
    passes, raster, samples, zlib,
};
use crate::Result;

pub(super) fn from_raster(
    raster: &PngRaster,
    palette: &[[u8; 3]],
    limits: PngLimits,
) -> Result<PngContents> {
    header::require_header(raster.header, limits)?;
    passes::expanded_size(raster.header, limits)?;
    if palette.len() > 256 {
        return Err(invalid(format!(
            "PNG palette length {}; expected 0..=256",
            palette.len()
        )));
    }
    let mut contents = empty_contents(raster, palette)?;
    order::validate_order(&contents, raster.header, limits)?;
    metadata::validate_palette(&contents, raster.header)?;
    raster::validate_samples(raster, &contents)?;
    let index = contents.chunks.len() - 2;
    contents.chunks[index].payload = zlib::deflate(&samples::encode_samples(raster)?)?;
    Ok(contents)
}

fn empty_contents(raster: &PngRaster, palette: &[[u8; 3]]) -> Result<PngContents> {
    let mut chunks = vec![header_chunk(raster)?];
    if !palette.is_empty() {
        chunks.push(PngChunk {
            kind: ChunkType::new(*b"PLTE")?,
            payload: palette.iter().flatten().copied().collect(),
        });
    }
    chunks.push(empty_chunk(*b"IDAT")?);
    chunks.push(empty_chunk(*b"IEND")?);
    Ok(PngContents { chunks })
}

fn empty_chunk(kind: [u8; 4]) -> Result<PngChunk> {
    Ok(PngChunk {
        kind: ChunkType::new(kind)?,
        payload: Vec::new(),
    })
}

fn header_chunk(raster: &PngRaster) -> Result<PngChunk> {
    let header = raster.header;
    let mut payload = header.width.to_be_bytes().to_vec();
    payload.extend_from_slice(&header.height.to_be_bytes());
    payload.extend_from_slice(&[
        header.bit_depth,
        header.color.wire(),
        0,
        0,
        u8::from(header.interlaced),
    ]);
    Ok(PngChunk {
        kind: ChunkType::new(*b"IHDR")?,
        payload,
    })
}
