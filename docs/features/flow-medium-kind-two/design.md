# Kind-two implementation constraints

The independent FMO2 baseline covers all six documented model layouts. The
production shared prefix reader now handles the common name, seven curves,
shared arrays and bounded wide-pattern spans without changing kind-one behavior.
The typed kind-two body and mixed-kind profile APIs remain unfinished.

The backend-prefix reader in `flowmediumbackend.py` consumes the first-table
kind-one key, but stops immediately after a kind-two tag. The complete grammar
in `flowmediumliquids.py` reads a sixteen-byte table key for both kinds before
calling the body reader. A typed first-kind-two entry point must therefore check
and skip that table key before decoding the version-zero body. It must retain
the distinct body base key without assuming equality between the two keys.

Model tags are signed values 0–5 and should be parsed once into a typed selector.
Models zero, one and two contain two or three curves. Model four interleaves a
state byte between six initial scalars and the last scalar. Model five stores
both first and second counts before either array group; its third count follows
the second arrays. All three counts are independent. Its raw 32-bit words require
a separate resource bound and must not be assigned a floating-point meaning.

Keep existing public option literals compatible. Kind-two-specific bounds can
contain the current body limits alongside the additional 32-bit-word bound.
Existing-field edits must preserve every nested curve/model boundary, array
cardinality, state-field presence and the following opaque bytes, then reopen
the full carrier. Model tags and count/version framing remain structural.

The existing public curve view currently borrows a kind-one owner. Reusing it
for common and model curves needs a measured internal adapter or another shared
view implementation; it must retain immutable ownership, existing semantics and
Send/Sync guarantees. No new public view stub is shipped ahead of its caller.

Private body parsing for later mixed-kind profiles must validate the selected
entry's kind rather than assuming every entry has the backend's first kind.
The public first-body entry points still need their explicit first-kind gates.
Complete mixed-kind references must resolve against separate first-table key
maps for each kind, while preserving duplicate physical entries and ordered
references. The FMR1 same-wire cases prevent kind inference from body bytes.

Generated fixtures do not establish installed kind-two acceptance. The typed
result must keep that exclusion alongside engineering/model evaluation, native
runtime state and application acceptance until separate evidence is available.
