# Archive layout development checkpoints

Historical development checkpoints. Later reports supersede earlier counts and remaining-work lists. See [the current acceptance review](acceptance-review.md) for the present scope and integration gate.

## Remaining implementation and integration gate

The [modern display-item checkpoint](display-items.md),
[view-bucket checkpoint](view-buckets.md), [light checkpoint](lights.md) and
[version-range checkpoint](version-range.md), [older shaded checkpoint](old-shaded.md)
and [early shaded checkpoint](early-shaded.md) reconstruct all 1,086 complete
reference-readable part/assembly streams in this corpus. Their fixtures
and field-oracle counts supersede the earlier totals
above. This corpus milestone does not establish complete schema parity.

The [legacy bucket checkpoint](legacy-buckets.md) extends the unexercised schema
coverage for the four older supported snapshots while preserving that corpus gate.

- Graphics-reference wire coverage and its independent plane evidence are recorded
  in [graphics-references.md](graphics-references.md); unresolved faceted-data bodies
  remain unsupported.
- Populated ink grids, empty dimensions and isolated state edits are covered in
  [markup-ink.md](markup-ink.md).
- Per-configuration bodies, view matrices, visibility pairs and high-word-first
  tails for admitted contexts are covered in [configurations.md](configurations.md).
- Decal lists, keyed maps, masks, face associations and reused appearance records
  are covered in [decals.md](decals.md).
- Textured-plane records with null bitmap pointers and complete transforms are
  covered in [textured-planes.md](textured-planes.md); bitmap bodies remain unresolved.
- The middle range extends admitted contexts to 16000–19000 with complete facet,
  component, material, scene and bucket boundary fixtures in
  [middle-range.md](middle-range.md).
- The lower range extends admitted contexts to 14000–19000 with texture, thread,
  face, component-number and detailing boundaries in [lower-range.md](lower-range.md).
- The 13000 range adds older scalar widths, retained silhouette words, appearance,
  annotation and ink boundaries in [transition-range.md](transition-range.md).
- Context resolution and exact external face-cache links now have owned and native
  checks; broader geometry and native application validation remain unfinished.
- Reuse existing MFC strings, exact numeric values, class/object maps and face
  tessellation records. Preserve cycles, aliases and object-tag spelling; do not
  reset inherited state between nested serializers.
- Extend environments, splitters, fonts, lights, visual properties, buckets and
  display items to the remaining version and caller contexts.
- Add earlier scene and unit layouts, remaining shaded-data layouts,
  textured-plane bitmap bodies and earlier configuration layouts. Unknown-meaning
  fields remain explicit raw typed values; unresolved bodies are unsupported.
- Immutable archive graphs, validated owned construction/edits, revision-scoped
  identities and authentic reverts have focused checks. The
  [stored-archive edit report](edit-history.md) adds exact coverage and structural
  field history; geometry and external-block checks remain separate.
- Preserve the complete 1,086-stream forced-byte gate and expand independent field
  oracles to the remaining schema branches. Add their owned version/degeneracy
  vectors, malformed input/budget checks, whole-document isolation and process/
  concurrency comparisons, then run all five CI checks.
- Update the public README and final CHANGELOG entry, review this evidence, create
  one PR, merge when green and remove the finished branch. Native application
  acceptance stays separate from archive validity and geometric validity.

[Clause 13](specification-13.md) is a working-tree snapshot from the approved
`../inventor-interface` repository, whose reference commit is
`dd0537b3a142d8dda00ef846891efc736a6aac46`; its documentation is evolving. Corliss's
typed decoded fields and isolated edits inform the public API design. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.

- The 12000 range retains discarded edge objects, float coordinates and caller-specific
  collector gates; see [earlier-range.md](earlier-range.md).

- The remaining 11000 interval adds configuration and component boundaries,
  including inline configuration tessellation; see [precollector-range.md](precollector-range.md).

- The 10000 interval retains transient viewer graphs and earlier facet/bucket
  boundaries; see [component-transition.md](component-transition.md).

- The 9000 interval covers grouped components, word-key bucket maps and the early
  assembly configuration/decal boundaries; see [grouped-components.md](grouped-components.md).

- Explicit native file context now controls tangent-list omission independently
  of display-list version; see [file-contexts.md](file-contexts.md).

- The 8000 interval covers earlier annotation strides, environment/body/unit
  omissions and the documented uncounted-window exclusion; see [window-boundaries.md](window-boundaries.md).

Early complete-root view transitions and Bug #37 are recorded in [early view roots](early-view-roots.md).

- The 6000 interval retains historical scene, wall, component, pen and halo-list layouts; see [historical roots](historical-roots.md).

- The 5000 interval covers early appearance, face-identity and display-item fields; see [early appearances](early-appearances.md).

- The 4850 range covers exploded-view and scene background-fit gates; see [earliest scenes](earliest-scenes.md).

- The 4800 range covers component metadata, property fonts, BOS flags and populated decal boundaries; see [earliest components](earliest-components.md).

- The 4600 range covers face-pair arrays, scene flags, 24-slot property fonts and polyline layouts; see [earliest faces](earliest-faces.md).

- The 4500 range covers all five older light kinds, appearance words, hidden atoms, weld lists and decal options; see [earliest lights](earliest-lights.md).

- The 4400 range covers earlier appearance payloads, face words and null legacy wall materials; see [earliest walls](earliest-walls.md). Non-null legacy wall-material bodies remain unresolved and explicitly unsupported.

- The lower bound is now 4300, with direct body-property maps, earlier state words and versioned OLE table lists; see [earliest bodies](earliest-bodies.md).

- Archive object lookup now checks revision-scoped handles; see [revision identities](revision-identities.md). [Edit reports](edit-history.md) now provide structural field history; whole-document cross-stream acceptance remains a separate gate.

- Shared line-style and layer-manager CObject bodies retain aliases and versioned map framing; see [UI objects](ui-objects.md).

- Component properties and holders preserve version-sized font arrays and shared cyclic graphs; see [component properties](component-properties.md).

- Typed document context resolution reads exact markers and bounded face-directory declarations; whole-document byte reconstruction is covered in [document context](document-context.md). Named blocks and requested external facets are checked in [external face links](external-face-links.md).

- The lower bound is now 4200, including implicit face, body and model visual pointers and adjacent viewer, table and material gates; see [earliest visuals](earliest-visuals.md).
- The 4100 checkpoint adds the pre-`0x1052` appearance layout, with absent rendering fields and the retained old appearance word; the expanded [earliest visuals](earliest-visuals.md) corpus contains 76 roots and four complete containers.
- The 4000 checkpoint adds the optional root model-size double at `0xFCB` and face/decal records at `0xFD8`. The expanded [earliest visuals](earliest-visuals.md) corpus contains 110 roots and six complete containers, retaining all prior fixtures.
- The 3905 checkpoint covers the earlier environment, 23-font properties, omitted scenes and assembly shrink-wrap words. The [earliest visuals](earliest-visuals.md) corpus now contains 168 roots and eight complete containers; pre-scene property graphs use bucket CObject keys.
- The 3890 checkpoint adds caller-specific legacy line widths before `0xF41`. The [earliest visuals](earliest-visuals.md) corpus now covers 196 roots and ten complete containers, preserving the previous cases.
- The 3800 checkpoint adds 15-font properties and pre-`0xF1E` material and texture layouts. The [earliest visuals](earliest-visuals.md) corpus now covers 224 roots and twelve complete containers, retaining the previous cases.
- The 3555 checkpoint covers independently gated appearance words and assembly reference IDs. The [earliest visuals](earliest-visuals.md) corpus now contains 258 roots and fourteen complete containers, retaining the previous cases.
- The 3508 checkpoint adds independently gated visibility, configuration, pattern and hierarchy records. The [earliest visuals](earliest-visuals.md) corpus contains 298 roots and sixteen complete containers, preserving the previous cases.
- The 3330 checkpoint adds the independently gated facet-display and shader words. The [earliest visuals](earliest-visuals.md) corpus contains 326 roots and eighteen complete containers, retaining the previous cases.
- The 3238 checkpoint adds early PV and keep-scene light words, with all five old-light classes. The [earliest visuals](earliest-visuals.md) corpus contains 404 roots and twenty complete containers, retaining the previous cases.

The 3196 checkpoint covers front-view, font-cap, pen-cap and bucket-pointer gates. The [earliest visuals](earliest-visuals.md) corpus now contains 548 roots and twenty-two complete containers, retaining all previous cases.

Edition 0.206 adds complete modern drawing `Contents/VBLists` framing. Its separate
[implementation and evidence](../drawing-buckets/README.md) reuse the shared
codecs while preserving the drawing backend facet context. Drawing
`Contents/DisplayLists` remains unresolved.

The [unsigned-version checkpoint](unsigned-versions.md) removes the artificial
19000 ceiling, with 85 rich part/assembly roots, 25 drawing streams and three
signed-alias documents. All earlier unresolved intervals remain excluded.

The [earlier resolved archive checkpoint](pre-facet-roots.md) admits contexts
from 3065, with 94 complete roots and six containers. Cache-backed parts retain
their non-null objects and exact external face references. Unresolved inline
facet objects remain explicit payload failures; empty geometry is preserved distinctly.

The [legacy pane, line and symbol checkpoint](legacy-panes.md) admits contexts
from 2999, with 66 additional complete roots and six containers. Old line flags
retain their full 32-bit words; omitted pane strings and symbol planes stay absent.

The [camera, viewer and text checkpoint](legacy-camera-layouts.md) admits contexts
from 2902, with 66 additional complete roots and 8192 independent field checks.

The [earlier appearance checkpoint](legacy-visual-layouts.md) admits contexts from
2891, with 50 complete roots and six containers. Lightweight-rendering words and
appearance names follow separate presence gates; cache-backed faces remain complete
through edits, reopen and revert.

The [earlier add-in and body-property checkpoint](legacy-addin-layouts.md) admits
contexts from 2796, with 50 complete roots and six containers. It preserves absent
add-in fields and body-property pointers without inserting default values.

The [implicit-width checkpoint](legacy-width-layouts.md) admits contexts from
2771, with 34 complete roots and six containers. Unicode text determines implicit
widths, while later stored counts remain independent of text length.

The [bucket thread-pattern checkpoint](legacy-thread-layouts.md) admits contexts
from 2749, with 94 complete roots and six containers. Populated 2D and 3D buckets
retain their aliases and complete shaded models across the thread-storage boundary.

The [polygon-region checkpoint](legacy-region-layouts.md) admits contexts from
2725, with 94 complete roots and six containers. Independent oracles retain the
raw region words as well as their derived flags; earlier layouts omit the words.

The [viewer-mode checkpoint](legacy-mode-layouts.md) admits contexts from 2711,
with 94 complete roots and six containers. Absent, one-word and three-word mode
records remain distinct, including complete shape-changing edits.

The [line-visibility checkpoint](legacy-visibility-layouts.md) admits contexts
from 2650, with 150 complete roots and six containers. It preserves omitted
visibility words and records why the older holder-name branch is unreachable.

The [temporary edge and triple checkpoint](legacy-edge-layouts.md) admits contexts
from 2606, with 206 complete roots and twelve containers. It retains discarded
edge words, nested view triples and the versioned absence of component string maps.

The [resumed inline-facet checkpoint](resumed-inline-layouts.md) admits contexts
from 2596 and proves complete inline part and assembly edits before the
unresolved facet-object interval. Its 130 roots and four containers preserve
empty and cached layouts across the 2597 boundary.

The [ordinary text checkpoint](ordinary-text-layouts.md) admits contexts from
2563, with separate text-width and material-classification boundaries, 158
complete roots and four inline part/assembly containers.

The [legacy rendering checkpoint](legacy-rendering-layouts.md) admits contexts
from 2388. Its 262 complete roots cover material rendering flags and auxiliary
facet textures on both sides of their bounds, with four whole-document fixtures.

The [legacy texture checkpoint](legacy-texture-layouts.md) admits contexts from
2359, covering absent material textures and per-view shaded-edge words with
190 complete roots and four whole-document fixtures.

The [legacy environment checkpoint](legacy-environment-layouts.md) admits contexts
from 2277, covering absent environment-tail and facet-coordinate fields with
190 complete roots, twenty independent facet records and four containers.

The [legacy model checkpoint](legacy-model-layouts.md) admits contexts from
2130. Its 226 complete roots cover absent body/part metadata and component atom
lists, including inline and externally cached models and four containers.

The [legacy boundary checkpoint](legacy-boundary-layouts.md) admits contexts
from 2078. Its 370 complete roots and forty independent facet records cover
polygon attributes, body metadata, facet stamps and cone-culling fields.

The [legacy font checkpoint](legacy-font-layouts.md) admits contexts from 2011,
with 262 complete roots, explicit font records, solid-fill list boundaries,
twenty-eight independent facet records and four containers.

The [legacy zone checkpoint](legacy-zone-layouts.md) admits contexts from 1875,
covering exploded lists, component zones and textured planes with 262 roots,
2112 independent plane-transform fields and eight whole-document fixtures.

The [discarded edge-word checkpoint](legacy-discard-layouts.md) admits contexts
from 1857. Its 262 complete roots cover hidden-silhouette list introduction and
the seven-version extra edge word, with eight whole-document fixtures.

The [early text checkpoint](legacy-text-layouts.md) admits contexts from 1645,
covering omitted text tails, facet edge-ID flags and physical node words despite
native logical resets, with 262 complete roots and four containers.

The [early UI checkpoint](legacy-ui-layouts.md) admits contexts from 1286, with
442 complete roots covering secondary splitters, ghost transforms, font/sketch
suffixes, body metadata and facet edge-type arrays, plus eight containers.

The [early ghost-item checkpoint](legacy-ghost-layouts.md) admits contexts from
1215, with 406 complete roots covering ghost lists, hidden counts, part timestamps
and older facet arrays and planar words, plus eight whole-document fixtures.

The [early coordinate-scope checkpoint](legacy-scope-layouts.md) admits contexts
from 1180, with 154 complete roots proving 3D coordinates before the 2D header,
explicit Z edits, independent facets and eight whole-document fixtures.

The [early model-visual and light checkpoint](legacy-model-viz-layouts.md) admits
contexts from 989, with 962 complete roots covering model pointers, pane quality,
facet edge arrays and all five old-light layouts, plus eight containers.

The [early component checkpoint](legacy-component-layouts.md) admits contexts
from 903, with 402 complete roots covering direct assembly identity fields,
browser-word omission, polygon type records and logical view activity, plus
eight whole-document fixtures.

The [legacy curvature checkpoint](legacy-curvature-layouts.md) admits contexts
from 773, with 290 complete roots proving legacy array metadata and payload edits,
the calculated-colour transition and concurrent reconstruction, plus eight
whole-document fixtures.

The [root-owned item checkpoint](legacy-root-items-layouts.md) admits contexts
from 767, with 282 complete roots covering global and per-view lists, zero-view
and shared-bucket cases, extended counts and a single-word edit, plus eight
whole-document fixtures.

The [font-manager omission checkpoint](legacy-fontless-layouts.md) admits
contexts from 596, with 618 complete roots proving compact and extended MFC
relocation, omitted polygon line properties and explicit legacy array widths,
plus eight whole-document fixtures.

The [version-zero checkpoint](legacy-zero-layouts.md) removes the lower-version
cutoff, with 1394 complete roots covering the earliest appearance, material,
light, pane, edge and bucket-list predicates, plus eight whole-document fixtures.

The [named-view integration witnesses](named-branch-layouts.md) exercise later
section-plane and selection branches in 100 full roots and four whole files.
The [executed version-predicate audit](version-predicate-audit.md) records branch
observations across the owned full-root inventory and preserves remaining gaps.

The [stored-archive edit outcome](edit-history.md) returns exact coverage and
revision-bound structural field history with each validated edit. Geometry and
external-block checks are explicitly excluded from that operation's scope.

The [early PW scene regression](early-pw-scenes.md) covers generic thread-map keys
from 2771, including implicit wall keys, minimum record sizes, background suffix
gates and shared aliases. Its 130 complete roots and four owned containers also
pin the corrected reference-helper activity-word handling.
