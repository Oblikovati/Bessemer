### 14.92 Composer geode-server lookup, null entries and duplicate registration

*Status: Native supplied-state map operations verified; file discovery, registration context and full reference resolution remain incomplete · Evidence: C-468*

Clause 14.91's local equality candidates do not by themselves reproduce the native server. This clause specifies the lower-level geode overloads of `CLitObjGlobalServer::Get` (**0x180141620**) and `Add` (**0x1801413E0**) in **litfoundations.dll**, SHA-256 **757d047481a6a2d31bf44179c140a0d7a8b3b90d619e291cba492da5d10c6bbf**. The complete inner lookup **0x180143890**, insertion **0x180143790**, key search **0x180143340** and lifetime helper **0x180145BC0** corroborate the branches below. These are runtime maps, not additional wire fields or stored offsets.

The outer map at server member **+0x30** selects a per-file value. Both public methods lowercase their copied file-key string before searching it. Within that value, the two string-to-pointer maps at **+0x18** and **+0x28** use exact GUID-key comparison without lowercasing the GUID. Here these maps are called **primary** and **override**, respectively, to describe lookup precedence rather than assert original member names.

The string methods are ordinal imports from `mfc140u.dll`: **8409** lowercases the file key and **2903** compares keys. The examined **C:/Windows/System32/mfc140u.dll**, SHA-256 **fc9b1f94752d7080436824b480e83ba1ec930e3f7baa881f4e842b882e715cfc**, exports those ordinals at **0x180002DB0** and **0x18000DFD0**. Their complete instruction ranges call **`_wcslwr_s`** and tail-call **`wcscmp`**, respectively. Thus the native lowercase operation depends on CRT locale, while GUID comparison is case-sensitive wide-string comparison. This is static provider evidence; no running process's DLL selection or locale has been measured.

**Lookup.** An absent file key returns false without changing the caller's output pointer. For an existing file key, the inner lookup searches override first, then primary. A key found in either map returns true and copies its pointer value, including a null value. A miss in both maps returns false and leaves the output pointer unchanged. Therefore a successful lookup can return a null pointer, and a null override entry shadows a non-null primary entry. Callers must distinguish lookup success from the pointer's truthiness.

**Public registration.** `Add(file, guid, incoming)` initializes a local result pointer to null and calls `Get`. It tests that pointer rather than the returned boolean. If it is non-null, the existing entry is retained. The method obtains the incoming geode's first parent, and, if present, calls that parent's removal virtual method. If the local result is null, `Add` creates the file-map value if necessary and invokes the inner insertion helper. The public method returns zero on its normal paths; the Python model below instead reports branch diagnostics.

The inner insertion rule is more specific than unconditional overwrite:

| Supplied primary state for GUID | Supplied override state | Effect |
|---|---|---|
| Key absent | Either | Insert incoming pointer into primary |
| Existing pointer equals incoming | Either | Leave both maps unchanged |
| Existing pointer differs | Override key already exists, including null | Leave both maps unchanged |
| Existing pointer differs | Override key absent | Insert incoming pointer into override |

These rules apply after the public non-null-result check. For example, fresh registration of a non-null pointer keeps the first pointer on subsequent public registrations. A primary null entry followed by a non-null registration inserts an override. An existing null override can cause public registration to reach the helper yet leave both maps unchanged. Supplied override-only state can also continue to shadow a newly inserted primary entry. No duplicate GUID winner can therefore be inferred from serialized occurrence order alone without establishing the actual registration and supplied-state context.

`solidworks/composergeodeserver.py` models these operations with explicit `seed`, `get` and `add` methods. Opaque u64 tokens represent pointer identity; token zero is null. It supports **NUL-free ASCII keys under C-locale ASCII lowercasing**. It rejects non-ASCII or NUL-containing keys rather than silently substituting Python Unicode normalization for native behavior. The current native CRT locale remains unverified. Empty keys are permitted by this map-level model; the higher-level `GetGeode` method has separate empty-reference/merge behavior and is not replaced by this model. Supplied maps preserve null entries, lookup misses retain a caller-provided initial token, and duplicate diagnostics report a requested first-parent detach without executing graph or lifetime side effects.

For the **38 installed projects**, the verifier explicitly registers **1,787 geode tokens** under each geometry member's name, then queries all **4,445 XML reference occurrences** using uppercase variants of those supplied file keys. It obtains **4,416 matches** and **29 misses**, agreeing with 14.91's local index. This verifies the model on supplied sample state; it does **not** prove native registration order, actual runtime file keys, actor-factory branch selection or external-file resolution. Every sample binary GUID is unique, so duplicate/null precedence additionally relies on native branch evidence and synthetic cases, not on invented corpus fixtures.

`composer_geode_server_evidence.py` pins six complete server/helper ranges, both MFC ordinal bodies, exact bytes, imports and hashes. `composer-geode-server-native-evidence.json` and `composer-geode-server-corpus.json` retain the evidence and every supplied-state query. Six focused tests cover file-key case changes, exact GUID case, preserved miss output, first non-null registration, override precedence, null shadows, insertion branches, invalid supplied state and all installed references. Qualified-reference parsing, path fallback order, merge/cloning behavior, graph ownership, native load context and the complete project schema remain unresolved. No global byte-coverage percentages change.

All **1,493 regression tests pass** (166.759 seconds). Specification lint reports zero findings (654 headings, 96 anchors).

Evidence: C-468.

### 14.93 Composer reference splitting and nonempty-GUID lookup fallbacks

*Status: Native lexical parsing, path helper branches and final lookup sequence verified; external resource resolution and empty-GUID merging remain incomplete · Evidence: C-469*

This continues `CLitActorFactory::GetGeode` at **0x1803916B0** in the hash-pinned `litkernel.dll` from C-467. The complete function, seven `CLitIO` helper ranges in `litfoundations.dll` and four MFC string-method bodies establish the operations below. The module hashes remain those in 14.91–14.92. These are transformations of stored reference text and supplied runtime context; they introduce no additional binary fields.

**External-reference splitting.** The first case-sensitive search is for **`Ref:Ext:`**, starting at UTF-16 code-unit position zero. Any occurrence selects the external branch; it is not an anchored-prefix check. The next search is for the first **`#` anywhere in the original text**, not just after that marker. The stored reference is then transformed as follows:

| Condition | File-reference text | Remaining GUID text |
|---|---|---|
| Marker absent | No external file text assigned | Original reference unchanged |
| Marker present, `#` absent | Substring from absolute code-unit position 8 to end | Empty string |
| Marker present, first `#` at H | Substring starting at absolute position 8, count `max(H−8,0)` | Text after that first `#`, retaining any later `#` |

The absolute start of eight does not move with the marker. For example, `xxRef:Ext:file#g` yields file text `t:file` and GUID `g`; `#junkRef:Ext:f#g` yields empty file text and GUID `junkRef:Ext:f#g`. `Ref:Ext:file` yields file text `file` with an empty GUID, which selects later merge handling rather than a normal nonempty-GUID lookup. These are native branch results, not recommended writer forms. Ordinary `ref:ext:` with different case does not select this branch. No URL decoding, trimming, UUID parsing or case folding is applied to the split text.

The MFC ordinal bodies are **4511/0x180068D10** (`Find`, calling `wcsstr`), **8451/0x1800132B0** (one-argument `Mid`, delegating to the count form), **8452/0x1800131C0** (count form with signed start/count clamping), and **12600/0x18003C210** (`Right`, with count clamping). They operate on two-byte code units. The decoder therefore uses UTF-16 units, including surrogate pairs and unpaired surrogates, rather than Python character positions. The provider is the hash-pinned system `mfc140u.dll` examined in 14.92. Its export table contains enough repeated function addresses to exceed `pefile`'s default repeated-symbol limit; the verifier raises both export limits and confirms each ordinal against the actual export-address table. This avoids treating a parser's truncated inventory as an absent export.

**Lexical path helpers.** `GetFile` (**0x18015F3B0**) uses the last backslash if any backslash exists, otherwise the last forward slash, and returns the following text. It does not choose whichever separator occurs last. `GetFileFolder` (**0x18015F880**) considers only backslash and returns the preceding text, or the literal **`.\`** when none exists. `GetFileBasePath(path,false)` (**0x18015F8E0**) removes text from the last dot anywhere in the full string; it does not first establish a filename boundary. `GetFileBase(path,false)` (**0x18015F6B0**) first performs that same last-dot truncation, then strips through the original last backslash only when there was no dot or that backslash preceded the dot. It does not strip a forward-slash directory. Thus dots in directory names and mixed separators can produce results different from standard filesystem path libraries. The optional true-mode extra-extension branches of the two base helpers are outside this profile; the examined caller passes false.

For a reference without the external marker, the initial geometry file key is selected from the supplied project path. If `GetFile(project_path)`, lowercased under the modeled ASCII/C-locale context, equals **`scenario.xml`**, the key is `GetFileFolder(project_path) + "\\geometry.geom"`. Otherwise it is `GetFileBasePath(project_path,false) + ".smggeom"`. `GetGEOMExtension(true)` at **0x180071400** supplies lowercase `smggeom`; its false branch returns `smgGeom`. The scenario branch preserves literal concatenation: input `scenario.xml` with no backslash yields **`.\\geometry.geom`**, including two consecutive backslashes, without normalization.

For an external reference, `GetGeode` copies the split file text to its external-reference output before executing `ResolvePath`, resource-loading and possible fallback logic. A missing marker leaves that output unassigned. The standalone projection uses `None` to denote this unassigned case; it does not claim the native caller's output was cleared. The initial external file text is not itself proof of a resolved file key. Actual `ResolvePath`, resource loading, callbacks and merge operations remain separate from the lexical decoder.

**Final lookup sequence for a nonempty GUID.** Given the resulting file key F and original supplied project path P, the final block calls the server in 14.92 in this order, stopping only when its pointer output becomes non-null:

1. F unchanged.
2. `GetFile(F with every '/' replaced by '\\')`.
3. `GetFileBase(P,false) + ".geom"`.
4. The literal `geometry.geom`.

`GetOldGEOMExtension` at **0x180071510** returns `geom`, and `GetOldGEOMNameInSMG` at **0x1800715D0** returns `geometry.geom`. The third candidate derives from the original project context, including for an external reference, rather than from the external file text. Candidate strings are neither deduplicated nor normalized further by this block. A server result of true with a null pointer continues to the next attempt; a false result preserves the existing null output. The native function also contains an error-callback check after the first attempt, outside the supplied-state model. Empty GUIDs take earlier merged-file/group handling and are not assigned this simple profile.

`solidworks/composerreferences.py` exposes `split_reference`, the four lexical path helpers in their described false modes, `initial_file_key`, `fallback_file_keys`, and `lookup_nonempty`. Embedded NUL is unsupported. Initial-file scenario comparison requires NUL-free ASCII context under C-locale lowering; substring splitting retains arbitrary non-NUL UTF-16 patterns. `lookup_nonempty` requires an explicitly supplied resulting file key and project path, and rejects an empty GUID because that requires the unresolved merge pipeline. It queries a supplied `GeodeServer` without file access or graph mutation; complete-reference, path-resolution and graph-side-effect flags remain false. It does not treat a raw qualified reference as an already resolved external resource.

All **4,445 installed XML references** are plain references without `Ref:Ext:`. Under explicit member-name registration and project-member context, the replay returns **4,416 non-null matches**, **29 misses**, and **4,532 total lookup attempts**: the matches stop at the first attempt, while every miss performs all four attempts. This is supplied-state validation, not a claim of measured runtime registration or path resolution. Qualified, malformed-marker, surrogate and unusual-path cases are supported by native/synthetic evidence; the installed corpus provides no qualified-reference fixtures.

`composer_references_evidence.py` pins **twelve complete native ranges**, ordinal bindings, literal bytes, source hashes and every installed projection in `composer-references-native-evidence.json` and `composer-references-corpus.json`, with synthetic split results in `composer-references-synthetic.json`. Six focused tests cover canonical and noncanonical markers, first-separator rules, UTF-16 positions, lexical path quirks, scenario handling, all fallback stopping positions, duplicate attempts, null-pointer continuation, context guards and every installed reference. External `ResolvePath` policy, actual file-key registration, callback effects, empty-GUID merging, graph cloning, full XML values and all installed project schemas remain incomplete. Global stored-byte coverage is unchanged.

All **1,499 regression tests pass** (165.543 seconds). Specification lint reports zero findings (655 headings, 96 anchors).

Evidence: C-469.
