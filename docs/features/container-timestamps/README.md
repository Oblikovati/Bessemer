# Container timestamps

Issue: #250. Clause 7.3.1 specifies native ZIP timestamp packing from explicit
raw metadata. `container_timestamps` models those inputs and edits existing
entry timestamps against the frozen baseline below.

The frozen corpus contains 22,398 requests from `ziptimestamps.native_timestamp`
and 113 local/central timestamp pairs from the existing native blank part,
assembly and drawing fixtures. It covers all six input words' single bits and
complements, year values 0 through 4,096, threshold neighborhoods, noncanonical
cross-products, absent metadata and packed overrides. An independent weighted
integer oracle checks every reference result.

The Rust baseline checks the complete request corpus, native stream names and
both raw header intervals. It also verifies that the existing parser preserves
all tested packed patterns without requiring valid calendar dates and rejects
inconsistent stored copies. This baseline predates the runtime API and includes
no native writer execution.

`ZipTimestampRequest::packed_word` models the explicit raw inputs.
`ZipEntryTimestamps::from_bytes` captures a bounded, immutable current file;
`entry_named` returns a revision-bound entry. `replace_timestamp` edits both
stored copies, reparses the result, and retains the entire original file along
with completion, exact structural coverage, diagnostics and byte history.

```rust
use bessemer::{Document, container_timestamps::{ZipEntryTimestamps, ZipTimestampRequest}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = Document::from_bytes(&std::fs::read("part.sldprt")?)?;
    let current = document.to_bytes()?;
    let times = ZipEntryTimestamps::from_bytes(&current, 16_000_000)?;
    let entry = times.entry_named("PreviewPNG")?;
    let request = ZipTimestampRequest {
        calendar_words: Some([20, 40, 13, 29, 9, 2026]),
        packed_override: 0,
    };
    let edit = times.replace_timestamp(&entry.id(), request)?;
    std::fs::write("timestamped.sldprt", edit.result().as_bytes())?;
    Ok(())
}
```

The byte limit applies before copying the supplied checkpoint; the caller's
`Document::to_bytes` serialization is a separate operation. Taking that current
checkpoint preserves any pending payload edits. The timestamp edit then changes
only its two four-byte fields and retains filler, decoys and every stored payload.
Noops keep the original identity; independently parsed or changed snapshots have
different identities, so foreign or stale entry handles fail.

Coverage checks the complete directory and local-header consistency. It does
not inflate payloads, validate their CRCs, interpret calendar validity, recover
native metadata provenance or establish application acceptance. Stored-bit
approximation error is zero. No clock, timezone or release-date policy is used.

Run the checks with:

```text
cargo test --locked --test container_timestamp_fixtures
python -m unittest discover -s docs/features/container-timestamps -p test_timestamps.py
```

`freeze_timestamps.py` regenerates both bounded packets. The Python tests compare
regeneration byte for byte under an injected temporary output root. Source and
native fixture hashes are recorded in [baseline.json](baseline.json). The
planned operation and its acceptance requirements are in [design.md](design.md).
The humanizer skill is unavailable; maintainer prose review remains pending.

The focused runtime suite passed 12 tests and the existing container suite
passed all seven tests. The parent test executed the separate-process worker
twice. All 7,910 native-entry edit cases matched literal expected offsets and
reopened inverses restored the original complete files. Input isolation, later
Document changes and eight workers on each native fixture were checked. See
[api-verification.json](api-verification.json) for hashes, check scope and pending
full-gate/hosted acceptance.

All 22,398 frozen requests also pass through the complete editor on an existing
native entry, with both stored words and the whole output compared against
independent expected bytes.
