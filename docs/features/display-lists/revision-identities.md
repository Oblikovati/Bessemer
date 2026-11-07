# Revision-scoped display object identities

Feature #28 requires object identities scoped to immutable display archives.
`DisplayObjectId` remains the persisted MFC map index. It can repeat in another
archive and therefore cannot identify a loaded object across edits or documents.
`DisplayArchive::object_handle` binds a loaded index to its archive revision;
`resolve_object` returns that object's defining tag only in the owning revision.
Class-map slots, missing indices and foreign revisions return typed invalid errors
with the operation and offending object index. Null has no object handle.

A handle is an owned `Send + Sync` value. It retains a private reference-counted
revision token, so dropping the original archive cannot allow an allocation to
reuse its identity while a handle survives. The token contains no geometry or
mutable state. Existing owned archive contents remain the source of isolation.
No global counter, thread-local state or serialized pointer participates in lookup.

Clones and equivalent content edits share a revision. Any changed content edit
creates a fresh revision, including an authentic byte revert. Reverting restores
the original bytes but does not revive handles from an earlier revision. Fresh
decodes and forced constructions also create fresh revisions even when their
bytes match. Applications must acquire new handles from the resulting archive;
a raw map index alone is not evidence of semantic continuity. Operation history
and explicit identity remapping remain required work.

Aliases and definitions yield equal handles for the same revision and object.
Resolution scans the existing bounded ordered field sequence for the defining
new-object tag. This adds one small revision-token allocation per fresh revision
and a reference-count increment per handle, without an object index cache or a
change to graph storage. Handles deliberately have no serialization API.

```rust,no_run
# fn example(archive: &bessemer::display_lists::DisplayArchive) -> bessemer::Result<()> {
use bessemer::display_lists::DisplayObjectId;
let handle = archive.object_handle(DisplayObjectId::new(2)?)?;
let definition = archive.resolve_object(&handle)?;
assert_eq!(definition, archive.clone().resolve_object(&handle)?);
# Ok(())
# }
```

The focused integration tests check all loaded objects and aliases in an owned
shared-font fixture, class and missing slots, separate decodes/constructions,
document-kind separation, clones, equivalent edits, changed edits, authentic
reverts, failed budget validation and concurrent lookup/edit isolation.
Run `cargo test --test display_object_identity`.

The humanizer skill was unavailable; prose remains subject to maintainer review.

Validation on Windows x64 with Rust 1.99.0 passed the six focused tests, Clippy
with warnings denied and `cargo xtask pr`, including 179 documentation tests.
The native audit reconstructed all 1,086 accepted documents in two processes
and retained 1,849,563 independent field comparisons with unchanged corpus hash
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform and ThreadSanitizer gates remain pending the complete feature PR.
