### 11.1283 Parting-line feature-data base and direct edge references

*Status: measured legacy research boundary — Evidence: C-2967*

The native feature-data import resolves to `moFeatureData_c::Serialize` at `0x4B50C220`; the exact export table binding is verified. Its generic owner reference points to the enclosing, still-partial parting-line feature and is a valid parent back-reference. Direct member `+0x78` then contains a packed count and object references (44 `moCompEdge` entries in the first sample). The mapped schema1 `moDirectionSpec_c` header follows; its body remains unread at this checkpoint. The three advancing cursors are469362,497016 and475380.

Evidence `part-legacy-parting-line-data-prefix-evidence.json` frames34304 additional bytes with17568 charged unknown across three advancing streams. All four cohort rows and53 other first-child exclusions remain represented; aircraft sketch C0 stays deferred at32457. No target native code executes, production readers are unchanged, and coverage credit is zero. These measurements do not establish full part-schema completion; part work retains priority over assemblies.

### 11.1284 Parting-line direction and data collections

*Status: measured legacy research boundary — Evidence: C-2968*

The direction serializer preserves its optional object array, signed word `+0x10`, vector-presence byte and gated vectors/point, then signed word `+0x14`. The first sample has one `moPlaneRef_w`, word0, no first vector and final word3. Data then stores float64 `+0x18` (0.01745329251994333 in the first sample), signed word `+0x1D0`, tagged component body `+0x28`, direct collection `+0xC0`, words `+0x1D8` and `+0x1E4`, and direct collection `+0x30`. The measured component body is `moCompAtomBody_c`; the first sample has44 component-edge references and an empty `+0x30` collection. Direct collection `+0x108` starts with44 entries; the first `moMPLEdgeElem` class header is consumed, leaving its body unread. No concrete native container type is inferred from count framing alone. Cursors are481126,508780 and487332.

Exact own moMoldPartingLineData_c methods add ten scoped native property associations to the existing fields in 11.1283-11.1285. The complete retained 1298-byte canonical serializer and selected typed codecs bind the same original class; the associations add no archive fields.

| Member | Native property | Qualified representation |
|---|---|---|
| 0x18 | Angle | Raw binary64 getter/setter and archive double |
| 0x28 | CompAtomBody | Declared moCompAtomBody_c pointer getter/direct setter and tagged codec |
| 0x30 | ConstructionList | Declared const suObList reference getter and existing direct virtual codec |
| 0x78 | ListOfEdges | Declared const suObList reference getter and existing direct virtual codec |
| 0x108 | TopolSelList | Declared suObList reference getter and existing direct virtual codec |
| 0x1D8 | SplitStraddleFacesFlag | Full32 integer getter/setter and signed32 archive word |
| 0x1E4 | SavePlusMinusBoundaryCurveFlag | Full32 integer getter/setter and signed32 archive word |
| 0x1E0 | CoreCavityFlag | Full32 integer getter/setter and signed32 archive word |
| 0x1DC | SplitAtAngleFlag | Full32 integer getter/setter and signed32 archive word |
| 0x1E8 | NoDraftAnalysisFlag | Full32 integer setter and signed32 archive word; no getter asserted |

Selected STORE emits these suffix groups without the corresponding LOAD version tests. LOAD independently queries current unsigned effective versions: 0x28/0x1D8/0x1E4 require at least 0xA10, ConstructionList 0xA14, TopolSelList 0xA3F, CoreCavityFlag 0xA9B, SplitAtAngleFlag 0xAB4, and NoDraftAnalysisFlag 0xAB9. The reached older branches assign 1 to CoreCavityFlag and 0 to SplitAtAngleFlag/NoDraftAnalysisFlag. The earlier 0xA10 skip is not an explicit reset. ListOfEdges is serialized before the mode branch. Older list reconstruction/conversion remains opaque; these checkpoints do not guarantee complete state preservation or add old-context reader admission.

The list contracts are declared native reference APIs, not immutable copies, ownership guarantees or all-item type restrictions. Raw words retain all bits despite Flag names; no closed Boolean/enum domain, angle unit, coordinate frame or geometric outcome follows. Raw 0x1D0, collections 0xC0/0x188 and direction 0x10 remain unassigned. Callback/provider effects and current/file identity remain qualified. Evidence: C-3556.

Evidence `part-legacy-parting-line-direction-data-evidence.json` frames35480 additional bytes with18138 charged unknown across three advancing streams. All four cohort rows and53 other first-child exclusions remain represented; aircraft sketch C0 stays deferred at32457. No target native code executes, production readers are unchanged, and coverage credit is zero. These measurements do not establish full part-schema completion; part work retains priority over assemblies.

### 11.1285 Parting-line element bodies and data suffix

*Status: measured legacy research boundary — Evidence: C-2969*

Native serializer `0x18019C7D0` reads tagged references at offsets `+0x08`, `+0x10`, `+0x18`, `+0x28` and split-info `+0x30`, followed by signed word `+0x20`. `moMPLEdgeElem` serializer `0x18019C910` adds signed word `+0x70`. Split-info serializer `0x18019C980` reads a tagged component-face reference and signed word `+0x70`. All44 edge elements in the first sample complete. Direct collection `+0x188` is empty; the gated data suffix words `+0x1E0`, `+0x1DC`, `+0x1E8` are1,0,0. This completes the measured nested data object. The enclosing feature then stores raw word `+0xAF0` and signed body count; the measured count is1. Cursors507832,535486 and514226 stop before that body.

Evidence `part-legacy-parting-line-elements-tail-evidence.json` frames80306 additional bytes with37704 charged unknown across three advancing streams. All four cohort rows and53 other first-child exclusions remain represented; aircraft sketch C0 stays deferred at32457. No target native code executes, production readers are unchanged, and coverage credit is zero. These measurements do not establish full part-schema completion; part work retains priority over assemblies.

### 11.1286 Parting-line compressed kernel body and feature boundary

*Status: measured legacy research boundary — Evidence: C-2970*

Each measured body has suppression byte0, signed length, kernel selector0 and the compressed Parasolid payload. The first length is11933; its11932-byte payload has the known compression GUID, chunk size11928 and one zlib block (11896 compressed bytes,22819 inflated bytes). Size checks, zlib completion and checksum, a complete XT partition parse with one BODY node and zero trailing bytes all pass; the first partition contains431 nodes. Evidence pins the installed base schema path and SHA256 as well as compressed/inflated hashes. The complete compressed payload remains charged unknown: structural validation and the separate estimated unknown count do not establish all semantic meanings. Final signed feature words `+0xAF8` and `+0xAFC` are -1 and0 in the first sample. The measured parting-line feature is complete at519778,547447 and526206; the next unconsumed class is `moMoldPartingGeom_c` in each advancing stream.

Evidence `part-legacy-parting-line-kernel-finish-evidence.json` frames35887 additional bytes with35869 charged unknown across three advancing streams. All four cohort rows and53 other first-child exclusions remain represented; aircraft sketch C0 stays deferred at32457. No target native code executes, production readers are unchanged, and coverage credit is zero. These measurements do not establish full part-schema completion; part work retains priority over assemblies.
