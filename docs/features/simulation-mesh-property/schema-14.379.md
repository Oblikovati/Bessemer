### 14.379 Complete retained Mesh Property replay and resolved inline settings dispatch

*Status: all 357 retained Mesh Property streams complete; populated nested extensions remain open · Evidence: C-759*

Mesh constructor **1810FE4E0** allocates object member +8A8 through **1810FEE90**. That constructor installs primary table **182032C28**; its +80 slot (**182032CA8**) points to thunk **18170C95C**, bound by the PE import table to **CosUtilities.dll!CSWPersistReferenceContainer::Serialization**. C-759 checks the native slot, thunk bytes, exact import symbol/IAT and constructor/function hashes. Thus the inline settings dependency in 14.378 is the established reference-container grammar, not a guessed fixed-size structure: signed reference count, versioned references, then its eight-byte container tail.

The second inline collection delegates direct records to **181119560**, reading directly from IStream. A record has five four-byte words, two additional words from **870018**, then four words. From **850002**, mode word +2C follows; mode 1 introduces a signed count of records serialized by **1811197D0**. From **850030**, mode word +38 and a second analogous count/collection follow. These are nested extensions, not sibling streams. The bounded Mesh decoder preserves both modes, accepts zero nested counts, and explicitly stops at populated nested bodies until 1811197D0 is decoded.

`cosworks_mesh_layout.py` follows the initial/non-monotonic gates in 14.378, then bounded text lists, the imported reference container, direct records, duration words and later array/map suffixes. The >=850001 object arrays preserve five words and two counts followed by their two word arrays. The >=890703 keyed region preserves each 44-byte parent header, signed child count and 40-byte child records. The >=890907 region is a signed count followed by raw doubles; >=890909 adds word +73C. The >=891103 newest map count is preserved, with an explicit stop for populated records pending their complete grammar. Count limits and available bytes are checked before each array consumption.

Fresh source-envelope/nested-payload hash verification and replay complete **all 357 retained Mesh Property streams / 94876 bytes** to exact EOF with full reconstruction. The retained data contains **353 direct inline records**. There are 391 candidate attempts, 357 complete; failed alternatives remain retained rather than silently changing versions. Native successful profiles do not establish populated 1811197D0 or newest-map extensions, and do not prove universal runtime study-index selection.

Fourteen tests pass across Mesh and embedded dependencies. Mesh-specific tests cover the **800078..809999 / 810000..810003 / >=810004** non-monotonic field window, newest empty collections, strict EOF/truncation, negative count rejection and explicit version contracts. The shared reference/archive readers retain their existing model-context and unknown-class guards.

C-759 resolves complete framing for C-758's retained Mesh payload identities and its settings-object dispatch. Historical writer equivalence, populated nested extensions, newest-map details, physical meanings and full installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-759.
