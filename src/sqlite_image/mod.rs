//! Bounded in-memory SQLite snapshots for Electrical and CAM database carriers.
mod connection;
mod query;
mod schema;
mod types;
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;
pub use types::*;

pub(crate) fn validate_limits(limits: SqliteLimits) -> Result<()> {
    connection::check_limits(limits)
}

/// Owned checked image; no connection or mutable engine state survives publication.
#[derive(Clone, Debug)]
pub struct SqliteImage {
    original: Arc<[u8]>,
    objects: Arc<[SqliteSchemaObject]>,
    limits: SqliteLimits,
}

impl SqliteImage {
    /// Checks the complete image before publication; e.g. `SqliteImage::from_bytes(&raw, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: SqliteLimits) -> Result<Self> {
        connection::check_input(bytes, limits)?;
        let database = connection::open_image(bytes, limits)?;
        connection::check_integrity(&database, bytes.len())?;
        let objects = schema::read_objects(&database, limits)?;
        Ok(Self {
            original: bytes.into(),
            objects: objects.into(),
            limits,
        })
    }
    /// Borrows ordered schema metadata; e.g. `let objects = image.objects()`.
    pub fn objects(&self) -> &[SqliteSchemaObject] {
        &self.objects
    }
    /// Borrows the original image without engine rewriting; e.g. `assert_eq!(image.as_bytes(), raw)`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.original
    }
    /// Reads every row of one exact table; e.g. `let rows = image.table_rows("tew_project")?`.
    pub fn table_rows(&self, name: &str) -> Result<SqliteRows> {
        let object = self
            .objects
            .iter()
            .find(|object| object.kind == "table" && object.name == name);
        if object.is_none() {
            return Err(failure(
                FailureClass::Missing,
                name,
                format!("table {name:?}; expected exact stored table name"),
            ));
        }
        let database = connection::open_image(&self.original, self.limits)?;
        let sql = format!("SELECT * FROM {}", query::quote_identifier(name));
        query::read_rows(&database, &sql, name, self.limits, &mut 0)
    }
    /// Reports completed checks and explicit exclusions; e.g. `let checked = image.coverage()`.
    pub fn coverage(&self) -> SqliteCoverage {
        SqliteCoverage {
            wire_bytes: self.original.len(),
            checked_schema_objects: self.objects.len(),
            mode: SqliteCheckMode::FullIntegrity,
        }
    }
    /// Keeps application and ordering claims separate; e.g. `let exclusions = image.diagnostics()`.
    pub fn diagnostics(&self) -> &[SqliteDiagnostic] {
        &[
            SqliteDiagnostic::NativeApplicationNotChecked,
            SqliteDiagnostic::NativeRowPriorityNotEstablished,
            SqliteDiagnostic::ForeignKeyConsistencyNotChecked,
        ]
    }
}

fn failure(class: FailureClass, entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("SQLite image", entity, class, cause)
}

fn engine_error(entity: &str, error: rusqlite::Error) -> BessemerError {
    failure(
        FailureClass::Invalid,
        entity,
        format!("SQLite {error}; expected complete image/query within declared limits"),
    )
}
