//! Edits one exact PNG channel sample and replaces its document stream.
use bessemer::{
    Document,
    preview::{PngLimits, PngPreview},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, index, value, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE STREAM SAMPLE_INDEX VALUE DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream, 256 * 1024 * 1024)?;
    let preview = PngPreview::from_bytes(&bytes, limits())?;
    let replacement = edited_sample(&preview, index.parse()?, value.parse()?)?;
    document.replace_stream(stream, replacement)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn edited_sample(
    preview: &PngPreview,
    index: usize,
    value: u16,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut samples = preview.raster().samples.clone();
    let count = samples.len();
    let sample = samples
        .get_mut(index)
        .ok_or_else(|| format!("PNG sample index {index}; expected below {count}"))?;
    *sample = value;
    Ok(preview.with_samples(samples, limits())?.to_bytes()?)
}

fn limits() -> PngLimits {
    PngLimits {
        maximum_pixels: 100_000_000,
        maximum_inflated_bytes: 512 * 1024 * 1024,
        maximum_chunks: 100_000,
    }
}
