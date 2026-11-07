# Filled surfaces, keepers and curve contacts

Issue #54 implements the four tagged classes from clause 11.231 / C-273:
`moFillRefSurface_c`, `moFillRefSurfIdKeeper_c`, `moCurveContact_c` and
`moCurveContactData_c`. The scope is ordinary external-XML parts above version
`0x27CB`; earlier migrations remain unsupported.

Filled surfaces use the complete body-feature base followed by direct owned
data. Three direct arrays, raw +70, a direct array-bearing keeper, three words,
one byte and five further words occupy 45 bytes when arrays are empty. The
keeper at +118 adds no tag or presence flag. A tagged keeper has the same direct
array body. Its native surface-helper ancestry does not by itself expand the
separate body-header keeper family; the five recorded native filled roots have
null body-header keepers.

The +10C byte retains all bits. Its signed-char load interpretation is derived
from that byte, not inserted as another wire value. Stored +74 remains unchanged,
including value 2, although the native writer may mutate its runtime value to 1.
No scalar enum domains or geometry meanings are inferred.

Curve contacts preserve their direct array, doubles, words, typed angular and
synchronization pointers, and the three full-width trailing words. Empty contact
bodies occupy 62 bytes. Contact data preserves wrapper +18, generic object +08
and u32 +10, in that wire order; its empty body has eight bytes and is distinct
from `moContactData_c`. The generic pointer accepts a cached containing array,
while wrapper, angular and synchronization pointers remain typed.

The independent fixture set contains 264 complete graphs across eleven versions,
63,543 primitive observations and 27 owned streams. It covers shared complete
angular dimensions, synchronization references, wrappers, distinct equal roots,
direct keeper arrays, raw-byte sign cases, IEEE bit patterns and three isolated
word edits. The source verifies actual target identities and full byte partitions.
Rust checks minimum widths, every truncated prefix, appended bytes, old context,
impossible counts, typed-pointer rejection and unqualified cached references.
Public tests check document preservation and input isolation after successful and
rejected edits.

Run `cargo test --lib model_archives::filled_surface_tests` and
`cargo test --test resolved_filled_surfaces`; `cargo xtask pr` runs the repository
gate. Run `python -B -X utf8 -m unittest discover -s docs/features/filled-surfaces`
for independent regeneration. Source hashes must remain stable throughout a run.

`schema-reference.json`, `native-binding-reference.json` and `reference.json`
bind the specification, installed model/part DLLs and fixture observations.
Native reconstruction is reported separately from synthetic success, geometry
validity and SolidWorks application acceptance.

Two fresh native runs agree on every outcome and failure cause. Reconstruction
advances from 761 to 764 complete inputs, with 142 explicit stops and no lost
successes. Cases 132, 161 and 351 complete; case 747 reaches the later
`moEdgeRefIdRep_c` stop without receiving prefix credit. An independent pinned
source audit verifies all 764 accepted streams and four stream hashes per input,
covering 48,978,769 feature-stream bytes. The same source snapshot reproduces
five original C-273 filled-root spans and their 9,158 bytes, including raw scalar
values and null body-header keepers. Records are in `accepted-reference.json`
and `native-sentinel-reference.json`.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
