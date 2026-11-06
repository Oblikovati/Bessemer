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
pub struct ConditionalField {
    pub path: String,
    pub offset: u32,
    pub encoding: u8,
    pub count: u32,
    pub raw: Vec<u8>,
}
pub struct ConditionalScope {
    pub path: String,
    pub start: u32,
    pub end: u32,
}
impl ConditionalField {
    pub fn width(&self) -> usize {
        match self.encoding {
            0 => 1,
            1 | 2 => 4,
            3 => 8,
            other => panic!("Fixture encoding {other}; expected0..3"),
        }
    }
}
