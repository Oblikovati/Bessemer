### 14.267 Native Cosworks study-version array and reader fallback

*Status: Counted integer framing verified on 51 native streams and tied to installed calls; precise per-study update policy remains partial · Evidence: C-643*

The exact stream **`Cosworks/StudyManager/StudyVersionNumbers`** occurs in **51** C-590 native envelopes. Fresh hash-checked extraction verifies its complete shape as **one little-endian i32 count C, followed by exactly C little-endian i32 values**, with no trailing bytes. Every observed count is nonnegative: **28 streams have one value, 18 have two, four have three and one has six**. The payloads total **532 logical bytes**.

Every one of these 51 streams has a coexisting **StudyManager/StudyListOrder** record in the retained inventory, and **all 51 counts equal that record's item count**. Values range from **870022 to 891107**, with 22 distinct observed integers. **Four arrays contain more than one distinct value.** Preserve every value and its order; neither a single repeated document version nor an unordered set reproduces these records. Count correspondence is strong structural evidence, but native row identity/activation is not independently executed by this audit.

The installed reader **0x1814BD4B0** references UTF-16 literal **StudyVersionNumbers at 0x1820517F8**, constructs CwPersistStream in mode 1, reads an int count, then reads positive-count int entries into an owner-associated collection. If opening this stream fails, a retained fallback loop instead appends the owner version field for the previously read study-order count. That absent-stream fallback must not overwrite explicit mixed values when the stream is present.

The writer **0x1814C1520** creates the same named stream, writes its computed count and then one int per loop iteration. It normally starts each value from the current global version integer, while guarded branches can retain an existing per-index version from the owner's array; some branches update the stored array as well. This confirms a version-maintenance path distinct from pure serialization of one global constant. Full conditions governing preservation, filtered study names and owner-array correspondence remain separate decoding dependencies.

The four-byte primitive provider is verified in 14.262. The bounded native audit requires **size = 4 + 4*C** and limits C before unpacking, whereas the installed reader's signed positive-loop behavior and ignored primitive I/O status retain the limitations already described there. No additional string prefix, checksum, padding or eight-byte long is inferred for this array.

`cosworks_study_versions_audit.py` verifies outer and nested payload hashes, all 51 count/size relationships and all 51 paired name counts. `cosworks-study-versions-native-evidence.json` preserves full payloads, ordered values, paired names and hashes of both installed reader/writer decompilations. No target, query or Simulation API executes and no global byte-coverage credit is assigned. Version-update policy, study-manager property fields, native runtime validation and complete installed-format coverage remain unfinished.

Evidence: C-643.

### 14.271 Bounded native study-version array decoder

*Status: Independent counted-i32 decoder validated on all 51 retained arrays; version policy and complete semantic coverage remain unfinished · Evidence: C-647*

The framing inspector **`solidworks/cosworkspersist.py`** now exposes **read_integer_array** for the StudyVersionNumbers shape recovered in 14.267. It reads a **signed little-endian i32 count**, requires a nonnegative value within the caller's limit (default **10,000**), checks the complete remaining size against **4*C**, and returns an immutable IntegerArray with original count bytes, original payload bytes, ordered signed values and final endpoint. A zero count requires exactly a four-byte stream. Truncation, oversized/negative counts and trailing bytes are rejected before unpacking the values.

The decoder retains signed bit interpretation, duplicate values and ordering. It does not map version integers to product releases, substitute a document-wide version, infer absent-stream fallback or modify the paired study-name list. Rejecting a negative count is an explicit bounded-inspection policy; it does not emulate the installed positive-loop reader's treatment of malformed negative counts.

The focused Cosworks decoder suite now passes **nine tests**, including signed extrema and zero-length arrays, exact byte reconstruction, negative/over-limit counts, insufficient values and surplus bytes. Earlier text/list tests remain passing. **cosworks_integer_decoder_validation.py** additionally verifies raw SHA-256 values, ordered decoded integers, counts and exact reconstruction of **all 51 retained native arrays / 532 bytes** against C-643. `cosworks-integer-decoder-validation.json` records those per-stream checks and current implementation/test hashes without overwriting the earlier C-639 framing-validation checkpoint.

This adds a usable lossless decoder for the recovered array framing while preserving the unresolved distinction between serialized version slots and the application's version-update policy. It does not register full Simulation semantic coverage, execute installed code or assign global byte-coverage credit. Manager-slot meaning, historical property layouts, native writer round-trips and complete installed-format coverage remain unfinished.

Evidence: C-647.

### 14.272 Native Cosworks UniqueIds used/allocation header and word spans

*Status: Complete observed framing on 51 native streams linked to installed reader/writer; identifier semantics remain partial · Evidence: C-648*

The exact stream **`Cosworks/StudyManager/UniqueIds`**, referenced by installed UTF-16 literal **0x182066470**, occurs in **51** retained C-590 envelopes. Fresh hash-checked extraction verifies **two little-endian i32 header values U and A, followed by exactly 2*U four-byte words**, giving complete size **8 + 8*U**. All observed headers satisfy **0<=U<=A**. The payloads total **3,952 logical bytes**.

The installed reader **0x1814BD4B0** reads the two headers into globals **0x18260239C and 0x1826023A0**. It allocates **8*A bytes**, then reads **2*U long values** into four-byte slots. The primitive provider's long width is verified in 14.262. The installed writer **0x1814C1520** emits both header values followed by the same **2*U four-byte** values; it does not write the entire allocation span. These operations support the neutral interpretations **used-entry count U** and **allocation-entry count A**, without proving the allocator's growth policy or each entry's application meaning.

| Native observation | Result |
|---|---|
| zero used entries | 33 streams |
| allocation count 0 | 32 streams |
| allocation count 1000 | 19 streams |
| populated streams | 18 |
| largest used count | 106 |
| total used eight-byte spans | 443 |

One zero-used stream therefore still carries allocation count 1000 while containing only its eight-byte header. Allocation capacity must not be mistaken for serialized payload count or used to discard bytes as inferred padding. The audit does not allocate according to A; it checks the bounded actual payload before unpacking U entries.

Each native eight-byte span consists of two retained u32 words. **Every second word is zero in all 443 observed spans**, and there are no duplicate adjacent-word pairs within any one stream. This supports further investigation of a possible combined identifier representation, but does not establish native u64 semantics, signedness, object-reference targets or cross-document uniqueness. Preserve both words and their order, including currently zero words, rather than truncate them or apply a guessed identifier formula.

If the installed reader cannot open UniqueIds, its retained branch clears the relevant count/pointer globals. That runtime fallback is distinct from a present empty stream or a present allocated-but-unused record. Native format validity beyond the observed nonnegative/count relationships, malformed allocation values and application lookup behavior remain unresolved.

`cosworks_unique_ids_audit.py` verifies outer/payload SHA-256 values, every header/count/size relationship and both installed reader/writer decompilation references. `cosworks-unique-ids-native-evidence.json` retains full raw bytes, ordered words, adjacent pairs, header values and source paths. No target allocator, installed code or Simulation API executes and no global byte-coverage credit is assigned. Identifier assignment/consumers, property-slot meanings and complete installed-format coverage remain unfinished.

Evidence: C-648.

### 14.276 Bounded lossless Cosworks UniqueIds table decoder

*Status: Native used/allocation/i64 framing implemented and validated; runtime mapper and reference resolution remain partial · Evidence: C-652*

**read_unique_ids** in `solidworks/cosworkspersist.py` reads the layout established in 14.272–14.273: **signed i32 used count, signed i32 allocation count, then used-count signed i64 keys**, all little-endian. It returns an immutable UniqueIdTable with original eight-byte header, complete raw payload, both counts, ordered keys and final endpoint. The parser requires nonnegative used count, allocation>=used, used<=the caller's limit (default **10,000**) and exact stream size **8+8*used**. Truncated headers/entries and trailing bytes are rejected.

Allocation capacity is retained without allocating or consuming capacity-sized data. A zero-used table can therefore preserve a large nonzero allocation header while remaining an eight-byte record. Negative capacity is rejected through the allocation>=used condition. This is a bounded structural policy, not an emulation of the native allocator, last-key cache, one-based lookup or application failure behavior.

The decoder preserves signed values, nonzero high words, duplicate keys and their order. It does not remove duplicates merely because the observed mapper normally reuses existing keys, and it does not assume that future keys' high words are zero. Semantic table invariants and input-pair reconstruction remain distinct from framing inspection.

The focused Cosworks suite now passes **12 tests**. New cases exercise signed i64 extrema, values above 32 bits, duplicates, metadata-only large capacities, negative/inconsistent counts, configured entry limits, truncated entries and surplus bytes. Earlier text/study-list/i32-array tests remain passing. **cosworks_unique_decoder_validation.py** checks all **51 retained native tables, 443 keys and 3,952 bytes** against raw SHA-256 values, both header counts and original word pairs; every table reconstructs exactly.

`cosworks-unique-decoder-validation.json` records those per-stream checks and current implementation/test hashes, preserving earlier validation checkpoints. No installed target, mapper, allocator or COM API executes and no global byte-coverage credit is assigned. Identifier input meanings, actual model-reference resolution and complete installed-format coverage remain unfinished.

Evidence: C-652.

### 14.444 StudyVersionNumbers counted array and caller-specific selection

*Status: array read/write framing and scoped native carriers complete; all upstream runtime selection not yet established — Evidence: C-824*

`Cosworks/StudyManager/StudyVersionNumbers` uses literal `1820517F8`. Its body is a **signed 32-bit count**, followed immediately by that many **signed 32-bit saved-version values**, with no text wrapper, per-entry tag or additional locally read field. Root loader `1814BD4B0` opens the stream, reads the count and values through `CwPersistStream`, and appends each value in order to the manager's version collection (`+108` collection object, backing data `+110`, size `+118`). Both located load branches use the same grammar. If the stream is absent, these branches populate the version collection from root member `+188`, repeated for the study count obtained from the companion study listing. That fallback is runtime state construction and consumes no nonexistent version-stream bytes.

Writer `1814C1520` writes the study count and one four-byte version per study. Depending on its surrounding runtime conditions, it either chooses current global version `18254E3B0` and updates the corresponding in-memory entry, or preserves an existing indexed entry. This distinction matters: all saved study versions cannot be assumed equal to the current writer or root version. The branch conditions and bounds are retained in C-824's static evidence; the array reader preserves every raw value without normalizing it.

A separate verified caller, `1811CCD80`, reads the same counted version array into a local collection and uses the root version as an absence fallback. It passes an indexed value to study virtual `+60`; installed table slot `182063EA0` contains `18142ACF0`, binding this slot to the study loader discussed in 14.443. The call occurs only under that caller's surrounding study-name/type/configuration and storage conditions. This establishes an indexed saved-version route into the loader, not that this particular branch loaded the retained topology sample or that every other virtual caller chooses versions identically.

`cosworks_study_versions.py` requires a nonnegative bounded count and exact payload length `4 + 4*count`; it retains the raw bytes and signed values. Malformed counts, truncated arrays and trailing bytes are rejected. Native readers' positive-count loops do not make negative counts valid complete arrays for the independent schema. C-824 verifies installed module/literal/table/function hashes and replays all **51** matching installed carriers, totaling **532 bytes**, exactly. Together they contain **82** version entries. No other matching carrier path was found in this inventory. Synthetic checks cover empty and multivalue arrays, raw integer extremes, every truncated prefix, trailing words and count bounds. No target code executed; complete installed-format coverage, domain meanings elsewhere and all caller-specific version-selection conditions remain open. Global coverage credit remains zero.

Evidence: C-824.
