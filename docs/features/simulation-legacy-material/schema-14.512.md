### 14.512 Corpus-derived legacy MaterialList fixed-record candidate

*Status: exact corpus-wide candidate partition; historical reader/writer binding remains unresolved — Evidence: C-894*

The70 retained named-study **MaterialList** streams from14.458 remain incompatible with the current shared Material serializer at its text08 frontier. A separate **corpus-derived candidate**, not a replacement installed-reader grammar, partitions their remaining bytes without changing any retained version:

| Candidate region | Proposed framing | Observed total |
|---|---|---|
|Existing compatible header |14.458 prefix through text60 |6,902 bytes |
|Fixed records |Header word30 records, each16 raw identifier bytes followed by8 raw value bytes |196 records /4,704 bytes |
|Trailing texts |Exactly21 existing counted Cosworks text primitives per stream |1,470 texts /8,820 bytes |

This partition reconstructs all **20,426 bytes** of all70 streams and reaches exact candidate EOF. Header word30 is1 in2 streams,2 in22,3 in44 and9 in2. No stored version is forced lower or replaced to obtain these endpoints. The current parser remains unchanged and its70 failures remain retained.

The sixteen-byte candidate identifier fields contain consistent leading ASCII prefixes before a zero byte, but their suffix bytes vary and can contain unrelated-looking data. The candidate preserves **all16 bytes**; it does not treat the field as a clean padded string or zero the suffix. Supplementary prefix counts are EX:70,NUXY:66,DENS:45,GXY:3,SIGXT:2,SIGYLD:2,ALPX:4,KX:2 andC:2. Their spelling alone does not establish physical-property names, units or a universal identifier domain. The following eight bytes also remain raw; no finite-value, normalization or unit constraint is introduced.

Every observed trailing text has exact bytes **020000000000**, the existing six-byte empty counted-text encoding. The candidate requires21 independently parsed texts rather than consuming the remainder as an arbitrary opaque block. That repeated count remains a corpus observation pending a producer/consumer binding; it is not yet a verified historical format invariant or21 named fields.

**cosworks_legacy_material_candidate.py** exposes `candidate_success` and `candidate_eof`, while keeping **complete=false** even on an exact endpoint. It retains byte-level spans and hashes and rejects negative/over-limit counts, truncated records/texts and trailing bytes. Three new tests check raw identifier suffix and raw64 preservation, explicit noncompletion, count bounds, truncation and exact-EOF rejection. **cosworks_legacy_material_candidate_audit.py** first repeats the native C-838 extraction/hash audit and its five tests, then checks all70 candidate partitions and the three new tests.

This evidence identifies a concrete historical-layout lead for reader/writer investigation. It establishes **zero newly completed schema records**: historical dispatch, fixed-record field meaning, trailing-text production rules and native writer equivalence remain unbound. The13,524 bytes after the existing headers are now retained under an exact candidate partition, but are not credited as verified schema decoding. No target code executes and no global coverage credit is claimed.

Evidence: C-894.
