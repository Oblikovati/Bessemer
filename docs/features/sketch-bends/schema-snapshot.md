### 11.249 Sketch bends and stored sketch handles

*Status: B / ? — native binding and ten populated sketch bends in four complete containing roots verified; kind-7 owned data and raw option semantics remain partial.*

`moSketchBend_c` has primary vtable `0x3DC47278`, locator `0x3DC751D8` and serializer `0x3DB69880` in hash-bound installed `sldsmmu.dll`. Its native ancestry includes `moOneBend_c`. The serializer calls established One Bend serializer `0x3DB64540` before its derived fields. This inherited body includes the body feature, common bend, dimensions, references, transforms and kernel-body conditions described in 11.243. It is not replaced by the shorter bend-group base used by Jog.

For modern archives with inherited raw bend kind `+0xAC8 != 7`, the derived suffix is:

1. One tagged sketch pointer, used to restore the following handles' sketch associations. Native storing normally obtains it from embedded handle `+0xDF0`; for a special owner state it instead uses member `+0xE00`.
2. Three tagged entity-handle pointers, associated with embedded runtime handles `+0xDF0,+0xE18,+0xE40`, in that wire order.
3. One tagged curve-reference pointer associated with runtime member `+0xDE0`.
4. One i32 at runtime `+0xE70`.

The suffix occupies **14 bytes** when all five pointers use two-byte null/map tags. Fresh tagged objects add their own bodies. The embedded runtime handles do not imply direct untagged wire handles: the store side invokes object-pointer serialization for each. Native load reads temporary handle objects, assigns the containing sketch when nonnull, then assigns into embedded members. It destroys temporary second/third handles after assignment. The diagnostic reader preserves the stored objects and shared identity, rather than reproducing runtime object destruction or replacing the stored references with copied handles.

Historical loading reads an extra u8 before the sketch below version `0x674`, omits the curve reference below `0x659`, and omits the final word at or below `0x77E`. All three gates precede the supported modern range. They add no modern optional bytes. Raw word meanings, permissible handle combinations and native owner-state semantics remain unresolved.

Kind **7** takes a distinct path: immediately after the One Bend base it invokes the virtual serializer of the owned object at `+0xBE8`, then stores the shared suffix and an additional raw kernel body associated with `+0xBF0`. Loading removes a minimum kernel body after reading it. The owned class/body is not yet proven, so kind 7 stops explicitly **before consuming the owned data**. No substitute tag, guessed body width or EOF search is used. Inherited positive kernel-body branches retain their own explicit unsupported stops.

The installed corpus supplies **ten fresh Sketch Bend records**, all with bend kind 1 and final word 0, in four completed `moSMBaseFlange_c` containing roots. Their stored pointer classes are `sgSketch`, `sgPointHandle`, two `sgLineHandle` objects and a null curve reference. Containing-root sizes are **13,470**, **25,484**, **41,038** and **31,810 bytes**, totaling **111,802 bytes**. These sizes include their nested records and are not isolated sketch-bend payload lengths. Source-stream and containing-root SHA-256 values are retained in `sketch-bend-targeted-corpus.json`.

All **906** prior contexts retain their complete root prefixes unchanged. The audit gains **21 roots** across four configurations, reaching **25,505 complete roots**, **825 complete lists** and **81 explicit stops**. `samples/tutorial/api/fpf.sldprt` now ends exactly at **90,663 decoded bytes**. The other three cases advance to `moSMMiteredFlange_c` or `moFormToolInstance_c`. Totals are **747 exact-EOF lists / 62,085,577 decoded bytes**, with **78 additional complete lists retaining four-byte unknown tails**. Checkpoints 0.268 and 0.269 retain the comparison. Framing completion provides no whole-document semantic coverage credit.

Two focused tests verify stored pointer order and identity, signed scalar preservation, bounded truncation/context/kind-7 rejection, all ten native records, all four containing-root hashes and the exact new EOF or following unsupported class. Full part/assembly/drawing/all-installed-project semantic bit coverage remains open.

Validation: all **911 regression tests pass** (95.075 seconds); specification lint reports zero findings (473 headings, 96 anchors).

Evidence: C-291.
