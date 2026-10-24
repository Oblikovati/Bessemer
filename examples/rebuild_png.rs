//! Reconstructs static PNG chunks and emits exact samples and normalized RGBA16 for oracles.
use bessemer::preview::{PngLimits, PngPreview};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, destination, samples, rgba] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected INPUT OUTPUT SAMPLES RGBA16",
            arguments.len()
        )
        .into());
    };
    let preview = PngPreview::from_bytes(&std::fs::read(source)?, limits())?;
    let rebuilt = PngPreview::new(preview.contents().clone(), limits())?;
    std::fs::write(destination, rebuilt.to_bytes()?)?;
    std::fs::write(samples, big_endian_samples(&preview.raster().samples))?;
    std::fs::write(rgba, big_endian_samples(preview.rgba16().as_flattened()))?;
    Ok(())
}

fn big_endian_samples(samples: &[u16]) -> Vec<u8> {
    samples
        .iter()
        .flat_map(|value| value.to_be_bytes())
        .collect()
}

fn limits() -> PngLimits {
    PngLimits {
        maximum_pixels: 100_000_000,
        maximum_inflated_bytes: 512 * 1024 * 1024,
        maximum_chunks: 100_000,
    }
}
