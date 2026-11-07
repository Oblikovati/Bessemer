# Mirrored solids

Bessemer reads, edits and writes ordinary non-DB external-XML mirrored solids
newer than `0x27CB`, following specification 11.214 / C-242. The complete
body-feature prefix is followed by a typed face reference, two independently
selected planes, separate direct arrays and list, typed surface references,
a matrix helper, fixed-width signed indices and retained raw fields.

Any nonzero signed plane selector preserves six stored doubles. Version gates
use the documented strict comparisons. Direct containers allocate no outer
object-map slots. Existing matrix-helper support is reused. Its complete nested
`suUIntArray` records retain a signed fixed-width count, growth word and unsigned
elements, including high bits and duplicate values. Invalid extents fail before
reading entries. No runtime filtering, cleanup or geometry normalization runs.

The independent reference reader consumed 198 complete graphs at eleven versions
with 23,487 primitive observations. Fixtures cover independent collection sizes,
all signed selector extremes, raw floating bits, empty and populated helpers,
unsigned extremes, distinct equal objects, shared targets and repeated aliases.
Parsed identity checks are separate from byte spans and slot counts. Nineteen
containing streams use the actual 97-slot document context, including an
independently assembled single-word edit. Five private Rust tests compare all
fields, bytes and slots; reject every truncated prefix and trailing byte; check
version minima, declared roles and bounded counts. Three public tests reconstruct
documents and verify edit/source isolation. Two Python tests regenerate the corpus.

Native primary tables and ancestry bind the mirrored-solid body-feature prefix
and matrix helper. The separately hash-bound unsigned-array serializer comes
from `sldmfcu.dll`; both installed module hashes match the retained evidence.
Standalone `moSurfaceRef_w` is not registered by the reference reader, so populated
fixtures use the already supported `moPlaneRef_w` subclass with its complete body.

Two fresh native processes agree on all 906 outcomes and causes with an unchanged
executable hash. All 736 previously complete cases remain complete, and four more
complete, reaching 740. Two other cases advance to later unsupported classes;
they receive no completion credit. The remaining 166 cases fail explicitly, so
the broad corpus assertion still fails. A verified immutable copy of 1,222
reference Python modules independently consumed every accepted feature stream
through exact EOF, covering 46,492,703 bytes. Input hashes and retained unknown-byte
totals are recorded with the audit. This proves archive coverage within that
corpus; it does not prove geometry or application acceptance.

Archive reuse, historical and DB contexts, geometric validity, SolidWorks
application acceptance and all-schema parity remain unproven. No dependency,
exported Rust API or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.
