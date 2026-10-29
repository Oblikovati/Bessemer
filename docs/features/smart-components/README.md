# Smart components

Issue #54; behavior version 1. Section 11.359 / C-401 binds ten complete
classes in ordinary modern part archives. The top feature retains its full
model-feature base, required nonnull data object and nullable saved external
object. The ordinary external-XML context excludes the condition-0x80 word
and string branches; no constructor default is inserted into the wire body.

The data object stores direct arrays, a direct list and a direct sensor map,
plus typed context and folder pointers. Each folder retains its complete
feature/folder base and a direct array. Items retain the complete node body
with its byte-free resolved hook, followed by their distinct suffixes. Owner
pointers retain their original identities, including references back to a
containing feature. Transform wrappers and context-chain components use
complete existing readers; external objects do not trigger file loading.

Sensor maps use ordinary ReadCount and ordered CString/object pairs. Duplicate
keys, null values and shared identities remain in wire order. They are not
collapsed using the runtime map's overwrite behavior. Positive counts are
bounded by minimum remaining widths before iteration. Direct members add no
pointer tags, presence words or identity-map slots.

Independent fixtures contain 540 complete graphs across nine versions and six
styles, with 155,636 observed primitive reads and 70 complete owned streams,
including ten edit goldens. They cover complete inherited concept bodies,
owner cycles, real component and transform targets, context/external objects,
direct and extended empty/populated counts, duplicate keys, nulls and aliases.
Three public document tests passed, including exact reconstruction, reopen,
input isolation, rejected edits and all ten independent owned edit goldens.

All seven focused tests passed, including every truncated prefix, appended
bytes, unselected contexts, required data, typed pointer rejection, duplicate
keys and cached owner identity. Immutable regeneration reproduced every owned
byte. The actual Source native regression passed the populated smart-component
example, disabled-scope zero credit and appended-byte rejection.

Two native corpus processes returned identical statuses and diagnostics:
898 of 906 accepted, up from 897, with new case 546 and no regressions. Eight
explicit stops remain incomplete. Source verified exact EOF for all 898
accepted streams, totaling 80,350,527 resolved bytes, after checking four
input hashes per configuration.

The Windows Rust 1.99.0 `cargo xtask pr` gate passed: 1,146 library
tests, 277 doctests and 2,673 total passing tests across 197 test
executables, with three intentionally ignored.

This evidence does not establish whole-document, assembly, drawing or
installed-project completion. The maintainer humanizer skill was unavailable;
prose review remains with review.
