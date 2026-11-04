# Named sketch bitmap resource byte arrays

Feature [#190](https://github.com/Oblikovati/Bessemer/issues/190) tracks
published section 11.1661 / C-3345. `BitmapResourceArray` now reads complete logical arrays, constructs independent
arrays and publishes isolated payload edits. Local operation gates passed; publication remains open. The stream still comes from `Document`; the new
codec interprets only its logical count and opaque payload.

The logical body is a compact 16/32 count followed by exactly that many raw
bytes to EOF. It contains no MFC class/object header and adds no map slot.
Reuse the existing bounded collection-count reader. A decoded resource retains its original count-width preference. Growth
promotes a short count when the new count reaches 65535; subsequent edits
use that original preference. Exact payload reverts restore the original
encoded bytes through the existing preservation helper. Count and payload bounds, exact EOF,
no-op identity and input isolation remain explicit checks. Empty complete
arrays must remain distinguishable from missing or incomplete inputs.

The named-storage methods consume a supplied storage object and current
member 58 name. The observed `Contents/name` association is not a universal
native storage-owner proof. Callers must select the logical stream explicitly.
The count is independent of width, height and enable state. Offline edits
must not simulate postread resizing or infer pixel formats, RGB grouping,
row origins, alpha endpoints, rendering or runtime allocation behavior.

Preparation matched all 42 direct publication pins. The report records four
logical entries, 924,273 bytes including 24 header bytes, and 924,249 payload
bytes. There are two distinct payload hashes across the four separately
sourced entries. These population facts grant no global accounting credit.
The original report is retained byte for byte. Native module provenance,
fresh original-resource reads, owned count-boundary oracles, read/write and
whole-container reversal, fresh-process identity, concurrency and full gates
remain required. Later RGB and renderer contracts are separate work.

The humanizer skill is unavailable; the prose pass is left to review.

Twenty independent literals cover both valid prefix widths below 65535, the
65534/65535 boundary, empty arrays and opaque full-byte payloads through 1 MiB.
Original Source count/raw primitives decoded all 20 cases with 50 primitive
reads, a complete byte partition and no added map slots. All1,513,405 proper
prefixes were rejected; 20 appended remainders were detected by the explicit
whole-array EOF condition. This is syntax validation, not image decoding.

A separate fresh Source pass read the four original logical resources and
matched container hashes, outer compression/CRC metadata, count prefixes,
payload hashes and exact EOFs. It bound924,273 logical bytes and 924,249
payload bytes with two distinct payload hashes. No feature stream was read.
Both frozen Source content/import audits passed and both processes exited
successfully. Their snapshots and log/collector hashes are retained. The
Rust codec is implemented; mutation and publication acceptance are described
below.

`from_bytes` validates bounded counts, payload extent and exact EOF before
publication. `from_document` reads an explicitly named logical stream after
container checks. `new` chooses the shortest count encoding. `edit_payload`
returns the immutable source and result together with changed/unchanged
completion, exact count/payload coverage, measured reparse byte error,
semantic exclusions and both logical field histories. Empty payload fields
remain present and keep explicit empty ranges. No MFC map slots are added.

The first six Rust tests passed all 20 literals, independent minimal-header
construction, noncanonical-width no-ops, 140 changed/unchanged replacement
requests and reversals, input-buffer isolation, exact/one-below budgets,
missing-versus-empty behavior and all 1,513,405 proper-prefix rejections.
Every published type compiles as Send+Sync. The native gate passed five
payload forms in each original resource, including shrinkage below the
short-count boundary, complete document reopen and exact whole-container
reversal. Fresh-process and concurrency results are retained below. The full local gate is recorded below;
remote gates remain pending. No dependency or version was changed.

Two fresh Rust processes passed all nine focused tests, including the native
container gate and eight-worker byte comparison. Both exported the same
120 actual edited arrays for independent Source-reader acceptance. The
[operation record](rust-operation-verification.json) retains the exact source
and executable hashes. The full local gate is recorded below; remote gates remain pending.

Original frozen Source readers accepted all 120 actual exports through EOF
without adding an MFC map slot. The [actual-output record](actual-output-source-verification.json)
and [snapshot](fixture-reference-snapshot.json) retain the packet digest
and import/hash guards. The first full local gate rejected seven missing
public enum-variant docstrings. Those descriptions were added, and the
subsequent complete gate passed.

The [full local gate](full-local-gate-verification.json) passed 3,964 tests
with zero failures and 217 explicit exclusions across 316 summaries.
The process exited successfully. Its Rust, test, Cargo and CI content matches
the locally integrated feature commit. Exact-head remote platform and
ThreadSanitizer checks remain required after the current PR is merged.
