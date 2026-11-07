# Flow solver ordinary selectors

Issue: #241. Reference: `../inventor-interface/solidworks/flowsolverselectors.py`.

`FlowCarrier::solver_latest(identifier)` selects the last physical occurrence of
an ordinary solver identifier. Its `u32` argument fixes the accepted identifier
domain at the Rust boundary. IDs `0xffffff9b`, `0xffffff9c` and `0xffffff9d`
return typed unsupported errors, including when the carrier is empty. An absent
ordinary identifier returns `Ok(None)`.

```rust,ignore
let selected = carrier.solver_latest(14)?;
```

Physical enumeration retains all three control IDs and every duplicate. The
existing `latest` header-reader profile still accepts `0xffffff9c`; solver
selection does not alter that contract. Selection borrows the immutable source
and returns its existing revision-bound occurrence identity. Payload replacement
creates a new revision whose selector returns the same physical ordinal with the
new bytes and identity.

The frozen reference corpus covers low and high byte boundaries, every unsigned
single bit and complement, duplicate order, the three reserved IDs and absence.
Both physical versions 7 and 8 are checked against the same independent decisions.
Python tests check the last-occurrence equation and regenerate under a temporary
root. This selector does not decode control payloads, reconstruct native index
state or establish SolidWorks application acceptance.

```text
python -m unittest discover -s docs/features/flow-solver-selectors -p test_selectors.py
cargo test --locked --test flow_solver_selectors
```

The humanizer skill is unavailable; maintainer prose review remains pending.
