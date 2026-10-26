# Configuration object dispatch

Seven existing match arms moved verbatim from `object_body.rs` into
`configuration_objects.rs`. They cover eight runtime classes: the configuration
manager, part configuration, linked atom node, two scene classes, scene data
and two configuration-table features. The main dispatcher delegates to this
module before its remaining match. Unrecognized names return without reading
bytes or changing the object map.

The extraction keeps the main dispatcher below 500 lines before adding Toolbox
records. A source comparison against commit `34d1967` verifies each moved arm
and the remaining dispatcher exactly. Existing owned fixtures pin these
configuration, scene and table bodies. All 463 library tests passed, with the
three external corpus gates still ignored by default.

`dispatch-verification.json` records the comparison and native corpus replay.
This is a structural change; Toolbox behavior belongs to a separate commit.
The humanizer skill is unavailable; its prose pass remains with the maintainer.
