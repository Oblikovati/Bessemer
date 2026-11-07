# Resumed inline facets before the object interval

Reference baseline: specification edition 0.239, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 130 complete part and assembly roots cover versions 2596, 2597, 2605 and
2606, with 28105 independently expected traced fields. Eight roots at 2596
contain two complete inline facets, covering sparse and populated arrays in
both 2D and 3D view contexts. Separate independently built facet records check
all typed facet contents and forced bytes, including legacy triangles and old
tolerance records. The remaining roots retain empty and externally cached
models across the boundary.

Inline facets are documented at 2596. Versions 2597 through 3195 instead use an
unresolved facet-object representation; encountering that payload still returns
an unsupported error. Empty and cached roots do not contain that representation
and remain supported. Explicit component identifiers determine whether an
assembly stores inline facets or external references.

Independent edited fixtures change facet stamps together with polygon, text,
line and edge fields. Tests check exact reconstruction, edits, authentic reverts,
input isolation, truncation rejection and concurrent constructors. Four owned
containers cover part and assembly streams before and after edits. Replacement
and reopen preserve unrelated streams and resolve the declared display context
without a face directory. Process verifiers compare roots and whole containers
in two fresh processes.

This checkpoint lowers the admitted archive context to 2596. Earlier documented
branches and unresolved representations remain tracked by #28. Byte fidelity
alone does not establish geometric validity or native application acceptance.
