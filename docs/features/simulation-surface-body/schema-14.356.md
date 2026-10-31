### 14.356 Sheet-body storage dispatch and SurfaceBody framing

*Status: storage dispatch corrected; 226 of 232 retained SurfaceBody payloads complete · Evidence: C-732–C-733*

The component reader calls sheet objects through primary virtual slot **+60**, not the solid-body slot +28. Constructor `1813302F0` installs primary table `18205A5B0`; slot `18205A610` points to **1813314F0**. The +28 entry `181331CF0` handles material propagation and is not a storage reader. C-732 records both function spans, constructor/caller instructions, installed module hashes, literals, and raw version constants. This corrects the reader characterization attached to +28 in edition 0.708; solid-body dispatch and its native replay results are unaffected.

`1813314F0` first delegates to `180A32600`, the smaller Property/Material serializer documented in 14.352, and then independently opens **SurfaceBody** (`18205A678`). Thus inherited Property, Material, SurfaceBody, and optional UserShells are separate streams. C-732's retained inventory under Sheet Bodies contains 238 Property streams / 186372 bytes, 224 Material / 760056 bytes, and 232 SurfaceBody / 34672 bytes. Counts need not match, and absence of a sibling does not justify concatenating other payloads.

SurfaceBody starts with a four-byte signed word (destination +70), then counted UTF-16 text (+68). Continue in the following wire order; destination offsets do not assign physical meanings.

| Inclusive minimum version | Wire payload |
|---|---|
| 830015 | word +80 |
| 830023 | word +88 |
| 830025 | two words +84/+8C |
| 830029 | four words +98/+9C/+A0/+A4, then five raw doubles +A8/+B0/+B8/+C0/+C8 |
| 830031 | word +90 |
| 850013 | two words +100/+94 |
| 870008 | signed count followed by that many counted texts, list +108 |
| 870022 | word +144 |

Before 830023, the native branch also tests the loaded +70 word and may derive in-memory fields from inherited Property; those assignments consume no extra SurfaceBody bytes. The later gates in the table all exceed 830023 and do not add fields to that older path. The current writer emits every listed field unconditionally. Historical writer equivalence is not established by that current writer. Separately opened UserShells has its own count/word records and is not an inline SurfaceBody suffix.

`cosworks_surface_body_layout.py` and the C-733 audit freshly hash-check native envelopes/payloads, preserve raw contiguous spans, and require exact EOF and reconstruction. **226 of 232 streams / 33778 of 34672 bytes** complete. There are 237 candidate attempts, 228 complete. All observed complete text lists are empty; populated list behavior has synthetic tests only. Three passing tests cover all version thresholds, truncation/remainder rejection, list bounds, and explicit integer-version validation.

The **six remaining payloads / 894 bytes** have sizes 146 or 152 bytes and only candidate version **860006**. At the two-word +100/+94 tail, four bytes remain where the installed reader expects eight. The parser rejects this mismatch; it does not lower the supplied version, invent padding, discard a field, or claim historical compatibility. Resolve the historical layout/version association before granting complete framing to these identities. Root/study-array candidates remain distinct from proven runtime index selection.

C-732 is static evidence and inventory; C-733 adds native SurfaceBody replay only. Inherited sheet Property, the six historical tail mismatches, populated auxiliary records, unsupported solid-body embedded classes, and field semantics remain open. No target executes and global coverage credit remains zero.

Evidence: C-732–C-733.
