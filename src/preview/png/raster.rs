use super::{
    PngColor, PngContents, PngRaster,
    header::{be16, sample_count},
    invalid,
    samples::payload,
};
use crate::Result;

pub(super) fn validate_samples(raster: &PngRaster, contents: &PngContents) -> Result<()> {
    let expected = sample_count(raster.header)?;
    if raster.samples.len() != expected {
        return Err(invalid(format!(
            "PNG sample count {}; expected {expected}",
            raster.samples.len()
        )));
    }
    let maximum = maximum_sample(raster, contents);
    if let Some(value) = raster.samples.iter().find(|value| **value > maximum) {
        return Err(invalid(format!(
            "PNG sample {value}; expected 0..={maximum} for {:?}",
            raster.header.color
        )));
    }
    Ok(())
}

fn maximum_sample(raster: &PngRaster, contents: &PngContents) -> u16 {
    if raster.header.color == PngColor::Indexed {
        payload(contents, *b"PLTE")
            .map_or(0, |palette| palette.len() / 3)
            .saturating_sub(1) as u16
    } else {
        sample_maximum(raster.header.bit_depth)
    }
}

pub(super) fn rgba16(raster: &PngRaster, contents: &PngContents) -> Vec<[u16; 4]> {
    let palette = payload(contents, *b"PLTE").unwrap_or_default();
    let transparency = payload(contents, *b"tRNS").unwrap_or_default();
    raster
        .samples
        .chunks_exact(raster.header.color.channels())
        .map(|samples| rgba_pixel(samples, raster, palette, transparency))
        .collect()
}

fn rgba_pixel(
    samples: &[u16],
    raster: &PngRaster,
    palette: &[u8],
    transparency: &[u8],
) -> [u16; 4] {
    let scale = u16::MAX / u16::max(1, sample_maximum(raster.header.bit_depth));
    match raster.header.color {
        PngColor::Grayscale => {
            let g = samples[0] * scale;
            [g, g, g, gray_alpha(samples[0], transparency)]
        }
        PngColor::Rgb => rgb_pixel(samples, scale, rgb_alpha(samples, transparency)),
        PngColor::Indexed => palette_pixel(samples[0] as usize, palette, transparency),
        PngColor::GrayscaleAlpha => {
            let g = samples[0] * scale;
            [g, g, g, samples[1] * scale]
        }
        PngColor::Rgba => rgb_pixel(samples, scale, samples[3] * scale),
    }
}

fn rgb_pixel(samples: &[u16], scale: u16, alpha: u16) -> [u16; 4] {
    [
        samples[0] * scale,
        samples[1] * scale,
        samples[2] * scale,
        alpha,
    ]
}

fn palette_pixel(index: usize, palette: &[u8], transparency: &[u8]) -> [u16; 4] {
    [
        u16::from(palette[index * 3]) * 257,
        u16::from(palette[index * 3 + 1]) * 257,
        u16::from(palette[index * 3 + 2]) * 257,
        u16::from(transparency.get(index).copied().unwrap_or(255)) * 257,
    ]
}

fn gray_alpha(sample: u16, transparency: &[u8]) -> u16 {
    if transparency.len() == 2 && sample == be16(transparency) {
        return 0;
    }
    u16::MAX
}

fn rgb_alpha(samples: &[u16], transparency: &[u8]) -> u16 {
    if transparency.len() == 6
        && samples
            .iter()
            .zip(transparency.as_chunks::<2>().0)
            .all(|(sample, bytes)| *sample == be16(bytes))
    {
        return 0;
    }
    u16::MAX
}

pub(super) fn sample_maximum(depth: u8) -> u16 {
    if depth == 16 {
        return u16::MAX;
    }
    (1u16 << depth) - 1
}
