# Database packed time

`DatabaseTimeComponents` implements the six-component arithmetic documented in
clause 9.2, C-3597. Its behavior version is `C3597-database-time-v1`. It is used
with the persisted `DatabaseHeaderFields::packed_time` word.

Packing preserves each signed32 input's bits and performs the native additions
and shifts modulo 2^32. Seconds use an arithmetic right shift by one. The date
and clock contributions are combined with OR. Inputs are not masked into
calendar ranges before those operations.

Unpacking extracts the fixed stored fields: year 1980 through 2107, month 0
through 15, day and hour 0 through 31, minute 0 through 63, and even seconds 0
through 62. Every extracted set packs back to its original stored word.
Arbitrary input components can wrap, overlap or lose bits when packed.

The default Windows VFS callback supplies month minus one, so the ordinary month
origin is zero. The helper neither invokes that callback nor adjusts months.
A zeroed input record packs to `0x88000000`. Odd positive seconds lose one on
unpacking. Seconds -1 and -2 both contribute an all-ones clock word. These are
arithmetic results, without a calendar-validity or wall-clock guarantee.

```rust
use bessemer::database_header::DatabaseTimeComponents;
let fields = DatabaseTimeComponents::from_packed_word(0x5d384126);
assert_eq!(fields.year, 2026);
assert_eq!(fields.month, 9);
assert_eq!(fields.to_packed_word(), 0x5d384126);
```

Independent Python fixtures use arbitrary-precision multiplication, floor
division and a final modulo instead of the production wrapping steps. The
8,422 vectors include zeroed inputs, all six signed/overflow boundary domains
and deterministic full-width inputs. Tests also replay every bit and 262,144
combinations of the stored word's sixteen-bit halves.

Both native database streams in the existing blank-part fixture reconstruct
their extracted fields. Six component-to-header replacements and fresh
reversals, including two odd-second no-ops, pass through complete containing
documents, preserving unrelated logical streams and the input document. Eight concurrent workers verify the
immutable component type.

Run `cargo test --test database_time` and
`python -B docs/features/database-time/fixture_bytes.py`.
The [header-buffer evidence](../database-header/README.md) records the containing
layout and its validation scope. Native provider execution, calendar conversion,
complete database storage, commit and durability remain open.

The humanizer skill is unavailable; prose review remains with the maintainer.
