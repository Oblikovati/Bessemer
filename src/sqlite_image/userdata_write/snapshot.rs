use super::*;

pub(super) fn read_database_snapshot(
    database: &Connection,
    image: &SqliteImage,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<UserdataDatabaseSnapshot> {
    checks::check_ordinary_tables(database, limits, used)?;
    checks::check_foreign_keys(database, limits, used)?;
    let metadata = read_header_metadata(database, limits, used)?;
    let mut snapshot = empty_database_snapshot(metadata);
    for object in image
        .objects()
        .iter()
        .filter(|object| object.kind == "table")
    {
        let sql = format!("SELECT * FROM {}", query::quote_identifier(&object.name));
        let rows = query::read_rows(database, &sql, &object.name, limits, used)?;
        snapshot.rows += rows.rows().len();
        snapshot.cells += rows.rows().iter().map(Vec::len).sum::<usize>();
        snapshot.tables.insert(object.name.clone(), rows);
    }
    Ok(snapshot)
}

fn empty_database_snapshot(metadata: Vec<SqliteRows>) -> UserdataDatabaseSnapshot {
    UserdataDatabaseSnapshot {
        tables: BTreeMap::new(),
        metadata,
        rows: 0,
        cells: 0,
    }
}

fn read_header_metadata(
    database: &Connection,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<Vec<SqliteRows>> {
    // #234: initialize the pager before reading its actual page_size.
    query::read_rows(
        database,
        "PRAGMA page_count",
        "page initialization",
        limits,
        used,
    )?;
    let statements = [
        "PRAGMA application_id",
        "PRAGMA user_version",
        "PRAGMA encoding",
        "PRAGMA auto_vacuum",
        "PRAGMA page_size",
        "PRAGMA schema_version",
    ];
    statements
        .iter()
        .map(|sql| query::read_rows(database, sql, "application metadata", limits, used))
        .collect()
}
