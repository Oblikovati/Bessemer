# Stored entity visual roles

`PartResolvedFeaturesArchive::entity_visual_state` projects one complete physical
record from its revision-bound leading layer-number field. This first archive
route covers model features, sketch entities and nested display objects. The
configuration, display-list and drawing-wrapper routes remain pending under #181.

```rust
use bessemer::entity_visual::EntityVisualStoredEdit;

let visual = source.entity_visual_state(&layer_number_field)?;
let stored = visual.words();
let result = source.edit_entity_visual_state(
    &visual,
    EntityVisualStoredEdit::ColorRgb([12, 34, 56]),
)?;
let complete_bytes = result.archive().to_bytes()?;
```

The copied words distinguish layer number from line style. They retain the full
ByLayer word, cache byte, color word, signed width selector and optional raw
custom-width float. Named role requests change only their selected bit. RGB
requests preserve the unresolved high byte; only the exact `FFFFFFFF` color word
is missing. Width requests must match the actual stored float presence. Values
are stored projections; effective layer choices, native setter cascades, unit
conversion and rendering remain outside this operation.

Edits reuse complete archive validation and its completion, diagnostics, coverage,
fidelity and field history. History keeps source-path order, so a changed width
pair reports the custom float before the standard selector. No-ops retain the
source revision. Foreign or stale state fails before cloning fields; failed
requests leave source snapshots unchanged. The raw field API remains available
with its existing names and behavior.

The complete caller corpus and independent Source oracles remain the acceptance
basis. This API checkpoint does not complete #181 or establish geometry validity,
native application acceptance, or the remaining carrier and publication gates.
The humanizer skill is unavailable; prose review remains for the maintainer.
