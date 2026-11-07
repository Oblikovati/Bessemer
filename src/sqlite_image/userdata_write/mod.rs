mod checks;
mod compare;
mod snapshot;
mod update;
use super::{
    SqliteImage, SqliteLimits, SqliteRows, SqliteValue, connection, engine_error, failure, query,
};
use crate::{FailureClass, Result};
use rusqlite::Connection;
use std::collections::BTreeMap;

pub(crate) struct SqliteUserdataMutation {
    pub image: SqliteImage,
    pub before: SqliteValue,
    pub after: SqliteValue,
    pub checked_tables: usize,
    pub checked_rows: usize,
    pub checked_cells: usize,
    pub accounted_bytes: usize,
}

struct UserdataWriteTarget {
    row: usize,
    column: usize,
    previous: SqliteValue,
}

struct UserdataDatabaseSnapshot {
    tables: BTreeMap<String, SqliteRows>,
    metadata: Vec<SqliteRows>,
    rows: usize,
    cells: usize,
}

struct PreparedUserdataWrite {
    database: Connection,
    before: UserdataDatabaseSnapshot,
    target: UserdataWriteTarget,
    replacement: SqliteValue,
    used: usize,
}

impl SqliteImage {
    pub(crate) fn edit_electrical_userdata(
        &self,
        key: [i32; 2],
        slot: u8,
        text: &str,
    ) -> Result<SqliteUserdataMutation> {
        checks::check_write_request(self, slot, text)?;
        PreparedUserdataWrite::prepare(self, key, slot, text)?.execute(self, key, slot, text)
    }
}

impl PreparedUserdataWrite {
    fn prepare(image: &SqliteImage, key: [i32; 2], slot: u8, text: &str) -> Result<Self> {
        let database = connection::open_writable_image(&image.original, image.limits)?;
        let mut used = 0;
        let before = snapshot::read_database_snapshot(&database, image, image.limits, &mut used)?;
        let target =
            update::resolve_write_target(&database, &before, key, slot, image.limits, &mut used)?;
        let replacement = SqliteValue::Text(text.as_bytes().into());
        Ok(Self {
            database,
            before,
            target,
            replacement,
            used,
        })
    }

    fn execute(
        mut self,
        source: &SqliteImage,
        key: [i32; 2],
        slot: u8,
        text: &str,
    ) -> Result<SqliteUserdataMutation> {
        if self.target.previous == self.replacement {
            return Ok(self.publish(source.clone()));
        }
        update::commit_text_update(&mut self.database, key, slot, text, source.limits)?;
        let result = self.serialized_result(source)?;
        self.verify_result(&result)?;
        Ok(self.publish(result))
    }

    fn serialized_result(&mut self, source: &SqliteImage) -> Result<SqliteImage> {
        let bytes = self
            .database
            .serialize(rusqlite::MAIN_DB)
            .map_err(|error| engine_error("serialize userdata", error))?
            .to_vec();
        let result = SqliteImage::from_bytes(&bytes, source.limits)?;
        compare::check_unchanged_schema(source, &result)?;
        Ok(result)
    }

    fn verify_result(&mut self, result: &SqliteImage) -> Result<()> {
        let database = connection::open_image(result.as_bytes(), result.limits)?;
        let after =
            snapshot::read_database_snapshot(&database, result, result.limits, &mut self.used)?;
        compare::check_database_change(&self.before, &after, &self.target, &self.replacement)
    }

    fn publish(self, image: SqliteImage) -> SqliteUserdataMutation {
        mutation_result(image, self.before, self.target, self.replacement, self.used)
    }
}

fn mutation_result(
    image: SqliteImage,
    before: UserdataDatabaseSnapshot,
    target: UserdataWriteTarget,
    after: SqliteValue,
    accounted_bytes: usize,
) -> SqliteUserdataMutation {
    SqliteUserdataMutation {
        image,
        before: target.previous,
        after,
        checked_tables: before.tables.len(),
        checked_rows: before.rows,
        checked_cells: before.cells,
        accounted_bytes,
    }
}
