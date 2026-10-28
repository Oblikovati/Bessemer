# Weld symbols

Feature #54 implements `moWeldSymbol_c`, `moCosmeticWeldBeadSymbol_c`,
`moSFSymbol_c` and `moCosmeticWeldBeadAttachPoint_c` from 11.345 / C-387,
11.346 / C-388 and the supported composition in 11.347 / C-389.
Behavior version: `C388-weld-symbol-v1`.

The common weld reuses the complete detail-item base, then retains eighteen
CStrings, raw double/vector/word fields, five CStrings, two words, two finish
pointers and three directly owned dimension texts at +188/+2E0/+438. Native
constructor `0x4C827600` and the typed constructor callback prove those exact
text instances. They add no object headers, map slots or serialized array count.
Actual fonts referenced from the direct bodies retain normal shared identities.
The cosmetic weld adds exactly forty bytes: four signed words, two raw doubles
and two signed words.

Surface finish reuses the detail base, two words, ten CStrings, a word, one direct
dimension text and another word. Strictly above `0x42DE` it adds a signed word
and raw double; strictly above `0x46BF` it adds two CStrings and two signed words.
Runtime prefix extraction and text initialization do not add wire fields or
justify normalizing stored text. The cosmetic attachment reuses the complete
attachment base, then reads one typed component-feature pointer and raw double.
Native RTTI and complete serializers bind the exact detail/attachment subtype
families for both fresh and cached pointers; unrelated classes remain rejected.

Independent fixtures cover 216 complete graphs, 57,278 primitive observations
and 28 owned streams. Nine versions cover finish comparisons and inherited
format gates; six identity styles include fresh/cached subtype parents, repeated
roots, shared finish symbols, shared fonts and stored nulls. Populated direct texts
retain nested point/curve arrays. Raw numeric cases retain signed zero, signaling
NaN payloads, infinities and signed-word extrema. Four independent edit goldens
change only the selected signed word or attachment double.

Native empty-body closed forms are 503 bytes for a weld and 543 for a cosmetic
weld, with three 129-byte direct texts. Empty surface finish is 190, 202 or 212
bytes across its two gates; an empty cosmetic attachment is 113 bytes. These are
fixture-specific empty cases, not fixed extents for populated bodies. All four
bare empty objects allocate only their outer class/object slots.

Run `cargo test --lib weld_symbol_tests`, `cargo test --test resolved_weld_symbols`
and `cargo xtask pr`. Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/weld-symbols`.
The shared [detail-body builder](builder-reuse.md) retains every earlier baseline.
Other weld-tree dependencies, historical/internal contexts, scalar meanings,
runtime repair, geometry validity, application acceptance and full schema parity
remain incomplete. Humanizer is unavailable; maintainer prose review remains open.

The final local PR gate passed 883 library tests, all integrations and 274
doctests. Two fresh native processes matched all 906 outcomes and causes,
retaining every one of the 818 complete reconstructions. Case 547 now reaches
`moAttachPointData_c`; cases 615 and 905 reach `moParametricPoint_c`. None receives
completion credit. All 88 remaining inputs fail explicitly, so both aggregate
corpus commands exited 101. Independent readers verified EOF and four-stream
hashes for all 818 accepted configurations, 91 recorded prefix roots across
three source graphs, twelve native symbol objects and 29 direct text instances.
