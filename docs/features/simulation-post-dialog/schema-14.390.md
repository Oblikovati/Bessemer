### 14.390 Dialog tail and exact retained stream framing

*Status: complete framing for all 224 retained Results dialog streams; semantic and installed-format completeness not claimed · Evidence: C-770*

`cosworks_post_dialog_layout.py` extends C-769 through the return of **DGXPOST CcwPostDlgParam::ReadFromStream, 1800953D0**, preserving each consumed span and rejecting trailing bytes by default. Fresh replay in `cosworks-post-dialog-native-evidence.json` reaches **exact EOF for 224 / 224 streams / 496979 bytes**, adding the remaining **78352 bytes** beyond the earlier plane/reference prefix. This is completion of the retained dialog framing inventory, not a claim of all-version, semantic, or all-installed-format completion. Candidate version associations remain explicit in the evidence.

The modern branch (>=800004) continues after reference container B30 with the following ordered additions. Native destination offsets are hexadecimal; each word is four bytes, raw64 is eight uninterpreted bytes, color is the established four-component/sixteen-byte reader, and text is the imported signed UTF-16 character-count grammar. These are object destinations, not file offsets.

| First version (inclusive) | Additional modern-tail reads, in wire order |
|---|---|
| 810015 | text B08 |
| 810020 | two raw64 BC8/BD0, two words BD8/BDC |
| 820008 | word BE4 |
| 820021 | word BEC |
| 820026 | word BF0, reference container BF8 |
| 830008 | moving-object words 48/50 |
| 830010 | word B10 |
| 830012 | word C44 |
| 830017 | words CD0/CD4/CDC |
| 830021 | word D04 |
| 830022 | word AEC |
| 830024 | word CE0 |
| 830026 | word 2C |
| 830030 | words D08/D0C |
| 850003 | words CFC/D00 |
| 850010 | word D38, raw64 D40 |
| 850017 | nine words D48 through D68, text D78 |
| 850021 | words D6C/D70/D74 |
| 850022 | signed sensor count and sensor records |
| 850029 | word 230 |
| 860007 | signed count DA4 and that many words |
| 870007 | word BC0, reference container B78 |
| 870009 | signed string-list count and list records |
| 870015 | word 234 |
| 880002 | word EB8 |
| 880003 | word EBC |
| 880005 | word EC0 |
| 880017 | raw64 EC8 |
| 880018 | word ED0 |
| 890004 | word EF4 |


**CDspViewMovingObjects::ReadFromStream = DGXPOST:1800ED960** consumes only its two four-byte words at version >830007. Thus the parent's historical branch (<800004) invokes this helper without consuming additional bytes, then reaches the common final path through default assignments. Such assignments, enum migrations and display preparation consume no wire bytes. The independent decoder now handles that historical control flow instead of declaring it modern plane framing; this branch has synthetic/static rather than retained native evidence here.

**CcwpProbeSensor::ReadFromStream = 18009C1B0** consumes name text and signed point count, then that many points. **CcwpProbeSensorPoint::ReadFromStream = 18009C3D0** consumes eleven four-byte words, ordered at destinations 8 through 30. Both require version >=810015, satisfied by the later parent sensor-list gate. Native replay contains **eight sensor records**. Counted integer arrays and sensor/point lists are independently bounded at 10000 items, and negative counts are rejected rather than interpreted as empty.

The string list at DD0 uses signed byte count followed by that many raw bytes per item for versions **870009 through 890610**. From **890611**, each item instead uses the signed UTF-16 character-count text helper. The raw-byte form is retained as bytes without inventing a terminator or character encoding. The retained dialog inventory has no nonzero entries for this list or the DA4 integer array; populated list framing is tested synthetically. Reference containers BF8/B78 reuse the C-769 record grammar and scoped archive decoder.

| First version (inclusive) | Common final reads, in wire order |
|---|---|
| 890008 | word EF8 |
| 890014 | word F04 |
| 890107 | words AE0/ADC |
| 890109 | equation delegate only when initial object word +10 equals 65 |
| 890118 | word F90 |
| 890206 | word F94 |
| 890212 | words F98/F9C, texts FA8/FA0 |
| 890307 | word FB0 |
| 890311 | words FD0/FD4/FB4, raw64 FC0, color FD8 |
| 890319 | word FE8 |
| 890504 | word FEC |
| 890507 | mesh-quality delegate |
| 890513 | word 1168 |


The common final reads above follow either historical or modern control flow. Their version thresholds are later than the historical branch, so no common final payload fields are added to that branch. Native defaults and early goto/return paths must not be mistaken for extra reads. F90 gates the subsequent F94-and-later chain, and F94 gates the subsequent later chain; the monotonically increasing thresholds in the table preserve those dependencies.

The conditional equation delegate is **cwPostCommon.dll:180019320**, imported by DGXPOST at **1802CC370** as `CcwEquationPlotParam::ReadFromStream`; C-770 verifies provider hash, exact import/export identity, function bytes and decompilations. It consumes text at 8, word at 10, texts at 18 and 20, signed variable count, and that many variable records. Each **CcwEquationVariable::ReadFromStream, 180019420**, consumes text, two words, and raw64. Minimum version is **890109**. The condition tests the initial four-byte field read into parent object +10 (stream offset 4), which must equal **65**. Preparing the equation for display consumes no bytes. No retained dialog uses this type; populated equation framing has static/synthetic evidence only.

**ReadFromStreamMeshQualityChkPlotParams = DGXPOST:180096590** consumes ten words at FFC, 1000, 1004, 1008, 1018, 1118, 111C, 1120, 1124, 1128; twelve interleaved pairs of raw64 (192 bytes); and twelve words (48 bytes). This base block is **280 bytes** from version **890507**. Version >=890508 adds raw64 at 1110; >=890510 adds word 112C; >=890511 adds word 1160. The parent separately adds word 1168 at >=890513. The interleaved pair loop is not two grouped arrays.

Fourteen tests pass, covering the explicit version-input contract, adjacent native gates, strict EOF and truncation, earlier composed prefixes, populated reference/sensor/equation records, and the string-list encoding transition. Independent text limits remain defensive: the imported helper's >1024 native clamping is rejected, and PostStyle's separate local buffer defaults to 512 characters in the decoder. Unsupported archive classes and invalid counts remain explicit failures. Field meanings, unretained variants, and coverage across all installed project formats still require further work; C-770 awards no global coverage credit.

Evidence: C-770.
