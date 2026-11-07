use super::*;
use crate::sqlite_image::{SqliteDiagnostic, SqliteLimits};

/// Whole-project limits in addition to each SQLite operation's explicit limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalProjectLimits {
    /// Complete SQLite-image and query limits.
    pub sqlite: SqliteLimits,
    /// Project row occurrences across every catalog occurrence.
    pub maximum_projects: usize,
    /// Project-to-database links, including repeated and duplicate occurrences.
    pub maximum_database_links: usize,
    /// File row occurrences across all projected database links.
    pub maximum_file_links: usize,
    /// Distinct selected member occurrences checked as SQLite images.
    pub maximum_checked_images: usize,
    /// Aggregate image/query/result accounting; not peak engine memory.
    pub maximum_accounted_bytes: usize,
}

/// Immutable complete catalog projection with its source revision retained.
#[derive(Clone, Debug)]
pub struct ElectricalProjectView {
    pub(super) source: ElectricalArchive,
    pub(super) projects: Arc<[ElectricalProject]>,
    pub(super) coverage: ElectricalProjectCoverage,
}

impl ElectricalProjectView {
    /// Borrows every complete project occurrence; e.g. `let projects = view.projects()`.
    pub fn projects(&self) -> &[ElectricalProject] {
        &self.projects
    }
    /// Borrows the immutable parent carrier; e.g. `let original = view.source().to_bytes()`.
    pub fn source(&self) -> &ElectricalArchive {
        &self.source
    }
    /// Reports exactly the selected checks; e.g. `let checked = view.coverage()`.
    pub fn coverage(&self) -> ElectricalProjectCoverage {
        self.coverage
    }
    /// Keeps stored links separate from application acceptance; e.g. `let excluded = view.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalProjectDiagnostic] {
        &[
            ElectricalProjectDiagnostic::Sqlite(SqliteDiagnostic::NativeApplicationNotChecked),
            ElectricalProjectDiagnostic::Sqlite(SqliteDiagnostic::ForeignKeyConsistencyNotChecked),
            ElectricalProjectDiagnostic::Sqlite(SqliteDiagnostic::NativeRowPriorityNotEstablished),
            ElectricalProjectDiagnostic::DrawingBodiesOpaque,
            ElectricalProjectDiagnostic::NativeFileLoadNotExecuted,
        ]
    }
}

/// One complete stored row with names, values and scan ordinal retained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalRecord {
    pub(super) names: Arc<[String]>,
    pub(super) values: Arc<[SqliteValue]>,
    pub(super) ordinal: usize,
}

impl ElectricalRecord {
    /// Borrows exact column names; e.g. `let names = record.columns()`.
    pub fn columns(&self) -> &[String] {
        &self.names
    }
    /// Borrows uncoerced SQLite values; e.g. `let values = record.values()`.
    pub fn values(&self) -> &[SqliteValue] {
        &self.values
    }
    /// Selects an exact column spelling; e.g. `let directory = record.value("pro_directory")`.
    pub fn value(&self, name: &str) -> Option<&SqliteValue> {
        self.names
            .iter()
            .position(|stored| stored == name)
            .and_then(|index| self.values.get(index))
    }
    /// Reports the selected query's scan ordinal; e.g. `let row = record.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

/// One project row and every matching project database occurrence.
#[derive(Clone, Debug)]
pub struct ElectricalProject {
    /// Exact catalog member occurrence, bound to the retained source revision.
    pub catalog: ElectricalMemberId,
    /// Complete tew_project row, including unknown columns.
    pub record: ElectricalRecord,
    /// Complete catalog tew_version rows, separate from project-local versions.
    pub catalog_versions: SqliteRows,
    /// Every literal matching project-database occurrence in directory order.
    pub databases: Arc<[ElectricalProjectDatabase]>,
}

/// One selected project database and its documented complete row sets.
#[derive(Clone, Debug)]
pub struct ElectricalProjectDatabase {
    /// Exact selected project-database occurrence.
    pub member: ElectricalMemberId,
    /// Complete tew_file rows and their zero/one/many drawing matches.
    pub files: Arc<[ElectricalFileLink]>,
    /// Complete tew_location rows.
    pub locations: SqliteRows,
    /// Complete tew_function rows.
    pub functions: SqliteRows,
    /// Complete tew_bundle rows.
    pub bundles: SqliteRows,
    /// Complete tew_userdata rows without native first-row selection.
    pub userdata: SqliteRows,
    /// Complete project-local tew_version rows, with every distinct version column.
    pub versions: SqliteRows,
}

/// A stored file record and its literal archive-name relationship.
#[derive(Clone, Debug)]
pub struct ElectricalFileLink {
    /// Complete original row, without numeric or text coercion.
    pub record: ElectricalRecord,
    /// Literal requested drawing name; absent when fil_filename is not TEXT.
    pub member_name: Option<String>,
    /// Every exact matching member occurrence, including duplicates.
    pub members: Arc<[ElectricalMemberId]>,
}

/// Executed project interpretation scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalProjectCheckMode {
    /// Complete selected SQLite images/tables and literal archive-name joins.
    LiteralProjectLinks,
}

/// Whole-operation counters; no missing or truncated required result is successful.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalProjectCoverage {
    /// Selected check scope.
    pub mode: ElectricalProjectCheckMode,
    /// Distinct selected SQLite member occurrences.
    pub checked_images: usize,
    /// Complete selected image byte extents, counted once per occurrence.
    pub checked_image_bytes: usize,
    /// Distinct selected table reads, counted once per image/table pair.
    pub checked_tables: usize,
    /// Complete rows in those selected reads.
    pub checked_rows: usize,
    /// Published project row occurrences.
    pub projects: usize,
    /// Published project-to-database occurrences.
    pub database_links: usize,
    /// Published file row occurrences.
    pub file_links: usize,
    /// Conservative logical image/query/result accounting, not allocator peak memory.
    pub accounted_bytes: usize,
}

/// Explicit exclusions from a completed stored-record projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalProjectDiagnostic {
    /// SQLite acceptance exclusion retained from the image reader.
    Sqlite(SqliteDiagnostic),
    /// Drawing matches do not decode EWG contents or geometry.
    DrawingBodiesOpaque,
    /// Provider conversions, migrations and native file loaders were not executed.
    NativeFileLoadNotExecuted,
}
