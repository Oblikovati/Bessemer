# Draft features and direct identifier keepers

Bessemer reads, edits and writes ordinary non-DB external-XML `moDraft_c`
features above version `0x27CB`, excluding archive-condition one. The feature
reuses the complete body-feature base before its documented payload. The two
tagged keeper classes, `moDraftSIdKeeper_c` and `moEdgeFaceSIdKeeper_c`, share the
same complete keeper codec. Specification 11.174 / C-202 establishes their field
order, direct-member construction and imported serializer dispatch.

The reader preserves all signed option and presence words. Nullable arrays use
i32 presence followed by a direct packed pointer array, with no outer array tag.
Face-group presence is always read in this scope; a nonzero word introduces the
raw +72C word and counted groups. Each group retains its own signed face count.
Points use a signed count followed by direct three-double records. Empty groups,
repeated/null faces, independent arrays and full-width values remain distinct.
Native option conversion and environment-dependent runtime updates are not run.

The optional dynamic array reuses the existing bounded strided-array reader.
Element size zero ends the descriptor. Otherwise capacity, growth and element
count precede exactly size times count bytes, retained as raw 64-bit words.
Nonzero size must be divisible by eight. A large valid stride with zero elements
retains its metadata and consumes no payload; impossible extents and malformed
strides reject before allocation. Raw doubles, including NaN payloads, are not
normalized.

The direct keeper contains a signed entity-ID list, a packed reference array,
then a second signed entity-ID list only above `0x3746`. Entity IDs contain their
tagged representation pointers directly. The feature's embedded keeper adds no
class tag or map entry. Above the same version gate, the feature also stores its
late dimension and two signed words. The whole direct draft minimum is 311 bytes
through `0x3746` and 325 bytes afterward; keeper minima are six and ten bytes.
These are independently verified framing sizes, not native geometric validity.

Independent readers verify 90 complete graphs across three classes, five
versions and six styles. The observations retain 18,343 original primitive reads;
only the ten independently observed raw dynamic payload spans are projected into
lossless 64-bit words, yielding 18,383 comparison observations. Present-empty,
absent and populated arrays/descriptors/groups remain separate. Complete shared
dimensions, directions, word arrays, component faces/surface bodies, planes and
entity representations are included. One case exercises a populated inherited
body with a chooser referencing its own draft owner.

Nine owned streams import the actual 97-slot configuration map and finish at
101, 103, 115 or 117 slots. An independent golden changes only raw +830. Public
tests reconstruct complete documents, preserve raw values before native option
conversion, compare that edit and verify isolation after source mutation and
rejected group-count/stride edits. Seven private tests compare every primitive,
byte and map slot, reject prefixes and extra bytes, bound all signed counts,
reject unrelated cached references, distinguish valid empty large strides from
impossible payloads, and prove direct minima without additional object slots.
Three Python tests regenerate the observations and independent boundary cases.

`native-ancestry-reference.json` retains the three primary bindings and parent
chains under verified hashes of both installed modules. `schema-reference.json`
binds the approved section; `reference.json` and `minimum-reference.json` retain
the independent observations and reader hashes. Native geometry, runtime option
meanings, application acceptance, populated native dynamic arrays/keepers,
historical/DB/special contexts and complete file-schema parity remain unproven.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

Two fresh native processes preserve all 710 earlier complete configurations and
reach 716 of 906. Outcomes and causes agree for every case, with a stable
executable hash. Five other cases advance to later explicit dependencies; none
receives completion credit. An independent immutable snapshot of 1,170
approved-source Python modules verifies all 716 accepted streams through exact
EOF: 43,627,181 feature bytes, with all four original input hashes checked per
configuration. Temporary source copies and audit code were removed after their
processes terminated. The remaining 190 cases retain explicit failures, so the
broad complete-corpus assertion still fails.

The full local `cargo xtask pr` gate passes 571 library tests, four xtask tests,
every integration suite and 265 doctests. Three explicit external-corpus tests
remain ignored; their separately executed native replays are recorded above.
