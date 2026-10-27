# Projected-line subtypes and surface base wrappers

Feature #54 implements the three exact native class bindings in clause 11.335 /
C-377. Bug #46 covers the required projected-line data role: complete draft and
intersection bodies now join the existing base and project bodies in that finite
family. Null data and owners, unrelated objects and unrelated cached arrays
remain rejected. The supported scope is ordinary external-XML parts above
version `0x27CB`.

Draft and intersection data reuse the complete nonnull feature-owner and
nullable direct-array prefix. The signed presence word is retained, including
noncanonical nonzero values and present-empty arrays. Draft then stores a raw
binary64 value and a direction-spec pointer. Intersection stores one signed word
and three tagged `suObList` pointers. Fresh lists use their complete existing
bodies; null and shared lists retain pointer identity. Direct arrays and tagged
lists keep distinct framing and map accounting. No scalar domains are inferred
from the native observed values.

The fresh `moSurfaceRef_w` class uses the existing complete surface-reference
body: common wrapper, then four doubles. Its ordinary modern scope satisfies the
native historical double gate. The existing typed axis-surface role already
admits this base class; the generic wrapper family is unchanged.

Independent fixtures cover 390 complete graphs across thirteen versions, 69,354
primitive observations and 35 owned streams. They include complete PLine
parents, standalone data with shared complete PLine owners, self-owned data,
direct entity IDs, raw WORD/DWORD values, count escapes, independently sized
arrays, nullable presence extremes, typed surface callers and five isolated
scalar edits. Draft doubles cover negative zero, signed NaN payloads and both
infinities. Source identity checks and primitive partitions are guarded by a
stable source snapshot. The source reader's permissive expected-class argument
is not a pointer-role oracle.

Ten private tests cover full reconstruction, every truncated prefix, appended
bytes, old context, minimal subtype widths, count bounds, exact floating-point
bits, required owners/data, typed directions/lists, cached-list identity and the
existing axis-surface role. Three public tests cover complete document
reconstruction, isolated word/double edits and source isolation after successful
and rejected edits.

Run `cargo test --lib model_archives::projected_line_subtype_tests`,
`cargo test --test resolved_projected_line_subtypes` and `cargo xtask pr`.
Independent regeneration uses `python -B -X utf8 -m unittest discover -s docs/features/projected-line-subtypes`.

Two fresh native processes agree on all 906 outcomes and causes. Eight inputs
complete, raising reconstruction from 767 to 775 without losing any prior
success. All four reported draft-role cases complete. Case 153 reaches the
later documented `moPntSurfRefPlnData_c` role rejection under Bug #46 and receives
no completion credit. The independent pinned-source audit verifies all 775
accepted streams and four stream hashes per input, covering 49,945,035
feature-stream bytes. Under the same snapshot, nine original native
configurations reproduce every recorded root interval/hash and the three base
surface, four draft and four intersection objects' stored fields, presence
words and list counts. These source sentinels are separate from whole-file
Bessemer completion. Reports are in `accepted-reference.json` and
`native-sentinel-reference.json`.

Source/native bindings and executed checks are retained beside this document.
Historical migrations, alternate contexts, raw geometric meanings, geometry
validity, SolidWorks application acceptance and broader all-schema parity remain
open. The humanizer skill is unavailable; prose review remains a maintainer task.
