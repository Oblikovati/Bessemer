pub struct ReferenceCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub kind: i32,
    pub keys: Vec<Vec<u8>>,
    pub key: Vec<u8>,
    pub state: Option<u8>,
    pub loaded_state: Option<u8>,
    pub references: Vec<Vec<u8>>,
    pub end: u32,
    pub tail: Vec<u8>,
    pub runtime: bool,
}
