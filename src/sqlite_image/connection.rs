use super::{SqliteLimits, engine_error, failure};
use crate::{FailureClass, Result};
use rusqlite::{Connection, MAIN_DB, config::DbConfig, limits::Limit};

pub(super) fn check_input(bytes: &[u8], limits: SqliteLimits) -> Result<()> {
    check_limits(limits)?;
    check_header(bytes, limits.maximum_image_bytes)?;
    if bytes[18..20] != [1, 1] {
        let cause = format!(
            "header versions {:?}; expected rollback-journal [1, 1]",
            &bytes[18..20]
        );
        return Err(failure(FailureClass::Unsupported, "journal mode", cause));
    }
    Ok(())
}

fn check_header(bytes: &[u8], maximum: usize) -> Result<()> {
    if bytes.len() >= 100 && bytes.len() <= maximum && bytes.starts_with(b"SQLite format 3\0") {
        return Ok(());
    }
    let magic = &bytes[..bytes.len().min(16)];
    let cause = format!(
        "{} bytes, magic {magic:?}; expected complete SQLite image of 100..={maximum} bytes",
        bytes.len()
    );
    Err(failure(FailureClass::Invalid, "header", cause))
}

fn check_limits(limits: SqliteLimits) -> Result<()> {
    let bounds = [
        limits.maximum_image_bytes,
        limits.maximum_schema_objects,
        limits.maximum_rows,
        limits.maximum_columns,
        limits.maximum_value_bytes,
        limits.maximum_output_bytes,
        limits.maximum_vm_steps,
    ];
    if bounds
        .iter()
        .all(|value| (1..=i32::MAX as usize).contains(value))
    {
        return Ok(());
    }
    let cause = format!("{limits:?}; expected every bound in 1..=2147483647");
    Err(failure(FailureClass::Invalid, "limits", cause))
}

pub(super) fn open_image(bytes: &[u8], limits: SqliteLimits) -> Result<Connection> {
    let mut database =
        Connection::open_in_memory().map_err(|error| engine_error("connection", error))?;
    configure_defensive(&database)?;
    configure_limits(&database, limits)?;
    configure_progress(&database, limits.maximum_vm_steps)?;
    database
        .deserialize_read_exact(MAIN_DB, bytes, bytes.len(), true)
        .map_err(|error| engine_error("deserialize", error))?;
    database
        .execute_batch("PRAGMA query_only=ON; PRAGMA trusted_schema=OFF; PRAGMA temp_store=MEMORY;")
        .map_err(|error| engine_error("read-only configuration", error))?;
    Ok(database)
}

fn configure_defensive(database: &Connection) -> Result<()> {
    let settings = [
        (DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true),
        (DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false),
        (DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE, false),
        (DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE, false),
    ];
    for (setting, enabled) in settings {
        database
            .set_db_config(setting, enabled)
            .map_err(|error| engine_error("configuration", error))?;
    }
    Ok(())
}

fn configure_limits(database: &Connection, limits: SqliteLimits) -> Result<()> {
    for (setting, bound) in engine_limits(limits) {
        database
            .set_limit(setting, bound)
            .map_err(|error| engine_error("engine limits", error))?;
    }
    Ok(())
}

fn engine_limits(limits: SqliteLimits) -> [(Limit, i32); 5] {
    [
        (
            Limit::SQLITE_LIMIT_LENGTH,
            limits.maximum_value_bytes as i32,
        ),
        (
            Limit::SQLITE_LIMIT_SQL_LENGTH,
            limits.maximum_value_bytes as i32,
        ),
        (Limit::SQLITE_LIMIT_COLUMN, limits.maximum_columns as i32),
        (Limit::SQLITE_LIMIT_ATTACHED, 0),
        (Limit::SQLITE_LIMIT_WORKER_THREADS, 0),
    ]
}

fn configure_progress(database: &Connection, mut remaining: usize) -> Result<()> {
    database
        .progress_handler(
            1,
            Some(move || {
                let exhausted = remaining == 0;
                remaining = remaining.saturating_sub(1);
                exhausted
            }),
        )
        .map_err(|error| engine_error("VM budget", error))?;
    Ok(())
}

pub(super) fn check_integrity(database: &Connection, expected_bytes: usize) -> Result<()> {
    check_extent(database, expected_bytes)?;
    let mut statement = database
        .prepare("PRAGMA integrity_check")
        .map_err(|error| engine_error("integrity", error))?;
    let mut rows = statement
        .query([])
        .map_err(|error| engine_error("integrity", error))?;
    let first = integrity_row(&mut rows)?;
    if first.as_deref() == Some("ok") && integrity_row(&mut rows)?.is_none() {
        return Ok(());
    }
    let cause = format!("first result {first:?}; expected exactly one ok row");
    Err(failure(FailureClass::Integrity, "integrity_check", cause))
}

fn integrity_row(rows: &mut rusqlite::Rows<'_>) -> Result<Option<String>> {
    rows.next()
        .map_err(|error| engine_error("integrity", error))?
        .map(|row| row.get::<_, String>(0))
        .transpose()
        .map_err(|error| engine_error("integrity", error))
}

fn check_extent(database: &Connection, expected_bytes: usize) -> Result<()> {
    // #234: page_size reports the connection default until the deserialized pager is read.
    let page_count = pragma_integer(database, "PRAGMA page_count")?;
    let page_size = pragma_integer(database, "PRAGMA page_size")?;
    if page_size > 0
        && page_count > 0
        && page_size.checked_mul(page_count) == Some(expected_bytes as i64)
    {
        return Ok(());
    }
    let cause = format!(
        "{page_count} pages of {page_size} bytes versus {expected_bytes} image bytes; expected exact complete extent"
    );
    Err(failure(FailureClass::Integrity, "page extent", cause))
}

fn pragma_integer(database: &Connection, sql: &str) -> Result<i64> {
    database
        .query_row(sql, [], |row| row.get(0))
        .map_err(|error| engine_error(sql, error))
}
