# Surface cuts and trim surfaces

Bessemer reads, edits and writes ordinary non-DB external-XML `moSurfCut_c`
and `moTrimRefSurface_c` records newer than `0x27CB`, following specification
11.222 / C-250. Native primary tables, constructors, imported body-feature
serializers and ordinary node hooks bind both layouts.

Surface cuts preserve typed atom-feature, face, constant-surface and plane
references, signed words, two independent direct arrays and a direct body
chooser. Trims write selected members of an allocated runtime data object
directly: two arrays, profile, direction and u32; the feature's direct chooser;
then a solid reference, direct entity-reference keeper array, another array and
two signed words. The constructed runtime array at +80 is absent from the wire.
Neither the allocated trim data nor its keeper adds an outer object-map slot.

The existing optional chooser now delegates to a shared direct-body helper.
That extraction was committed separately after all ten existing body-feature
tests passed against 576 owned cases and 37,239 field observations. Its field
scopes and bytes remain unchanged. Direct choosers retain a list followed by a
typed atom-feature owner, without the optional helper's u16 presence tag.

The independent reader consumed 84 complete graphs at seven versions with
33,927 primitive observations. Fixtures cover inherited profile/face boundaries,
independent array and keeper lengths, signed/u32 extremes, complete shared face
and plane references, suppressed/serialized solid and constant-surface bodies,
complete profiles and directions, nulls, repeated pointers, distinct equal
features and chooser self-owners. Parsed identities and direct extents are
checked separately from primitive partitions and slot counts. Fourteen containing
streams use the actual 97-slot document context, including independent signed-word
edit goldens for both feature classes.

Six private Rust tests compare every field, byte and slot; reject every prefix,
trailing byte and old context; verify direct wire minima and declared pointer
roles; reject impossible escaped extents; and require explicit schema context
for serialized kernel bodies. Three public tests reconstruct all containing
documents and verify isolated successful/rejected edits for both feature classes.
Two Python tests regenerate every fixture and parsed identity check.

Two fresh native processes agree on all 906 outcomes and causes with a stable
executable hash. All 744 previously complete cases remain complete, and three
more complete, reaching 747. Three other cases advance to later explicit
dependencies; they remain failures. The remaining 159 configurations still
fail the broad corpus assertion. An immutable, verified copy of 1,236 reference
Python modules independently consumed every accepted feature stream through
exact EOF, covering 46,872,589 bytes. The report retains all four stream hashes
and each feature stream's unknown-byte total. This proves archive coverage within
that corpus, without adding geometry or application acceptance credit.

Present reference bodies are synthetic structural fixtures, without a claim of
valid solid topology. Runtime cleanup and transfer behavior, geometry and
SolidWorks application acceptance, historical/DB/special contexts and complete
documented-schema parity remain unproven. No dependency, exported Rust API or
package version changed. The humanizer skill is unavailable; its prose pass
remains with the maintainer.
