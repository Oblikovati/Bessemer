# Detail-folder interoperability

Bessemer reads and reconstructs complete ordinary-part `moDetailFolder_c`
objects, including their optional graphics buckets. Supported named fields can
be edited through the existing resolved-feature API. Unsupported dependencies
still return typed failures. This advances issue #54 and the broader #2 goal.

The folder body contains the common feature, 18 direct lists, a compact
transform, a typed BOM pointer, 14 more lists, a signed bucket-presence value,
the optional direct bucket body, two final lists and a signed suffix. It does
not contain the model-feature `+354` word. Object references use the inherited
model map; BOM references require `moDisplayBom_c`.

The implementation shares the existing display bucket envelope, polygon and
edge bodies, visibility arrays, line widths and display-item payload codecs.
Display adapters preserve their original field paths, version gates, collection
budgets and object-map ownership. The structural commits retained all 388
previously accepted corpus cases and their failure causes before enabling the
model adapter.

Archive wire version and runtime file version are separate. The approved
reference defaults to runtime 19000 for this ordinary model dialect; Bessemer
uses that declared context rather than deriving it from the wire marker.
Display contexts retain their explicit runtime-version input.

## Verification

Six owned full-object fixtures cover absent, 3D, 2D and populated buckets,
a named folder and an independently constructed expected name edit. A seventh
fixture covers all 32 current display-item payload variants. The populated
bucket exercises tagged thread keys, signed empty counts, compact transforms,
noncanonical presence values, raw floating-point bits and restoration of 3D
coordinates after a 2D list.

The independent fixture generator uses the approved reference in
`../inventor-interface/solidworks`, checks exact EOF and records source hashes
and primitive read inventories in `reference.json`. Numeric polygon blocks are
split into independently observed f64 coordinates without changing their bits.
Six Rust tests check reconstruction, primitive coverage, editing, all truncated
full-object prefixes, trailing bytes, malformed counts and typed BOM roles.
Three Python tests check fixture regeneration and oracle normalization.

The complete local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0.
Two fresh 906-case runs accepted 404 samples and explicitly rejected 502;
all previous 388 accepted samples remained accepted. The corpus gate therefore
still fails overall. `frontier-checkpoint.json` records the packet hash,
repeatability, newly accepted indices and remaining first failures.
`accepted-reference.json` records an independent complete reference read of
all 404 accepted streams, totaling 16,303,192 feature bytes, against the same
four-stream hashes. No detail-folder first failure remains in this corpus.

Stored-schema reconstruction does not certify geometry, topology or native
application acceptance. Missing feature classes remain outside successful
results. No dependency or package version changed. The humanizer skill is
unavailable in this session; its prose review remains with the maintainer.
