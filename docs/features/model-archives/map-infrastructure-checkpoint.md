# Header import map support

Feature #44 needs native `AddToLoadArrayDirect` insertion after the root object
has been mapped. The owned reference oracle places the root class at 1, the root
object at 2 and the header's external object at 3. The next serialized class and
object therefore occupy 4 and 5. Importing a header object does not register its
class and does not consume model stream bytes.

The self-reference contributes a slot only when it is non-null. A present
external-reference list contributes each item in order, including null and
repeated items. The header's typed object graph determines each imported class
and schema; the importer must not replace stored schemas with a default.
The existing header validator requires external-list object entries to be
external objects. That constraint remains unchanged. Importing the
entire header map would add unrelated class slots and shift the configuration's
native indices.

`ObjectReader::append_loaded_object` reuses the existing slot insertion path for
both ordinary new objects and header imports. Existing new-object decoding calls
the helper with its parsed class, preserving the earlier map layout. A null
import takes a slot too. Imported entries cannot be used as class descriptors;
small and wide reference tags resolve through the same existing map checks.

Four focused Rust tests pin native slot order, repeated imports, imported nulls,
class/object distinction and wide references. Independent literal class tags
check that the writer's existing reservation primitive does not emit bytes or
register the imported class. A native integration test under `tests/model_archives/`
parses the owned configuration header, derives its self-reference schema and
loads the actual configuration's root and next node tag. It verifies slot 3 for
the import and slot 5 for the node, with unchanged header bytes. The header's
object identity is distinct from configuration slot 3.

The full local PR gate passed with the structural helper and four focused tests,
including all 204 doctests. After test helper cleanup, the four tests passed
again. The added native integration test and final clippy run also passed. The
complete model feature still needs its own final PR gate and five CI checks.

This is private shared infrastructure, not model stream support. The complete
model caller must still derive the ordered imports from a validated header,
retain header object identities separately from local map indices, and charge
imports against operation budgets. It must preserve the header graph while
decoding and editing the configuration. Scoped maps, complete serializers,
reconstructed section lengths and whole-document acceptance remain required.

The change remained outside PR #43 and is now on `codex/model-archives` after
that PR merged. It will be kept in a structural commit separate from model
behavior changes. The humanizer skill was
unavailable; prose remains subject to maintainer review.
