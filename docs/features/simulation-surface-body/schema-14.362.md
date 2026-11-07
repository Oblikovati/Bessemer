### 14.362 Exact evidence for the six historical SurfaceBody tail mismatches

*Status: mismatch identities and saved metadata localized; historical grammar remains unresolved · Evidence: C-740*

C-740 freshly reopens and hash-checks the six C-733 incomplete SurfaceBody payloads rather than changing their caller versions. All occur in installed sample `samples/tutorial/api/tjoint.sldprt`, under StudyNumber-0, -1, and -2, with two sheet-body streams per study. There are **four distinct complete payload byte sequences**: corresponding StudyNumber-1 and -2 bodies are byte-identical, while StudyNumber-0 differs. Six occurrences remain six inventory identities; deduplication is not a coverage increase.

The retained nested metadata contains only counted-text **CosworksVersoin = 860006** among the checked root primary/legacy and study-version-array streams. Neither CosworksVersoinNumber nor StudyVersionNumbers is present in this envelope. Therefore the current candidate set is not a conflict between two saved metadata values. This does not prove that every historical object was written with the root version, or that a study suffix determines a different version.

At supplied version 860006, all six reach the two-word +100/+94 boundary after consuming the complete preceding grammar. For each 146-byte payload, the consumed prefix is 142 bytes; for each 152-byte payload it is 148 bytes. Exactly four bytes remain in every case. Their raw signed-word interpretations are **1, 1** for the two StudyNumber-0 bodies and **0, 0, 0, 0** for the other four. C-740 preserves every whole payload as raw hex, every consumed prefix span, the unclassified final four bytes, source hashes and metadata bytes. Those terminal values are observations, not assigned enum meanings or native member destinations.

No complete C-733 SurfaceBody occurrence has a successful **860006** candidate, so the retained corpus supplies no same-version successful control for deciding which of the two expected destination words is absent or differently serialized. The discrepancy cannot be resolved by assigning the last word to +100 or +94 solely because EOF would then match. A historical serializer, independently verified writer profile, or stronger native version-selection evidence is still needed. A guessed 850012 profile, synthetic padding, or silently omitted +94 field is not enabled.

C-740 does not increase the complete count in C-739: **six occurrences / 894 bytes** remain incomplete. The evidence narrows the next investigation to a shared historical tail/version discrepancy in one sample rather than six unrelated extraction failures. No target executes and no global semantic coverage credit is granted.

Evidence: C-740.
