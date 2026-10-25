# Early PW scenes through bucket thread keys

Bug #41 concerned a documented caller outside the document-root scene field.
Bucket thread maps accept generic CObject keys from version 2771. A key can
therefore contain the registered `pwSceneData_c` body before root scenes begin
above 3934. The decoder previously assumed the later PW layout and read absent
fields as subsequent bytes.

The fixed reader omits stored wall keys before 3576, retains the mapping-fit word
only from 3601, reads three background suffix words above 3595 and reads the
camera-FOV flag above 3897. An implicit wall ordinal remains an in-memory reference
value; the archive does not invent a stored key for it. Minimum wall sizes are
14, 18 and 22 bytes across the two wall-record boundaries.

The independent writer builds a complete bucket containing a PW scene key and a
second key that aliases it. Inserting that graph into an owned full root relocates
every observed later MFC tag. Both kinds are covered at 2771, 3196, 3575, 3576,
3595, 3596, 3600, 3601, 3897, 3898, 3934, 3935 and 4300. Five scenarios cover
populated walls, an isolated shadow-type edit, negative empty counts and 256
minimal wall records with their edited counterpart. All 130 roots parse to EOF
in the reference reader and supply 115,921 independent fields.

Nine integration tests check those fields, forced writes, optional members,
shared object identities, field/object budgets, impossible wall counts, sampled
truncations and deterministic concurrent edits. Four complete owned containers
check context resolution, edit, reopen and authentic restoration while retaining
unrelated streams. Two-process root and document reports record exact bytes and
contexts. Before the fix, the first new root failed Bessemer's complete decode.

The fixture also exposed reference-helper bug #42. Its view-activity offset was
valid for the older boolean-normalizing layout, but newer views insert ghost
transform framing. The helper now retains the authoritative reader's raw activity
value from 916 onward and uses observed byte spans only below that boundary.
The new bucket gives `unknown_68` and `flags` distinct values, 6 and -7.
Regeneration preserved all bytes and oracles of 1,896 frozen roots across the
version-zero, early-component and named-view control groups.

The updated predicate audit reconstructed 11,313 owned roots in two processes;
362 of its 391 recognized version comparisons observed both outcomes. The native
gate retained all 1,086 accepted streams and 1,849,563 independent fields, with
unchanged manifest `94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
These checks establish stored-layout fidelity, not geometric validity or native
application acceptance. The broader schema goal remains open.

Windows x64 validation with Rust 1.99.0 MSVC passed `cargo xtask pr`, including
204 doctests. All 20 native drawing streams and containers also retained exact
two-process reconstruction. Cross-platform and ThreadSanitizer acceptance remains
pending the feature's CI gates.

Reproduce fixture and process checks with `generate_early_pw_scenes.py` and
`verify_early_pw_scenes.py`; pass the built `rebuild_display_archives` executable
to the latter. Run `cargo test --test display_pre_facet_roots early_pw_scene` for
the focused integration tests and `cargo xtask pr` for the repository gate.

The humanizer skill was unavailable; prose remains subject to maintainer review.
