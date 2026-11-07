use super::{PngContents, invalid, zlib};
use crate::Result;
use std::collections::BTreeSet;

pub(super) fn validate_text(contents: &PngContents, remaining: &mut usize) -> Result<()> {
    let mut palettes = BTreeSet::new();
    for chunk in &contents.chunks {
        let bytes = &chunk.payload;
        let expanded = validate_text_chunk(chunk.kind.bytes(), bytes, *remaining, &mut palettes)?;
        *remaining -= expanded;
    }
    Ok(())
}

fn validate_text_chunk(
    kind: [u8; 4],
    bytes: &[u8],
    maximum: usize,
    palettes: &mut BTreeSet<Vec<u8>>,
) -> Result<usize> {
    match &kind {
        b"tEXt" => {
            let (_, text) = keyword(bytes)?;
            no_nul(text)?;
            Ok(0)
        }
        b"zTXt" => compressed_text(bytes, maximum),
        b"iTXt" => international_text(bytes, maximum),
        b"iCCP" => profile(bytes, maximum),
        b"sPLT" => {
            store_palette_name(bytes, palettes)?;
            Ok(0)
        }
        _ => Ok(0),
    }
}

fn store_palette_name(bytes: &[u8], palettes: &mut BTreeSet<Vec<u8>>) -> Result<()> {
    let name = suggested_palette(bytes)?;
    if !palettes.insert(name.to_vec()) {
        return Err(invalid(format!(
            "duplicate sPLT name {name:?}; expected unique names"
        )));
    }
    Ok(())
}

pub(super) fn keyword(bytes: &[u8]) -> Result<(&[u8], &[u8])> {
    let (name, tail) = terminated(bytes, "keyword")?;
    let valid = name
        .iter()
        .all(|byte| (32..=126).contains(byte) || *byte >= 161);
    if name.is_empty()
        || name.len() > 79
        || !valid
        || name.first() == Some(&32)
        || name.last() == Some(&32)
        || name.windows(2).any(|pair| pair == b"  ")
    {
        return Err(invalid(format!(
            "PNG keyword {name:?}; expected 1..79 printable Latin-1 bytes without leading, trailing or repeated spaces"
        )));
    }
    Ok((name, tail))
}

fn terminated<'a>(bytes: &'a [u8], label: &str) -> Result<(&'a [u8], &'a [u8])> {
    let end = bytes.iter().position(|byte| *byte == 0).ok_or_else(|| {
        invalid(format!(
            "PNG {label} has no NUL in {} bytes; expected terminated field",
            bytes.len()
        ))
    })?;
    Ok((&bytes[..end], &bytes[end + 1..]))
}

fn compressed_text(bytes: &[u8], maximum: usize) -> Result<usize> {
    let (_, tail) = keyword(bytes)?;
    let compressed = method_zero(tail)?;
    let text = zlib::inflate_bounded(compressed, maximum)?;
    no_nul(&text)?;
    Ok(text.len())
}

fn profile(bytes: &[u8], maximum: usize) -> Result<usize> {
    let (_, tail) = keyword(bytes)?;
    let profile = zlib::inflate_bounded(method_zero(tail)?, maximum)?;
    if profile.is_empty() {
        return Err(invalid(
            "iCCP expanded length 0; expected nonempty profile bytes",
        ));
    }
    Ok(profile.len())
}

fn method_zero(bytes: &[u8]) -> Result<&[u8]> {
    let Some((method, tail)) = bytes.split_first() else {
        return Err(invalid(
            "PNG compression method absent; expected method zero and zlib stream",
        ));
    };
    if *method != 0 {
        return Err(invalid(format!(
            "PNG compression method {method}; expected 0"
        )));
    }
    Ok(tail)
}

fn international_text(bytes: &[u8], maximum: usize) -> Result<usize> {
    let (compressed, text) = international_fields(bytes)?;
    if !compressed {
        utf8(text)?;
        return Ok(0);
    }
    let decoded = zlib::inflate_bounded(text, maximum)?;
    utf8(&decoded)?;
    Ok(decoded.len())
}

fn international_fields(bytes: &[u8]) -> Result<(bool, &[u8])> {
    let (_, tail) = keyword(bytes)?;
    let [flag, method, rest @ ..] = tail else {
        return Err(invalid(format!(
            "iTXt framing {} bytes; expected flag, method and terminated language/translation",
            tail.len()
        )));
    };
    if *flag > 1 || *method != 0 {
        return Err(invalid(format!(
            "iTXt flag/method {flag}/{method}; expected flag 0 or 1 and method 0"
        )));
    }
    Ok((*flag == 1, international_strings(rest)?))
}

fn international_strings(rest: &[u8]) -> Result<&[u8]> {
    let (language, rest) = terminated(rest, "language tag")?;
    if !language
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
    {
        return Err(invalid(format!(
            "iTXt language {language:?}; expected ASCII language subtags"
        )));
    }
    let (translated, text) = terminated(rest, "translated keyword")?;
    utf8(translated)?;
    Ok(text)
}

fn suggested_palette(bytes: &[u8]) -> Result<&[u8]> {
    let (name, tail) = keyword(bytes)?;
    let Some((depth, entries)) = tail.split_first() else {
        return Err(invalid("sPLT sample depth absent; expected 8 or 16"));
    };
    let width = match depth {
        8 => 6,
        16 => 10,
        other => {
            return Err(invalid(format!(
                "sPLT sample depth {other}; expected 8 or 16"
            )));
        }
    };
    if entries.is_empty() || !entries.len().is_multiple_of(width) {
        return Err(invalid(format!(
            "sPLT entry bytes {}; expected positive multiple of {width}",
            entries.len()
        )));
    }
    Ok(name)
}

fn no_nul(bytes: &[u8]) -> Result<()> {
    if bytes.contains(&0) {
        return Err(invalid(format!(
            "PNG text contains NUL in {} bytes; expected NUL-free text",
            bytes.len()
        )));
    }
    Ok(())
}

fn utf8(bytes: &[u8]) -> Result<()> {
    no_nul(bytes)?;
    std::str::from_utf8(bytes)
        .map(|_| ())
        .map_err(|error| invalid(format!("PNG UTF-8 text {error}; expected valid UTF-8")))
}
