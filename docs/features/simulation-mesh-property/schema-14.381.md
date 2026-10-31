### 14.381 Newest Mesh integer-set suffix

*Status: newest populated suffix implemented from native loop; native populated keys remain absent · Evidence: C-761*

The >=**891103** suffix of Mesh reader **181109480** is an integer **set**, not a key/value map. It clears the destination tree at Mesh +320, reads a signed four-byte count, then reads exactly one signed four-byte key per iteration. Native tree insertion may merge equal keys; it consumes no separately serialized value. The stream closes immediately after this loop; subsequent mesh-state normalization is in-memory only.

This corrects the tentative “newest map” characterization in 14.378–14.380. The unrelated >=890703 keyed parent/child region remains distinct and keeps its existing wider records. C-761 freshly verifies the full reader function/gate bytes through the C-758 binding, so the set width is derived from its native read loop rather than guessed from the empty retained count.

`cosworks_mesh_layout.py` now reads bounded populated set keys and returns **newest_keys** in original wire order, preserving negative values and duplicates before native normalization. An independent lossless decoder must not deduplicate the wire sequence merely because the native in-memory container is a set. Negative/over-limit counts, truncated keys and remainder remain rejected. Older versions return no set field and consume no count.

Synthetic tests cover the **891102/891103** boundary, empty/one-key/multiple-key sets, negative keys and duplicates, truncation/remainder and invalid counts. C-761 freshly replays **all 357 retained Mesh streams / 94876 bytes**, verifies exact EOF/reconstruction and preserves every C-760 successful candidate's identity/version/endpoint/hash. **Eight** complete retained occurrences have the newest set field, all with **zero keys**. Twenty tests pass; populated key validation remains static/synthetic rather than native-fixture evidence.

The newest populated-set parser stop is therefore resolved for this recovered grammar. Full Mesh semantics, historical writer equivalence, native populated nested/set fixtures, outstanding other-carrier mismatches and all-installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-761.
