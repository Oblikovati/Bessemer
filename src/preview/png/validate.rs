use super::{
    PngContents, PngHeader, PngLimits, PngRaster, header, metadata, order, passes, raster, samples,
    text, zlib,
};
use crate::Result;

pub(super) fn validate(contents: &PngContents, limits: PngLimits) -> Result<PngRaster> {
    let header = header::parse_header(contents, limits)?;
    order::validate_order(contents, header, limits)?;
    let palette = metadata::validate_palette(contents, header)?;
    let expected = passes::expanded_size(header, limits)?;
    validate_metadata(
        contents,
        header,
        palette,
        limits.maximum_inflated_bytes - expected,
    )?;
    let compressed: Vec<u8> = contents
        .chunks
        .iter()
        .filter(|chunk| chunk.kind.bytes() == *b"IDAT")
        .flat_map(|chunk| chunk.payload.iter().copied())
        .collect();
    let expanded = zlib::inflate_exact(&compressed, expected)?;
    let raster = samples::decode_samples(&expanded, header)?;
    raster::validate_samples(&raster, contents)?;
    Ok(raster)
}

fn validate_metadata(
    contents: &PngContents,
    header: PngHeader,
    palette: usize,
    mut remaining: usize,
) -> Result<()> {
    for chunk in &contents.chunks {
        metadata::validate_chunk(chunk.kind.bytes(), &chunk.payload, header, palette)?;
    }
    text::validate_text(contents, &mut remaining)
}
