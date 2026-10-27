# Reference-plane point roles

Bug #46 includes two narrow `point_reference_88` roles in face-point and line-point
plane data. The perpendicular line-point reader shares the latter field reader.
They accepted only `moPointRef_w`, rejecting complete endpoint and loft-sync
references after reaching their tags. Eleven native configurations reproduced the
endpoint rejection at revision `030d35dbb01588628a0aec55aac7c30d9532e00b`.

Specification 11.164 / C-192 establishes that the endpoint serializer calls the
complete point-reference helper. Section 11.204 / C-232 establishes loft-sync
ancestry through the endpoint and its inherited serializer call. These three
already implemented classes form a finite point-reference family. The fix reuses
that family at both plane fields and the unchanged reference-point projection
role that already admitted the same three names. Nullability, curve requirements,
pointer bytes, map insertion and every class body retain their existing behavior.

Independent readers consumed 144 complete new/cached fixture graphs to exact EOF,
making 16,836 primitive observations. Three plane-data classes and six versions
cover the two inherited pointer-gate boundaries. There are 108 valid point-family
cases and 36 unrelated-wrapper cases. The byte parser proves complete bodies and
alias identity; the native ancestry evidence supplies expected role membership.
Unrelated `moRefWrapper_c` objects remain rejected for fresh and cached tags.

Before the fix, base-point and unrelated-target tests passed while fresh and
cached derived-point tests failed at the narrow role: three tests passed and two
failed. After the fix all five pass. They compare every positive field/byte/slot,
retain typed role errors for unrelated classes and reject every truncated prefix,
extra byte and older context. No class-prefix or generic subtype fallback was
added, and the shared display-role policy remains unchanged.

Eleven owned streams import the blank part's actual 97 slots. Ten are valid
complete planes, covering each plane kind and point subtype plus an independently
constructed endpoint-coordinate edit. The remaining stream deliberately supplies
an unrelated wrapper and must reject. Public tests reconstruct complete documents,
match the coordinate edit and preserve input streams after success and failure.
Three Python tests regenerate every fixture and check the explicit matrix.

`reference.json` binds stable before/after sibling reader hashes, generators,
reused builders, complete primitive oracles and map slots. The schema snapshot
retains the field and ancestry clauses. The bug remains part of the broader open
finite-role audit; no other pointer family is relaxed. The fix targets the interop
candidate because the affected runtime files do not exist on `main`, as recorded
in Bug #46. Native application acceptance, geometric validity, historical/special
archives and full schema parity remain unproven. No dependency or package version
changed. The humanizer skill is unavailable; its prose pass remains with review.

Two fresh native processes preserve all 675 prior complete configurations and
complete the eleven affected cases, reaching 686/906. Outcomes and causes agree
exactly; 220 remaining configurations still stop explicitly. An independent
reader audit reaches exact EOF for all 686 accepted streams, totaling 40,016,822
feature bytes, with all four original input hashes checked per case. The audit
used a verified immutable copy of 1,118 approved sibling Python files, checked
before and after decoding, then removed after its process exited.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 531 library
tests, four xtask tests, every integration suite and 265 doctests. Three external
corpus gates remain ignored by default; both native replays above ran explicitly.
