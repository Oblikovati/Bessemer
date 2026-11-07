# Entity representations and shared external references

The private model codec now supports the 40 selected entity-ID representation
classes registered by the approved reference reader. Their layouts retain the
two keeper pointers, zero to three raw signed words, and a direct source entity
ID where specified. Direct source IDs add no enclosing MFC slot. The external
feature reference keeps its model pointer, signed word and four-byte time value.
Unknown classes still fail explicitly; this is not an exhaustive representation
family or a claim about the semantic validity of entity identities.

Bug #46's entity-representation case is addressed locally by a finite model-only
assignability rule. A pointer declared `moEntityIdRep_c` accepts only a recognized
representation variant. The same rule applies to new objects and map aliases.
Unrelated classes, unknown subclasses and a representation used for an unrelated
role still fail. The shared display role validator remains unchanged and has an
explicit negative regression. The original component-plane case and a newly
reached dimension-handle case remain open, so #46 is not closed.

The model's external-file reader consumes the existing header field rules,
including their version gates and wire kinds. It maps header field labels to
model field paths while using the same model MFC map. String handles, nulls,
aliases, shared-file metadata, configuration references and timestamps retain
their stored representations. Unsupported future header field kinds fail with a
typed first error. Four crate-internal visibility changes expose the existing
metadata to this test-only codec; no header rule body or production header
decoder changed. `entity-header-reuse-proof.json` records that comparison and
the hashes of the reference's retained RTTI/virtual-dispatch evidence files.

| New owned suite | Cases | Independent scalar values |
|---|---:|---:|
| Selected representations, direct IDs, aliases and feature references | 482 | 1,144 |
| Shared external-file references | 24 | 396 |

Each case reconstructs exactly and rejects every shortened prefix and trailing
byte. Representation cases cover both ordinary model versions, all selected
classes, null/full keepers, self cycles and repeated direct-ID references.
External cases cover eight version boundaries and null, distinct and aliased
string handles. Source overwrite and four concurrent writers preserve every
external case. Two fresh reference processes reproduced both wire packets.
The three existing header suites passed all 22 tests after the metadata reuse.

Exact native reconstruction increased from 632 to 768 of 906 configurations,
covering 718 document paths. Accepted model versions are 11000 (one), 18000
(seven) and 19000 (760). The remaining first stops are 48 relations, 44
mate-reference folders, 42 length parameters, one part exploder, one angle
parameter and two derived pointer-role failures. Later failures can still appear
after these are implemented. The corpus gate remains failed at 768/906; no
missing body is skipped and no unread remainder is preserved as a successful
model. Configuration error attribution remains open under #47.

`entity-external-verification.json` records the executed checks and final Rust
sources. The schema snapshot binds clauses 11.8 and 11.18; fixture manifests bind
reference source and generator hashes. The complete #44/#2 requirements,
including corpus-wide edit/history/document acceptance, public model APIs,
native application checks and the platform matrix, remain open. No extra branch,
PR, version or release is included. The humanizer skill is unavailable; the
prose pass remains for maintainer review.
