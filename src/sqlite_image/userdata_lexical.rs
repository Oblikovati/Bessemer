use super::{SqliteRows, SqliteValue, query};
use crate::Result;

impl SqliteRows {
    pub(crate) fn electrical_object_rows(&self, object_id: i32) -> Result<Self> {
        let column = self.columns.iter().position(|name| name == "use_objectid");
        let rows: Vec<_> = self
            .rows
            .iter()
            .filter(|row| {
                column
                    .and_then(|index| row.get(index))
                    .is_some_and(|value| lexical_object_match(value, object_id))
            })
            .cloned()
            .collect();
        let accounted_bytes = selection_accounting(&self.columns, &rows, self.accounted_bytes)?;
        Ok(Self {
            columns: self.columns.clone(),
            rows: rows.into(),
            accounted_bytes,
        })
    }
}

fn lexical_object_match(value: &SqliteValue, object_id: i32) -> bool {
    match value {
        SqliteValue::Integer(value) => *value == i64::from(object_id),
        SqliteValue::RealBits(bits) => f64::from_bits(*bits) == f64::from(object_id),
        _ => false,
    }
}

fn selection_accounting(
    columns: &[String],
    rows: &[Vec<SqliteValue>],
    maximum: usize,
) -> Result<usize> {
    let mut used = 0;
    for name in columns {
        charge_lexical_bytes(&mut used, name.len() + size_of::<String>(), maximum)?;
    }
    for row in rows {
        charge_lexical_bytes(&mut used, size_of::<Vec<SqliteValue>>(), maximum)?;
        for value in row {
            let bytes = lexical_value_bytes(value) + size_of::<SqliteValue>();
            charge_lexical_bytes(&mut used, bytes, maximum)?;
        }
    }
    Ok(used)
}

fn charge_lexical_bytes(used: &mut usize, bytes: usize, maximum: usize) -> Result<()> {
    query::charge_accounted_bytes(used, bytes, maximum, "userdata selection")
}

fn lexical_value_bytes(value: &SqliteValue) -> usize {
    match value {
        SqliteValue::Text(bytes) | SqliteValue::Blob(bytes) => bytes.len(),
        _ => 0,
    }
}
