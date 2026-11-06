use super::{SqliteLimits, SqliteRows, SqliteValue, engine_error, failure};
use crate::{FailureClass, Result};
use rusqlite::{Connection, Row, types::ValueRef};

pub(super) fn quote_identifier(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

pub(super) fn charge(
    used: &mut usize,
    amount: usize,
    limits: SqliteLimits,
    entity: &str,
) -> Result<()> {
    charge_accounted_bytes(used, amount, limits.maximum_output_bytes, entity)
}

pub(super) fn charge_accounted_bytes(
    used: &mut usize,
    amount: usize,
    maximum: usize,
    entity: &str,
) -> Result<()> {
    let total = used.checked_add(amount);
    if total.is_none_or(|count| count > maximum) {
        return Err(failure(
            FailureClass::Invalid,
            entity,
            format!("{used} + {amount} output bytes; expected at most {maximum}"),
        ));
    }
    *used += amount;
    Ok(())
}

pub(super) fn read_rows(
    database: &Connection,
    sql: &str,
    entity: &str,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<SqliteRows> {
    read_bound_rows(database, sql, [], entity, limits, used)
}

pub(super) fn read_bound_rows<P: rusqlite::Params>(
    database: &Connection,
    sql: &str,
    parameters: P,
    entity: &str,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<SqliteRows> {
    let before = *used;
    let mut statement = database
        .prepare(sql)
        .map_err(|error| engine_error(entity, error))?;
    check_column_count(statement.column_count(), limits, entity)?;
    let columns = read_columns(&statement, limits, used, entity)?;
    let rows = collect_rows(&mut statement, parameters, limits, used, entity)?;
    Ok(SqliteRows {
        columns: columns.into(),
        rows: rows.into(),
        accounted_bytes: *used - before,
    })
}

fn collect_rows<P: rusqlite::Params>(
    statement: &mut rusqlite::Statement<'_>,
    parameters: P,
    limits: SqliteLimits,
    used: &mut usize,
    entity: &str,
) -> Result<Vec<Vec<SqliteValue>>> {
    let count = statement.column_count();
    let mut cursor = statement
        .query(parameters)
        .map_err(|error| engine_error(entity, error))?;
    let mut rows = Vec::new();
    while let Some(row) = cursor.next().map_err(|error| engine_error(entity, error))? {
        check_row_count(rows.len(), limits, entity)?;
        rows.push(read_row(row, count, limits, used, entity)?);
    }
    Ok(rows)
}

fn check_column_count(count: usize, limits: SqliteLimits, entity: &str) -> Result<()> {
    if count <= limits.maximum_columns {
        return Ok(());
    }
    let cause = format!(
        "{count} columns; expected at most {}",
        limits.maximum_columns
    );
    Err(failure(FailureClass::Invalid, entity, cause))
}

fn read_columns(
    statement: &rusqlite::Statement<'_>,
    limits: SqliteLimits,
    used: &mut usize,
    entity: &str,
) -> Result<Vec<String>> {
    let mut columns = Vec::new();
    for name in statement.column_names() {
        charge(
            used,
            name.len() + std::mem::size_of::<String>(),
            limits,
            entity,
        )?;
        columns.push(name.to_owned());
    }
    Ok(columns)
}

fn check_row_count(count: usize, limits: SqliteLimits, entity: &str) -> Result<()> {
    if count >= limits.maximum_rows {
        return Err(failure(
            FailureClass::Invalid,
            entity,
            format!(
                "more than {count} rows; expected at most {}",
                limits.maximum_rows
            ),
        ));
    }
    Ok(())
}

fn read_row(
    row: &Row<'_>,
    count: usize,
    limits: SqliteLimits,
    used: &mut usize,
    entity: &str,
) -> Result<Vec<SqliteValue>> {
    charge(
        used,
        std::mem::size_of::<Vec<SqliteValue>>(),
        limits,
        entity,
    )?;
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let raw = row
            .get_ref(index)
            .map_err(|error| engine_error(entity, error))?;
        values.push(read_value(raw, limits, used, entity)?);
    }
    Ok(values)
}

fn read_value(
    raw: ValueRef<'_>,
    limits: SqliteLimits,
    used: &mut usize,
    entity: &str,
) -> Result<SqliteValue> {
    let bytes = match raw {
        ValueRef::Text(bytes) | ValueRef::Blob(bytes) => bytes.len(),
        _ => 0,
    };
    check_value_bytes(bytes, limits, entity)?;
    charge(
        used,
        bytes + std::mem::size_of::<SqliteValue>(),
        limits,
        entity,
    )?;
    Ok(owned_value(raw))
}

fn check_value_bytes(bytes: usize, limits: SqliteLimits, entity: &str) -> Result<()> {
    if bytes <= limits.maximum_value_bytes {
        return Ok(());
    }
    let cause = format!(
        "{bytes} value bytes; expected at most {}",
        limits.maximum_value_bytes
    );
    Err(failure(FailureClass::Invalid, entity, cause))
}

fn owned_value(raw: ValueRef<'_>) -> SqliteValue {
    match raw {
        ValueRef::Null => SqliteValue::Null,
        ValueRef::Integer(value) => SqliteValue::Integer(value),
        ValueRef::Real(value) => SqliteValue::RealBits(value.to_bits()),
        ValueRef::Text(bytes) => SqliteValue::Text(bytes.into()),
        ValueRef::Blob(bytes) => SqliteValue::Blob(bytes.into()),
    }
}
