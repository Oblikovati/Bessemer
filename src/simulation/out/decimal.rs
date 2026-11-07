use super::SimulationOutDecimal;
use crate::simulation::{Result, invalid};

pub(super) fn parse(token: &[u8]) -> Result<Option<SimulationOutDecimal>> {
    let (negative, unsigned) = unsigned(token);
    let exponent_at = unsigned
        .iter()
        .position(|byte| matches!(byte, b'e' | b'E' | b'd' | b'D'));
    let (mantissa, exponent) = match exponent_at {
        Some(index) => (&unsigned[..index], &unsigned[index + 1..]),
        None => (unsigned, b"0".as_slice()),
    };
    if !integer_syntax(exponent) {
        return Ok(None);
    }
    let Some((digits, fractional)) = mantissa_digits(mantissa) else {
        return Ok(None);
    };
    let exponent = exponent_value(exponent, fractional)?;
    Ok(Some(SimulationOutDecimal {
        negative,
        digits: digits.into(),
        exponent,
    }))
}

fn mantissa_digits(mantissa: &[u8]) -> Option<(Vec<u8>, usize)> {
    let mut point = None;
    let mut digits = Vec::new();
    for (index, byte) in mantissa.iter().enumerate() {
        match byte {
            b'0'..=b'9' => digits.push(*byte - b'0'),
            b'.' if point.is_none() => point = Some(index),
            _ => return None,
        }
    }
    if digits.is_empty() {
        return None;
    }
    let first = digits
        .iter()
        .position(|digit| *digit != 0)
        .unwrap_or(digits.len() - 1);
    let fractional = point.map_or(0, |index| mantissa.len() - index - 1);
    Some((digits[first..].to_vec(), fractional))
}

fn exponent_value(token: &[u8], fractional: usize) -> Result<i64> {
    let text = String::from_utf8_lossy(token);
    let exponent = text.parse::<i64>().map_err(|_| {
        invalid(
            "OUT decimal exponent",
            format!("{text:?}; expected signed 64-bit exponent"),
        )
    })?;
    let fractional = i64::try_from(fractional).map_err(|_| {
        invalid(
            "OUT fractional digits",
            format!("{fractional}; expected signed 64-bit digit count"),
        )
    })?;
    exponent.checked_sub(fractional).ok_or_else(|| {
        invalid(
            "OUT decimal exponent",
            format!("{exponent} minus {fractional}; expected signed 64-bit adjusted exponent"),
        )
    })
}

pub(super) fn integer_syntax(token: &[u8]) -> bool {
    let (_, digits) = unsigned(token);
    !digits.is_empty() && digits.iter().all(u8::is_ascii_digit)
}

pub(super) fn canonical_integer(token: &[u8]) -> String {
    let (negative, digits) = unsigned(token);
    let first = digits.iter().position(|digit| *digit != b'0');
    let Some(first) = first else {
        return "0".into();
    };
    let magnitude = String::from_utf8_lossy(&digits[first..]);
    if negative {
        return format!("-{magnitude}");
    }
    magnitude.into_owned()
}

fn unsigned(token: &[u8]) -> (bool, &[u8]) {
    match token.first() {
        Some(b'-') => (true, &token[1..]),
        Some(b'+') => (false, &token[1..]),
        _ => (false, token),
    }
}
