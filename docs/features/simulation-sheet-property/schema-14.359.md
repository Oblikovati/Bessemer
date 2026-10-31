### 14.359 Embedded constant-surface wrapper and sheet Property progress

*Status: 225 of 238 retained inherited sheet Property streams complete · Evidence: C-736*

The installed model provider binds `moConstSurfRef_w` to serializer `4B90DBB0`, which calls the known reference-wrapper/base-solid serializer `4B90DB10` before reading a signed geometry-presence word. For supported embedded versions, the base grammar is the existing common wrapper followed by word +A8; a nonzero geometry-presence word remains an explicit kernel-body/attribute boundary. A zero presence ends this class body. This is a native-bound reuse of a shared grammar, not a class-name heuristic. The independently bound `moCompRefSurface_c` serializer `4BBBA630` delegates to component-reference base `4BBBA440` then reads one four-byte word at +100. The local reader uses the bounded component-holder base grammar plus this word.

`moCompSurfaceBody_c` primary slot +28 at table `4CE941E8` points to `4BBBBAD0`. Its serializer delegates to shared body grammar `4BBB7B50`, then only sets an in-memory flag on loading; that flag assignment consumes no bytes. The local embedded reader therefore reuses the established component-body grammar: component-reference prefix, face-reference object, bounded word array, then three words. It does not append the unrelated boundary-reference alias fields.

C-736 freshly checks installed model-module hashes, retained native function bytes for the constant-surface/component-reference serializers, primary pointers, and source evidence from thicken/configuration-body-state analysis. `cosworks_embedded_const_surface.py` extends the archive registry locally; ordinary model-context checks remain intact, and nonzero kernel geometry or unsupported older versions still stop explicitly.

Fresh replay of the same **238 inherited sheet Property occurrences / 186372 bytes** now completes **225 / 165557 bytes** exactly. This adds **111 complete occurrences / 148641 bytes** over C-735 and preserves every earlier successful candidate's version, endpoint, and reconstruction hash. There are 243 candidate attempts, 227 complete. The three former text-failure occurrences now have a complete alternative attempt after archive dependencies are enabled; failed alternatives remain retained and are not themselves declared valid. Framing success still does not prove a universal native study-index mapping.

The **13 remaining occurrences / 20815 bytes** first stop at `moCompPlaneSectionBody_c` (8) or the explicit older-version constant-surface wrapper guard (5). These are first boundaries only, not proof that their remaining bytes have one unknown field. C-736 supersedes the 124-occurrence remaining-framing count in 14.358 while retaining that checkpoint. Seven tests pass, including explicit zero/nonzero kernel-presence behavior and the component-reference suffix boundary, alongside inherited Property and embedded ID tests.

No target executes. Older wrapper profiles, plane-section components, the separate six SurfaceBody tail mismatches, auxiliary records, and field semantics remain open. Global semantic coverage credit remains zero and full installed-format coverage remains incomplete.

Evidence: C-736.
