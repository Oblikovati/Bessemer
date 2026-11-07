pub struct LoadCase {
    pub name: String,
    pub load_mode: bool,
    pub origin: u8,
    pub raw: Vec<u8>,
    pub version: i32,
    pub count: u32,
    pub member: i32,
    pub width: u32,
    pub end: u32,
    pub fields: Vec<LoadField>,
    pub scopes: Vec<LoadScope>,
    pub counts: Vec<u32>,
    pub texts: Vec<LoadText>,
    pub mode_tail: Option<u32>,
    pub complete: bool,
    pub runtime: bool,
    pub engineering: bool,
}
pub use crate::stored_fields::StoredField as LoadField;
pub struct LoadScope {
    pub path: String,
    pub start: u32,
    pub end: u32,
}
pub struct LoadText {
    pub path: String,
    pub start: u32,
    pub end: u32,
    pub units: u32,
    pub prefix_units: u32,
    pub prefix_raw: Vec<u8>,
}
