use crate::archive::{self, Directory, Entry};
use crate::records::{invalid, region};
use crate::{BessemerError, FailureClass, Result, compression, writer};
use std::collections::BTreeMap;

/// Size and compression information from a stream's directory record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamInfo<'a> {
    /// Case-sensitive UTF-8 stream path.
    pub name: &'a str,
    /// Size on disk, before edits.
    pub compressed_size: usize,
    /// Size after decompression, before edits.
    pub uncompressed_size: u32,
}

/// An owned document container with operation-local stream replacements.
///
/// No-op serialization returns the original file, including filler bytes.
#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) original: Vec<u8>,
    pub(crate) directory: Directory,
    pub(crate) replacements: BTreeMap<String, Vec<u8>>,
    version: u32,
    key: u32,
}

impl Document {
    /// Parses directory structure without decompressing streams.
    ///
    /// ```no_run
    /// let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let header = region(bytes, 0, 8)?;
        let key = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let version = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        if version != 4 {
            return Err(BessemerError::new(
                "parse",
                "header",
                FailureClass::Unsupported,
                format!("version {version}; expected observed container version 4"),
            ));
        }
        let directory = archive::parse(bytes, version)?;
        Ok(Self {
            original: bytes.to_vec(),
            directory,
            replacements: BTreeMap::new(),
            version,
            key,
        })
    }

    /// Returns the signature-table key, read big-endian.
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// println!("{:08X}", doc.key());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn key(&self) -> u32 {
        self.key
    }

    /// Returns the container version (currently only 4 is supported).
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// assert_eq!(doc.version(), 4);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Lists original stream information in central-directory order.
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// for stream in doc.streams() { println!("{}", stream.name); }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn streams(&self) -> impl Iterator<Item = StreamInfo<'_>> {
        self.directory.entries.iter().map(|entry| StreamInfo {
            name: &entry.name,
            compressed_size: entry.stored.len(),
            uncompressed_size: entry.expanded,
        })
    }

    /// Extracts a stream and checks its expanded size and CRC-32.
    ///
    /// Allocation is bounded by its declared size; use `read_stream_limited` for untrusted files.
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// let preview = doc.read_stream("PreviewPNG")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read_stream(&self, name: &str) -> Result<Vec<u8>> {
        self.read_stream_limited(name, u32::MAX as usize)
    }

    /// Extracts a stream with an explicit maximum expanded byte count.
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// let preview = doc.read_stream_limited("PreviewPNG", 16 * 1024 * 1024)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read_stream_limited(&self, name: &str, maximum: usize) -> Result<Vec<u8>> {
        let entry = self.entry(name)?;
        let size = self
            .replacements
            .get(name)
            .map_or(entry.expanded as usize, Vec::len);
        if size > maximum {
            return Err(BessemerError::new(
                "extract",
                name,
                FailureClass::Unsupported,
                format!("expanded size {size}; expected <= {maximum}"),
            ));
        }
        if let Some(replacement) = self.replacements.get(name) {
            return Ok(replacement.clone());
        }
        compression::expand(&self.original[entry.stored.clone()], entry)
    }

    /// Replaces an existing stream without changing any other stream's stored bytes.
    ///
    /// This low-level operation does not validate the stream's model semantics.
    ///
    /// ```no_run
    /// # let mut doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// doc.replace_stream("PreviewPNG", std::fs::read("preview.png")?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn replace_stream(&mut self, name: &str, replacement: Vec<u8>) -> Result<()> {
        self.entry(name)?;
        u32::try_from(replacement.len()).map_err(|_| {
            invalid(format!(
                "replacement length {}; expected <= u32::MAX",
                replacement.len()
            ))
        })?;
        self.replacements.insert(name.to_owned(), replacement);
        Ok(())
    }

    /// Serializes the container, preserving the whole original file when no edits exist.
    ///
    /// ```no_run
    /// # let doc = bessemer::Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    /// std::fs::write("copy.sldprt", doc.to_bytes()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if self.replacements.is_empty() {
            return Ok(self.original.clone());
        }
        writer::serialize(self)
    }

    fn entry(&self, name: &str) -> Result<&Entry> {
        self.directory
            .entries
            .iter()
            .find(|entry| entry.name == name)
            .ok_or_else(|| {
                BessemerError::new(
                    "lookup",
                    name,
                    FailureClass::Missing,
                    format!("stream {name:?}; expected an existing case-sensitive name"),
                )
            })
    }
}
