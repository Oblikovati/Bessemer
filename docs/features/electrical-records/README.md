# Electrical project records and drawing links (#231)

`ElectricalArchive::project_records` implements the complete stored-record
projection in `inventor-interface/solidworks/electrical.py`, specification 14.25.
It checks every selected SQLite image and reads all documented catalog/project
tables before publishing a result. The immutable view retains its source carrier
and revision-bound member identities.

```rust,ignore
let view = archive.project_records(limits)?;
for project in view.projects() {
    let directory = project.record.value("pro_directory");
    let versions = &project.catalog_versions;
}
```

Every exact `Database/tew_app_project.sqlite` occurrence contributes all of its
`tew_project` rows. A TEXT `pro_directory` forms the literal prefix
`Projects/{directory}/`. Every matching project-database occurrence contributes
complete `tew_file`, `tew_location`, `tew_function`, `tew_bundle`, `tew_userdata`
and `tew_version` rows. Catalog and project-local version fields remain separate.
The reader imposes no universal project ID, row count, directory convention,
sentinel meaning, migration rule or version equality.

Each TEXT `fil_filename` forms a literal drawing member name. Zero, one or many
matching member identities remain visible. Duplicate catalogs, database members,
project rows and drawings retain their occurrence order. NULL, numeric and
missing directory/filename fields do not become path strings. TEXT used for
names must be UTF-8; undecodable names return a typed error. There is no path
normalization or filesystem extraction, including for `..` and backslashes.

Records retain exact column names, scan ordinals and uncoerced SQLite values,
including unknown fields. `file_type_constant` optionally names the observed
build's 23 integer `EwFileType` constants using the independent C-264 metadata
and native `fil_filetype` binding. Unknown codes and non-integer values remain
stored values without a guessed constant or provider conversion. Naming an enum
constant does not establish that it is valid for persistence.

## Resource and acceptance contracts

The caller supplies each SQLite operation's limits and whole-projection bounds
for projects, database links, file links, selected image occurrences and aggregate
accounting. All project bounds must be positive; nested SQLite limits are checked
even for an empty archive. Image/table caches are local to one operation and
keyed by source member occurrence and exact table name. Repeated relationships
do not repeatedly query or validate the same image/table pair.

Accounting includes source member metadata, selected image extents and schemas,
selected table results, row/column clones, result wrappers, drawing names and
matched identities. It is an explicit logical accounting policy, not a peak
allocator or SQLite memory measurement. Source-carrier and per-image/query limits
also apply. Overflow or exhaustion fails the complete operation; no truncated
graph is published. Missing required tables and corrupt selected images carry
the affected member occurrence in the error. An absent catalog completes with
an empty view. Unselected opaque members remain outside the SQLite check scope.

Coverage reports selected images/bytes, distinct table reads, checked rows and
published relationship counts. Diagnostics retain SQLite foreign-key/native
priority/application exclusions and identify opaque EWG bodies and unexecuted
native file loaders. Literal links do not prove geometry, migration acceptance,
native provider conversions or the 38-field typed load contract in 14.26.

## Evidence

The EPR1 packet freezes complete reference projections for four original
Electrical template archives and three owned archives. Owned cases include an
empty catalog set, duplicate catalogs/databases/rows/drawings, NULL and numeric
names, missing drawings, literal parent-path components, Unicode directories,
all 23 build-bound file-type constants, unknown integers and non-integer codes.
The original reference project-record helper supplies every row and relationship
from independently parsed literal ZIP entries; a metadata inventory supplies
enum names. SHA-256 values bind each archive and
projection in `tests/fixtures/electrical-records/integrity.json`.

Bug #236 records Python ZipInfo's Windows backslash normalization, which falsely
joined a deliberately distinct owned member to eight drawing relationships.
The raw ZIP oracle retains that member's stored spelling before calling the
unchanged project-record helper. Native graph expectations are unchanged.
Bug #237 records an initially uncommitted owned SQLite fixture transaction that
froze zero enum rows. The builder now commits and checks both live and serialized
row counts against all 28 construction inputs before freezing any expectations.

The focused gate passed twelve Rust graph tests, eleven unchanged SQLite
regression tests, three Python fixture tests and Clippy with warnings denied.
It covers exact and one-below aggregate/count limits, corrupt or missing selected
tables, empty results, caller-buffer mutation and eight concurrent independent
replays. Two fresh processes produced identical 1,114,623-byte output containing
every original carrier and complete graph projection. `local-verification.json`
and `process-verification.json` retain source/log/output hashes and exclusions.
The earlier SQLite full-suite run is a separate source cut and does not cover
this new project-record implementation; its full gate remains pending.

```powershell
python -m unittest discover -s docs/features/electrical-records -p 'test_*.py' -v
cargo test --test electrical_records --test sqlite_images --locked
```

The [userdata views](../electrical-userdata/README.md) now provide the documented
two-part SQL predicate, lexical object-ID matches and selected signed-ID gate.
Remaining work includes complete native userdata delegate/provider state, the
typed file-load projection (14.26), database mutation and native migration/loader
acceptance. Overall schema parity remains
incomplete. The humanizer skill is unavailable; its prose pass remains for review.
