# First relationship operation

Feature #197 will begin with a read/preserve/reopen operation on
`InspectionRecords`. It will expose ordered load attempts, an initially empty
first-insertion map view, and the writer's empty-key filter as distinct views.
The caller must select retained direct DOM nodes or element-only traversal;
neither option is presented as the unknown native caller policy. This is a
projection of an existing envelope, not a relationship writer or graph validator.

## Shared parser changes

`src/metadata/reader.rs` already bounds and validates each lexical event before
publication. Reuse that builder for an operation-local ordered parse when relationships are
requested. The additional bounded pass avoids retaining ordered content in every
ordinary record index.
`XmlModel` currently keeps element children and concatenated direct text, which
cannot represent `a<child>b</child>c` as descendant text `abc`. It also drops
comments and processing instructions. Use an optional operation-local ordered content
representation containing child-element references, decoded text, comments and
processing instructions. Retain original qualified element spellings in that
optional representation. Default validation and Composer parsing omit it;
Relationship projection requests it explicitly. A checked ordered view is
required at the relationship projection boundary. This separation follows the
retained read-cost measurement and restores the original element record layout. Expanded namespace matching used by other consumers
must remain unchanged.

Text and general-reference events can form one DOM text node. Coalesce adjacent
text segments; keep CDATA, comments, processing instructions and element
boundaries explicit. The selected reference DOM drops empty CDATA sections;
retain that distinction when projecting this explicit DOM policy. Test grouping
against the pinned helper before exposing attempt counts. Charge limits from the original event stream, including events
that a selected projection omits. Do not reinterpret a lower projected node
count as permission to admit an over-budget document.

## Projection and identity

Select the first exact unprefixed `RelationshipsDictionary` child under the
selected package, including an unprefixed default-namespace spelling. Preserve
physical occurrence identity for every attempt, including non-element attempts.
Use the existing document revision and retained immutable XML as provenance;
never derive identity from a textual relationship key. Return completion,
explicit check mode, diagnostics, coverage and copied history together.

Apply the recorded Qt whitespace set to field names and extracted text. Repeated
key fields replace the temporary key; repeated value fields append. Select first
exact `string` and `ArrayOfString` children and preserve descendant text order.
The initially empty map keeps the first equivalent UTF-16 key and copies its
value sequence. Published inspection order follows first occurrence and does not
claim native map iteration order. Rust XML strings exclude isolated surrogate
code units and NUL; the wider pure Source helper domain remains a separate test.

## Acceptance and deletion gate

Before adding edits, prove the entire first operation using the owned ordered
fixture and decoded historical tutorial. Assert all attempts, selected map
entries, writer-filtered entries, source identities, diagnostics and exact
unchanged XML/encrypted envelope bytes after reopen. Cover both explicit node
policies, absent and duplicate dictionaries, qualified-name collisions, adjacent
references, CDATA, nested text, duplicate keys, empty values and Qt whitespace.

Run parser regression fixtures unchanged for existing metadata and Composer
consumers. Exercise each exact limit and one-below rejection, source isolation,
concurrent operations and complete fresh-process result equality. Replay the
actual Rust exports through a frozen original Source helper. Remove temporary
collectors after their evidence is retained. A failed or inconclusive comparison
blocks acceptance; it does not justify changing the Source baseline. Native
application reload, graph meaning and writing remain separate requirements.
