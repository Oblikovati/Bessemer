use super::{SimulationOutDecimal, SimulationOutLegend, decimal};
use crate::simulation::Result;
use std::sync::Arc;

pub(super) struct FieldSyntax {
    pub label: Arc<str>,
    pub symbol: Arc<str>,
    pub lexeme: Arc<str>,
    pub value: SimulationOutDecimal,
    pub token_offset: usize,
}

pub(super) fn field(content: &[u8]) -> Result<Option<FieldSyntax>> {
    let Some(equals) = content.iter().rposition(|byte| *byte == b'=') else {
        return Ok(None);
    };
    let Some((label, symbol)) = symbol_parts(trim(&content[..equals])) else {
        return Ok(None);
    };
    let token = trim(&content[equals + 1..]);
    let Some(value) = decimal::parse(token)? else {
        return Ok(None);
    };
    let token_offset = token_start(content, equals);
    Ok(Some(FieldSyntax {
        label: normalize_label(label).into(),
        symbol: String::from_utf8_lossy(symbol).into_owned().into(),
        lexeme: String::from_utf8_lossy(token).into_owned().into(),
        value,
        token_offset,
    }))
}

pub(super) fn legend(
    content: &[u8],
    line_index: usize,
    stream_offset: usize,
) -> Option<SimulationOutLegend> {
    let rest = trim(content).strip_prefix(b"EQ.")?;
    let comma = rest.iter().position(|byte| *byte == b',')?;
    let token = trim(&rest[..comma]);
    if !decimal::integer_syntax(token) {
        return None;
    }
    Some(SimulationOutLegend {
        value: decimal::canonical_integer(token).into(),
        description: String::from_utf8_lossy(trim(&rest[comma + 1..]))
            .into_owned()
            .into(),
        line_index,
        stream_offset,
    })
}

fn valid_symbol(symbol: &[u8]) -> bool {
    symbol.first().is_some_and(u8::is_ascii_uppercase)
        && symbol
            .iter()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || *byte == b'_')
}

fn normalize_label(label: &[u8]) -> String {
    label
        .split(|byte| *byte == b'.' || byte.is_ascii_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| String::from_utf8_lossy(part))
        .collect::<Vec<_>>()
        .join(" ")
}

fn trim(bytes: &[u8]) -> &[u8] {
    bytes.trim_ascii()
}

fn symbol_parts(prefix: &[u8]) -> Option<(&[u8], &[u8])> {
    let prefix = prefix.strip_suffix(b")")?;
    let open = prefix.iter().rposition(|byte| *byte == b'(')?;
    let symbol = trim(&prefix[open + 1..]);
    if !valid_symbol(symbol) {
        return None;
    }
    Some((&prefix[..open], symbol))
}

fn token_start(content: &[u8], equals: usize) -> usize {
    let whitespace = content[equals + 1..]
        .iter()
        .take_while(|byte| byte.is_ascii_whitespace())
        .count();
    equals + 1 + whitespace
}
