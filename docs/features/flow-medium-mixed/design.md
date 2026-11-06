# Mixed-profile implementation constraints

The existing complete kind-one profile API must retain its public option
literals, exact check mode and accepted corpus. The mixed-record entry point
needs a typed kind selector once per entry and separate first-table maps for
kinds one and two. Physical entries remain ordered even when keys repeat.
Second-table entries share an outer key domain but retain their declared kind
and conditional state-byte presence.

Reuse the existing metadata reader, both complete body readers, GUID parser,
carrier editing and physical history paths. Avoid a second copy of the profile
scanner: isolate shared framing and reference-budget logic, preserve the
kind-one adapter, then introduce the mixed adapter against FML1. Structural
changes require the owned earlier packets to pass unchanged before mixed-kind
behavior is added.

The private kind-one body reader currently checks the backend's first kind,
which would reject a later kind-one entry when kind two appears first. Move
that gate to the public first-body entry point before the mixed scanner calls
it. A per-entry scanner must select and validate its own kind. Both body readers
must retain explicit body offsets through edits and full-carrier reopening.

Carry independent per-body limits alongside total wide-word, raw 32-bit-word,
name-unit, object and reference limits. Empty collections spend zero budget;
all user-configured independent limits remain positive. Coverage must distinguish
complete supported record framing from engineering and installed kind-two
acceptance. Unknown tags, unsupported versions, unresolved typed references,
populated final collections and trailing bytes fail the declared exact profile.

Existing-field edits must revalidate the whole mixed record, preserve all
entry boundaries and reject key/reference changes that leave a dangling typed
reference. Body and metadata composition must check current checkpoint identity
and the selected body offset. Tests must include no-ops, independently expected
patches, history, fresh reversals, source isolation and concurrent deterministic
results across the complete packet before publishing the new API.
