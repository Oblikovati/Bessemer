# Mixed-profile implementation constraints

The existing complete kind-one profile API must retain its public option
literals, exact check mode and accepted corpus. The mixed-record entry point
needs a typed kind selector once per entry and separate first-table maps for
kinds one and two. Physical entries remain ordered even when keys repeat.
Second-table entries share an outer key domain but retain their declared kind
and conditional state-byte presence.

Reuse the existing metadata reader, both complete body readers, GUID parser,
carrier editing and physical history paths. The private profile-wire module now shares table framing, GUID decoding,
reference membership and budget spending with the existing kind-one scanner.
Preserve that adapter and reuse these checks in the mixed
reader verified against FML1. Structural
changes require the owned earlier packets to pass unchanged before mixed-kind
behavior is added.

The kind-one first-body gate now remains at the public entry point. Its private
body reader accepts an explicit offset independently of the backend's first
kind, so a later kind-one body can follow kind two. Independent bounds still
apply, and edits retain that body offset through full-carrier reopening. The
mixed scanner must select and validate each entry's own kind before calling
either body reader.

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

The mixed reader and existing-field editor now implement this supported profile.
`FlowMediumKind` is normalized once at each entry boundary. Both typed key maps
are operation-local ordered sets; immutable entry slices retain every physical
duplicate and ordered reference. Existing kind-one public options and its exact
check mode remain unchanged. Complete engineering and installed kind-two
acceptance still require independent evidence.
