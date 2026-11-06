use super::{ElectricalArchive, ElectricalMemberId, ElectricalProjectDatabase};
use crate::sqlite_image::{
    SqliteCoverage, SqliteDiagnostic, SqliteImage, SqliteLimits, SqliteRows,
};
use crate::{BessemerError, Result};

/// Exact signed-long key passed to the documented native SQL predicate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataKey {
    /// Object identifier, without identity-gate normalization.
    pub object_id: i32,
    /// Object number, including negative values.
    pub object_no: i32,
}

/// Selected native ID-gate outcome after provider conversion to signed long.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataTransition {
    /// Stored ID after the gate; rejected zero retains the current ID.
    pub stored_id: i32,
    /// The selected delegated reader's success branch, distinct from SQL row existence.
    pub delegated_success: bool,
}

/// Projects the documented zero gate and negative normalization; e.g. `electrical_userdata_id_transition(7, 0)`.
pub fn electrical_userdata_id_transition(
    current_id: i32,
    loaded_id: i32,
) -> ElectricalUserdataTransition {
    if loaded_id == 0 && current_id != 0 {
        return ElectricalUserdataTransition {
            stored_id: current_id,
            delegated_success: false,
        };
    }
    let stored_id = if loaded_id < 0 { -1 } else { loaded_id };
    ElectricalUserdataTransition {
        stored_id,
        delegated_success: stored_id != -1,
    }
}

/// Immutable complete two-part SQL query with checked image and parent revision retained.
#[derive(Clone, Debug)]
pub struct ElectricalUserdataQuery {
    source: ElectricalArchive,
    member: ElectricalMemberId,
    key: ElectricalUserdataKey,
    image: SqliteImage,
    rows: SqliteRows,
}

impl ElectricalArchive {
    /// Runs the fixed documented SQLite predicate without provider coercion; e.g. `let matches = archive.userdata_by_key(&id, key, limits)?`.
    pub fn userdata_by_key(
        &self,
        member: &ElectricalMemberId,
        key: ElectricalUserdataKey,
        limits: SqliteLimits,
    ) -> Result<ElectricalUserdataQuery> {
        let image = self.sqlite_member(member, limits).map_err(userdata_error)?;
        let rows = image
            .electrical_userdata_rows(key.object_id, key.object_no)
            .map_err(userdata_error)?;
        Ok(ElectricalUserdataQuery {
            source: self.clone(),
            member: member.clone(),
            key,
            image,
            rows,
        })
    }
}

impl ElectricalUserdataQuery {
    /// Borrows every matching row and the exact 22-column projection; e.g. `let rows = query.rows()`.
    pub fn rows(&self) -> &SqliteRows {
        &self.rows
    }
    /// Reports row existence without applying the ID gate; e.g. `let found = query.exists()`.
    pub fn exists(&self) -> bool {
        !self.rows.rows().is_empty()
    }
    /// Borrows the retained parent revision; e.g. `let raw = query.source().read_member(query.member())?`.
    pub fn source(&self) -> &ElectricalArchive {
        &self.source
    }
    /// Borrows the exact selected member occurrence; e.g. `let member = query.member()`.
    pub fn member(&self) -> &ElectricalMemberId {
        &self.member
    }
    /// Returns the unchanged signed key; e.g. `assert_eq!(query.key(), requested)`.
    pub fn key(&self) -> ElectricalUserdataKey {
        self.key
    }
    /// Reports full image integrity/schema checks; e.g. `let checked = query.coverage()`.
    pub fn coverage(&self) -> SqliteCoverage {
        self.image.coverage()
    }
    /// Borrows native-priority/application exclusions; e.g. `let excluded = query.diagnostics()`.
    pub fn diagnostics(&self) -> &[SqliteDiagnostic] {
        self.image.diagnostics()
    }
}

fn userdata_error(mut error: BessemerError) -> BessemerError {
    error.operation = "Electrical userdata query";
    error
}

impl ElectricalProjectDatabase {
    /// Retains every lexical numeric-ID match without SQL affinity; e.g. `let rows = database.userdata_for_object(26)?`.
    pub fn userdata_for_object(&self, object_id: i32) -> Result<SqliteRows> {
        self.userdata.electrical_object_rows(object_id)
    }
}
