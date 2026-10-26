# Ordered model header imports

The private model context now derives its load-array entries from a validated
configuration header. It appends a non-null self-reference first, then every
external-list item in stored order. An absent or null self-reference adds no
slot. A null list item still adds one, and duplicate references remain duplicate
slots. The entire header object/class map is not imported.

Each slot keeps its `HeaderReference` in the retained header graph, separately
from its model `ArchiveMapIndex`. The model's root index is not a header object
identity. The imported context owns an immutable `Arc<DocumentHeader>` so its
references remain resolvable after the original caller drops its header handle.
Lookups distinguish an imported null from an index outside the imported range.

Class descriptors come from `HeaderNode::archive_class`, also used by the
existing header encoder. The native class and stored schema therefore have one
projection. Header validation continues to require external objects in external
lists. The mixed fixture uses distinct external objects with distinct schemas;
it does not relax that rule. A separate raw-node projection test checks another
supported header kind without making it a valid external-list item.

The import helper requires model versions above `0x27BC` and equal header/model
outer versions. It checks the decoder's version again when applying the plan.
This is the import boundary, not a claim of complete root support there: the
component helpers still require versions above `0x27CB`. Body-version markers
inside the header remain the header codec's responsibility.

Import counts are checked before allocation, including arithmetic overflow. The
model's aggregate entry budget is charged before any map slots are appended.
Context or budget failures poison the model builder. Successful planning and
importing consume no model bytes and do not change the header graph.

Eleven tests passed, covering native slot order, null and absent self fields,
duplicate references, distinct schemas, missing identities, incorrect raw graph
roles, count overflow, version mismatches, budget isolation and parallel header
reads after dropping the original handle. The native part still places its
import at slot 3 and the following node at slot 5. The full PR gate passed after
the shared descriptor extraction, including all 204 doctests. After test cleanup,
all 29 private model tests and all-target clippy passed again.

Root section parsing, complete model serializers, scoped-map transitions,
derived lengths, whole-document edits and the corpus gate remain required. This
context is test-only with the rest of the model implementation until the first
complete root operation passes. The humanizer skill was unavailable; prose
remains subject to maintainer review.
