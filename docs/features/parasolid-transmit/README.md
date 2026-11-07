# Parasolid binary transmit files

`bessemer::parasolid` implements the complete binary transmit layout in
[clause 10](specification-10.md): partitions, rollback deltas, neutral `PS`
big-endian files, and bare `B` little-endian files. This is transmit behavior
version 1. It reads complete files, constructs new archives, edits owned typed
contents, and writes every field without an opaque payload fallback.

The library retains the title, normalized schema key, writer's type-table size,
base/edit/new layout descriptors, field definitions, record order, indices,
variable lengths, list terminators, and every delta section. A caller supplies
an immutable `BaseSchema` loaded from its own schema bytes or explicit typed
definitions. There is no installation lookup, global schema cache, or Python
runtime dependency. Installed schema files are not redistributed here.

Field values distinguish integers, pointers, raw unsigned/character/logical bytes,
shorts, UTF-16 code units, doubles, intervals, vectors, hvecs, and boxes. Scalars
have one element; fixed and variable arrays retain their exact element counts.
Floating-point values retain their IEEE-754 bits, including NaN payloads, signed
zero, and infinities. UTF-16 fields retain code units without assuming they form
a CString. Pointer flags remain raw bytes. Partition hvecs have three doubles;
delta hvecs retain all eleven. The initial delta list treats types 3 and 4 as
plain records; modified lists treat every record as a delta.

Base-schema declarations for `q` and `t` remain available as typed declarations,
but encountering them in an effective transmitted layout fails explicitly.
Nonzero `user_field_size` is unsupported. Numeric schema identities follow the
documented `SCH_<modeller>_<writer>_<base>` grammar. Older underscored identities
are rejected rather than guessed or normalized into a different schema.

`TransmitFile` publishes immutable Send/Sync contents and keeps its base schema
alive. Callers edit an owned copy and submit it through `with_contents`.
Validation checks schema identity, layout compatibility, array shape, field-value
kind and count, pointer-flag presence, title/body agreement, section framing, and
missing or orphan layouts. Decode budgets limit records and field elements;
counts are also checked against remaining bytes before allocation. An enclosing
stream's decompression limit can bound input before decoding.

Delta lists are delimited by terminators, never by the raw stored modification
count. A file ending after an initial record list is complete: its final section
has no changes. Bytes ending inside a required field or change section fail.
No-op saves, equivalent edits, and reverts retain original bytes. Actual rebuilds
can shorten nonminimal index encodings and normalize numeric key spellings.
Unknown field meanings and enum domains remain uninterpreted.

These are archive checks. They do not establish pointer-class compatibility,
topology, geometric consistency, self-intersection, tolerances, or kernel
acceptance. Delta references can legitimately target an associated partition,
so archive parsing does not require every target to occur in the same file.
The API returns typed transmit records, not a validated geometric body.

## Usage

```rust
use bessemer::parasolid::{BaseSchema, TransmitFile, TransmitLimits};

let base = BaseSchema::from_bytes(&std::fs::read("sch_13006.s_t")?)?;
let limits = TransmitLimits {
    maximum_records: 1_000_000,
    maximum_field_elements: 10_000_000,
};
let source = TransmitFile::from_bytes(&transmit_bytes, &base, limits)?;
let rebuilt = TransmitFile::new(&base, source.contents().clone())?;
let output = rebuilt.to_bytes()?;
```

`layout_fields(type_code)` resolves positional values to their effective field
names and definitions. `rebuild_transmit` accepts `BASE_SCHEMA INPUT OUTPUT`
pairs; `--bare` explicitly converts the numeric byte order. It always rebuilds
from typed fields rather than using the original-buffer path.

`set_world_highest_id` accepts `SOURCE BASE_SCHEMA STREAM HIGHEST_ID DESTINATION`.
It edits the first partition chunk's WORLD counter, then writes the carrier and
document while preserving other chunks and streams. This demonstrates a complete
typed field edit; its archive checks do not establish the counter's application
semantics or the validity of arbitrary model edits.

## Verification

Eight owned vectors include the blank part's native partition and deltas, plus
six independently assembled files. The synthetic cases cover every primitive,
both byte orders, scalar/fixed/variable arrays, empty arrays, all layout edits,
fully defined types, short and extended indices, maximum indices, raw delta
flags, eleven-double hvecs, and complete deltas ending after their first list.
[The generator](generate_fixtures.py) verifies these with the Python reference
parser and a named fixture schema source. The accompanying base schema is an
independent test schema with the declarations needed for those native and
synthetic cases. Normal Rust tests embed all inputs and need no installation.

Tests compare independent expected field kinds and values and force exact
reconstruction. Byte-order conversions match independently assembled output.
They cover every truncation against the independent valid-section boundary,
generated mutations at every byte, record/element budgets, oversized variable
lengths, invalid schemas and layouts, explicit descriptor count 254, fixed extent
65,534, pointer-flag modes, construction, edits, empty results, equivalent edits,
and reverts. They check input-byte and caller-value isolation. All public transmit
types have compile-time Send/Sync checks; eight concurrent workers compare
independent edits with serial reconstruction.

On Windows x64 with Rust 1.99.0/MSVC, the final implementation reconstructed all
2,869 installed transmit files: 1,851 partitions and 1,018 deltas, containing
2,396,293 records and 129,715,545 inflated bytes. The independent reader parsed
each source to its final byte; every Rust reconstruction matched every source
byte. The ordered corpus manifest SHA-256 is
`50e1e5f61e33572c1bbf7e30bf8b0ef3c7f7f0677e00735fb56bb83fe939b7cd`.
Its input concatenates the UTF-8 sample-relative path with `/` separators, a zero
byte, stream name, another zero byte, decimal chunk index, another zero byte,
and inflated transmit bytes. Samples are sorted by relative path; streams retain
directory order and chunks retain carrier order.

All 100 installed base-schema files were checked. The 88 with numeric identities
parsed and passed declared type/field-count checks; the 12 legacy underscored
identities were rejected by the documented numeric-key boundary. These results
do not claim compatibility with those legacy formats.

An independent whole-document check changed only the owned blank part's WORLD
`highest_id` field. Every other typed field remained unchanged; the delta chunk
and every other stream retained their exact compressed bytes. Separate processes
produced identical edited documents and identical forced rebuilds of all eight
vectors. Temporary verification programs and outputs were discarded.

`cargo xtask pr` passed locally with 116 integration tests, 62 documentation
examples, and three xtask unit tests. It is the formatting, clippy, build, test,
and documentation gate, run with the pinned toolchain directory first on `PATH` and a separate
`target/pinned-1.99.0` directory to avoid mixing local compiler versions. The
feature PR records the five platform checks and Linux ThreadSanitizer run.
No dependency was added. Native Parasolid kernel acceptance, SolidWorks
application acceptance, and performance benchmarks remain unexecuted. There is
no earlier Bessemer release for a release-to-release benchmark.

## Reference identity

The approved reference is `../inventor-interface`, commit
`dd0537b3a142d8dda00ef846891efc736a6aac46`, with uncommitted schema work.
The saved clause was checked against specification edition 0.114 on 2026-10-24.
The raw full-file SHA-256 was
`1212821923576a3b751de52a71b2301ceee49b4217b5106f8f78caff1489600e`.
The saved UTF-8/LF clause fragment has SHA-256
`91c33cdcf1ddc41f4a853fb68715e03beb2aa9da7aaff7f80b3e00d8cdcc9c11`.

| Reference input | SHA-256 |
| --- | --- |
| solidworks/xtbin.py | 3fb876a36e79402cf8ab90ca64e2c2de8e8a3f4b6290af41937ee6a9bfc8870d |
| solidworks/swcover.py | f11d8e93420921c7ba4aaa42406d95365a856472d654e207cc6fb178ae3225b2 |
| Installed sch_13006.s_t | cc98d0c96c959cc0216cee45e4adb785340ed99af7ecb6129f84633a738846af |

The humanizer skill is unavailable; its prose pass remains for maintainer review.
