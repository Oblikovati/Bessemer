use super::{
    MetadataFamily, MetadataStream, XmlEncoding, XmlFormat, XmlLimits, XmlSource, invalid,
    unsupported,
};
use crate::Result;

pub(super) const PLAIN: XmlFormat = XmlFormat {
    encoding: XmlEncoding::Utf8,
    bom: false,
    keyword_prefix: None,
};

pub(super) fn decode(
    stream: &MetadataStream,
    bytes: &[u8],
    limits: XmlLimits,
) -> Result<XmlSource> {
    require_size(bytes.len(), limits.maximum_bytes, "physical XML bytes")?;
    if bytes.is_empty() {
        return Ok(XmlSource {
            format: PLAIN,
            document: None,
        });
    }
    decode_nonempty(stream, bytes, limits)
}

fn decode_nonempty(stream: &MetadataStream, bytes: &[u8], limits: XmlLimits) -> Result<XmlSource> {
    let prefix = keyword_prefix(stream.family(), bytes);
    let body = if prefix.is_some() { &bytes[1..] } else { bytes };
    let (encoding, bom, skip) = detect(body);
    let document = decode_unicode(&body[skip..], encoding)?;
    require_size(
        document.len(),
        limits.maximum_bytes,
        "decoded UTF-8 XML bytes",
    )?;
    Ok(XmlSource {
        format: XmlFormat {
            encoding,
            bom,
            keyword_prefix: prefix,
        },
        document: Some(document),
    })
}

fn keyword_prefix(family: MetadataFamily, bytes: &[u8]) -> Option<u8> {
    if family != MetadataFamily::KeyWords || bytes.len() < 2 {
        return None;
    }
    let body = &bytes[1..];
    let framed = body.starts_with(b"<?xml")
        || body.starts_with(&[0xef, 0xbb, 0xbf])
        || body.starts_with(&[0xff, 0xfe])
        || body.starts_with(&[0xfe, 0xff])
        || body.starts_with(&[b'<', 0])
        || body.starts_with(&[0, b'<']);
    framed.then_some(bytes[0])
}

fn detect(bytes: &[u8]) -> (XmlEncoding, bool, usize) {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return (XmlEncoding::Utf8, true, 3);
    }
    if bytes.starts_with(&[0xff, 0xfe]) {
        return (XmlEncoding::Utf16Le, true, 2);
    }
    if bytes.starts_with(&[0xfe, 0xff]) {
        return (XmlEncoding::Utf16Be, true, 2);
    }
    if bytes.starts_with(&[b'<', 0]) {
        return (XmlEncoding::Utf16Le, false, 0);
    }
    if bytes.starts_with(&[0, b'<']) {
        return (XmlEncoding::Utf16Be, false, 0);
    }
    (XmlEncoding::Utf8, false, 0)
}

fn decode_unicode(bytes: &[u8], encoding: XmlEncoding) -> Result<String> {
    if encoding == XmlEncoding::Utf8 {
        return String::from_utf8(bytes.to_vec())
            .map_err(|error| invalid(format!("XML UTF-8 {error}; expected valid UTF-8")));
    }
    decode_utf16(bytes, encoding)
}

fn decode_utf16(bytes: &[u8], encoding: XmlEncoding) -> Result<String> {
    let (pairs, remainder) = bytes.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(invalid(format!(
            "UTF-16 byte count {}; expected an even number",
            bytes.len()
        )));
    }
    let words: Vec<u16> = pairs
        .iter()
        .map(|pair| match encoding {
            XmlEncoding::Utf16Le => u16::from_le_bytes(*pair),
            _ => u16::from_be_bytes(*pair),
        })
        .collect();
    String::from_utf16(&words).map_err(|error| {
        invalid(format!(
            "XML UTF-16 {error}; expected paired surrogate code units"
        ))
    })
}

pub(super) fn encode(source: &XmlSource) -> Vec<u8> {
    let Some(document) = &source.document else {
        return Vec::new();
    };
    let mut bytes = Vec::new();
    if let Some(prefix) = source.format.keyword_prefix {
        bytes.push(prefix);
    }
    bytes.extend_from_slice(bom_bytes(source.format));

    match source.format.encoding {
        XmlEncoding::Utf8 => bytes.extend_from_slice(document.as_bytes()),
        XmlEncoding::Utf16Le => bytes.extend(document.encode_utf16().flat_map(u16::to_le_bytes)),
        XmlEncoding::Utf16Be => bytes.extend(document.encode_utf16().flat_map(u16::to_be_bytes)),
    }
    bytes
}

pub(super) fn encoded_length(source: &XmlSource) -> Result<usize> {
    let Some(document) = &source.document else {
        return Ok(0);
    };
    let body = match source.format.encoding {
        XmlEncoding::Utf8 => Some(document.len()),
        _ => document.encode_utf16().count().checked_mul(2),
    };
    let bom = bom_bytes(source.format).len();
    body.and_then(|size| {
        size.checked_add(bom + usize::from(source.format.keyword_prefix.is_some()))
    })
    .ok_or_else(|| {
        invalid(format!(
            "encoded XML size for {} UTF-8 bytes; expected usize",
            document.len()
        ))
    })
}

pub(super) fn require_format(stream: &MetadataStream, source: &XmlSource) -> Result<()> {
    if source.document.is_none() {
        if source.format != PLAIN || !stream.family().allows_empty() {
            return Err(invalid(format!(
                "empty metadata {:?} with {:?}; expected canonical empty MaterialTree, CustomPropertySummary or Tables stream",
                stream.family(),
                source.format
            )));
        }
        return Ok(());
    }
    let text = source.document.as_deref().unwrap_or_default();
    require_prefix(stream, source, text)?;
    require_declaration(source.format, text)
}

fn require_prefix(stream: &MetadataStream, source: &XmlSource, text: &str) -> Result<()> {
    if source.format.keyword_prefix.is_none() {
        return Ok(());
    }
    if stream.family() != MetadataFamily::KeyWords {
        return Err(invalid(format!(
            "keyword prefix {:?} on {:?}; expected KeyWords stream",
            source.format.keyword_prefix,
            stream.family()
        )));
    }
    require_keyword_declaration(source, text)
}

fn require_keyword_declaration(source: &XmlSource, text: &str) -> Result<()> {
    if source.format.encoding == XmlEncoding::Utf8
        && !source.format.bom
        && !text.starts_with("<?xml")
    {
        return Err(invalid(format!(
            "keyword prefix {:?} without declaration; expected <?xml immediately after prefix",
            source.format.keyword_prefix
        )));
    }
    Ok(())
}

fn declared_encoding(text: &str) -> Option<&str> {
    if !text.starts_with("<?xml")
        || !text
            .as_bytes()
            .get(5)
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
    {
        return None;
    }
    let declaration = text.get(..text.find("?>")?)?;
    let position = declaration.find("encoding")?;
    let tail = declaration.get(position + 8..)?.trim_start();
    let value = tail.strip_prefix('=')?.trim_start();
    let quote = value.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let value = &value[1..];
    Some(&value[..value.find(quote)?])
}

fn require_declaration(format: XmlFormat, text: &str) -> Result<()> {
    let declaration = declared_encoding(text);
    if declaration_matches(format, declaration)? {
        return Ok(());
    }
    Err(invalid(format!(
        "XML declared encoding {declaration:?} with {format:?}; expected matching Unicode encoding and BOM requirements"
    )))
}

fn declaration_matches(format: XmlFormat, declaration: Option<&str>) -> Result<bool> {
    Ok(match declaration {
        Some(label) if label.eq_ignore_ascii_case("UTF-8") => format.encoding == XmlEncoding::Utf8,
        Some(label) if label.eq_ignore_ascii_case("UTF-16") => {
            format.encoding != XmlEncoding::Utf8 && format.bom
        }
        Some(label) if label.eq_ignore_ascii_case("UTF-16LE") => {
            format.encoding == XmlEncoding::Utf16Le
        }
        Some(label) if label.eq_ignore_ascii_case("UTF-16BE") => {
            format.encoding == XmlEncoding::Utf16Be
        }
        Some(label) => {
            return Err(unsupported(format!(
                "XML declared encoding {label:?}; expected UTF-8, UTF-16, UTF-16LE or UTF-16BE"
            )));
        }
        None => format.encoding == XmlEncoding::Utf8 || format.bom,
    })
}

pub(super) fn require_size(actual: usize, maximum: usize, label: &str) -> Result<()> {
    if actual <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "{label} count {actual}; expected at most {maximum}"
    )))
}

fn bom_bytes(format: XmlFormat) -> &'static [u8] {
    if !format.bom {
        return &[];
    }
    match format.encoding {
        XmlEncoding::Utf8 => &[0xef, 0xbb, 0xbf],
        XmlEncoding::Utf16Le => &[0xff, 0xfe],
        XmlEncoding::Utf16Be => &[0xfe, 0xff],
    }
}
