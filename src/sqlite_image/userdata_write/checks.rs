use super::*;

pub(super) fn check_write_request(image: &SqliteImage, slot: u8, text: &str) -> Result<()> {
    check_assignment_size(slot, text, image.limits.maximum_value_bytes)?;
    for object in image.objects() {
        if object.kind == "trigger" {
            return Err(unsupported_write(
                &object.name,
                "trigger-free stored tables",
            ));
        }
        check_stored_columns(object)?;
    }
    Ok(())
}

fn check_assignment_size(slot: u8, text: &str, maximum: usize) -> Result<()> {
    if slot < 20 && text.len() <= maximum {
        return Ok(());
    }
    let cause = format!(
        "slot {slot}, {} UTF-8 bytes; expected slot 0..19 and at most {maximum} bytes",
        text.len()
    );
    Err(failure(FailureClass::Invalid, "userdata text", cause))
}

fn check_stored_columns(object: &crate::sqlite_image::SqliteSchemaObject) -> Result<()> {
    let Some(columns) = &object.columns else {
        return Ok(());
    };
    if columns
        .rows()
        .iter()
        .any(|row| row.get(6) != Some(&SqliteValue::Integer(0)))
    {
        return Err(unsupported_write(
            &object.name,
            "ordinary stored columns without hidden/generated fields",
        ));
    }
    Ok(())
}

pub(super) fn check_ordinary_tables(
    database: &Connection,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<()> {
    let rows = query::read_rows(database, "PRAGMA table_list", "table kinds", limits, used)?;
    for row in rows.rows() {
        if row.get(2).is_some_and(|kind|matches!(kind,SqliteValue::Text(bytes) if bytes.as_ref()==b"virtual" || bytes.as_ref()==b"shadow")) {
            return Err(unsupported_write("table_list","ordinary stored tables without virtual/shadow modules"));
        }
    }
    Ok(())
}

pub(super) fn check_foreign_keys(
    database: &Connection,
    limits: SqliteLimits,
    used: &mut usize,
) -> Result<()> {
    let rows = query::read_rows(
        database,
        "PRAGMA foreign_key_check",
        "foreign keys",
        limits,
        used,
    )?;
    if !rows.rows().is_empty() {
        return Err(failure(
            FailureClass::Integrity,
            "foreign keys",
            format!(
                "{} violations; expected zero complete foreign-key violations",
                rows.rows().len()
            ),
        ));
    }
    Ok(())
}

fn unsupported_write(entity: &str, expected: &str) -> crate::BessemerError {
    failure(
        FailureClass::Unsupported,
        entity,
        format!("unsupported schema {entity:?}; expected {expected}"),
    )
}
