//! Typed Parasolid binary transmit partitions and rollback deltas (clause 10).
mod decode;
mod definitions;
mod encode;
mod field_decode;
mod field_encode;
mod field_validate;
mod identity;
mod layout_codec;
mod record_decode;
mod schema;
mod schema_text;
mod types;
mod validate;
mod wire;

use crate::archive_codec::PreservedContents;
pub use crate::archive_codec::StoredDouble;
use crate::{BessemerError, FailureClass, Result};
pub use identity::{NodeId, NodeReference, SchemaKey, SchemaNumber};
pub use schema::{BaseNodeType, BaseSchema};
use std::sync::Arc;
pub use types::{
    DeltaChanges, DeltaSection, FieldDefinition, FieldExtent, FieldKind, FieldValues,
    LayoutDefinition, LayoutEdit, NewNode, NodeRecord, RecordList, TransmitBody, TransmitContents,
    TransmitFormat, TransmitLimits, TransmitPointer,
};

/// Complete immutable transmit archive, retaining its explicit base schema and effective layouts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransmitFile {
    base: BaseSchema,
    contents: PreservedContents<TransmitContents>,
    effective: Arc<validate::EffectiveLayouts>,
}

impl TransmitFile {
    /// Decodes and validates an entire file with explicit schema and allocation limits.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// let limits = TransmitLimits { maximum_records: 1_000_000, maximum_field_elements: 10_000_000 };
    /// let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, limits)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(bytes: &[u8], base: &BaseSchema, limits: TransmitLimits) -> Result<Self> {
        let contents = decode::file(bytes, base, limits)?;
        let effective = Arc::new(validate::contents(base, &contents)?);
        Ok(Self {
            base: base.clone(),
            contents: PreservedContents::decoded(bytes, contents),
            effective,
        })
    }

    /// Constructs a complete archive after validating every layout, field, and section boundary.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// # let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, TransmitLimits { maximum_records: 1000000, maximum_field_elements: 10000000 })?;
    /// let rebuilt = TransmitFile::new(&base, file.contents().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(base: &BaseSchema, contents: TransmitContents) -> Result<Self> {
        let effective = Arc::new(validate::contents(base, &contents)?);
        Ok(Self {
            base: base.clone(),
            contents: PreservedContents::constructed(contents),
            effective,
        })
    }

    /// Borrows all stored fields without granting mutable aliases to published results.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// # let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, TransmitLimits { maximum_records: 1000000, maximum_field_elements: 10000000 })?;
    /// println!("{} layouts", file.contents().layouts.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn contents(&self) -> &TransmitContents {
        self.contents.current()
    }

    /// Resolves positional record fields to their effective names and primitive definitions.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// # let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, TransmitLimits { maximum_records: 1000000, maximum_field_elements: 10000000 })?;
    /// println!("{} PARTITION fields", file.layout_fields(3)?.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn layout_fields(&self, node_type: u16) -> Result<&[FieldDefinition]> {
        self.effective
            .get(&node_type)
            .map(Vec::as_slice)
            .ok_or_else(|| BessemerError {
                operation: "transmit layout lookup",
                entity: format!("node type {node_type}"),
                class: FailureClass::Missing,
                cause: "layout absent; expected an encountered node type".to_owned(),
            })
    }

    /// Validates an owned edit and returns an isolated archive, leaving the source unchanged on failure.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// # let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, TransmitLimits { maximum_records: 1000000, maximum_field_elements: 10000000 })?;
    /// let edited = file.with_contents(file.contents().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_contents(&self, contents: TransmitContents) -> Result<Self> {
        let effective = Arc::new(validate::contents(&self.base, &contents)?);
        Ok(Self {
            base: self.base.clone(),
            contents: self.contents.edited(contents),
            effective,
        })
    }

    /// Writes every typed record, returning original bytes for equivalent edits and reverts.
    ///
    /// ```no_run
    /// # use bessemer::parasolid::*;
    /// # let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # let file = TransmitFile::from_bytes(&std::fs::read("partition.x_b")?, &base, TransmitLimits { maximum_records: 1000000, maximum_field_elements: 10000000 })?;
    /// std::fs::write("copy.x_b", file.to_bytes()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.contents.original_bytes() {
            return Ok(bytes.to_vec());
        }
        encode::file(self.contents(), &self.effective)
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "Parasolid transmit",
        entity: "transmit archive".to_owned(),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}

fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "Parasolid transmit",
        entity: "transmit archive".to_owned(),
        class: FailureClass::Unsupported,
        cause: cause.into(),
    }
}
