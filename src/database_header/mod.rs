//! Conditional 512-byte database header buffers from clause 9.2 / C-3597.
//!
//! Profile selection declares the documented provider context. This codec checks
//! the header prefix and retains the uninterpreted tail; it does not validate a database.
mod codec;
mod time;
mod types;
pub use time::DatabaseTimeComponents;
pub use types::{DatabaseHeaderFields, DatabaseHeaderProfile};

use crate::Result;

/// An immutable header prefix and its owned, uninterpreted buffer remainder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseHeaderBuffer {
    profile: DatabaseHeaderProfile,
    fields: DatabaseHeaderFields,
    uninterpreted_tail: Box<[u8]>,
}

impl DatabaseHeaderBuffer {
    /// Checks one complete selected buffer; e.g. `DatabaseHeaderBuffer::from_bytes(&bytes, profile)?`.
    pub fn from_bytes(bytes: &[u8], profile: DatabaseHeaderProfile) -> Result<Self> {
        let (fields, tail) = codec::decode(bytes)?;
        Self::new(profile, fields, tail)
    }

    /// Constructs all 512 bytes from explicit fields and tail; e.g. `DatabaseHeaderBuffer::new(profile, fields, tail)?`.
    pub fn new(
        profile: DatabaseHeaderProfile,
        fields: DatabaseHeaderFields,
        uninterpreted_tail: Vec<u8>,
    ) -> Result<Self> {
        codec::validate(&fields, uninterpreted_tail.len())?;
        Ok(Self {
            profile,
            fields,
            uninterpreted_tail: uninterpreted_tail.into_boxed_slice(),
        })
    }

    /// Returns the declared provider profile; e.g. `assert_eq!(buffer.profile(), profile)`.
    pub fn profile(&self) -> DatabaseHeaderProfile {
        self.profile
    }

    /// Borrows the immutable stored fields; e.g. `let word = buffer.fields().packed_time`.
    pub fn fields(&self) -> &DatabaseHeaderFields {
        &self.fields
    }

    /// Exposes bytes beyond the interpreted name without assigning meaning; e.g. `let tail = buffer.uninterpreted_tail()`.
    pub fn uninterpreted_tail(&self) -> &[u8] {
        &self.uninterpreted_tail
    }

    /// Reports only the interpreted prefix extent; e.g. `assert!(buffer.interpreted_prefix_length() <= 512)`.
    pub fn interpreted_prefix_length(&self) -> usize {
        25 + self.fields.engine_name.len()
    }

    /// Reconstructs the complete buffer, retaining its tail; e.g. `assert_eq!(buffer.to_bytes(), bytes)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        codec::encode(&self.fields, &self.uninterpreted_tail)
    }
}
