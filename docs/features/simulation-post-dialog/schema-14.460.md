### 14.460 Standalone legacy DspPostInfo and DspPostStyle carriers

*Status:104 standalone records frame exactly with caller-compatible versions; sixteen record/context exceptions remain separate — Evidence: C-840*

The below700335 branch of dispatcher **1814AABE0** opens two additional streams relative to the legacy plot storage: **DspPostStyle** (literal **1820660F0**) and **DspPostInfo** (**182066110**). They are independent of the dynamically named plot body in14.459. Exact import/export matching binds their readers to **DGXPOST.dll:180044AE0** (`CDspPostStyle::ReadFromStream`) and **180044950** (`CDspPostInfo::ReadFromStream`). These are the same recovered helpers used inside newer dialog records, but this separate storage/caller binding is verified before reusing their parsers.

PostInfo retains the **460-byte raw-block** grammar of14.387, with its extra temporary word/character-count text only at exact version700002. Its internal fixed-block meanings remain unresolved. PostStyle retains the ordered raw-word/color/legend/text grammar of14.388, including independent color reads through DspOGL and versioned legend/style suffixes. Character counts and fixed blocks do not become byte-counted CwPersistStream strings or tagged object records merely because this is a standalone stream.

C-840 rechecks the caller/provider hashes, exact stream literals, both imported/exported symbols, reader/legend/decompilation byte identities and the existing DspOGL binding. Fresh extraction selects only direct legacy `Cosworks/<study>/<category>/<plot>/DspPostInfo` or `DspPostStyle` paths and verifies their original outer/nested payload hashes. Both primary and coexisting legacy root versions are retained as candidates, without silently replacing one with a lower number to fit.

| Stream | Retained records / bytes | Exact helper fits | Fits with below700335 caller-compatible version | Caller-compatible bytes |
|---|---|---:|---:|---:|
| DspPostInfo |60 /27,600 |60 |52 |23,920 |
| DspPostStyle |60 /17,040 |52 |52 |14,768 |
| Total |**120 /44,640** |**112** |**104** |**38,688** |

Every PostInfo sample is460 bytes; every PostStyle sample is284 bytes. The eight additional PostInfo helper fits have no retained candidate satisfying the standalone caller gate; their version-independent block width does not prove runtime selection of that obsolete route. Eight style streams fail the helper under their retained candidates and likewise have no caller-compatible completion. The audit preserves all alternate attempts, endpoints, original versions, raw successful spans and failed-prefix errors. No forced pre700330 style version or opaque skipping is accepted.

A companion's exact framing does not complete its neighboring named plot body or any other stream. In particular, these results are not used to erase the twelve shorter named-plot failures from14.459. Standalone PostInfo raw-block preservation also does not resolve its individual fields. The newly bound carrier occurrences are distinct from the embedded blocks already counted in the newer dialog audits; totals are not merged as duplicate semantic credit.

The five existing focused PostInfo/PostStyle tests pass, covering historical gates, raw block/legend/color framing, character counts, truncation and standalone/embedded endpoint contracts. `cosworks-legacy-post-blocks-native-evidence.json` records the new carrier associations and helper-versus-caller distinctions explicitly. Runtime study/object activation, the remaining exceptions, physical/UI meanings, native writer equivalence and the full installed-format objective remain unfinished. No target code executes and global semantic coverage credit remains zero.

Evidence: C-840.
