use super::invalid;
use crate::Result;
use std::ops::Range;

fn xml_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\r' | '\n')
}

pub(super) fn attribute_value_span(source: &str, span: Range<usize>) -> Result<(Range<usize>, u8)> {
    // Derive quote boundaries from the source so long names and spacing retain their full width.
    let raw = source.get(span.clone()).ok_or_else(|| {
        invalid(format!(
            "attribute range {span:?}; expected UTF-8 source boundaries"
        ))
    })?;
    let tail = quoted_tail(raw)?;
    let quote = tail
        .as_bytes()
        .first()
        .copied()
        .ok_or_else(|| invalid("attribute quote absent; expected single or double quote"))?;
    if !b"'\"".contains(&quote) || tail.as_bytes().last() != Some(&quote) {
        return Err(invalid(format!(
            "attribute quote {quote}; expected matching single/double quotes"
        )));
    }
    let start = span.start + raw.len() - tail.len() + 1;
    Ok((start..span.end - 1, quote))
}

fn quoted_tail(raw: &str) -> Result<&str> {
    let equals = raw.find('=').ok_or_else(|| {
        invalid(format!(
            "XML attribute {raw:?}; expected equals and quoted value"
        ))
    })?;
    Ok(raw[equals + 1..].trim_start_matches(xml_space))
}

pub(super) fn replace_span(source: &str, span: Range<usize>, replacement: &str) -> Result<String> {
    let before = source.get(..span.start).ok_or_else(|| {
        invalid(format!(
            "XML patch start {}; expected UTF-8 boundary",
            span.start
        ))
    })?;
    let after = source.get(span.end..).ok_or_else(|| {
        invalid(format!(
            "XML patch end {}; expected UTF-8 boundary",
            span.end
        ))
    })?;
    Ok([before, replacement, after].concat())
}

pub(super) fn escape(value: &str, quote: Option<u8>) -> Result<String> {
    super::grammar::check_characters(value)?;
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\r' => escaped.push_str("&#13;"),
            '\n' if quote.is_some() => escaped.push_str("&#10;"),
            '\t' if quote.is_some() => escaped.push_str("&#9;"),
            '\'' if quote == Some(b'\'') => escaped.push_str("&apos;"),
            '"' if quote == Some(b'"') => escaped.push_str("&quot;"),
            other => escaped.push(other),
        }
    }
    Ok(escaped)
}

pub(super) fn replace_leaf(source: &str, span: Range<usize>, escaped: &str) -> Result<String> {
    let opening_end = opening_end(source, span.clone())?;
    if source.as_bytes().get(opening_end.saturating_sub(1)) == Some(&b'/') {
        let qualified = qualified_name(&source[span.start + 1..opening_end]);
        return replace_span(
            source,
            opening_end - 1..opening_end + 1,
            &format!(">{escaped}</{qualified}>"),
        );
    }
    let closing = source
        .get(span.clone())
        .and_then(|raw| raw.rfind("</"))
        .map(|relative| span.start + relative)
        .ok_or_else(|| invalid(format!("XML element range {span:?}; expected closing tag")))?;
    let inner = opening_end + 1..closing;
    let replacement = replace_character_content(&source[inner.clone()], escaped)?;
    replace_span(source, inner, &replacement)
}

fn qualified_name(opening: &str) -> &str {
    let end = opening
        .find(|character: char| xml_space(character) || character == '/' || character == '>')
        .unwrap_or(opening.len());
    &opening[..end]
}

fn opening_end(source: &str, span: Range<usize>) -> Result<usize> {
    let mut quote = None;
    for index in span.start + 1..span.end {
        let byte = source.as_bytes()[index];
        if let Some(current) = quote {
            if byte == current {
                quote = None;
            }
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
        } else if byte == b'>' {
            return Ok(index);
        }
    }
    Err(invalid(format!(
        "XML element range {span:?}; expected quoted opening tag terminated by >"
    )))
}

fn replace_character_content(inner: &str, escaped: &str) -> Result<String> {
    let mut output = String::new();
    let mut offset = 0;
    let mut inserted = false;
    while offset < inner.len() {
        let (end, markup) = segment_end(inner, offset)?;
        if markup {
            output.push_str(&inner[offset..end]);
        } else if !inserted {
            output.push_str(escaped);
            inserted = true;
        }
        offset = end;
    }
    if !inserted {
        output.push_str(escaped);
    }
    Ok(output)
}

fn segment_end(inner: &str, offset: usize) -> Result<(usize, bool)> {
    let rest = &inner[offset..];
    for (start, end, markup) in [
        ("<!--", "-->", true),
        ("<?", "?>", true),
        ("<![CDATA[", "]]>", false),
    ] {
        if rest.starts_with(start) {
            let length = terminated_length(rest, end, offset)?;
            return Ok((offset + length, markup));
        }
    }
    if rest.starts_with('<') {
        return Err(invalid(format!(
            "XML content at byte {offset}; expected leaf text, CDATA, comment or PI"
        )));
    }
    Ok((offset + rest.find('<').unwrap_or(rest.len()), false))
}

pub(super) fn attribute_span(source: &str, start: usize) -> Result<Range<usize>> {
    let raw = source.get(start..).ok_or_else(|| {
        invalid(format!(
            "XML attribute start {start}; expected source boundary"
        ))
    })?;
    let tail = quoted_tail(raw)?;
    let quote = tail
        .chars()
        .next()
        .ok_or_else(|| invalid("XML attribute lacks quote; expected quoted value"))?;
    let end = tail[1..]
        .find(quote)
        .ok_or_else(|| invalid("XML attribute lacks closing quote; expected matching quote"))?;
    Ok(start..start + raw.len() - tail.len() + end + 2)
}

fn terminated_length(rest: &str, end: &str, offset: usize) -> Result<usize> {
    rest.find(end)
        .map(|position| position + end.len())
        .ok_or_else(|| {
            invalid(format!(
                "XML content at byte {offset}; expected terminator {end:?}"
            ))
        })
}
