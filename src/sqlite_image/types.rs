use std::sync::Arc;

/// Explicit positive allocation and execution limits for each image/query operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SqliteLimits {
    /// Complete original image byte bound.
    pub maximum_image_bytes: usize,
    /// Schema objects, including indexes, views and triggers.
    pub maximum_schema_objects: usize,
    /// Rows in one table or metadata result; excess rows fail the whole query.
    pub maximum_rows: usize,
    /// Columns in one table, schema definition or result.
    pub maximum_columns: usize,
    /// Bytes in one engine text/blob/row allocation.
    pub maximum_value_bytes: usize,
    /// Aggregate result accounting (value bytes and Rust cell/row/name sizes), not peak engine memory.
    pub maximum_output_bytes: usize,
    /// SQLite VM instructions per connection; exhaustion fails without partial output.
    pub maximum_vm_steps: usize,
}

/// Exact SQLite engine value without text decoding or floating-point canonicalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SqliteValue {
    /// SQL NULL, distinct from empty text/blob.
    Null,
    /// Signed SQLite integer.
    Integer(i64),
    /// Engine IEEE-754 double bits.
    RealBits(u64),
    /// Engine UTF-8 text bytes, including embedded NUL and invalid UTF-8 bytes.
    Text(Arc<[u8]>),
    /// Uninterpreted binary bytes.
    Blob(Arc<[u8]>),
}

/// Complete owned query result in SQLite scan order, including duplicate rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqliteRows {
    pub(super) columns: Arc<[String]>,
    pub(super) rows: Arc<[Vec<SqliteValue>]>,
    pub(super) accounted_bytes: usize,
}

impl SqliteRows {
    /// Borrows exact selected column names; e.g. `let names = rows.columns()`.
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    /// Borrows all complete rows; e.g. `assert!(rows.rows().is_empty())`.
    pub fn rows(&self) -> &[Vec<SqliteValue>] {
        &self.rows
    }
    /// Reports value-byte plus cell/row/name-size accounting; e.g. `let used = rows.accounted_bytes()`.
    pub fn accounted_bytes(&self) -> usize {
        self.accounted_bytes
    }
}

/// Stored schema object, ordered by SQLite's binary type/name ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqliteSchemaObject {
    /// Stored object type, without application interpretation.
    pub kind: String,
    /// Exact stored object name.
    pub name: String,
    /// Exact owning table name.
    pub table: String,
    /// Stored root-page number, including zero for non-page objects.
    pub root_page: i64,
    /// Original SQL text; implicit indexes retain NULL.
    pub sql: Option<String>,
    /// Complete PRAGMA table_xinfo rows for tables, including hidden/generated columns.
    pub columns: Option<SqliteRows>,
    /// Complete PRAGMA foreign_key_list rows for tables, retaining NULL target columns.
    pub foreign_keys: Option<SqliteRows>,
}

/// Exact engine check mode; this is not application or geometry validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqliteCheckMode {
    /// Full SQLite integrity_check, complete page extent and bounded schema reads.
    FullIntegrity,
}

/// Completed whole-image checks before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SqliteCoverage {
    /// Exact retained original image extent.
    pub wire_bytes: usize,
    /// Complete schema object count.
    pub checked_schema_objects: usize,
    /// Executed check mode.
    pub mode: SqliteCheckMode,
}

/// Explicit acceptance exclusions accompanying a successful snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqliteDiagnostic {
    /// No original native CAD loader or schema migration was executed.
    NativeApplicationNotChecked,
    /// Scan order does not establish native provider first-row priority.
    NativeRowPriorityNotEstablished,
    /// Integrity_check does not establish foreign-key semantic consistency.
    ForeignKeyConsistencyNotChecked,
}
