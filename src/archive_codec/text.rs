use super::{ArchiveCursor, malformed};
use crate::Result;

/// Character representation stored by an MFC CString.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    /// One byte per character, interpreted as Latin-1 like the reference reader.
    Latin1,
    /// UTF-16LE code units, preceded by the MFC wide-string marker.
    Utf16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthWidth {
    Byte,
    Word,
    Dword,
}

/// A CString's text, encoding, and retained length-prefix width.
#[derive(Debug, Clone)]
pub struct ArchiveString {
    text: String,
    encoding: TextEncoding,
    width: LengthWidth,
}

impl PartialEq for ArchiveString {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.encoding == other.encoding
    }
}

impl Eq for ArchiveString {}

impl ArchiveString {
    /// Builds text using the selected stored representation.
    ///
    /// ```
    /// use bessemer::history::{ArchiveString, TextEncoding};
    /// let comment = ArchiveString::new("Reviewed", TextEncoding::Latin1)?;
    /// assert_eq!(comment.text(), "Reviewed");
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(text: &str, encoding: TextEncoding) -> Result<Self> {
        let count = encoded_count(text, encoding)?;
        let width = shortest_width(count, encoding);
        Ok(Self {
            text: text.into(),
            encoding,
            width,
        })
    }

    /// Returns decoded text without altering the stored representation.
    ///
    /// ```
    /// # use bessemer::history::{ArchiveString, TextEncoding};
    /// assert_eq!(ArchiveString::new("é", TextEncoding::Latin1)?.text(), "é");
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the retained character encoding.
    ///
    /// ```
    /// # use bessemer::history::{ArchiveString, TextEncoding};
    /// assert_eq!(ArchiveString::new("λ", TextEncoding::Utf16)?.encoding(), TextEncoding::Utf16);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn encoding(&self) -> TextEncoding {
        self.encoding
    }

    pub(crate) fn decode(cursor: &mut ArchiveCursor<'_>) -> Result<Self> {
        let first = cursor.u8()?;
        if first != 0xff {
            return read_text(
                cursor,
                u32::from(first),
                LengthWidth::Byte,
                TextEncoding::Latin1,
            );
        }
        let word = cursor.u16()?;
        if word != 0xfffe {
            let (count, width) = extended_length(cursor, word)?;
            return read_text(cursor, count, width, TextEncoding::Latin1);
        }
        let (count, width) = wide_length(cursor)?;
        read_text(cursor, count, width, TextEncoding::Utf16)
    }

    pub(crate) fn encoded_size(&self) -> Result<usize> {
        let count = encoded_count(&self.text, self.encoding)?;
        let prefix = match self.width {
            LengthWidth::Byte => 1,
            LengthWidth::Word => 3,
            LengthWidth::Dword => 7,
        };
        let marker = if self.encoding == TextEncoding::Utf16 {
            3
        } else {
            0
        };
        text_byte_count(count, self.encoding)?
            .checked_add(prefix + marker)
            .ok_or_else(|| {
                malformed(format!(
                    "CString count {count}; expected representable encoded size"
                ))
            })
    }

    pub(crate) fn encode(&self, output: &mut Vec<u8>) -> Result<()> {
        let count = encoded_count(&self.text, self.encoding)?;
        if self.encoding == TextEncoding::Utf16 {
            output.extend_from_slice(&[0xff, 0xfe, 0xff]);
        }
        write_length(output, count, self.width);
        match self.encoding {
            TextEncoding::Latin1 => {
                output.extend(self.text.chars().map(|character| character as u8))
            }
            TextEncoding::Utf16 => {
                output.extend(self.text.encode_utf16().flat_map(u16::to_le_bytes))
            }
        }
        Ok(())
    }
}

fn extended_length(cursor: &mut ArchiveCursor<'_>, word: u16) -> Result<(u32, LengthWidth)> {
    if word != 0xffff {
        return Ok((u32::from(word), LengthWidth::Word));
    }
    Ok((cursor.u32()?, LengthWidth::Dword))
}

fn wide_length(cursor: &mut ArchiveCursor<'_>) -> Result<(u32, LengthWidth)> {
    let byte = cursor.u8()?;
    if byte != 0xff {
        return Ok((u32::from(byte), LengthWidth::Byte));
    }
    let word = cursor.u16()?;
    extended_length(cursor, word)
}

fn read_text(
    cursor: &mut ArchiveCursor<'_>,
    count: u32,
    width: LengthWidth,
    encoding: TextEncoding,
) -> Result<ArchiveString> {
    let length = text_byte_count(count, encoding)?;
    let raw = cursor.take(length)?;
    let text = decode_text(raw, encoding)?;
    Ok(ArchiveString {
        text,
        encoding,
        width,
    })
}

fn text_byte_count(count: u32, encoding: TextEncoding) -> Result<usize> {
    let unit_size = if encoding == TextEncoding::Utf16 {
        2
    } else {
        1
    };
    (count as usize).checked_mul(unit_size).ok_or_else(|| {
        malformed(format!(
            "CString length {count}; expected representable byte length"
        ))
    })
}

fn decode_text(raw: &[u8], encoding: TextEncoding) -> Result<String> {
    if encoding == TextEncoding::Latin1 {
        return Ok(raw.iter().map(|byte| char::from(*byte)).collect());
    }
    let units = raw
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]));
    char::decode_utf16(units)
        .collect::<std::result::Result<String, _>>()
        .map_err(|error| {
            malformed(format!(
                "CString surrogate {:04x}: {error}; expected valid UTF-16LE",
                error.unpaired_surrogate()
            ))
        })
}

fn encoded_count(text: &str, encoding: TextEncoding) -> Result<u32> {
    if encoding == TextEncoding::Latin1 && !text.chars().all(|character| character as u32 <= 255) {
        return Err(malformed(format!(
            "CString text {text:?}; expected Latin-1 characters U+0000..=U+00FF"
        )));
    }
    let count = match encoding {
        TextEncoding::Latin1 => text.chars().count(),
        TextEncoding::Utf16 => text.encode_utf16().count(),
    };
    u32::try_from(count)
        .map_err(|_| malformed(format!("CString count {count}; expected <= u32::MAX")))
}

fn shortest_width(count: u32, encoding: TextEncoding) -> LengthWidth {
    if count < 255 {
        return LengthWidth::Byte;
    }
    let limit = if encoding == TextEncoding::Latin1 {
        0xfffe
    } else {
        0xffff
    };
    if count < limit {
        return LengthWidth::Word;
    }
    LengthWidth::Dword
}

fn write_length(output: &mut Vec<u8>, count: u32, width: LengthWidth) {
    match width {
        LengthWidth::Byte => output.push(count as u8),
        LengthWidth::Word => {
            output.push(0xff);
            output.extend_from_slice(&(count as u16).to_le_bytes());
        }
        LengthWidth::Dword => {
            output.extend_from_slice(&[0xff, 0xff, 0xff]);
            output.extend_from_slice(&count.to_le_bytes());
        }
    }
}
