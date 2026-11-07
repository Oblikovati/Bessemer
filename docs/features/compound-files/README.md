# Compound-file stream interoperability

Feature [#106](https://github.com/Oblikovati/Bessemer/issues/106) adds the MS-CFB
carrier needed by legacy SolidWorks documents, nested storages and Simulation
CWR files. It implements existing-stream reading, reconstruction and replacement.
Payload schemas and structural insertion remain separate work; this does not
complete SolidWorks or Corliss parity.

`CompoundFile::from_bytes(bytes, limits)` reads version 3 (512-byte sectors) and
version 4 (4,096-byte sectors), including extended DIFAT, FAT, MiniFAT, the root
mini stream, storage hierarchy and every user stream. Before publication it
reconstructs a carrier, reopens it, and compares every logical stream, directory
metadata record and parent identity.

The structural check validates complete chain extents and exclusive sector
ownership, rejects cycles and aliases, and checks table markers, physical bounds,
free padding, directory reachability and distinct raw child names. Allocated
sectors without an established owner fail. Inactive directory bytes remain
verbatim, without assigning them active roles. Explicit file, individual stream,
aggregate stream, directory-slot and storage-depth limits bound the work.
Stream capacity may exceed its logical length. The reader claims the entire
chain, requires enough capacity, and returns only the logical prefix. Exact
declared counts still apply to MiniFAT and version-4 directory chains. Canonical
saves may omit surplus normal, mini or root-stream capacity; unchanged saves
retain it exactly. These cases are tracked in #109.

`entries()` returns active entries in original slot order, including the root.
Each has a revision-bound ID, parent ID, kind, raw UTF-16 name, logical size and
original 128-byte directory record. Unpaired name surrogates remain exact.
`entry_at_path` performs exact stored-name lookup from a Unicode path; it does
not substitute modern Unicode case folding for CFB collation. Storage objects
have logical size zero; their uninterpreted size bytes remain in the raw record.

`stream(id)` returns complete immutable user bytes. `to_bytes()` retains the
exact current carrier, including original slack when unchanged.
`reconstructed_bytes()` rebuilds allocation with the same logical preservation
contract. Reconstructed saves clear the transaction signature, zero-fill new
sector slack and normalize version-3 size high words. They can omit free physical
sectors. Directory slots, links, colors, names, class IDs, state bits, timestamps
and bytes outside root/stream allocation and size fields remain unchanged.

`edit_stream(id, replacement)` constructs an isolated snapshot, supporting growth,
shrinkage, empty streams and both directions across the 4,096-byte mini-stream
cutoff. It reopens the complete result and compares every other stream and
metadata record. A no-op retains exact bytes and identity. Changed results reject
old IDs and return completion, exact logical fidelity, every entry derivation,
coverage and required unchecked responsibilities together.

Snapshots own their bytes and immutable maps, are `Send`/`Sync`, and need no read
locks. Tests cover input mutation/drop, failure isolation, six concurrent edits
and byte-identical output.

The declared mode is `ExactAllocationAndDirectoryLinks`. Full Unicode directory
collation and red-black balance, payload semantics and native application
acceptance are unchecked. The writer retains existing links and names; it does
not insert, remove, rename or reorder objects. Completed carrier operations do
not establish that their payloads form valid CAD or Simulation models.

Ten independently built fixtures cover empty files, both sector sizes, nested
storage, normal/mini/empty streams, reversed physical chains, extended DIFAT,
uninitialized version-3 size high words, a red root and surplus normal/mini/root
capacity. Each includes an
independent complete canonical-output golden. Further tests reject short
prefixes, malformed headers, cycles, cross-linked streams, unowned allocations,
bad directory links and exceeded budgets.

The installed baseline contains 43 files: three version-4 CWR files and 40
version-3 carriers, including blocks, legacy parts and spreadsheets/templates.
It covers 9,090,560 physical bytes, 528 user streams and 8,763,115 logical bytes.
Every stream was replaced in isolation. All 528 output files were reopened
through the actual frozen reference reader, comparing every stream and metadata
record. Discovery covers the recorded sample roots and benchmark CWR, not every
installed or nested compound file.

For version 3, the reference projection uses the low size DWORD required by
MS-CFB. Reader functions are unchanged; returned bytes are sliced to that logical
extent. A separate legacy fixture pins this compatibility rule and the zero high
word required on write.

The root-color regression is tracked in [#107](https://github.com/Oblikovati/Bessemer/issues/107).
The first implementation generalized the black root in the example. The field
definition permits both color values, and installed CWR roots use zero. The
parser retains the general 0/1 validation and exact color without recoloring.

Range-lock allocation above 2 GB is tracked in #110. The allocator reserves the
entire 4,096-byte sector covering offsets `0x7FFFFF00..0x7FFFFFFF`, marks it with
ENDOFCHAIN, and excludes it from data, directory and table chains. Address
mapping, split copies, FAT self-coverage and exclusive ownership have independent
boundary tests. No complete multi-gigabyte native file was executed; that remains
an explicit verification exclusion.

Run `python docs/features/compound-files/freeze_compound.py` to replay the owned
and installed baselines. Add `--export` for the local native probe, then run
`cargo test --test compound_files -- --include-ignored`. Run
`verify_native_edits.py` after the native test. `--write` is reserved for reviewed
baseline updates. Modules are copied and hash-bound before import, with retained
origin checks. No installed native application executes.

Normative references:

- [Header](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/05060311-bfce-4b12-874d-71fd4ce63aea)
- [Directory fields](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/60fe8611-66c3-496b-b70d-a504c94c9ace)
- [Root entry](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/026fde6e-143d-41bf-a7da-c08b2130d50e)
- [Other entries and legacy high-DWORD compatibility](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/b37413bb-f3ef-4adc-b18e-29bddd62c26e)
- [DIFAT sectors](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/0afa4e43-b18f-432a-9917-4f276eca7a73)
- [Stream capacity](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/b089deda-be20-4b4a-aad5-fbe68bb19672)
- [Reserved range-lock sector](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/947df128-1f4a-4c02-b6ed-6d1d0c04c02f)
- [Validation and corruption](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/3c5249cc-1dc2-46f0-8faf-06c6a36f0085)

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
