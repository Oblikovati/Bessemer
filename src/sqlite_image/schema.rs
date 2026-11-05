use super::{SqliteLimits, SqliteSchemaObject, SqliteValue, failure, query};
use crate::{FailureClass, Result};
use rusqlite::Connection;

pub(super) fn read_objects(
    database: &Connection,
    limits: SqliteLimits,
) -> Result<Vec<SqliteSchemaObject>> {
    let mut used = 0;
    let raw = schema_rows(database, limits, &mut used)?;
    let mut objects = Vec::new();
    for row in raw.rows() {
        let mut object = object_from_row(row)?;
        read_table_metadata(database, &mut object, limits, &mut used)?;
        objects.push(object);
    }
    Ok(objects)
}

fn schema_rows(
    database: &Connection,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<super::SqliteRows> {
    let limits = SqliteLimits {
        maximum_rows: limits.maximum_schema_objects,
        ..limits
    };
    let sql = "SELECT type,name,tbl_name,rootpage,sql FROM sqlite_schema ORDER BY type,name";
    query::read_rows(database, sql, "schema", limits, used)
}

fn read_table_metadata(
    database: &Connection,
    object: &mut SqliteSchemaObject,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<()> {
    if object.kind != "table" {
        return Ok(());
    }
    let columns = pragma_rows(database, "table_xinfo", &object.name, limits, used)?;
    for column in columns.rows() {
        check_column_name(column)?;
    }
    object.columns = Some(columns);
    object.foreign_keys = Some(pragma_rows(
        database,
        "foreign_key_list",
        &object.name,
        limits,
        used,
    )?);
    Ok(())
}

fn check_column_name(column: &[SqliteValue]) -> Result<()> {
    let Some(name) = column.get(1) else {
        return Err(failure(
            FailureClass::Invalid,
            "table_xinfo",
            format!("{column:?}; expected name at column 1"),
        ));
    };
    // Rusqlite's column-name accessor panics on invalid UTF-8; reject it before SELECT *.
    schema_text(name)?;
    Ok(())
}

fn pragma_rows(
    database: &Connection,
    pragma: &str,
    name: &str,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<super::SqliteRows> {
    let sql = format!("PRAGMA {pragma}({})", query::quote_identifier(name));
    query::read_rows(database, &sql, name, limits, used)
}

fn object_from_row(row: &[SqliteValue]) -> Result<SqliteSchemaObject> {
    let [kind, name, table, SqliteValue::Integer(root_page), sql] = row else {
        return Err(failure(
            FailureClass::Invalid,
            "schema row",
            format!("{row:?}; expected type,name,tbl_name,integer rootpage,nullable SQL"),
        ));
    };
    Ok(SqliteSchemaObject {
        kind: schema_text(kind)?,
        name: schema_text(name)?,
        table: schema_text(table)?,
        root_page: *root_page,
        sql: nullable_text(sql)?,
        columns: None,
        foreign_keys: None,
    })
}

fn nullable_text(value: &SqliteValue) -> Result<Option<String>> {
    if *value == SqliteValue::Null {
        return Ok(None);
    }
    schema_text(value).map(Some)
}

fn schema_text(value: &SqliteValue) -> Result<String> {
    let SqliteValue::Text(bytes) = value else {
        return Err(failure(
            FailureClass::Invalid,
            "schema text",
            format!("{value:?}; expected UTF-8 SQLite text"),
        ));
    };
    String::from_utf8(bytes.to_vec()).map_err(|error| {
        failure(
            FailureClass::Unsupported,
            "schema UTF-8",
            format!("{error}; expected UTF-8 schema identifier/SQL"),
        )
    })
}
