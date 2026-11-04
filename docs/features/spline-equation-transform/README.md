# Spline equation transforms

Bug #95; behavior version 2. Section 11.360 / C-402 proves that the equation
branch selects the fixed mgXform archive operator: thirty u32 words, or 120
bytes. The former compact restore call and its synthetic fixture were wrong.
The reader now uses the existing fixed-transform codec, preserving every word
and rejecting unsupported perspective bits in word 28.

The recorded migration retains the old fixture hashes and a rejected compact
sample. Of 934 cases, 448 without equation transforms remain byte-for-byte
unchanged. The 486 equation cases use independently assembled fixed words and
the actual reference reader. Their scalar oracle now contains 86,266 values.
The original final-word edit remains covered, and an additional independent
transform-word edit checks reconstruction, reopening and source isolation.
No production fallback accepts the erroneous compact grammar.

This defect is in the unreleased candidate feature: neither affected production
module exists on main. It stays with that feature until integration. No crate
version or released changelog section changes. All 16 focused spline tests and independent immutable fixture regeneration passed.
Two native processes matched all 906 statuses and causes, preserving 888
accepted inputs and 18 explicit rejections. The aggregate corpus still fails.
The full local gate passed 1,102 library tests and 277 doctests,
with 2,607 passing tests across 189 executables and 3 intentional ignores. Raw geometry meanings and native
application acceptance remain outside this framing correction. The humanizer
skill was unavailable; prose review is left to the maintainer.

The [latest equation wire audit](latest-schema-audit.json) checked clauses
11.1699 and 11.1702–11.1704 against the existing codec. Six CString fields
and the signed compatibility word remain raw, and the fixed transform keeps
all thirty words. The new selected native consumer bindings do not introduce
load-time expression evaluation, declared-character filtering, compatibility
word replacement or matrix application. Frames, expression grammar, geometry
and native workflow equivalence remain separate. No behavior version or
expected fixture was changed.

The [qualified runtime audit](qualified-runtime-schema-audit.json) records eight
selected clauses through edition 0.3011. They refine parser output quantities,
candidate construction, integer wrapping, matrix composition and mutable runtime
budgets without adding stored fields. Those runtime operations remain
unimplemented. The 100-sample check is not exact geometry validation, and the
coefficient-basis R*T composition does not define how to apply the archived
transform. The existing wire decoder is unchanged; no native execution,
behavior-version change or coverage gain is claimed.
