### 14.345 BeamManager and MassElementManager Property headers

*Status: installed manager bindings and all 514 retained direct headers complete; nested folders/object schemas remain partial · Evidence: C-721*

The study reader opens **BeamManager** at literal **0x1820643C0** and invokes the manager object's storage slot **+8**. Study construction stores the result of **0x180A5F050** at study member **+0x168**. That manager constructor installs table **0x181F556C0**, whose slot **0x181F556C8** selects **0x180A63A70**. The independently opened **MassElementManager** literal is **0x182064450**; study construction stores constructor **0x1810CA210**'s result at **+0x190**, and table **0x18202FEF0** slot **0x18202FEF8** selects **0x1810CEEC0**. C-721 binds the installed constructor/reader spans, vtable slots, named literals and thresholds to the verified module hash.

**BeamManager/Property.** The read-mode branch opens **Property** and consumes signed four-byte values in this exact order:

| Condition | Runtime destination |
|---|---|
| Always | Manager **+0x14C** |
| **v >= 810024** | Manager **+0x168** |
| **v >= 820006** | Manager **+0x148** |

The two threshold doubles at **0x181F55888** and **0x181F55890** are verified as **810024** and **820006**. The complete header therefore occupies **4**, **8** or **12 bytes**, respectively. Destination order is not ascending: **+0x168 precedes +0x148**. The current storing branch emits all three four-byte words. Their counter/identifier/physical meanings are not assigned merely from their position.

**MassElementManager/Property.** The read-mode branch opens **Property** and consumes one signed four-byte value at manager **+0x28**. The corresponding insertion uses the four-byte `long` overload. No version gate is applied to this header in the recovered function. Its value is retained as raw state, not asserted to equal the number of nested objects.

`solidworks/cosworkspersist.py::read_manager_property(data, kind, version=None)` supports these two exact manager names. BeamManager requires an explicit nonnegative signed-i32 version; MassElementManager's fixed header requires no version inference. The result retains the original bytes, ordered runtime destinations and signed values. Each selected layout requires exact input size, so missing/trailing bytes and unsupported kinds fail explicitly. This API does not decode every stream named `Property`.

**Native headers.** Fresh C-590 envelope/nested hash checks and signed-word reconstruction complete all matching direct manager headers across the retained ancestries:

| Manager | Occurrences | Bytes | Observed header sizes |
|---|---:|---:|---|
| BeamManager | **353 / 353** | **4228** | **351** twelve-byte and **two** eight-byte headers |
| MassElementManager | **161 / 161** | **644** | All four bytes |

Beam parsing uses retained root primary/legacy candidates and, where available, the audit's per-study candidate association. As elsewhere, candidate success does not prove runtime version dispatch or numeric-suffix/index identity. The four-byte BeamManager profile is established by code and boundary tests, without a native fixture here.

**Nested contents remain separate.** Beam storage handling also opens **BeamFolders**, **Beams** and **JointGroups**. The BeamFolders read branch is gated at **v >= 820007**, using installed double **0x181F553C0**. Its stored child objects and the other beam bodies are not defined by the small Property header. MassElementManager opens **MassElements**, enumerates child storages, constructs each mass object through **0x1810C9F40**, and delegates to its virtual slot **+0x260**. The writer formats child names using **MassElement %d**, starting at one. The loader's enumeration and object bodies remain incomplete here; it is not justified to reconstruct that storage solely from the Property word or a guessed sequence count.

Three focused tests pass for both Beam version boundaries, destination order, signed values, fixed mass header, truncated/trailing inputs, unsupported kinds and version bounds. The existing twelve primitive decoder tests also pass. C-721 retains all **514 payloads / 4872 bytes**, candidate results, values, installed bindings and source/test hashes in `cosworks-manager-property-native-evidence.json`. No target code, native storage enumeration or writer executes. Full manager bodies, field meanings, historical/context variants and the all-installed-format goal remain incomplete; global semantic byte-coverage credit stays zero.

Evidence: C-721.
