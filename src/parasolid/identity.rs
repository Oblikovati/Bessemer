use super::invalid;
use crate::Result;

/// Normalized numeric component of a Parasolid schema key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaNumber(u32);

impl SchemaNumber {
    /// Constructs an explicit schema or modeller number.
    ///
    /// ```
    /// assert_eq!(bessemer::parasolid::SchemaNumber::new(13006).value(), 13006);
    /// ```
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the normalized number.
    ///
    /// ```
    /// assert_eq!(bessemer::parasolid::SchemaNumber::new(37102).value(), 37102);
    /// ```
    pub const fn value(self) -> u32 {
        self.0
    }

    pub(super) fn parse(text: &str) -> Result<Self> {
        if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid(format!(
                "schema number {text:?}; expected unsigned decimal u32"
            )));
        }
        text.parse::<u32>()
            .map(Self)
            .map_err(|error| invalid(format!("schema number {text:?}: {error}; expected u32")))
    }
}

/// The three independently normalized numbers in `SCH_<modeller>_<writer>_<base>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaKey {
    /// Modeller version.
    pub modeller: SchemaNumber,
    /// Writer's schema.
    pub writer: SchemaNumber,
    /// Required base schema.
    pub base: SchemaNumber,
}

impl SchemaKey {
    /// Parses the complete key, preserving no ambiguous unparsed suffix.
    ///
    /// ```
    /// let key = bessemer::parasolid::SchemaKey::parse("SCH_3701250_37102_13006")?;
    /// assert_eq!(key.base.value(), 13006);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn parse(text: &str) -> Result<Self> {
        let parts: Vec<&str> = text.split('_').collect();
        let ["SCH", modeller, writer, base] = parts.as_slice() else {
            return Err(invalid(format!(
                "schema key {text:?}; expected SCH_<modeller>_<writer>_<base>"
            )));
        };
        Ok(Self {
            modeller: SchemaNumber::parse(modeller)?,
            writer: SchemaNumber::parse(writer)?,
            base: SchemaNumber::parse(base)?,
        })
    }

    pub(super) fn encoded(&self) -> String {
        format!(
            "SCH_{}_{}_{}",
            self.modeller.value(),
            self.writer.value(),
            self.base.value()
        )
    }
}

/// Node identity, excluding null and bounded by the 31-bit wire-index representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Validates an actual node identity, before the wire's +1 adjustment.
    ///
    /// ```
    /// assert_eq!(bessemer::parasolid::NodeId::new(32770)?.value(), 32770);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> Result<Self> {
        if (1..=0x7fff_fffe).contains(&value) {
            return Ok(Self(value));
        }
        Err(invalid(format!("node ID {value}; expected 1..=2147483646")))
    }

    /// Returns the identity without its wire adjustment.
    ///
    /// ```
    /// assert_eq!(bessemer::parasolid::NodeId::new(1)?.value(), 1);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Null or a node identity; deltas can refer to nodes in an associated partition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeReference {
    /// Wire index 1.
    Null,
    /// Wire index is this identity plus one.
    Node(NodeId),
}

impl NodeReference {
    pub(super) fn from_stored(value: u32) -> Result<Self> {
        if value == 1 {
            return Ok(Self::Null);
        }
        let identity = value
            .checked_sub(1)
            .ok_or_else(|| invalid("stored node index 0; expected 1 for null or node ID + 1"))?;
        NodeId::new(identity).map(Self::Node)
    }

    pub(super) fn stored(self) -> u32 {
        match self {
            Self::Null => 1,
            Self::Node(identity) => identity.value() + 1,
        }
    }
}
