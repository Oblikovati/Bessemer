### 11.244 UI bends and compressed raw flat-pattern bodies

*Status: B / ? — three native type bindings, three complete containing flat patterns and two populated raw-kernel frames verified; alternate native helper-state fixtures, other kernel forms and engineering meanings remain partial.*

`moUiBend_c` selects serializer `0x3DB6B440` in the installed hash-bound `sldsmmu.dll`. It first serializes the established `moBodyFeature_c` base. In the supported ordinary external-XML modern model context (version `>0x27CB`), its remaining fields depend on the sheet-metal helper state at runtime `+0x850`:

1. **Helper state 0:** one tagged `moOneBend_c` pointer associated with runtime `+0x808`. No count, ID word or line-data array follows in this path. Runtime association updates and clearing cached member `+0x530` consume no archive fields. With a null/mapped bend pointer the suffix is two bytes.
2. **Helper state 1:** i32 raw ID at `+0x810`, then an i32 declared ruled-line count. The native load loop reads direct line records only for positive counts; nonpositive counts produce no records. Each direct `moRuledBendLineData_c` body contains binary64 at `+0x10`, followed by i32 at `+0x18`. No object tag or map slot surrounds these direct bodies.
3. Still in helper state 1: tagged `moBendSketchLineData_c` pointer at `+0x848`, followed by direct `suObArray` at `+0x850`, with its own `ReadCount` and tagged elements. This is the wire suffix implemented by native helper `0x3DB6BE80`.

The declared line count's load gate (`>0x1448`) and the final suffix gate (`>0x141D`) both precede the supported modern range. Helper values other than 0 or 1 stop explicitly. Native storing filters the ruled-line collection to nonnull records whose runtime handle `+0x08` is nonnull, writes the filtered i32 count and serializes those records directly. That runtime handle is not a field of the line record's serializer. The decoder retains the declared count and raw values before runtime filtering or reconstruction.

`moRuledBendLineData_c` selects `0x3DB65EA0`; its direct body is exactly **12 bytes**, independent of a tagged class header. `moBendSketchLineData_c` selects `0x3DB5CBE0`: i32 values at runtime `+0x08,+0x0C`, then another i32 at `+0x10` only for version `>0x42CB`. Its body therefore occupies **8 or 12 bytes** before any tagged-object framing. All integer and double engineering domains remain unresolved. The native UI-bend loader constructs ruled-line records inline with their proven runtime vtable; tagged sketch-line records preserve normal object-map identity.

Resolving UI bends exposes the flat-pattern body-presence branch from 11.166. Native flat-pattern serializer `0x3DB5FBC0` calls `mo_loadBodyFromArchiveRaw` directly when i32 presence `+0xA08` is nonzero; **there is no suppression byte** before this body's raw frame. The newly supported selected form is:

1. i32 length `L`, excluding its own four bytes and including the following kernel-type byte.
2. u8 kernel type, required to be 0 for this selected Parasolid form.
3. Exactly `L-1` bytes containing a size-prefixed compressed chunk with GUID `231DD571DA8148A2A85898B21B89EF99`.

The raw native loader `0x4C2608F0` reads the length for ordinary noncompressed archive streams at file versions greater than `0xC9B`, then the kernel-type byte for versions greater than `0x811`. It also has compressed-stream, skip/length-1 and kernel-conversion paths; those are not generalized from these fixtures. The raw store wrapper `0x4C261910` and its context-dependent delegation are retained as native evidence. The existing normal kernel-body reader's suppression-byte prefix remains a separate framing layer.

`kernelarchive.raw_compressed_body` requires the selected GUID-bearing frame, bounded length and kernel type 0, then delegates the complete payload to the established compressed-Para-solid validator. The chunk's own size, every compressed/inflated block extent, zlib end markers, terminator and exact chunk endpoint must agree. The inflated partition must parse completely and contain exactly one BODY node. Unsupported forms stop explicitly; no opaque length-based skip is accepted as decoded geometry. Existing unknown-field accounting from the Parasolid parser remains in force.

Two native flat-pattern frames are retained with source offsets, serialized-frame/payload/inflated SHA-256 values and block descriptions. In API 2012-sm Config-0, bytes **142,322–144,469** form a **2,147-byte** frame with `L=2,143`, inflating to **4,493 bytes**. In Config-1, bytes **146,301–152,536** form a **6,235-byte** frame with `L=6,231`, inflating to **13,692 bytes**. Their containing flat-pattern roots complete at 9,100 and 12,908 bytes respectively. Gear Mount supplies a third complete flat-pattern root of 3,592 bytes without this populated raw-body branch.

The three containing roots total **25,600 bytes** and contain **16 fresh UI-bend records**, all in native helper state 0. Both helper paths, positive direct-line counts, tagged/shared sketch-line identity and nonpositive count handling are exercised synthetically; no native helper-state-1 fixture is claimed.

The 906-context audit now contains **25,416 complete roots**, **819 complete lists** and **87 explicit stops**, with every preceding complete root record unchanged. This adds three roots and three exact-EOF lists: Gear Mount (184,271 decoded bytes), API 2012-sm Config-0 (144,611) and Config-1 (152,678), totaling **481,560 bytes**. Totals are **741 exact-EOF lists / 60,491,872 decoded bytes** and **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.263.json` and `resolved-feature-list-frontier-0.264.json` retain the comparison. These results do not establish complete semantic document/project bit coverage.

Five focused tests cover fixed record widths and late gates, both helper layouts, direct/tagged collection identity, count rejection and nonpositive declarations, inherited feature composition, raw-frame versus suppression-prefixed framing, malformed/truncated/context-invalid frames and all three native containing-root/list boundaries. Native positive raw-body frames are re-parsed independently and checked against their retained compressed and inflated hashes. Historical/DB modes, alternate stream/kernel forms, raw engineering semantics and complete part/assembly/drawing/all-installed-project coverage remain open.

Validation: all **891 regression tests pass** (90.738 seconds); specification lint reports zero findings (468 headings, 96 anchors).

Evidence: C-286.
