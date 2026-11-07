# Ordinary display dimensions

Feature #44 adds the complete documented ordinary modern base display dimension
and distance, radial, angular and revolved variants. The reader composes the
display-object/item bases, embedded dimension text, placement, references, units,
favorites, attachments, jogs, late fields and the selected suffix. It reuses the
existing text, units, attachment, dimension-handle and compact-transform codecs.
The runtime class is parsed once into a finite dimension kind.

The embedded transform at member 0x5C0 stores thirty u32 words. It is retained
in that form instead of being passed to the compact-transform decoder. Its
documented perspective stop is explicit. Non-finite values, raw flag and presence
values, signed indices, duplicate arrow keys and reference identities survive
reconstruction. Native load-time normalization is not applied to stored fields.

Typed parameter arrays accept width zero or eight-byte doubles. Counts are
checked before iteration, including the documented point/vector/callout bounds.
An independently assembled three-value parameter array passes a three-entry
budget and fails at two. Optional link data, item object 0x288, watermarks,
unimplemented new nested objects and serialized-transform perspective remain
explicit unsupported forms. Their prefixes cannot become successful objects.

The independent packet contains 336 cases and 54,251 scalar expectations across
fourteen versions. It covers all five complete dimension classes, populated and
absent optional branches, standalone reference/arrow/favorite/callout records,
direct item bodies and parameter arrays. Actual tagged dependencies replace
loaded placeholders: handles own their dimensions, units retain self references,
and the display owner/related dimension can reference the containing object.
The embedded text has no outer tag or map slot. New dimension handles are checked
against the finite documented family, with null and unrelated objects rejected.

Every fixture reconstructs exactly. All truncated prefixes and trailing bytes
fail. An edit matches separately assembled reference-validated bytes, reparses
with the expected fields and preserves the original. Source-overwrite checks
cover every case; four parallel writers reproduce the same owned result.
Malformed dynamic counts, bounded counts, unknown widths and unsupported
presence/perspective forms have focused checks. Stored non-finite bits and all
thirty serialized-transform words are checked directly. The scalar oracle drops
no fields. Three wire fixtures reproduce in two fresh reference processes.

All twelve display-dimension tests and all 247 model tests passed on the final
source, along with all-target Clippy and all 51 Python tests.

Native exact reconstruction increases from 866 to 886 of 906 configurations.
Two final processes produce the same outcomes. The twenty remaining first stops
are seven extrusions, five bosses, one revolution and seven role/class mismatches.
Four newly reached sketch user objects are extension-line indicators; one logical
help array uses `suObArray` rather than `su_CObArray`. Bug #46 records these five
alongside its older dimension-handle and component-reference mismatches. They
remain rejected pending finite class/alias regressions; the corpus gate still
fails. Later failures may appear once these dependencies are implemented.

The verification manifest binds tests, source, fixtures and nineteen schema
clauses from edition 0.356. The full PR gate remains the earlier geometric/kernel
checkpoint and must rerun before push. Public operation results/history,
selected-configuration diagnostics, native application/geometry acceptance and
the platform matrix remain open. Humanizer was unavailable; prose remains for
maintainer review.
