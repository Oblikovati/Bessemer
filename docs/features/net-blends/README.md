# Net blends, curve sets and component groups

Issue #54 implements clause 11.226 / C-268 for ordinary external-XML parts above
version `0x27CB`. The five tagged classes are `moNetBlend_c`, `moNetBlendCut_c`,
`moContactData_c`, `moCompEntGroup_c` and `moCompMultiEdge_c`. The native evidence
binds both feature serializers to the complete body-feature base and direct
embedded net-blend data. Embedded data, curve sets and options add no tagged
headers or identity-map slots.

Each of the three curve sets retains a contact array, N direct dependent arrays,
N three-double points without another count, an independent signed count M and
its positive-count points, three direct DWORD arrays and four raw words. Zero or
negative M consumes no points and remains unchanged. Curve and centerline options
retain their byte widths and original values, including the stored `0x80` option
that native runtime loading may migrate.

The direct body-options record has 28 bytes. Its thin byte selects the first
display dimension whenever nonzero; signed mode 3 additionally selects the
second. The exact thin byte remains stored. Surface-degree loading adds three
signed words only above `0x2F4B`; its first word retains all four bytes rather
than a Booleanized runtime value. Empty direct data has 194 bytes before that
gate and 206 after it, with no dimensions when thin is zero.

Contact data keeps an unqualified object pointer and a raw u32. Component groups
retain their base, raw word and two direct lists. Multi-edge records use a full
u32 count, typed component-edge pointers and the complete direct owned
implementation, including raw doubles and points. Null-owned fallback framing
uses the same body. Typed reference wrappers admit the two constructor-proven
component classes through their existing finite family.

The independent fixture set contains 570 graphs across nineteen versions,
205,111 primitive observations and 34 owned streams. It covers independent and
negative point counts, all thin branches, the degree gate, inherited component
and wrapper gates, wide packed DWORD counts, signed/unsigned extremes, IEEE bits,
shared dimensions, cached contacts, cyclic groups and distinct equal roots.
Four independently assembled edits change only the first feature/group/multi-edge
word. Source-parsed identities and complete primitive partitions are checked.

Rust checks every truncated prefix, appended bytes, old context, impossible
packed/signed/u32 extents and unrelated cached pointer roles. Public document
tests check exact bytes, unchanged storage and input isolation after successful
and rejected edits. Run `cargo test --lib model_archives::net_blend_tests` and
`cargo test --test resolved_net_blends`; `cargo xtask pr` runs the repository gate.
Run `python -B -X utf8 -m unittest discover -s docs/features/net-blends` for
independent regeneration. Source hashes must remain stable throughout each run.

`schema-reference.json`, `native-binding-reference.json` and `reference.json`
bind the specification, installed model/geometry DLLs and fixture observations.
Native corpus reconstruction is recorded separately from synthetic fixture
success, geometry validity and SolidWorks application acceptance. Scalar meanings
and legacy serialization outside the declared modern context remain unproven.

Native reconstruction advances from 757 to 761 complete inputs, preserving all
prior successes. Both fresh processes reproduce all 906 outcomes and causes.
Cases 618, 624, 625 and 632 complete; two other cases reach later failures and
receive no completion credit. The independent pinned-source audit verifies all
761 accepted inputs and four stream hashes per input, covering 48,364,098
feature-stream bytes. A separate source replay reproduces the five original
C-268 net-blend root spans and their 40,237 bytes exactly. See
`frontier-checkpoint.json`, `accepted-reference.json` and
`native-sentinel-reference.json`.

The newly exposed `sgExtEnt_c.reference` rejection of `moSingleFaceRef_w` in case
351 is catalogued under Bug #46, comment 6440280841. It remains a separate role
audit; the net-blend reader does not broaden that field's accepted family.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
