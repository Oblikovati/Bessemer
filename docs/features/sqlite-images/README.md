# In-memory SQLite images (#233)

`bessemer::sqlite_image` implements the complete in-memory schema and named-table
row reads in `inventor-interface/solidworks/electrical.py`. Specification edition
0.3134 sections 14.24–14.28 describe the Electrical use; the installed CAM TechDB
uses the same SQLite carrier. Electrical's `sqlite_member` method applies this
reader to one revision-bound archive occurrence.

```rust,ignore
let image = bessemer::sqlite_image::SqliteImage::from_bytes(&raw, limits)?;
let projects = image.table_rows("tew_project")?;
assert_eq!(image.as_bytes(), raw);
```

## Completed operation

Image publication requires the exact SQLite signature, a complete rollback-mode
image, a matching page extent, exactly one `ok` result from full
`PRAGMA integrity_check`, and all schema objects ordered by type and name.
Table metadata includes every `table_xinfo` and `foreign_key_list` row. Stored
object names, SQL text, root pages, implicit-index NULL SQL, generated columns
and nullable foreign-key target columns remain distinct.

Each named-table read returns every `SELECT *` row or fails the whole request.
Empty tables complete successfully. Missing names, including case variants,
produce typed errors. Quoted identifiers are escaped after exact stored-name
lookup. There is no caller-supplied SQL or filesystem database connection.

Values retain NULL, signed integers, engine double bits, text bytes and blob
bytes separately. Text cell bytes are not decoded, so embedded NUL and invalid
UTF-8 survive. Schema identifiers and SQL must be UTF-8; unsupported encodings
fail explicitly before the dependency's column-name accessor could panic.
Duplicate rows and unknown columns remain present. Scan order does not establish
the native provider's first-row priority.

The original image is owned and returned without SQLite reserialization. Every
query uses a separate read-only in-memory connection. Published images and rows
retain no connection, callback or mutable engine state; clones share immutable
owned storage. Failed queries leave the original snapshot usable.

## Limits and engine

Every operation takes positive limits for image bytes, schema objects, rows,
columns, individual value/row bytes, aggregate output accounting and VM steps.
All bounds must fit a positive signed 32-bit integer. Schema output shares one
aggregate budget; table queries each get a fresh output and VM budget. A progress
callback checks every SQLite VM instruction. Exhaustion returns an error and
publishes no partial result.

Output accounting adds value bytes and Rust cell, row and column-name sizes. It
does not measure peak process memory, SQLite's internal allocations, allocator
overhead or temporary schema conversion storage. The complete input, object,
row, column and value limits also bound those traversals. This is an explicit
resource policy rather than a claimed exact memory measurement.

The pinned rusqlite 0.40.2 dependency uses bundled libsqlite3-sys 0.38.2, with
serialization, limits and progress hooks enabled. Its safe stream-deserialization
API owns the engine allocation. Bessemer adds no unsafe code. Connections enable
defensive and query-only modes, disable trusted schemas and attachment creation
or writes, keep temporary storage in memory and disable auxiliary worker threads.
No extension-loading feature is enabled. Dependency denial and advisory checks
passed for this lockfile.

## Independent evidence

The frozen SQI1 packet contains 18 images: five owned cases, all twelve distinct
SQLite payloads from the four owned installed Electrical template archives, and
the authenticated installed CAM TechDB image. Python sqlite3 independently
produces every schema, column, foreign-key and table-row projection. Its text
factory distinguishes undecoded TEXT bytes from BLOB values. Packet and image
hashes are retained in `tests/fixtures/sqlite-images/integrity.json`.

Owned cases cover an empty schema, duplicate rows, NULL versus empty values,
signed integer extremes, infinity, invalid UTF-8 text, embedded NUL, quoted names,
generated columns, views, triggers, indexes, violated foreign-key references,
WITHOUT ROWID ordering and 512/1024/65536-byte pages. Rejection checks include
corruption, truncation, surplus bytes, WAL dependence, every invalid limit axis,
row/output exhaustion and an interrupted required integrity operation.

The initial implementation queried `page_size` before `page_count`. SQLite
returned the connection default before loading the deserialized pager, rejecting
valid 1024-byte-page Electrical images. Bug #234 records that observation. The
extent check now initializes page metadata first; all exact extent checks and
original expected results remain in place.

The focused gate passed eleven Rust tests, two independent Python fixture tests
and Clippy with warnings denied. Eight workers matched serial snapshots while
mixing successful and rejected reads. Two fresh processes produced identical
40,380,981-byte framed output covering every original image and complete schema
and row projection. Their elapsed times were 30.35 and 19.60 seconds, with observed
peak working sets of 115,613,696 and 114,618,368 bytes. These measurements do not
claim a speedup over an earlier release. `local-verification.json` and
`process-verification.json` retain the exact scope and hashes. Full-suite
verification for this source is still pending; the earlier Electrical full gate
does not cover the new SQLite dependency or implementation.

```powershell
python -m unittest discover -s docs/features/sqlite-images -p 'test_*.py' -v
cargo test --test sqlite_images --locked
cargo deny --all-features check
cargo audit
```

## Remaining scope

This operation does not write SQLite cells or reconstruct application schemas.
Project/drawing relationships, native file-load mappings and delegated userdata
query selection remain under #231. Integrity checking does not verify foreign-key
consistency, native migration/loader acceptance, EWG drawings or geometry. Those
exclusions accompany successful images as typed diagnostics. Overall documented
schema parity remains incomplete.

The humanizer pass remains for maintainer review because that skill is unavailable.
