use super::*;

pub(super) fn resolve_write_target(
    database: &Connection,
    snapshot: &UserdataDatabaseSnapshot,
    key: [i32; 2],
    slot: u8,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<UserdataWriteTarget> {
    let sql = "SELECT * FROM tew_userdata WHERE use_objectid=?1 AND use_objectno=?2";
    let rows = query::read_bound_rows(database, sql, key, "userdata target", limits, used)?;
    check_unique_target(&rows, key)?;
    let table = snapshot.tables.get("tew_userdata").ok_or_else(|| {
        failure(
            FailureClass::Missing,
            "tew_userdata",
            "absent ordinary table; expected exact tew_userdata spelling",
        )
    })?;
    target_from_rows(table, &rows, slot)
}

fn check_unique_target(rows: &SqliteRows, key: [i32; 2]) -> Result<()> {
    let count = rows.rows().len();
    if count == 1 {
        return Ok(());
    }
    let class = if count == 0 {
        FailureClass::Missing
    } else {
        FailureClass::Invalid
    };
    Err(failure(
        class,
        "userdata key",
        format!("key {key:?} matched {count} rows; expected exactly one complete match"),
    ))
}

fn target_from_rows(
    table: &SqliteRows,
    selected: &SqliteRows,
    slot: u8,
) -> Result<UserdataWriteTarget> {
    let name = format!("use_data{slot}");
    let column = target_column(table, &name)?;
    let values = selected.rows().first().ok_or_else(|| {
        failure(
            FailureClass::Missing,
            "userdata key",
            "absent selected row; expected exactly one match",
        )
    })?;
    let row = target_ordinal(table, values)?;
    let previous = target_previous(values, column, &name)?;
    Ok(UserdataWriteTarget {
        row,
        column,
        previous,
    })
}

fn target_previous(values: &[SqliteValue], column: usize, name: &str) -> Result<SqliteValue> {
    values.get(column).cloned().ok_or_else(|| {
        failure(
            FailureClass::Integrity,
            name,
            "missing selected value; expected complete row width",
        )
    })
}

fn target_column(table: &SqliteRows, name: &str) -> Result<usize> {
    table
        .columns()
        .iter()
        .position(|column| column == name)
        .ok_or_else(|| {
            failure(
                FailureClass::Missing,
                name,
                format!("missing column {name:?}; expected stored userdata text slot"),
            )
        })
}

fn target_ordinal(table: &SqliteRows, values: &[SqliteValue]) -> Result<usize> {
    table
        .rows()
        .iter()
        .position(|row| row == values)
        .ok_or_else(|| {
            failure(
                FailureClass::Integrity,
                "userdata row",
                "selected row absent from complete snapshot; expected identical row values",
            )
        })
}

pub(super) fn commit_text_update(
    database: &mut Connection,
    key: [i32; 2],
    slot: u8,
    text: &str,
    limits: SqliteLimits,
) -> Result<()> {
    bound_page_growth(database, limits)?;
    let sql = format!(
        "UPDATE tew_userdata SET use_data{slot}=?1 WHERE use_objectid=?2 AND use_objectno=?3"
    );
    let transaction = database
        .transaction()
        .map_err(|error| engine_error("begin userdata update", error))?;
    let changed = transaction
        .execute(&sql, rusqlite::params![text, key[0], key[1]])
        .map_err(|error| engine_error("update userdata", error))?;
    if changed != 1 {
        return Err(failure(
            FailureClass::Integrity,
            "userdata update",
            format!("{changed} changed rows; expected exactly one matching row"),
        ));
    }
    transaction
        .commit()
        .map_err(|error| engine_error("commit userdata update", error))
}

fn bound_page_growth(database: &Connection, limits: SqliteLimits) -> Result<()> {
    let maximum = requested_page_limit(database, limits)?;
    let sql = format!("PRAGMA max_page_count={maximum}");
    let accepted: i64 = database
        .query_row(&sql, [], |row| row.get(0))
        .map_err(|error| engine_error("page limit", error))?;
    if usize::try_from(accepted).ok() == Some(maximum) {
        return Ok(());
    }
    Err(failure(
        FailureClass::Invalid,
        "page limit",
        format!("accepted {accepted}; expected at most {maximum} pages"),
    ))
}

fn requested_page_limit(database: &Connection, limits: SqliteLimits) -> Result<usize> {
    let size: i64 = database
        .query_row("PRAGMA page_size", [], |row| row.get(0))
        .map_err(|error| engine_error("page size", error))?;
    let size = usize::try_from(size)
        .ok()
        .filter(|size| *size > 0)
        .ok_or_else(|| {
            failure(
                FailureClass::Invalid,
                "page size",
                format!("{size}; expected positive page size"),
            )
        })?;
    Ok(limits.maximum_image_bytes / size)
}
