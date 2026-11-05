/// Caller-declared context for the conditional installed header grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseHeaderProfile {
    /// The documented default Windows provider's 512-byte header read.
    /// Selecting this profile does not discover or execute the original provider.
    DefaultWindows512,
}

/// Raw persisted header fields; names and packed time have no text/calendar normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseHeaderFields {
    /// Stored BE32 time word, including zero-based month provenance and invalid component patterns.
    pub packed_time: u32,
    /// Stored S; admitted values are powers of two in 32 through 65536.
    pub sector_size: u32,
    /// Stored P; admitted values are every integer in 512 through 65536.
    pub page_size: u32,
    /// Raw name bytes under the declared u16 count, including embedded NUL and non-UTF-8.
    pub engine_name: Vec<u8>,
}
