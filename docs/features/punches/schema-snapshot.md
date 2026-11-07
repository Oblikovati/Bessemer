### 11.235 Punch features and internal record revisions

*Status: B / ? — native-bound revision-dependent layout and two populated revision-7 roots verified; raw option meanings and historical/native fixture diversity remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moPunch_c` selects serializer `0x18032DB30` in `sldMoPartu.dll`. It writes the body-feature base, then reads an **i32 internal record revision** at runtime `+0x858`. This revision is independent of the enclosing file version and MFC class schema. Constructor `0x18032CBC0` initializes it to 7.

The selected loader's suffix is:

1. Internal revision at `+0x858`, followed by tagged `moDisplayDim_c` pointers at `+0x800` and `+0x808`.
2. If revision is at least 1, i32 at `+0x864`; otherwise the native runtime field is initialized to zero.
3. Nullable `suObArray` at `+0x810`, using the established i32-presence/conditional-direct-body grammar. If revision is below 3, another nullable array at `+0x818`.
4. Two i32 values at `+0x860`, `+0x840`, then nullable `suObArray` at `+0x820`.
5. If revision is below 2, one direct `mgPoint_c` (three binary64 values) and one i32. The native loader appends them to its runtime point/integer arrays. Otherwise, serialize the point dynamic array at `+0x848`, then integer dynamic array at `+0x830`, using the grammar below.
6. If revision is at least 4, i32 at `+0x910`; at least 5, i32 at `+0x914`; at least 6, two i32 values at `+0x918`, `+0x85C`. Missing fields receive native defaults; the raw decoder records only fields actually present.
7. If revision is at least 7, one **tagged** `suObArray` pointer at `+0x920`. This differs from the nullable direct-array grammar used earlier.
8. Extension sequence described below.
9. If revision is below 2, a direct `moPunchSIdKeeper_c` body at runtime pointer `+0x908`, after the parent post-load call. Its primary vtable selects thunk `0x1803AC9DA`, bound to the existing `moEdgeFaceSIdKeeper_c::Serialize` layout. The keeper's enclosing-file-version gates still apply.

The point and integer dynamic-array loaders begin with a **u32 element width in bytes**, not an element count. Width zero ends that array immediately. A nonzero width must be divisible by 24 for point entries or 4 for integer entries. Next read two raw u32 allocator metadata words and a u32 entry count, then exactly `width × count` bytes of entries. Preserve width, metadata and count independently. For example, point width 48 means each entry carries two points; integer width 8 means two four-byte integers per entry. Bound count using remaining bytes before reading. The native writers additionally require the second metadata word to be 1 or 2; the loader passes those metadata fields to its allocator, so their broader meanings are not inferred here.

The point loader chain is `0x18032DAD0` → `0x18032C780` → `0x18032C650`; the integer chain is `0x18032DA70` → `0x18032C750` → `0x18032C570`. Constructor type declarations and the writers at `0x18032C8A0`/`0x18032C7B0` independently establish point versus integer widths. A runtime dynamic-array pointer or allocation size is not itself a wire field.

The extension loop reads one i32 tag at a time:

| Tag | Following payload | Next action |
|---|---|---|
| `0x3D2` | i32 | Continue reading tags |
| `0x13E` | binary64 | Continue reading tags |
| `0x1466` | generic tagged object pointer | Continue reading tags |
| Any other value, including 0 | No payload consumed by this loader | Terminate the loop |

The native loader consumes and discards recognized extension values; the diagnostic reader retains their order, values and object identity. It also retains the actual terminating tag, which need not be zero. An unknown terminating tag does not authorize skipping an invented payload. The selected writer emits a zero terminator and no recognized extension entries.

The writer emits its current scalar/dynamic-array suffix unconditionally rather than reproducing every older load branch. Therefore the loader revision gates must not be treated as a historical writer specification. Tests exercise revisions -1, 0 through 8 using the selected native loader's comparisons; the two installed positive fixtures both store revision 7. Stored option scalars, allocator metadata, point/integer values and consumed extension payloads remain unknown for semantic coverage accounting.

Two complete native punch roots total **12,839 bytes**: Bottom Housing (6,506) and Main Housing (6,333). Each has one point-array entry of width 24, one integer entry of width 4, no extension entries and a zero terminal tag. Main Housing's feature list completes; Bottom Housing reaches the explicit `sgIntersection_c` zero-marker-body stop. Source stream/root hashes and raw words are retained. These results establish selected framing without claiming whole-document or all-project-format bit coverage.

The 906-context audit contains **24,985 complete roots**, **794 complete lists** and **112 explicit stops**, with every preceding complete root record unchanged. This adds 41 roots and one exact-EOF list: Main Housing, **1,982,364 decoded bytes**. Totals are **721 exact-EOF lists / 55,171,677 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.254.json` and `resolved-feature-list-frontier-0.255.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 860 regression tests pass (99.668 seconds); specification lint reports zero findings.

Evidence: C-277.
