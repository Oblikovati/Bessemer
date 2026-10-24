use super::{FieldDefinition, SchemaNumber, invalid};
use crate::Result;
use std::collections::BTreeMap;
use std::sync::Arc;

/// One base node type, retaining only its transmitted fields in schema order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseNodeType {
    /// Latin-1 type name.
    pub name: String,
    /// Latin-1 description.
    pub description: String,
    /// Raw variable-layout declaration.
    pub variable: u32,
    /// Transmitted field list, excluding fields marked transmit=0.
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SchemaContents {
    number: SchemaNumber,
    node_types: BTreeMap<u16, BaseNodeType>,
}

/// Explicit immutable base schema; no global cache or implicit installation lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseSchema(Arc<SchemaContents>);

impl BaseSchema {
    /// Parses a complete Latin-1 Parasolid schema file and checks its declared type/field counts.
    ///
    /// ```no_run
    /// let schema = bessemer::parasolid::BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let text: String = bytes.iter().map(|byte| char::from(*byte)).collect();
        let (number, node_types) = super::schema_text::parse(&text)?;
        Self::new(number, node_types)
    }

    /// Constructs a base schema from explicitly supplied transmitted definitions.
    ///
    /// ```no_run
    /// # let source = bessemer::parasolid::BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// let schema = bessemer::parasolid::BaseSchema::new(source.number(), source.node_types().clone())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(number: SchemaNumber, node_types: BTreeMap<u16, BaseNodeType>) -> Result<Self> {
        if node_types.is_empty() {
            return Err(invalid(
                "base schema has 0 types; expected at least one type definition",
            ));
        }
        for node in node_types.values() {
            super::definitions::latin1_length(&node.name, u32::MAX as usize)?;
            super::definitions::latin1_length(&node.description, u32::MAX as usize)?;
            super::definitions::validate_fields(&node.fields, false)?;
        }
        Ok(Self(Arc::new(SchemaContents { number, node_types })))
    }

    /// Returns the normalized schema identity required by transmit keys.
    ///
    /// ```no_run
    /// # let schema = bessemer::parasolid::BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// println!("{}", schema.number().value());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn number(&self) -> SchemaNumber {
        self.0.number
    }

    /// Borrows immutable types and their transmitted fields.
    ///
    /// ```no_run
    /// # let schema = bessemer::parasolid::BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
    /// println!("{} base types", schema.node_types().len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn node_types(&self) -> &BTreeMap<u16, BaseNodeType> {
        &self.0.node_types
    }
}
