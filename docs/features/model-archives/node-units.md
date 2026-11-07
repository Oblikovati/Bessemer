# Node and units model records

The private model dispatcher now reads node-name and visual-property objects,
plus all 19 units class names described by the reference. Unit aliases share
their base, coded, energy, length or density serializer. Nested length objects
use the existing shared map, including self-references. The private nested-pointer
reader currently accepts the reference's length/percent serializer family;
native RTTI assignability for percent units must be confirmed before exporting
the model API. Unknown new object bodies
return a typed unsupported failure and poison the containing builder.

The node reader retains raw flags, signed IDs and version-gated fields. It
distinguishes an absent `unknown_2c` from a stored zero. Units preserve integer
words, IEEE-754 bits, optional nested objects and short/extended count framing.
Collection counts are charged to the operation budget and must fit the remaining
object tags. The legacy node and units-list boundaries remain explicit failures.

`freeze_node_units.py` builds independent wire cases from the schema and reads
them through the sibling reference implementation. It freezes 524 unit cases
covering 27 versions and 19 class names, with 5,385 expected scalar fields, and
seven node cases with 36 expected fields. Cases include signed values, signed
zero, a signaling double NaN payload, aliases, noncanonical presence bytes and
extended counts. Cyclic references are traced only at their first object body.
Two independent reference processes produced identical fixture artifacts.

The owned part's native units object occupies bytes 953..2440 of the complete
configuration. All 1,487 bytes reconstruct from typed fields, and 234 independent
scalar fields match. Its inherited map is supplied explicitly from the reviewed
root, header import, node-name and visual-property declarations; decoding with a
fresh map fails. The native node body at 26..78 also reconstructs exactly with
the header-derived map. These are bounded record tests, not a complete model
root operation or a production offset-based reader.

Ten focused tests passed, including every incomplete native units/node prefix,
zero and extended counts, impossible counts, incorrect node-name roles,
unsupported versions, input isolation and four independent parallel parses.
All-target clippy and the full PR gate passed after the new serializers,
including all 204 doctests.

The remaining shared-model continuation, part-specific lightweight suffix and
resolved section are still required before the first complete 24,028-byte root
operation. Public model APIs remain unexported. The humanizer skill was
unavailable; prose remains subject to maintainer review.
