# Reference dimensions and dimension-plane tags

Bessemer reads, edits and writes ordinary modern reference dimensions above
version `0x27CB` in non-DB external-XML part archives. The base reuses the complete
dimension record, then reads its direct transform, independently selected vectors,
plane tag, sketch-dimension handle and signed raw words. The one- and two-entity
classes retain their own unsigned suffix words and generic entity pointers.

The reader preserves original presence bytes, transform bits, entity kind words
and zero-valued raw fields. It does not apply the native loader's runtime repairs.
Version gates at `0x2F6F`, above `0x3747`, above `0x42EE` and at `0x46C7` remain
separate. Existing reference-dimension handles accept these complete dimension
records through their established finite dimension family.

Dimension-plane kind two contains six direct doubles without another object tag.
Every other kind consumes one tagged pointer. Kind one requires a component face;
kind zero retains any supported object. Unknown kinds accept null pointers and
reject nonnull pointers explicitly. Stored kinds remain unchanged; runtime
effective-kind metadata does not introduce wire fields.

Independent Python readers verified 250 complete graphs across five classes,
ten versions and five styles. Cases include complete fresh sketches, all four
supported derived sketch-dimension handles, populated component faces, shared
point identities, direct planes, unknown null kinds, negative zero and NaN bits.
Five complete owned streams import the blank part's actual configuration map.
Public tests reconstruct documents, compare an independently assembled unsigned
word edit and verify input isolation after success, source mutation and rejection.
Private tests compare primitive observations and exact bytes, reject every prefix
and extra byte, and check invalid plane-reference classes and kinds.

The schema snapshot records sections 11.169 and 11.341, with source and section
hashes in `schema-reference.json`. `reference.json` binds the fixture observations
and complete reader-source hashes. Native application acceptance, engineering
meaning of raw values, fresh unsupported external-feature bodies, DB and older
archives, drawing-owner tails and full documented schema parity remain unproven.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

Two fresh processes reconstruct 705 of the 906 native corpus configurations,
preserving all previous 696 successes and adding nine. Three other cases advance
to later explicit stops. The two runs report identical outcomes and causes, and
the executable hash remains unchanged during replay. An independent immutable
snapshot of 1,131 Python modules verifies all 705 accepted streams through exact
EOF: 41,536,707 feature bytes, with all four original input-stream hashes checked
per configuration. The temporary snapshot was removed after verification.
The remaining 201 cases retain explicit failures; the broad corpus assertion
continues to fail until every configuration is supported.
