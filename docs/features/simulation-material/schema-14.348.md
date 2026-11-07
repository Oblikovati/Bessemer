### 14.348 Shared material replay across solid, shell, beam and mass contexts

*Status: all 1226 retained study-manager Material occurrences complete, including populated point tables; AWDocument and broader semantics remain separate · Evidence: C-724*

The material grammar from 14.347 is not extended merely by assuming that every stream named `Material` has the same format. A static executable-section search identifies **15 instruction-validated direct calls** to **0x1816E15A0**. Four additional recovered storage routines, **0x180A32600**, **0x180A5BA00**, **0x181345A50** and **0x1813ACC90**, explicitly open the verified Material literal **0x181F50D20** and delegate to that serializer. C-724 hashes their installed byte spans/decompilations, checks each discovered direct-call opcode/displacement and rematches the shared material/helper spans to C-723. This bounded direct-call search is not an exhaustive dynamic call graph or a proof of every runtime handler selection.

**Native scope.** Fresh C-590 extraction covers every retained path matching **(Cosworks|CosmosXpress)/StudyManager/.../Material**. It checks each outer envelope and nested stream SHA-256 and tries the corresponding storage root's retained primary/legacy version candidates, plus the existing audit's per-study candidate association where available. No candidate is inferred from payload length or substituted to eliminate failures.

| Manager ancestry | Complete / inventoried occurrences | Complete bytes |
|---|---:|---:|
| SolidManager | **837 / 837** | **2946340** |
| ShellManager | **237 / 237** | **802740** |
| BeamManager | **150 / 150** | **522038** |
| MassElementManager | **2 / 2** | **7096** |
| Total | **1226 / 1226** | **4278214** |

The scope includes **1225 Cosworks** and **one CosmosXpress** study-manager occurrence. Every occurrence reaches exact EOF for at least one retained candidate with complete reconstruction hash and consumed-span byte count equal to the original payload. Both prior mass-child identities and hashes remain complete. No sibling Property body or manager container is included in these byte totals.

**Populated native point tables.** Taking one successful candidate per occurrence for counting yields **8712 material-property records** and **18390 delegated table bodies**. Of these, **18 property tables** and **38 delegated tables** have nonempty point arrays. This advances the populated-point-array evidence beyond the two empty-table mass-child samples in C-723. It validates the applicable interleaved-double-pair framing, not the physical meaning of either coordinate, interpolation/extrapolation policy or material-model evaluation.

There are **1230 successful candidate attempts** and **67 failed alternate attempts** retained. Four occurrences therefore admit more than one successful argument. The first successful candidate used for the aggregate record totals is an audit counting convention, not proof that the native application selects that version. Successful representative arguments span **800078 through 891107**; unsampled gates and historical forms retain their static/synthetic status.

**Distinct AWDocument carrier.** The inventory also contains **six CosmosXpress/AWDocument/Material streams / 150 bytes**. They lie outside the study-manager scope and are recorded separately, without applying this parser or awarding completion. The basename alone does not tie that carrier to the shared serializer. Other unsampled material carriers likewise remain outside this native claim.

`cosworks_material_corpus_audit.py` asserts the exact 1226/4278214 scope, preserves previous successes and retains per-occurrence candidates, end/reconstruction hashes, span counts, property/delegate counts and populated-table counts in `cosworks-material-corpus-native-evidence.json`. Its four focused material tests also pass. Complete attempts are summarized without duplicating every raw span in this larger artifact; the hashed parser/source checkpoints and original native payload identities remain explicit. No target code or native writer executes.

This completes framing for all retained study-manager Material occurrences in C-590. It does not complete the AWDocument carrier, whole solid/shell/beam/mass objects, units/model semantics, every historical/context form, native writer equivalence or the full installed-format denominator. Global semantic byte-coverage credit remains zero.

Evidence: C-724.
