# Empty-container reader regression baseline

Bugs: #251 (Bessemer reader) and #252 (reference builder). The reference reader
accepts the complete 30-byte zero-stream version-4 wire for every registered
key. Bessemer currently rejects it before lookup because it assumes a nonempty
directory. The Rust CLI reproduction and Source-positive fixtures pin this
mismatch before the reader changes.

`empty.bin` contains all 1,000 registered signature rows and independently
assembled empty wires. Every wire was accepted by the unchanged Source
`swcontainer.Document`. `native.bin` pins the local, central and end signatures
against the three existing owned native fixtures. Cache, source and fixture
hashes are recorded in [baseline.json](baseline.json).

The baseline Rust tests check complete wire shapes, key ordering and native
signature locations. They do not assert the current rejection as intended
behavior, and they do not establish that the Bessemer reader bug is fixed.
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
