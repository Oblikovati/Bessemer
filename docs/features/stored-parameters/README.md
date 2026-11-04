# Stored parameters (#202)

This work adds typed stored parameter snapshots and exact value edits over the
admitted complete part archives. Corliss's model parameter views are the design
reference. The existing Bessemer codecs already carry the parameter bodies;
the new operation will reuse their validation, snapshots, handles and history.

The first baseline covers density, length, angle, integer, attribute scalar and
attribute string parameters. The inherited node name and second parameter-name
pointer stay distinct, including nulls and aliases. Floating-point payloads,
signed integers, CString framing and alternate/override presence bytes retain
their complete stored domains. A stored alternate is not an evaluated effective
value. Units, equations, native setters and geometry are not inferred.

## First complete operation and acceptance

Build complete owned graphs with the existing independent object-map assembler.
Observe both name identities and literal primary/alternate/string values through
an unchanged frozen Source reader, with exact EOF and primitive coverage. Pin
an independent value edit before adding public projection or edit methods.

Then prove read, selected replacement, complete document reopen and byte-exact
reversal. Expand to both configuration and resolved-feature contexts, correct
physical class ownership, local/inherited aliases, all scalar/text/presence
boundaries, wrong/foreign/stale selection, history, coverage, diagnostics and
input isolation. Concurrency, fresh actual Source exports, native outcomes and
the full local/remote gates are required before publication. This baseline does
not supply a public stub or close the broader model/geometry requirements in #2.

## Owned reference baseline

The original Source reader accepted 30 complete resolved-feature graphs:
24 combinations of six parameter classes and four pointer/value styles, plus
six independent primary-value edits. Each graph reached exact EOF with complete
primitive byte coverage and the expected inherited and final object-map sizes.
The checks compare both name pointers, including shared identity, nulls and
distinct names. Floating-point comparisons use their raw bytes, including NaN
payloads, signed zero, infinities and a subnormal value. Integer endpoints,
noncanonical presence bytes, embedded NULs, a surrogate pair and nonminimal
CString framing are retained in the owned wire fixtures.

Run `python -B -X utf8 docs/features/stored-parameters/freeze_owned_reference.py`
from the repository root to reproduce them. The script freezes every reference
Python file before importing readers, checks import origins and source hashes,
and compares generated fixtures byte for byte with the reviewed baseline.
The complete snapshot is retained as lossless gzip with a small JSON index.
Configuration-context operations, public projection/edit methods and native
parameter acceptance remain open.
