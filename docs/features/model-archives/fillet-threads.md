# Fillets and cosmetic threads

Feature #44 now reads complete ordinary fillets, variable fillets, both
surface-ID keepers, three radius objects, edge/vertex dimension records and
cosmetic-thread features and records. The body-feature base uses the same
serializer as the existing extrusion and revolution readers. Lists, maps and
parallel arrays retain their order, duplicates and raw flags.

Fillet caches contain typed suppressed or serialized kernel bodies. Serialized
bodies require explicit schema context. Their expansion and record counts share
the enclosing operation's allocation limits. Two cached records are checked
independently at identifiers -7 and -11. The complete populated fillet charges
76 entries; a budget of 75 rejects it.

Thread instances store transforms, IDs and words as three parallel arrays. Each
array charges its entries. Two instances and two cache references charge eight
entries; seven rejects. A point follows its presence byte only when the byte
equals one. Values two and 255 remain distinct stored bytes and do not consume
a point. Repeated early and late value pairs retain separate field paths.

The independent packet contains 990 cases and 74,824 scalar expectations over
22 version boundaries. Every case reconstructs exactly, survives source
overwrite and rejects truncated prefixes and trailing bytes. An edited fillet
matches independently assembled bytes and preserves the original. Parallel
writes retain both cached bodies. Malformed signed counts and missing scoped
thread identities reject. The instance-ID and cache-data fixtures use explicitly
loaded objects: their new object bodies remain unsupported.

Fresh reference processes reproduce the fixture bytes. The native corpus
advances from 899 to 903 exact reconstructions out of 906. The remaining three
streams stop later at `moLPattern_c`; the complete corpus gate still fails.
The schema snapshot and verification report bind the selected source clauses,
fixture bytes and implementation. This checkpoint does not establish geometry
validity or native application acceptance. Public result/history contracts,
configuration diagnostics and the platform matrix remain open.

Humanizer was unavailable; prose remains for maintainer review.
