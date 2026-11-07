### 14.358 Inherited sheet-body Property replay and constant-surface wrapper boundary

*Status: 114 of 238 retained sheet Property streams complete; explicit archive/text failures retained · Evidence: C-735*

The verified sheet storage method `1813314F0` calls base serializer `180A32600` at instruction `181331517`, before opening the separate SurfaceBody stream. C-735 checks this direct call instruction and the sheet storage vtable pointer against installed `cosworks.dll`. It implements the smaller base Property layout from 14.352 for `SolidManager/Solid Components/<component>/Sheet Bodies/<body>/Property`; SurfaceBody data is not an inline continuation.

The inherited Property begins with the 14.282 persist-ID prefix: two words, signed blob count, raw blob, and two independent native object archives from version 840002. Then read two counted UTF-16 texts (destinations +38/+40), three four-byte words (+48/+4C/+50), and a fourth word (+54) from **830004**. The current writer always emits the fourth word and both archives; historical reader gates remain separate. The decoder uses explicit caller version candidates and the bounded local embedded ID registry from 14.357, retaining archive boundaries and rejecting unknown classes rather than skipping them.

`cosworks_sheet_property_layout.py` and `cosworks_sheet_property_audit.py` freshly verify source envelope/payload hashes for **238 occurrences / 186372 bytes**. **114 / 16916 bytes** reach exact EOF and reconstruct losslessly. The remaining **124 / 169456 bytes** remain incomplete: 121 occurrences first stop at unsupported embedded class **moConstSurfRef_w**, and three fail while reading a counted text. Failure offsets and candidate alternatives remain in the evidence. Across 243 candidate attempts, 114 complete. This is a partial framing result; byte counts for incomplete occurrences include their whole payloads, not just unread suffixes.

Five tests pass: three inherited-Property tests for the 830004 gate, blob/archive boundaries, truncation/remainder, negative counts, and explicit version contracts, plus the two local embedded-ID guard/order tests. Synthetic null archives are test fixtures only. Native payloads still use the existing strict embedded archive reader; no part-model context is fabricated.

`moConstSurfRef_w` is a distinct runtime class and is not treated as an alias merely because other reference wrappers have known prefixes. Its native primary serializer and inherited suffix must be checked before expanding this registry. The three text failures require separate payload/version analysis and are not attributed to that class without evidence. The six SurfaceBody tail mismatches remain separate. No target executes; global semantic coverage credit remains zero and full installed-format coverage remains incomplete.

Evidence: C-735.
