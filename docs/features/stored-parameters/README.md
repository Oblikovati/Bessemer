# Stored parameters (#202)

This work adds typed stored parameter snapshots and exact value edits over the
admitted complete part archives. Corliss's model parameter views are the design
reference. The existing Bessemer codecs already carry the parameter bodies;
the operation reuses their validation, snapshots, handles and history.

The first baseline covers density, length, angle, integer, attribute scalar and
attribute string parameters. The inherited node name and second parameter-name
pointer stay distinct, including nulls and aliases. Floating-point payloads,
signed integers, CString framing and alternate/override presence bytes retain
their complete stored domains. A stored alternate is not an evaluated effective
value. Units, equations, native setters and geometry are not inferred.

## Typed operation

Both archive types expose `stored_parameters()` in physical definition order
and `stored_parameter(&definition)` for a revision-bound object-tag selector.
Alias-only and null occurrences do not create duplicate parameter definitions.
Each immutable result owns its complete source and exposes the physical class,
both name pointers and their exact text, primary and alternate values, the raw
presence byte, copied field history, coverage, completion and diagnostics.
Name dependencies can belong to the inherited configuration; resolve their
handles through `state.source().resolve_field(...)`.

`edit_stored_parameter(&state, request)` replaces one primary value, an existing
alternate value or a raw presence byte through the complete archive editor.
Replacement values must retain their stored type. Presence edits must keep
the existing field shape: changing a nonzero byte to another nonzero byte is
supported; adding or removing a dependent alternate field is rejected.

```rust,ignore
use bessemer::model_archives::{ModelParameterStoredEdit, ModelParameterValue};
let state = archive.stored_parameter(&definition)?;
let changed = archive.edit_stored_parameter(
    &state,
    ModelParameterStoredEdit::Primary(ModelParameterValue::Integer(12345)),
)?;
let complete_bytes = changed.archive().to_bytes()?;
```

The owned tests cover exact edits, complete document reopen, byte-exact reversal,
wrong/foreign/stale selections, history, input isolation and concurrent edits.
Broader configuration-context fixtures for all six classes, fresh actual Source
exports, native outcomes and the full local/remote gates remain required before
publication. Higher-level model and geometry parity remain open in #2.

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
The configuration operation currently has a complete blank-part density test;
the six-class independent wire baseline above is in resolved-feature context.
That distinction remains part of the acceptance scope.
