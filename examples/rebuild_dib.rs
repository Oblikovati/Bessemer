//! Reconstructs a packed DIB and emits top-to-bottom pixel values for independent verification.
use bessemer::preview::{DibPixels, DibPreview};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, destination, raster] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected INPUT OUTPUT RASTER",
            arguments.len()
        )
        .into());
    };
    let source = DibPreview::from_bytes(&std::fs::read(source)?, 100_000_000)?;
    let rebuilt = DibPreview::new(source.contents().clone(), 100_000_000)?;
    std::fs::write(destination, rebuilt.to_bytes()?)?;
    std::fs::write(raster, raster_bytes(source.decode_pixels(100_000_000)?))?;
    Ok(())
}

fn raster_bytes(pixels: DibPixels) -> Vec<u8> {
    match pixels {
        DibPixels::Indexed(indices) => indices
            .into_iter()
            .flat_map(|index| [u8::from(index.is_some()), index.unwrap_or_default()])
            .collect(),
        DibPixels::Bgrx(pixels) => pixels.into_iter().flatten().collect(),
    }
}
