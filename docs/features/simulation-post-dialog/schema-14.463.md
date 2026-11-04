### 14.463 PostInfo ContourType header and preference conversion

*Status: header08 enum identity and conversion policy bound; contour display meanings and other header roles remain incomplete — Evidence: C-843*

PostInfo header word **+08** is a stored **ContourType** enum, rather than an untyped preference integer. `GetPreferences_Adv`, **DGXPOST:180044330**, passes that member to exported **CPostGlob::ConstTrans(ContourType)** at **1800EF350** and writes the resulting integer into `plotAdvPreference` destination+58. Conversely, `SetDefaultOptionByPreferences`, **180045000**, passes a preference integer from source+1580 to exported **ConstTrans(int,ContourType*)** at **1800EF3B0**, with destination PostInfo+08. Exact exported enum-bearing signatures and native call/assignment instructions establish that member identity.

The two leaf conversions are deliberately not modeled as a bijection:

| Stored ContourType input | Preference integer result |
|---|---:|
|0 |0 |
|1 |1 |
|4 |2 |
|5 or6 |3 |
|Other raw integer |6 |

| Preference integer input | ContourType result |
|---|---:|
|0 |0 |
|1 |1 |
|2 |4 |
|Other raw integer, including3 |6 |

Therefore contour5 converted to preference3 and back becomes6. This is native conversion behavior, not permission to rewrite stored5 into6 or name unknown enum values. The independent decoder keeps the signed/raw stored word and adds only a derived **preference_integer_view**. Actual visual names or rendering styles for these contour codes remain unresolved.

The default-setting method has an additional runtime override: if an object at PostInfo **+1E8** exists and its type word+10 is **22,57 or70**, it assigns contour4 directly instead of converting the preference. The installed bit test uses a bounded offset from22 and mask **0x1000800000001**, whose selected offsets are0/35/48. This pointer/type context lies outside the serialized460-byte block; default-state output cannot be inferred from the stored header alone. `cosworks_post_contour_policy.py` models the bounded integer conversion and explicit override inputs without reading runtime objects or changing wire data.

`DiffOGLRender`, **180043FF0**, compares PostInfo words **04,08,10,0C** and returns whether any differ. It does not compare active-count fields14/18 or the twelve cut primitives. This narrows that named comparison routine's behavior but does not prove those are the only fields that affect rendering throughout the application, nor assign meanings to the remaining raw words.

C-843 rechecks both conversion exports and complete leaf branch ranges, including targets beyond the first return, the named preference/default methods, rendering comparison and native payload identities. All **60 retained fixed blocks** store contour value **6**, yielding preference view **3**; original raw6 remains unchanged. The prior52 caller-compatible versus eight helper-only blocks remain separately qualified. No new stored bytes or global semantic credit are added by annotation.

Ten tests pass, including three new tests for forward/reverse mappings and their noninvertibility, exact runtime override codes and adjacent nonmatching values, and raw native-field preservation. Unknown contour code meanings, other header semantics, runtime preference state, writer equivalence and full installed-format completeness remain unfinished. No target code executes.

Evidence: C-843.
