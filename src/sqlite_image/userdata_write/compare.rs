use super::*;

pub(super) fn check_unchanged_schema(before: &SqliteImage, after: &SqliteImage) -> Result<()> {
    if before.objects() != after.objects() {
        return Err(failure(
            FailureClass::Integrity,
            "schema",
            format!(
                "{} source versus {} result objects; expected identical schema definitions and root pages",
                before.objects().len(),
                after.objects().len()
            ),
        ));
    }
    Ok(())
}

pub(super) fn check_database_change(
    before: &UserdataDatabaseSnapshot,
    after: &UserdataDatabaseSnapshot,
    target: &UserdataWriteTarget,
    value: &SqliteValue,
) -> Result<()> {
    if before.metadata != after.metadata || before.tables.keys().ne(after.tables.keys()) {
        return Err(failure(
            FailureClass::Integrity,
            "logical database",
            "changed application metadata or table set; expected exact preservation",
        ));
    }
    for (name, original) in &before.tables {
        let result = after.tables.get(name).ok_or_else(|| {
            failure(
                FailureClass::Missing,
                name,
                "missing result table; expected complete original table set",
            )
        })?;
        check_table_change(name, original, result, target, value)?;
    }
    Ok(())
}

fn check_table_change(
    name: &str,
    before: &SqliteRows,
    after: &SqliteRows,
    target: &UserdataWriteTarget,
    value: &SqliteValue,
) -> Result<()> {
    if before.columns() != after.columns() || before.rows().len() != after.rows().len() {
        return Err(failure(
            FailureClass::Integrity,
            name,
            "changed columns or row count; expected complete original shape",
        ));
    }
    for (ordinal, (original, result)) in before.rows().iter().zip(after.rows()).enumerate() {
        let edited = (name == "tew_userdata" && ordinal == target.row).then_some(target.column);
        check_row_change(name, ordinal, original, result, edited, value)?;
    }
    Ok(())
}

fn check_row_change(
    table: &str,
    row: usize,
    before: &[SqliteValue],
    after: &[SqliteValue],
    edited: Option<usize>,
    value: &SqliteValue,
) -> Result<()> {
    if row_matches_requested(before, after, edited, value) {
        return Ok(());
    }
    let cause = format!(
        "row {row} changed outside exact TEXT assignment; expected every unselected stored value unchanged"
    );
    Err(failure(FailureClass::Integrity, table, cause))
}

fn row_matches_requested(
    before: &[SqliteValue],
    after: &[SqliteValue],
    edited: Option<usize>,
    value: &SqliteValue,
) -> bool {
    before.len() == after.len()
        && before
            .iter()
            .zip(after)
            .enumerate()
            .all(|(column, (original, result))| {
                result
                    == if edited == Some(column) {
                        value
                    } else {
                        original
                    }
            })
}
