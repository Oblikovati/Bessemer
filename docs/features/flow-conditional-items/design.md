# Complete conditional-item operation

Implement the complete shared item grammar before exposing record-specific
typed entry points. The same checked item decoder must serve record 126 with
dimension three and records 315/128 with dimension one. Record 128 also supplies
a normalized wide-vector element width: nine patterns in version six, six in
version seven. Word vectors and byte vectors retain their own independent
counts; they do not inherit this multiplier.

Record 126 version one stores one count followed by three parallel wide arrays.
Version two accepts any signed inner word below one, preserves its signed
member, accepts only modes zero through two and requires dimension three. Only
mode zero stores a gate. A zero gate omits the entire count/list; every nonzero
gate selects it. Each list byte independently selects an absent or present
item. An item's own activity byte independently selects its active body.

Active item versions zero through three begin with exactly two wide/byte groups.
All active items store the required dimension and member; versions below three
also store the legacy signed word. Version zero has no mode byte. Version one
requires mode zero and stores two word vectors. Versions two and three store
two subrecords. Versions four and five instead store a counted group sequence,
each group containing wide and byte vectors followed by a subrecord. Each
subrecord preserves three signed words and its first word vector; only kind two
adds the independent extra vector. Other signed kinds remain accepted.

Record 315 has three independently counted leading wide vectors, an independent
extra count with a mode word present only when nonzero, then exactly two group
headers. Each group requires version zero, mode zero through two and protocol
one. Its raw member and optional unsigned gate remain bit patterns. Record 128
uses one such group inside its decoded body; its raw version prefix remains
outside body compression.

Reuse existing typed record identities, carrier checkpoints, physical history,
bounded exact codecs and raw-word access. Normalize the supported outer/item
versions, expected dimension, mode and block multiplier at their boundaries.
Bound decoded bytes, physical items, groups, subrecords, vector elements,
descriptors and total values independently before allocation or iteration.
Preserve ordered duplicates and present empty fields without allocating one
object for every numeric value.

The shared raw-version/body read and edit paths now live in
`src/flow/legacy_tables/wire.rs`. Record 108 calls those helpers with its original
version gate, body expansion and operation name. This preserves its validation
order, errors, zero-body cases and exact stored no-ops while providing the same
transport boundary for record 128. The family parser uses these transport helpers
and the shared exact scalar codecs. Its typed fields and ancestry are compared
with the complete frozen FCI1 expectations.

The first operation must read and edit every existing nonstructural stored
field, completely reparse the result and require unchanged physical scope and
field layouts. Version/count fields remain structural. Mode, gate, presence,
activity and subrecord-kind edits need the same full structural check. IDs bind
fields to the original occurrence and revision. No-ops preserve stored wire and
checkpoint identity. Changed results retain full carrier history and the exact
source checkpoint. Test all raw bits, malformed framing and independent bounds,
and compare serial, concurrent and separate-process results.

Do not assume native compressed bytes are the canonical output of Bessemer's
encoder. FCI1 freezes original streams as well as their decoded expectations.
Verify changed decoded bytes independently and preserve exact original streams
through no-ops and retained source checkpoints. Establish which freshly encoded
streams reverse byte for byte before making that stronger claim. Native
allocation, runtime conversions, inverse maps and engineering meanings remain
explicit exclusions from exact serialized acceptance.
