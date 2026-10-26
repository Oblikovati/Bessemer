# Detail-folder interoperability

The next complete resolved-feature dependency is `moDetailFolder_c`, responsible
for 48 first failures at the current 388/906 checkpoint. Its ordinary body reads
the common feature, 18 packed direct lists, a compact transform, a typed BOM
pointer, 14 lists, a signed optional-bucket presence, the optional direct
gcBucket body, two further lists and a signed suffix. It does not include the
model-feature `+354` word. The complete reader remains pending.

The existing display bucket grammar is being reused. The first structural step
shared the envelope, modern and legacy polygon/edge bodies, line-width grammar
and typed object/transform context. Display callbacks retain object-map ownership,
field paths, version gates, coordinate dimensions and budgets. The legacy
polygon's typed discarded-edge reference remains an actual read. The nearly
full model dispatcher now delegates unit-class dispatch to the units module,
reducing it from 497 to 478 lines.

Fifty-one existing bucket and drawing tests passed with their independent fields,
forced reconstruction, malformed inputs and unchanged expected bytes. Full
`cargo xtask pr` passed without warnings: 413 library tests, integration suites
and 265 doctests, with three external gates ignored by default. Two explicit
906-case probes produced outcomes and failure causes identical to the merged
pre-refactor checkpoint: 388 accepted and 518 rejected. The broad parity gate
still fails. This structural step changes no supported wire behavior or public API.

Independent follow-up fixtures have been prepared outside production source:
absent bucket, 3D bucket, 2D bucket, named feature, independently expected name
edit and populated 2D bucket. The populated case checks restoration to 3D for
thread/halo lists and a tagged thread key. Their source reads reach EOF; the
primitive inventories and all-prefix checks will be retained with the actual
model-reader implementation. They are not Bessemer acceptance claims.

Archive version and runtime file version are separate. The approved reference's
`dl_ui._file_version` defaults to runtime 19000; it does not infer that value
from the model wire version. The ordinary model context already declares that
current runtime dialect. Display contexts keep their explicit file-version input.

The next structural dependency is shared display-item parsing, including legacy
text character counts and compact transforms. After it is shared, connect the
model bucket adapter and complete detail-folder reader, preserve the existing
display fixtures, and prove owned construction/editing through EOF. Keep all
unsupported dependencies explicit. Native application and geometry acceptance
remain separate from stored-grammar validation.

This work advances issue #54 and the broader #2 goal. No dependency, version,
release, push or PR was introduced by this checkpoint. The humanizer skill is
unavailable in this session; its prose review remains with the maintainer.
