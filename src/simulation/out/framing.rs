use super::{SimulationOutLimits, SimulationOutLineInput};
use crate::records::dword;
use crate::simulation::{Result, invalid, require};

pub(super) fn decode(
    bytes: &[u8],
    limits: SimulationOutLimits,
) -> Result<Vec<SimulationOutLineInput>> {
    validate_transfer(bytes, limits.maximum_bytes)?;
    for (index, byte) in bytes[4..].iter().enumerate() {
        validate_encoding(*byte, index)?;
        validate_ending(&bytes[4..], index)?;
    }
    lines(&bytes[4..], limits.maximum_lines)
}

fn validate_transfer(bytes: &[u8], maximum: usize) -> Result<()> {
    require(
        bytes.len() >= 5 && bytes.len() <= maximum,
        "OUT transfer",
        format!(
            "{} bytes; expected 5..={maximum} complete wrapped bytes",
            bytes.len()
        ),
    )?;
    let expected = (bytes.len() - 5) / 0xa00000 + 1;
    require(
        dword(bytes, 0)? as usize == expected,
        "OUT transfer count",
        format!("{}; expected {expected}", dword(bytes, 0)?),
    )
}

fn validate_encoding(byte: u8, index: usize) -> Result<()> {
    if byte <= 127 && (byte >= 32 || matches!(byte, 0 | 9 | 10 | 13)) {
        return Ok(());
    }
    Err(invalid(
        "OUT encoding",
        format!(
            "byte {byte} at stream offset {}; expected seven-bit bytes and NUL/tab/CRLF controls only",
            index + 4
        ),
    ))
}

fn validate_ending(body: &[u8], index: usize) -> Result<()> {
    let paired = match body[index] {
        b'\r' => body.get(index + 1) == Some(&b'\n'),
        b'\n' => index > 0 && body[index - 1] == b'\r',
        _ => true,
    };
    if paired {
        return Ok(());
    }
    Err(invalid(
        "OUT ending",
        format!(
            "byte {} at stream offset {}; expected CRLF pairs",
            body[index],
            index + 4
        ),
    ))
}

fn lines(body: &[u8], maximum: usize) -> Result<Vec<SimulationOutLineInput>> {
    let mut lines = Vec::new();
    let mut cursor = 0;
    while cursor < body.len() {
        require(
            lines.len() < maximum,
            "OUT lines",
            format!(
                "at least {} lines; expected at most {maximum}",
                lines.len() + 1
            ),
        )?;
        let (line, consumed) = take_line(&body[cursor..]);
        lines.push(line);
        cursor += consumed;
    }
    Ok(lines)
}

fn take_line(tail: &[u8]) -> (SimulationOutLineInput, usize) {
    let length = tail
        .iter()
        .position(|byte| *byte == b'\r')
        .unwrap_or(tail.len());
    let crlf = length < tail.len();
    let line = SimulationOutLineInput {
        content: tail[..length].into(),
        crlf,
    };
    (line, length + if crlf { 2 } else { 0 })
}

pub(super) fn encode(
    lines: &[SimulationOutLineInput],
    limits: SimulationOutLimits,
) -> Result<Vec<u8>> {
    require(
        lines.len() <= limits.maximum_lines,
        "OUT construction",
        format!(
            "{} lines; expected at most {}",
            lines.len(),
            limits.maximum_lines
        ),
    )?;
    let mut bytes = vec![0; 4];
    for (index, line) in lines.iter().enumerate() {
        validate_line(line, index, lines.len())?;
        append(&mut bytes, &line.content, limits.maximum_bytes)?;
        if line.crlf {
            append(&mut bytes, b"\r\n", limits.maximum_bytes)?;
        }
    }
    finalize_transfer(bytes)
}

fn validate_line(line: &SimulationOutLineInput, index: usize, count: usize) -> Result<()> {
    require(
        line.crlf || index + 1 == count,
        "OUT construction",
        format!("unterminated line {index}; expected CRLF before the final line"),
    )?;
    require(
        !line
            .content
            .iter()
            .any(|byte| matches!(byte, b'\r' | b'\n')),
        "OUT construction",
        format!("line {index} contains CR/LF; expected content without line endings"),
    )
}

fn finalize_transfer(mut bytes: Vec<u8>) -> Result<Vec<u8>> {
    require(
        bytes.len() >= 5,
        "OUT construction",
        format!("{} bytes; expected nonempty copied body", bytes.len()),
    )?;
    let count = (bytes.len() - 5) / 0xa00000 + 1;
    require(
        count <= i32::MAX as usize,
        "OUT transfer count",
        format!("{count}; expected positive signed 32-bit count"),
    )?;
    bytes[..4].copy_from_slice(&(count as u32).to_le_bytes());
    Ok(bytes)
}

fn append(output: &mut Vec<u8>, region: &[u8], maximum: usize) -> Result<()> {
    require(
        region.len() <= maximum.saturating_sub(output.len()) && output.len() <= maximum,
        "OUT construction budget",
        format!(
            "{} bytes plus {}; expected at most {maximum}",
            output.len(),
            region.len()
        ),
    )?;
    output.extend_from_slice(region);
    Ok(())
}
