# Registered empty-container reader fix

Bug: #251. Preserve the existing nonempty reader and close the zero-stream
container gap using the documented header/end grammar and registered key table.
The runtime reader resolves an empty layout through its registered key;
nonempty records keep their existing signature-inference path.

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

## Kernel decision and recorded migration

Container behavior version 3 accepts registered zero-stream version-4 containers.
Version 2 rejected these inputs because its end scan and layout required a first
record. The independent Source-positive corpus and normative header/end layout
justify retiring that rejection. Previously supported nonempty inputs retain
their parser, keys, stored payloads and no-op bytes. No on-disk migration or
package version bump is required. Replays that expected this empty-input error
must record successful empty iteration under behavior 3 instead.

The private signature corpus contains exactly the 1,000 supplied key sets,
normalized as unsigned big-endian header keys and sorted before lookup. It is
owned with the reader; no installed DLL, cache file or native application is
needed at runtime. Unknown empty keys remain unsupported because there is no
first record from which to recover their signatures. Complete zero-entry results
are successful; missing stream operations remain explicit failures.

## Integration condition

A fresh fetch confirmed that `main` at `7e5e8fac` contains only `.gitignore`,
`LICENSE` and `README.md`; the library is still on the candidate line. This
patch is prepared independently, but a standalone main-first fix cannot compile
until the initial library merge. The integration order was raised with the user;
no extra branch or PR is created while that clarification and PR #249's gates
remain pending. This does not authorize a version bump, tag or publication.
