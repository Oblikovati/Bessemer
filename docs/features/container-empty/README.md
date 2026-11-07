# Registered empty-container reader

Bugs: #251 (Bessemer reader) and #252 (reference builder). The reference reader
accepts the complete 30-byte zero-stream version-4 wire for every registered
key. The original Bessemer reader rejected it before lookup because it assumed
a nonempty directory. The retained CLI reproduction and Source-positive
fixtures pin that mismatch before the fix.

`empty.bin` contains all 1,000 registered signature rows and independently
assembled empty wires. Every wire was accepted by the unchanged Source
`swcontainer.Document`. `native.bin` pins the local, central and end signatures
against the three existing owned native fixtures. Cache, source and fixture
hashes are recorded in [baseline.json](baseline.json).

The initial baseline Rust tests check complete wire shapes, key ordering and
native signature locations. The runtime regression suite now checks successful
empty iteration and exact serialization for all registered keys.
The Python tests verify Source acceptance, input shapes and byte-for-byte
regeneration under an injected temporary root. No reference source is changed.

Run the baseline with:

```text
cargo test --locked --test container_empty_fixtures
python -m unittest discover -s docs/features/container-empty -p test_empty.py
```

The separate reference builder raises `UnboundLocalError` after its empty loop;
its result is not used as an empty-wire oracle. Native application acceptance
of a zero-stream CAD document is not claimed. The humanizer skill is unavailable;
maintainer prose review remains pending.

The reader uses a private owned signature corpus for empty directories and keeps
existing signature recovery for nonempty files. Unknown empty keys are typed
unsupported results. Missing stream reads and writes remain typed missing-entity
failures, while a complete zero-stream container is a successful result.

Eight empty-reader tests passed, including a parent that invoked its complete
wire-checking worker twice. All 18,000 filler layouts match the independent
Source acceptance checks, and the existing seven container and twelve timestamp
tests passed unchanged. This does not establish whole-model or application
acceptance. See [design.md](design.md) for the recorded behavior-3 migration and
[reader-verification.json](reader-verification.json) for exact scope and pending
main-first integration gates.
