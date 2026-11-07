# Direct header implementation decisions

Use the existing complete scalar/text body infrastructure and operation-local
builders. This grammar has no embedded archives. Reuse outer byte/field/entry/
text limits without exposing unused embedded-archive budgets. A private shared
limit translation is justified by this additional caller, but preserve existing
Material behavior in a separate extraction with complete before/after proofs.

Read physical words in order: member48, gated member4c, gated member70, then
member180. The member70 threshold is numerically later than member180 but occurs
before it on the wire. Text lists follow their own 870004/870008 gates. Every
count has an independent hard cap of 100000, distinct from the 10000 common-load
caps. Aggregate caller entry budgets charge both list counts. Empty lists are
complete bodies; do not infer child component counts from these words/lists.

Preserve each raw counted text frame, duplicates and order. Null, allocated
empty, unpaired-surrogate and odd frames have independent fixtures. A decoded
Source view may be None for an odd nonnull frame; that does not make it a null
wire value. Native retained lists are empty, so populated-list coverage must
remain synthetic. Add exact hard-cap and one-above cases with sufficient payload,
not only counts that also fail truncation.

Standalone reads require exact EOF. Complete construction must reparse the full
selected historical shape. Isolated edits retain source/replacement/result,
revision-bound identities, complete ordered history and exact surrounding bytes.
Coverage states exact stored-profile checks and native/runtime/physical/geometry
exclusions; it does not claim native application acceptance or current-writer
historical equivalence.

Keep all 395 original candidate outcomes. A retained success does not determine
native study-index dispatch. Component Property, folders, standalone persist IDs,
solid/sheet bodies and SurfaceBody are separate streams. The six documented
SurfaceBody tail mismatches remain explicit unresolved cases; no padding,
version substitution or omitted destination word is permitted.
