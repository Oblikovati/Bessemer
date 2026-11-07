# Part explosion records

Feature #44 now reads the documented part exploder, ordinary/body/chain steps,
explode lines, body transforms and direction specifications. The scoped
`moCompHolder_c` mapping reuses the component-reference body, as the reference
part reader explicitly specifies. The shared unsigned-word array reader also
serves the existing per-body sheet-metal record without changing its fields.

The owned packet contains 44 cases and 1,882 independent scalar expectations.
Cases cover old and modern versions, compact and extended counts, populated
arrays, both optional direction vectors, optional points and compact transforms.
They reconstruct exactly. Every truncated prefix and trailing-byte variant
fails. Required body/step references reject nulls and unrelated classes;
impossible word counts fail before iteration.

Changing the exploder's stored `raw_360` word to 12345 matches a separately
assembled and reference-validated byte fixture. Reparsing preserves the edited
fields, while the original reconstructs unchanged after source overwrite.
Four concurrent writers produce identical bytes for every owned case. These
checks establish archive ownership and deterministic encoding; they do not
execute an explode animation or establish geometry validity.

The first native run reached `moCompHolder_c` in case 340. Adding its documented
mapping made that configuration reconstruct exactly, increasing acceptance
from 850 to 851 of 906. The remaining 55 first failures are dimension handles
and the two tracked role failures. The complete corpus gate still fails.
Earlier corpus outcomes remain in `native-corpus-checkpoint.json`.

All 164 model tests passed, with two external-input tests explicitly ignored
and run separately. A test-helper extraction followed; all six explosion tests
and all-target Clippy passed on its final source. Production sources did not
change after the broader run. All 50 Python tests also passed.
The preceding full PR gate
belongs to the geometric/kernel checkpoint; rerun it before pushing the final
feature. Broader model-test and repeat-corpus results are bound separately in
`explosion-verification.json`. The public model API and full operation contracts
remain pending. Humanizer was unavailable; prose remains for maintainer review.
