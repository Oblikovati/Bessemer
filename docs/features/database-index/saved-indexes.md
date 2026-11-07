# Lossless saved indexes (#230)

`HashFileIndexBuffer` reads the selected file header and every complete linked
map page. It preserves raw free-head, split and maximum words, ordered duplicate
mappings and every uninterpreted byte. It does not apply the complete graph's
partition admission rules or follow bucket/free-page identities.

The caller selects this index-only scope through `from_bytes` with an explicit
`HashFileContext`. The existing provider, engine/pager extent, file-header,
magic and default-hash-test checks still apply. All map records must fit
completely, map links must stay within the supplied file, and map pages must
form an acyclic chain. Aggregate byte/page/map-record limits apply. The payload
budget applies to interpreted cell payloads; this operation interprets none.

## Edits and results

`edit_header` replaces the three raw 64-bit words. `edit_mapping` selects one
saved pair by ordinal, including a particular occurrence of a duplicate pair.
Neither operation sorts entries, changes record counts, repairs fields or
allocates native pages. Each copies the immutable source, replaces only the
declared word interval and reparses the saved index before publication.

The result carries its source, complete retained file bytes, changed/unchanged
completion, `ExactSavedIndexChain` coverage, zero achieved reparse error,
three ordered full-file history spans and diagnostics. Stored words and pair
identities can remain unsuitable for native allocation or lookup. This scope
does not establish a valid complete file graph, native duplicate precedence,
transaction durability or application/geometry acceptance.

```rust,ignore
let saved = HashFileIndexBuffer::from_bytes(&raw, context)?;
let pair = saved.mappings()[ordinal];
let edit = saved.edit_mapping(ordinal, pair)?;
assert_eq!(edit.result().to_bytes(), raw);
assert_eq!(edit.coverage().mode, HashIndexCheckMode::ExactSavedIndexChain);
```

## Independent checks

`saved-indexes.bin.gz` contains seven complete owned/native files and 34
changed/no-op requests with independent complete expected outputs. It includes
continuation maps, duplicate pairs, raw extreme words and empty map frames.
The fixture generator writes only literal word intervals; tests regenerate
the packet, verify every other byte and reverse each request to the original.

The complete native graph corpus also supplies 2486 header edits and 3162
mapping edits. Their expected intervals come from the observed root-only map
cohort; continuation addressing is checked by the owned fixtures. These
mutations deliberately preserve index framing while changing graph meaning.
Fresh saved-index snapshots reverse them byte for byte. Concurrency checks
compare eight independent edits on one immutable source.

The underlying saved-map framing was extracted from graph partition admission
in a separate behavior-preserving commit. Its original 16 Rust tests and all
22,748 native value edit/reversal expectations passed unchanged. These tests
do not execute the original native engine or infer its cache behavior.

The focused verification passed 24 Rust tests and 20 reference tests, including
the earlier 22,748 native value edits. Two fresh processes produced identical
3,149,824-byte saved-index reconstruction/edit/reversal output for all seven
frozen cases. `saved-index-local-verification.json` and
`saved-index-process-verification.json` retain source/log hashes and exclusions.
The full PR gate and Linux ThreadSanitizer remain separate acceptance checks.
