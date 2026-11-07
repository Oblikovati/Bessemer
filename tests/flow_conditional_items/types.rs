pub struct ConditionalCase {
    pub name: String,
    pub identifier: u32,
    pub version: i32,
    pub scale: u32,
    pub outer_version: Option<i32>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub fields: Vec<ConditionalField>,
    pub scopes: Vec<ConditionalScope>,
    pub counts: Vec<u32>,
    pub projections: Vec<(String, bool)>,
    pub complete: bool,
    pub runtime: bool,
    pub engineering: bool,
}
pub use crate::stored_fields::StoredField as ConditionalField;
pub struct ConditionalScope {
    pub path: String,
    pub start: u32,
    pub end: u32,
}
