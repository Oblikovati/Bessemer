### 14.262 Installed CwPersistStream string and four-byte primitives

*Status: Five provider routines recovered and tied to observed native framing; field semantics and failure fixtures remain partial · Evidence: C-638*

The CwPersistStream overloads called by 14.261 are imports from installed **CosUtilities.dll**, SHA-256 **`06253d275a6db7a33cd56a9c9dc0646c59aab5e1b58a2c86002f818243657e18`**. Export addresses identify CString extraction **0x18005B820**, CString insertion **0x18005B920**, shared four-byte extraction **0x18005B7C0**, shared int/long insertion **0x18005BA50**, and the BSTR read helper **0x18005BE30** reached by CString extraction.

The string writer allocates a BSTR from the supplied CString pointer through SysAllocString. For a nonnull BSTR it writes a four-byte length equal to **SysStringByteLen(BSTR)+2**, then writes that many bytes from the BSTR pointer, including its UTF-16 terminator. A null BSTR pointer produces length **0** and no payload. The prefix Write HRESULT is tested for nonnegative status before the payload call; the payload Write return and transferred-byte counts are not independently validated in this overload. Its nonnull CString conversion can inherit NUL-terminated source behavior; it must not be assumed to preserve arbitrary CString embedded-NUL suffixes.

The BSTR reader helper performs these retained checks:

1. Reject a null stream with HRESULT **0x80070057**. Otherwise free/clear the prior output BSTR and attempt to record the current stream position with Seek.
2. Read exactly **four bytes** of length, checking both HRESULT and transferred count. Length **0** returns with a null BSTR. Nonzero lengths below **2** fail with **0x80004005**; lengths above **0x100000** fail with **0x80070005**.
3. Allocate a BSTR byte buffer for **length-2** bytes with SysAllocStringByteLen, read that payload with an exact count check, then separately read **two bytes** and require both the exact count and a zero 16-bit terminator.
4. On applicable failure paths, free/clear the output and, when the initial Seek succeeded, attempt to restore the saved stream position. The rollback Seek result is not independently checked before returning the earlier error.

The helper contains no explicit even-length or UTF-16 scalar validation. The even byte counts observed in 14.259 remain native-corpus facts; a bounded schema reader may reject malformed odd lengths without claiming that this provider explicitly does so. **All 857 text spans retained by C-635** satisfy the recovered nonzero length bound and terminal-NUL condition. Zero-length/null cases are supported by installed code but are not represented by those observed spans.

CString extraction initializes a temporary BSTR to null, calls this helper, **discards its returned HRESULT**, assigns the resulting pointer through a CString wide-pointer assignment overload, frees the temporary and returns the persistence object. Thus helper validation does not prove that the wrapper propagates a read error to its caller. Exact assignment behavior for null pointers and embedded NUL remains an MFC-provider dependency; a lossless decoder should retain the complete declared payload regardless of application text views.

The shared extraction function reads **four raw bytes** through IStream.Read with a null transferred-count pointer; its exported aliases include int, unsigned int, long, unsigned long and float-reference overloads. It does not inspect the HRESULT. The int/long insertion function likewise writes four bytes of the supplied value with no count/status validation. These installed x64 implementations corroborate four-byte version-tail fields in 14.261; C++ long must not be expanded to eight bytes here. Missing stream pointers skip the primitive operation and return the persistence object.

`cosworks_persist_primitive_evidence.py` verifies the provider hash, export mapping and five exact function-span hashes, retaining static instructions and decompilation hashes in `cosworks-persist-primitives-native-evidence.json`. It also checks the 857 native spans. No IStream call, allocation routine or installed code executes. Field meanings, malformed/I/O-failure fixtures, other overloads and complete installed-format coverage remain unfinished; no global byte-coverage credit is assigned.

Evidence: C-638.

### 14.263 Bounded lossless Cosworks primitive and study-list decoder

*Status: Independent framing decoder validated on 597 retained native payloads; complete semantic decoding remains unfinished · Evidence: C-639*

**`solidworks/cosworkspersist.py`** provides a bounded, lossless inspector for the primitive framing in 14.262 and observed study-list shape in 14.259–14.260. **read_text** returns an immutable TextRecord containing original offset/end, declared byte count, four-byte header and complete payload including its terminal NUL. It distinguishes **null length 0** from **empty length 2**. Its default length limit is **0x100000**, configurable downward or upward by the caller; missing headers, truncated payloads, length 1 and nonzero payloads without the final two zero bytes are rejected.

The optional text view removes exactly the final terminator for display, preserving earlier embedded NUL and isolated surrogate code units. Odd-length payloads remain available losslessly as raw bytes but have no UTF-16 text view, matching the recovered absence of a parity check without guessing an encoding. A null record likewise has no text view, while an empty record has the empty-string view. The inspector does not emulate the native CString assignment's possible NUL truncation or ignored HRESULT behavior.

**read_study_order** reads a complete leading record, raw four-byte count and exactly that many records. It preserves duplicate items, null items and order, imposes a default **10,000-item** resource limit, checks that sufficient minimum headers remain before iteration and rejects any trailing bytes. It assigns no active-study meaning to the leading record and makes no root-versus-nested precedence choice. It is not registered as complete Simulation semantic coverage or used to deserialize native application objects.

**Seven focused tests** in `solidworks/test_cosworkspersist.py` pass: null/empty distinction; nonzero offsets and embedded NUL/surrogate retention; odd-byte retention; truncation at every byte boundary; invalid terminators/length limits; duplicate/null item preservation; and impossible counts/trailing-suffix rejection. These are independent framing and malformed-input checks, not target execution or a native writer round-trip.

`cosworks_persist_decoder_validation.py` verifies the retained raw payload SHA-256 values and decodes **262 version streams plus 335 study-order streams**, covering **1,031 text spans**. All **25,988 logical payload bytes** reconstruct exactly from preserved framing and payload bytes. For version streams, undecoded tail bytes are explicitly retained as the suffix rather than interpreted by this decoder; for study streams, every count/item boundary and final endpoint is checked. `cosworks-persist-decoder-validation.json` records parent evidence and implementation/test hashes. No global byte-coverage credit is assigned while field meanings, producer/consumer behavior and complete installed-format coverage remain unfinished.

Evidence: C-639.

### 14.329 Primary version decoder and conditional owner updates

*Status: bounded decoder with retained payload replay in both reader modes; numeric text conversion and caller-mode selection remain open · Evidence: C-706*

`solidworks/cosworkspersist.py::read_primary_version(data, numeric_version, full_read=True)` decodes the counted text and conditional four-byte tail of the exact primary `CosworksVersoin` carrier from 14.261–14.262. It requires an explicit nonnegative signed-i32 numeric version. It does not emulate `wcstod`, infer numeric conversion from arbitrary text, choose the caller's mode, open storage or select the legacy raw carrier. Boolean values are rejected as numeric-version arguments; the mode itself must be a boolean.

The result retains the original text header/payload, ordered signed word values and their raw bytes, consumed end offset, unconsumed remainder and a separate list of modeled owner updates. Owner offsets are runtime destinations, not offsets into the stored payload. The retained reader's distinction between consuming a word and assigning it matters:

| Version range | Always consumed after text | Additional full-read words | Modeled assignments in read order |
|---|---|---|---|
| At most **800018** | None | None | None |
| **800019–800076** | First word | None | First word to **+0x1C8** |
| **800077–830031** | First word | Second word | First word to **+0x1C8**; second to **+0x1A8** only in full mode |
| At least **830032** | First word | Second and third words | Second to **+0x1A8**, third to **+0x1C8**, both only in full mode; first is consumed without an owner assignment here |

In particular, a partial read of a modern three-word tail consumes the first word but leaves the second and third untouched, and makes neither owner assignment. The returned update list models only writes performed by this helper; absent updates do not mean the existing runtime owner fields are zeroed. The first word remains retained even when no owner assignment uses it. All words use four bytes, including the overload named `long`.

Full mode requires exact EOF after the version-selected fields. Partial mode bounds every consumed field but returns the remaining bytes explicitly, including any unread tail words. This stricter decoder rejects truncated input rather than emulating the native primitive's unchecked I/O status. Neither mode silently discards a suffix or treats it as padding. The API does not require text to equal the supplied numeric argument; their association remains the caller's responsibility.

**Retained validation.** C-706 verifies the C-635 payload source hash against C-637 and matches the retained reader decompilation to that checkpoint. It replays all **262** retained primary payloads, rechecks each SHA-256, compares all decoded word bits with the retained unsigned views, and reconstructs each complete input byte-for-byte under both modes. Partial reconstruction includes the explicit remainder; it does not claim that the target consumed those remaining bytes. Numeric arguments come from C-637's established digit-only fixture checks, without generalizing conversion to other strings.

Three new focused tests check the exact **800018/800019**, **800076/800077** and **830031/830032** boundaries, ordered owner updates, signed values, partial-mode suffix preservation, every truncation of a representative modern payload, trailing-byte rejection and argument bounds. Together with the existing primitive decoder tests, **15 tests pass**. No installed module, fresh native extraction, target reader or writer is executed. This adds a usable decoder for previously documented bytes; global native/semantic coverage credit remains zero. Numeric conversion edge cases, field business meanings, caller-mode identity, downstream virtual binding and full installed-format coverage remain incomplete.

Evidence: C-706.

### 14.332 Legacy version decoder and explicit carrier precedence

*Status: bounded observed-profile decoder and retained carrier selection checks; native open results, numeric conversion and legacy suffix meanings remain open · Evidence: C-709*

`solidworks/cosworkspersist.py::read_legacy_version(data)` decodes the observed `CosworksVersoinNumber` profile from 14.266: exactly **128 bytes**, six ASCII decimal digits represented as UTF-16LE units at bytes **0–11**, a zero UTF-16 unit at bytes **12–13**, and an uninterpreted **114-byte suffix**. It returns the six-character text, its decimal integer value, the exact fourteen-byte prefix and the complete opaque suffix. The suffix is neither padding nor discarded data. No byte pattern within it is assigned pointer, timestamp or numeric semantics.

This is deliberately an observed-profile decoder, not an emulator of the native `wcstod` call. Other digit counts, non-ASCII numerals, missing/repositioned terminators, different stream sizes and broader numeric syntax fail explicitly as unsupported. Native acceptance of other forms is not disproved by those decoder errors. In particular, the implementation does not execute a runtime numeric parser or inspect bytes beyond a caller-provided buffer.

**Selection API.** `select_version_stream(primary, legacy, primary_numeric_version=..., full_read=True)` accepts the two complete payloads or **None** for an unavailable carrier. It applies the primary-open precedence established in 14.326:

| Supplied availability | Result |
|---|---|
| Primary is present | Decode through 14.329 using the required explicit numeric argument; select primary. Legacy is not consulted. |
| Primary is None and legacy is present | Decode the observed legacy profile and select its six-digit value. |
| Both are None | Return source `compatibility-default`, numeric version **500070**, and no decoded record. |

An empty byte string is a present malformed carrier, not unavailability. Framing or argument errors propagate to the caller; a malformed primary does not trigger legacy fallback. This is a strict independent decoder policy consistent with primary-open precedence, not a claim that the native persistence wrapper propagates every I/O error. The API does not perform storage opens or map arbitrary HRESULTs to absence. It returns a source label alongside the selected numeric value and lossless record so callers can retain provenance rather than choosing the largest available number.

For primary payloads the numeric argument remains explicit: the API does not expand 14.329 into an unverified `wcstod` implementation. The **500070** fallback is a compatibility constant, not the current release version. Carrier selection alone does not establish the unresolved downstream study virtual binding, per-study index identity or the version used for every LBC payload.

**Retained verification.** C-709 rechecks the exact SHA-256 of all **262 primary** and **25 legacy** payloads and reconstructs all **287** byte-for-byte through the selected record fields. Every legacy suffix equals its retained C-642 bytes. The primary-source evidence hash is checked against C-637. All **21** coexisting primary/legacy pairs have differing numbers and select the primary under supplied availability; this models precedence and does not prove a target open occurred for any pair.

Four new focused tests cover opaque-suffix reconstruction, unsupported legacy profiles, primary precedence over a numerically larger legacy record, malformed-primary error propagation, absent-versus-empty input and the compatibility default. Together with the primary/primitive decoder tests, **19 tests pass**. The retained check accesses no installed module or fresh native document and executes no target code. Legacy suffix semantics and full installed-format coverage remain incomplete; no native or semantic byte-coverage credit is added. Native subtype-12 replay remains pending resolution of the approval-review usage-limit failure.

Evidence: C-709.
