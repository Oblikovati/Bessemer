# Registered empty-container reader fix

Bug: #251. Preserve the existing nonempty reader and close the zero-stream
container gap using the documented header/end grammar and registered key table.
The runtime fix remains pending after the independent baseline.

A zero-stream container has no first central or local record from which to
recover signatures. Resolve its already parsed unsigned header key against the
supplied registered signature corpus. Check the matching end signature, disk
numbers, equal zero counts, directory bounds, comment length and version before
publishing an empty Directory. Do not infer signatures from nonexistent records
or accept arbitrary unknown-key empty byte strings.

Keep the nonempty signature-recovery admission unchanged. Empty iteration and
exact no-op serialization must succeed; missing stream reads or writes must
remain typed missing-entity failures. Check all 1,000 Source-positive wires,
every proper canonical prefix, malformed header/end fields, unsupported keys,
retained filler, caller-buffer mutation and repeated/concurrent reads. Keep
existing native extraction, replacement, metadata and byte-preservation tests.

The fix must have its own commit and land on `main` before it is forwarded into
the candidate branch. Do not couple it to the timestamp API or a future new-file
builder. Keep PR #249's running head unchanged and open no second PR while it
remains pending. Record exact-source gate results before push and merge only
after all required checks pass. No version bump or release tag is authorized.

Bug #252 tracks the separate reference builder failure. Its helper is left
unchanged; documented empty-wire assembly and Source-reader acceptance provide
the independent oracle. Native application and whole-schema acceptance remain
separate. The humanizer skill is unavailable; maintainer prose review is pending.
