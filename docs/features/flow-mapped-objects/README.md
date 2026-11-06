# Flow mapped objects — issue #241

Specification 14.38 extends record 144's null-list profile with populated bodies.
The FMO1 fixture establishes independent raw expectations for the typed reader
and editor. It contains 192 cases: one native empty payload, two
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

`FlowMatchedObjectCount` validates the nonnegative signed-i32 domain once.
`FlowMappedContext` distinguishes unavailable context from an explicitly aligned
list, including an aligned empty list. `FlowCarrier::mapped_objects` reads the last
record 144 under positive entry, total-pair and total-numeric-word limits. It
checks products against remaining payload and remaining word budget before
allocation, with zero factors/dimensions handled explicitly. Missing M is a typed
failure whenever framing requires it. Returned views retain exact offsets, raw
bits, pairs, all legacy words and complete source/context checkpoints. Coverage
counts supplied M separately from zero-factor bodies with unresolved M.

`replace_field` edits an existing state byte, either word of an existing pair, or
an existing numeric word in any fixed, mapped or legacy block. It reparses the
complete carrier and list using the retained explicit context, checks the complete
patched payload, and returns source/result typed views plus complete physical
occurrence history. No-op edits keep original revision identities; fresh opposite
edits restore exact original carrier bytes. Pair edits retain supplied M and make
no claim that changed identifiers would produce that cardinality in a native map.
Versions, factors, presence and cardinality changes remain separate work.

Native runtime matching, concrete context object classes, engineering semantics
and application acceptance remain separate. The typed checks run with
`cargo test --test flow_mapped_objects --test flow_mapped_object_fixtures --test flow_feature_identities`.

The three native sources establish two absences and one eight-byte empty record.
There is no populated native fixture. Those eight bytes were already included in
the feature-identity corpus and receive no additional ledger credit here. This
fixture baseline supplies no global schema-coverage claim. The
maintainer humanizer pass remains for review because that skill is unavailable.
