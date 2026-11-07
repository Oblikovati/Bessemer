use super::{
    PngColor, PngContents, PngHeader,
    header::{be16, be32},
    invalid,
    raster::sample_maximum,
    samples::payload,
};
use crate::Result;

pub(super) fn validate_palette(contents: &PngContents, header: PngHeader) -> Result<usize> {
    let Some(palette) = payload(contents, *b"PLTE") else {
        return Ok(0);
    };
    let count = palette.len() / 3;
    let maximum = if header.color == PngColor::Indexed {
        1usize << header.bit_depth
    } else {
        256
    };
    if !palette.len().is_multiple_of(3) || count == 0 || count > maximum {
        return Err(invalid(format!(
            "PLTE byte length {}; expected 3..={} divisible by 3",
            palette.len(),
            maximum * 3
        )));
    }
    Ok(count)
}

pub(super) fn validate_chunk(
    kind: [u8; 4],
    bytes: &[u8],
    header: PngHeader,
    palette: usize,
) -> Result<()> {
    match &kind {
        b"cHRM" => length(kind, bytes, 32),
        b"gAMA" => positive_gamma(bytes),
        b"sRGB" => bounded_last_byte(kind, bytes, 1, 3),
        b"sBIT" => significant_bits(bytes, header),
        b"bKGD" => background(bytes, header, palette),
        b"hIST" => histogram(bytes, palette),
        b"tRNS" => transparency(bytes, header, palette),
        b"pHYs" => bounded_last_byte(kind, bytes, 9, 1),
        b"tIME" => timestamp(bytes),
        _ => Ok(()),
    }
}

fn positive_gamma(bytes: &[u8]) -> Result<()> {
    length(*b"gAMA", bytes, 4)?;
    if be32(bytes) == 0 {
        return Err(invalid("gAMA value 0; expected positive encoded gamma"));
    }
    Ok(())
}

fn bounded_last_byte(kind: [u8; 4], bytes: &[u8], size: usize, maximum: u8) -> Result<()> {
    length(kind, bytes, size)?;
    if bytes[size - 1] > maximum {
        return Err(invalid(format!(
            "{kind:?} final byte {}; expected 0..={maximum}",
            bytes[size - 1]
        )));
    }
    Ok(())
}

fn histogram(bytes: &[u8], palette: usize) -> Result<()> {
    if palette == 0 {
        return Err(invalid("hIST without PLTE; expected palette"));
    }
    length(*b"hIST", bytes, palette * 2)
}

pub(super) fn length(kind: [u8; 4], bytes: &[u8], expected: usize) -> Result<()> {
    if bytes.len() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "chunk {kind:?} length {}; expected {expected}",
        bytes.len()
    )))
}

fn significant_bits(bytes: &[u8], header: PngHeader) -> Result<()> {
    let expected = if header.color == PngColor::Indexed {
        3
    } else {
        header.color.channels()
    };
    length(*b"sBIT", bytes, expected)?;
    let maximum = if header.color == PngColor::Indexed {
        8
    } else {
        header.bit_depth
    };
    if let Some(value) = bytes.iter().find(|value| **value == 0 || **value > maximum) {
        return Err(invalid(format!(
            "sBIT value {value}; expected 1..={maximum}"
        )));
    }
    Ok(())
}

fn background(bytes: &[u8], header: PngHeader, palette: usize) -> Result<()> {
    if header.color == PngColor::Indexed {
        length(*b"bKGD", bytes, 1)?;
        if bytes[0] as usize >= palette {
            return Err(invalid(format!(
                "bKGD index {}; expected below {palette}",
                bytes[0]
            )));
        }
        return Ok(());
    }
    let count = if matches!(header.color, PngColor::Grayscale | PngColor::GrayscaleAlpha) {
        1
    } else {
        3
    };
    sample_words(*b"bKGD", bytes, count, header.bit_depth)
}

fn transparency(bytes: &[u8], header: PngHeader, palette: usize) -> Result<()> {
    match header.color {
        PngColor::Grayscale => sample_words(*b"tRNS", bytes, 1, header.bit_depth),
        PngColor::Rgb => sample_words(*b"tRNS", bytes, 3, header.bit_depth),
        PngColor::Indexed if !bytes.is_empty() && bytes.len() <= palette => Ok(()),
        other => Err(invalid(format!(
            "tRNS length {} for {other:?} with {palette} palette entries; expected gray key, RGB key or 1..=palette alpha values without an alpha channel",
            bytes.len()
        ))),
    }
}

fn sample_words(kind: [u8; 4], bytes: &[u8], count: usize, depth: u8) -> Result<()> {
    length(kind, bytes, count * 2)?;
    let maximum = sample_maximum(depth);
    if let Some(value) = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|word| be16(word))
        .find(|value| *value > maximum)
    {
        return Err(invalid(format!(
            "{kind:?} sample {value}; expected 0..={maximum}"
        )));
    }
    Ok(())
}

fn timestamp(bytes: &[u8]) -> Result<()> {
    length(*b"tIME", bytes, 7)?;
    if !(1..=12).contains(&bytes[2])
        || !(1..=31).contains(&bytes[3])
        || bytes[4] > 23
        || bytes[5] > 59
        || bytes[6] > 60
    {
        return Err(invalid(format!(
            "tIME fields {bytes:?}; expected year u16, month 1..12, day 1..31, hour 0..23, minute 0..59, second 0..60"
        )));
    }
    Ok(())
}
