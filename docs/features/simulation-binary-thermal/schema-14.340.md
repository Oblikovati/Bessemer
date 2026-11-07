### 14.340 Embedded component transforms and complete retained thermal framing

*Status: all 68 retained binary thermal streams complete; embedded class/context scope and semantic limits remain explicit · Evidence: C-716*

C-716 supplies the missing embedded dependencies from 14.339 and supersedes its **60/68** partial native result. It preserves all previous successful stream identities and payload hashes, adds the eight former stops, and uses only the same retained candidate-version sets. No unstored version, unknown-body skip or ordinary model-context relaxation is introduced.

**moCompXFormWrapper_c.** Installed model RTTI binds primary table **0x4D17F820**, serialization slot **0x4D17F848**, to **0x4C27B490**. That serializer calls common reference-wrapper body **0x4C2842D0**, then restores a direct **mgXform_c** at runtime **+0xA8**, followed by a second at **+0x130** when archive version **>0xB36**. The explicit embedded profile remains **4307–19000**, so both direct transforms are present throughout that supported range. They are not tagged transform-object pointers or presence-word-prefixed wrapper suffixes. Optional transforms already inside the common wrapper retain their own gates and are separate fields.

The embedded reader uses the already established explicit **0x4000** archive-helper context and historical common-wrapper gates. It does not call the ordinary model decoder with fabricated `ordinary_external_xml` state. Transform framing is the direct mgXform grammar from 11.6: a matrix-presence byte and optional matrix, translation, scale, flags, and any flag-selected payload. Matrix/physical interpretation and unsupported context paths retain their existing limits.

**moCompHolder_c.** Continuing through the transform wrapper exposes a tagged component-holder dependency. Its primary table **0x4CD931F8** selects slot target **0x4BBB9F90**, a verified five-byte relative jump to **0x4BBBA690**. This is the component-reference base serializer, not the component-feature serializer. The scoped reader therefore consumes the historical component-reference body only and does not append feature ID/time words. Direct old-reference, reference-information and transform sub-bodies remain explicit stops when their unsupported presence gates are set; no unknown optional body is skipped.

`cosworks_embedded_transform_wrapper.py` and `cosworks_embedded_component_holder.py` register these bodies only through an explicit embedded-archive extension. The shared callback accepts an additive `extra_readers` argument; its default class registry and legacy dispatch remain unchanged. Original model context checks are preserved.

**Native scope and results.** C-716 rechecks installed model module SHA-256 **346ad211760be406f587f338293d1793386518dddfc1bd461ac841dd0598487c**, the two RTTI slots, the holder jump bytes, resolved serializer/common-base byte spans and decompilation hashes. It freshly replays the same hash-checked C-590 thermal streams as C-715:

| Family | Complete streams | Complete bytes |
|---|---:|---:|
| Temperature | **39 / 39** | **178666** |
| Convection | **13 / 13** | **54408** |
| HeatFlux | **3 / 3** | **4415** |
| HeatPower | **8 / 8** | **19004** |
| Radiation | **5 / 5** | **11327** |

All **68 streams / 267820 bytes** reach exact EOF. The eight newly completed streams account for **24244 bytes**. Across one successful attempt per stream, **19 component-transform wrappers** are consumed at archive versions **6000 (4)**, **10000 (3)**, **11000 (2)** and **14000 (10)**. Every observed pair of direct transform suffixes consumes **68 bytes**, the two minimal no-matrix forms; populated optional matrix/perspective suffixes are not claimed as new native fixtures here.

There are **68 successful candidate attempts** and **seven failed alternate attempts** retained at legacy-reference framing. The earlier four legacy-reference furthest stops did not require changing that grammar: with the missing modern archive classes supported, another already retained candidate now completes each stream. This does not prove the native runtime selected that candidate, nor that the failed legacy candidate denotes a corrupt stream.

Three transform-wrapper tests pass for scoped versions, exact two-transform boundaries, every truncation and old-context rejection. Three holder tests pass for base-only signed fields, every truncation and unsupported-context/optional guards. The existing four optional-load regression tests also pass. C-716 retains the full native replay, per-wrapper spans, class bindings and source/test hashes in `cosworks-binary-thermal-transform-native-evidence.json`.

This closes binary thermal framing for the exact inventoried ancestries in 14.339. It does not complete physical field meanings, every historical/context branch, native writer equivalence, unrelated managers/ancestries or the full installed-format denominator. No target code executes and global semantic byte-coverage credit remains zero.

Evidence: C-716.
