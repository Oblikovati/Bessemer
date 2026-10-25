use std::error::Error;

pub fn take<'a>(remaining: &mut &'a [u8], length: usize) -> Result<&'a [u8], Box<dyn Error>> {
    let (head, tail) = remaining.split_at_checked(length).ok_or_else(|| {
        format!(
            "record length {length}; expected at most {} remaining bytes",
            remaining.len()
        )
    })?;
    *remaining = tail;
    Ok(head)
}

pub fn take_u32(remaining: &mut &[u8]) -> Result<u32, Box<dyn Error>> {
    let bytes = take(remaining, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub fn require_count(count: u32, available: usize, minimum: usize) -> Result<(), Box<dyn Error>> {
    if count as usize <= available / minimum {
        return Ok(());
    }
    Err(format!(
        "record count {count}; expected at most {}",
        available / minimum
    )
    .into())
}

pub fn require_finished(remaining: &[u8]) -> Result<(), Box<dyn Error>> {
    if remaining.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} trailing bytes; expected complete framed corpus",
        remaining.len()
    )
    .into())
}
