# Mold parting geometry

Feature issue: [#217](https://github.com/Oblikovati/Bessemer/issues/217).

The ordinary modern part codec needs the reference-surface base, the direct
edge-reference collection, the signed method and a directly serialized
`moMoldPartingGeomData_c`. The settings body has no tagged object header.
The reference is inventor-interface specification edition 0.3099, sections
11.1287 and 11.1290–11.1295.

The installed DLL's primary table selects a zero-return implementation for
virtual slot `+0xAB0`. This establishes inline inherited surface framing,
including populated sheet/body branches, through the existing strict kernel
codec. The byte locations and source identity are recorded in
[native-binding-investigation.json](native-binding-investigation.json).
This is static evidence; no target code was executed.

Selector `+0x170 == 1` invokes a pre-existing child at runtime member `+0x178`
directly. Its constructor binding and saved body remain unresolved. Neither a
tag nor a presence word may be inferred for that child. The measured legacy
selector-2 examples do not establish selector-1 framing. Implementation and
owned-document acceptance remain pending this investigation.

Native application acceptance, geometry acceptance, historical contexts and
whole-schema parity remain unproven. The humanizer skill is unavailable in
this environment; maintainer prose review remains outstanding.
