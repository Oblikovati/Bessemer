# Saved external unit components

Issue #54; behavior version 1 for saved external references. Section 11.360 /
C-402 proves that the component owner may reference the selected internal part
or a complete external-file object. A nonzero u8 presence then introduces a
tagged, nonnull external-file reference. It does not introduce an inline model
body. The owner and model may reference the same mapped object.

External owners or model presence require the typed proof produced by a
completed selected part archive. Standalone external unit records fail even
when their field bytes are otherwise complete. Ordinary internal/null units
retain their earlier scope. Component-detail provenance still requires an
internal unit with the exact selected owner and no external model; this change
does not weaken that separate contract.

Seven independently assembled owned streams cover zero and nonzero presence,
fresh and cached external records, shared owners, distinct model references,
and an independent configuration-ID edit golden. The complete external-file
body and its version rules reuse the established reader. Six focused tests and
immutable reference regeneration passed, including every prefix, appended
bytes, missing context, null or wrong-typed pointers and internal provenance.
Three public document tests and the actual two-source native reference
regression passed. Two fresh Bessemer processes matched all 906 statuses and
causes. Complete typed reconstruction increased from 889 to 890 inputs, adding
case 567 without regressions. The aggregate corpus still fails because 16
inputs are incomplete. The independent reader consumed all 78,584,936 resolved
bytes of the 890 accepted inputs through EOF, with all four input-stream hashes
verified against the pinned manifest. The full local gate passed 1,118 library tests and 277 doctests,
with 2,632 passing tests across 192 executables and 3 intentional ignores. Native application acceptance and
external-file loading behavior remain separate work.

The humanizer skill was unavailable; prose review is left to the maintainer.
