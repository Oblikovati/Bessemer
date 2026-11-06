mod budget;
mod file_types;
mod links;
mod read;
mod types;
use super::{ElectricalArchive, ElectricalMember, ElectricalMemberId};
use crate::sqlite_image::{SqliteImage, SqliteRows, SqliteValue};
use crate::{BessemerError, FailureClass, Result};
use std::{collections::BTreeMap, sync::Arc};
pub use types::*;

impl ElectricalArchive {
    /// Links every documented catalog/project/drawing occurrence; e.g. `let view = archive.project_records(limits)?`.
    pub fn project_records(
        &self,
        limits: ElectricalProjectLimits,
    ) -> Result<ElectricalProjectView> {
        read::read_project_view(self, limits)
    }
}

fn project_failure(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "Electrical project records",
        entity,
        FailureClass::Invalid,
        cause,
    )
}

struct ProjectReadContext<'a> {
    archive: &'a ElectricalArchive,
    members: Vec<ElectricalMember>,
    limits: ElectricalProjectLimits,
    coverage: ElectricalProjectCoverage,
    images: BTreeMap<usize, SqliteImage>,
    tables: BTreeMap<(usize, &'static str), SqliteRows>,
}
