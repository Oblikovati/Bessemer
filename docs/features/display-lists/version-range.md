# Display versions 18000–19000

Issue #28 remains open. Complete part/assembly streams now accept explicit
display versions 18000 through 19000, with file-version context 19000. These are
distinct inputs: the reference reader uses file version as caller-supplied
process context rather than deriving it from these stream bytes. Automatic
whole-document context resolution remains unfinished.

The audited reference serializers have five layout gates inside this range:

| Gate | Wire behavior |
| --- | --- |
| Version below `0x46BA` | Appearance properties include three strings, 17 float32 values, two unsigned integers and four signed integers in a discarded symbol record. |
| Version above `0x46B4` | DSPBR properties include the translucency extension. |
| Version above `0x46BA` | Scenes include field of view. |
| Version above `0x46C9` | Each 3D view includes the additional active-state integer. |
| Version above `0x4971` | DSPBR properties include the final two floats and three strings. |

The existing versioned 3D-view codec already handled its gate. The archive reader
now handles the other four while retaining every discarded symbol value.
Construction revalidates the selected context, so moving contents across a gate
requires the corresponding field-shape change. Earlier behavior remains
selectable through the explicit display version.

Twelve [independently encoded complete roots](version-range.json) cover both
sides of every gate, the range endpoints and an independently encoded exposure
edit. They match 5,788 material and scene fields and reproduce all bytes through
forced construction. Each includes two 3D views with two section planes apiece,
both root material declarations, supplementary Unicode, signed zero, infinities
and quiet/signaling float32 NaNs. The NaNs exposed [verification bug #35](scalar-trace-nans.md);
its fix preserves sign and payload without changing archive bytes or historical
fixture baselines.

Seven integration tests cover those fields and bytes, explicit presence rules,
wrong contexts, rejected shape changes across gates, discarded scalar bits, every
prefix of a version-18000 root, exact entry/object/field/byte budgets, source
ownership and eight concurrent edits. A container replacement/reopen/revert test
uses the fixture's explicit context; it does not claim cross-stream or native
application validity.

The [native audit](archive-audit.json) reconstructs 1,005 of 1,086 streams,
including all 22 version-18000 samples and all 983 version-19000 samples. The
expanded material oracle follows first object declarations through body, face,
component and root maps, excluding scene/bucket fields already covered by their
own traces. It matches 806,122 material fields, alongside 238,667 reference-geometry,
164,483 scene, 184,551 root display-item, 234,395 view-bucket and 58,111 light
fields. Two processes return identical reconstructed bytes, diagnostics and
combined traces. No previous accepted paths were lost; corpus SHA-256 remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

The 81 remaining native failures use versions 7000, 9000, 10000 and 11000.
Unexercised schema branches, other file-version contexts, cross-stream checks,
full model archives, geometric validity and application acceptance remain
required work. Passing this corpus subset does not establish full schema parity.

Local verification used Windows x64 and the pinned Rust 1.99.0 MSVC toolchain.
The commands below passed, including the full local PR gate. Cross-platform CI
and native application acceptance remain integration gates.

```text
python -B -X utf8 docs/features/display-lists/generate_version_range.py
cargo test --test display_version_range
cargo test --test display_scalar_trace
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The fixture manifest records the evolving reference source hashes. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
