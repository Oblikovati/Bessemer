//! Changes one DIB palette entry without changing its reserved byte or image commands.
use bessemer::Document;
use bessemer::preview::DibPreview;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, index, blue, green, red, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE STREAM INDEX BLUE GREEN RED DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream, 128 * 1024 * 1024)?;
    let preview = DibPreview::from_bytes(&bytes, 100_000_000)?;
    let color = [blue.parse()?, green.parse()?, red.parse()?];
    document.replace_stream(stream, recolored(&preview, index.parse()?, color)?)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn recolored(
    preview: &DibPreview,
    index: usize,
    color: [u8; 3],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut contents = preview.contents().clone();
    let count = contents.palette.len();
    let entry = contents
        .palette
        .get_mut(index)
        .ok_or_else(|| format!("palette index {index}; expected below {count}"))?;
    [entry.blue, entry.green, entry.red] = color;
    Ok(preview.with_contents(contents, 100_000_000)?.to_bytes()?)
}
