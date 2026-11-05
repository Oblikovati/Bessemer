# Flow mapped objects — issue #241

Specification 14.38 extends record 144's null-list profile with populated bodies.
The FMO1 fixture establishes independent raw expectations before the typed reader
and editor are introduced. It contains 192 cases: one native empty payload, two
source-supplied synthetic payloads and 189 owned version/context combinations.

The outer list and populated-body versions are independent words, each in the
strict profile 0–6. Presence bytes retain every nonzero pattern and state bytes
remain raw. Every populated body has nine numeric words, signed identifier pairs,
a stored nonnegative factor and factor × M numeric words. M is supplied runtime
matched-map cardinality; it is not the number of stored pairs or unique identifiers.
Duplicate pairs remain physical wire entries. The fixture never performs native
object matching or treats supplied context as validated native state.

Context aligns with every physical entry. Null entries require missing context,
including when a caller otherwise supplies a list. Positive factors require an
explicit M, while a zero factor consumes no mapped values and can retain unknown
M. Explicit zero is distinct from missing context. The maximum signed count with
a zero factor remains a descriptor and creates no huge numeric array.

Body version 5 alone has a sixteen-word prefix, three counted word arrays and
3 × n1 × n2 × n3 further words. Cases include populated and zero products, mixed
null/populated entries, independent list/body versions, full raw state/presence
bytes, signed pair boundaries, duplicate identifiers, negative zero, NaN payloads,
infinities and subnormals. All numeric words retain exact u64 patterns alongside
the documented binary64 interpretation. No physical axes or units are inferred.

Python obtains normalized expectations from the sibling `flowmappedobjects.py`.
Rust independently rebuilds every payload from those fields, checks exact entry
intervals, explicit products, raw bits and full carrier retention. Regeneration
uses temporary directories and verifies all native source hashes before extraction.
Run `cargo test --test flow_mapped_object_fixtures` and
`python -m unittest discover -s docs/features/flow-mapped-objects -p test_mapped.py`.

The typed context boundary, bounded reader and isolated wire edits remain next
work. That implementation must bound products against remaining payload and
configured total allocations before reading or allocating. Missing context must
remain a typed failure whenever framing requires M. Version-5 count products must
handle a zero dimension explicitly. Native runtime matching, concrete context
object classes, engineering semantics and application acceptance remain separate.

The three native sources establish two absences and one eight-byte empty record.
There is no populated native fixture. Those eight bytes were already included in
the feature-identity corpus and receive no additional ledger credit here. This
baseline ships no new production API or global schema-coverage claim. The
maintainer humanizer pass remains for review because that skill is unavailable.
