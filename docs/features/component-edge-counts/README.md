# Component-edge signed reference counts

Bug #204 corrects an offline reader mismatch with specification clause 11.404.
The stored reference count is signed 32-bit. The native load loop runs only
when it is positive; a negative count remains stored and does not suppress the
later conditional composite-reference pair. Bessemer previously rejected every
negative count through its strict collection helper.

The correction reuses `configuration_io::positive_count`: it retains the raw
signed field, skips nonpositive loops, and keeps the existing positive-count
resource and remaining-byte checks. Raw flags, cached points, update-stamp
limits, object identities and version/context admission are unchanged. The
pointer declaration is not treated as a new class whitelist: the reference
reader permits derived objects, and retained fixtures contain self aliases.

The independent oracle has 40 combinations of two edge classes, five counts
(`INT_MIN`, -1, 0, 1, 2) and four flag words (0, 8, 14, signed high-bit plus 8).
Original readers check complete bare objects and complete owned model streams,
including mapped pair order, nulls, exact EOF and inherited map identities.
They also reject every proper bare-record prefix and impossible positive counts.
The complete source copy, import origins and byte hashes are guarded.

The pre-fix public regression failed on `INT_MIN`. Its log is retained rather
than changing the old expected result. The historical 188 component fixtures
remain unchanged. Their old negative-count rejection assertion is replaced by
an impossible-positive-count assertion; the new matrix checks the corrected
negative behavior independently. This is a parser correction with no on-disk
format migration or package-version change. Historical rejection remains
reproducible from the recorded pre-fix revision and binary.

The corrected public suite passed all three tests, including 16 complete
document edits, reopens and fresh reversals. All six existing component-reference
tests passed, as did library/integration Clippy and formatting. A fresh guarded
original-reader run reproduced every fixture byte. `local-verification.json`
retains the exact logs and exclusions; the full PR gate and serial integration
remain separate requirements.

The owned fixtures do not establish native negative-count population, native
application acceptance or geometry validity. The reference reports that all
16,586 observed native component-edge bodies have count zero. Complete schema
and Corliss interoperability parity remains open in #2. The fix must integrate
on `main` before it is forwarded to the feature candidate; no additional PR is
opened while #191 is active.

```text
python -B -X utf8 docs/features/component-edge-counts/freeze_reference.py
cargo test --test model_component_edge_counts
```
