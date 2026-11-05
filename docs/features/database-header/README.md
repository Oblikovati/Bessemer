# Conditional database header buffers

`database_header` reads and reconstructs the complete 512-byte buffer selected
by clause 9.2's default Windows database-header path (C-3597). The caller declares
`DatabaseHeaderProfile::DefaultWindows512`. This profile records the assumed
provider context; bytes alone do not establish the original provider identity.
The behavior version is `C3597-database-header-v1`.

The codec checks the seven-byte `unqlite` signature and fixed BE32 magic,
preserves the packed time word, and reads S, P, the BE16 name count and raw name.
S must be a power of two from 32 through 65536. P accepts every integer from
512 through 65536. The separate native configuration rule that requires a
power-of-two page size does not narrow this stored P field.

Names retain embedded NUL and non-UTF-8 bytes. A declared name that exceeds the
487 bytes available in the selected buffer is rejected with its count in the
error. The native reader's clamp is not applied as an offline repair. Packed
time remains an unchanged u32, without calendar validation or conversion.

`DatabaseHeaderBuffer` owns its fields and trailing bytes. Construction requires
explicit fields plus enough trailing bytes to account for all 512 bytes. The
interpreted prefix length is reported separately: it is 29 bytes for engine
name `hash`. Remaining bytes are retained without assigning meaning or claiming
that they passed database validation.

```rust,no_run
use bessemer::database_header::{DatabaseHeaderBuffer, DatabaseHeaderProfile};
# let stream = std::fs::read("database-stream.bin")?;
let original = DatabaseHeaderBuffer::from_bytes(
    stream.get(..512).ok_or("missing header buffer")?,
    DatabaseHeaderProfile::DefaultWindows512,
)?;
assert_eq!(original.to_bytes(), stream[..512]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Tests cover all 65,025 admitted P values, every S value through 65537 plus
signed boundaries, all 488 fitting name lengths, arbitrary time words, every
proper buffer prefix, extra bytes and each signature byte. Sixteen frozen
literal buffers come from an independent Python builder. Two actual database
streams in the existing blank-part fixture reconstruct their headers exactly.
Time-word replacement and fresh reversal pass through complete containing
documents while preserving every unrelated logical stream and the source.

Run `cargo test --test database_header`, `cargo test --lib database_header`, and
`python -B docs/features/database-header/fixture_bytes.py`.

This operation validates the selected header prefix. Complete database extent,
engine association, hash pages, key/value storage, native I/O, commit and
durability remain open. The conditional MsgPack writer is a separate grammar.
The humanizer skill is unavailable; prose review remains with the maintainer.
