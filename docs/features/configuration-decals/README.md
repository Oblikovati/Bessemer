# Configuration decals

Issue #54; behavior version 1. Sections 11.324–11.326 / C-366–C-368 bind
complete ordinary modern decal parameters and the condition-200 +6D8 map.
Data-structure and object-owned offsets remain distinct. The minimum property
body is 423 bytes; its signed index count and two direct packed-list counts
account for eight structural bytes. Parameter meanings remain unnamed.

Owned-map objects contain a tagged decal pointer and raw signed value. Map
associations contain a raw signed key, signed pointer count and an ordered
array of tagged decal pointers. This differs from scene associations, which
contain one pointer. Nonpositive counts consume no entries while retaining
the original word. Positive counts are bounded before iteration. Duplicate
keys, index words and values discarded by native runtime tree construction
remain present in their original wire order.

Nested lists admit the ten complete fresh classes verified under this helper
condition: unit components, component faces, face/vertex references, five
specific surface-ID classes and feature references. Nulls and mapped references
retain identity. The local fresh-class scope applies recursively, including
entity IDs inside fresh face references. An unverified fresh class stops before
its body even if a global reader exists elsewhere. Scoped tables restore on
success and failure. Map pointers retain their declared decal role for fresh
and cached objects.

Independent Source observations cover 360 property graphs, 35,238 primitive
reads, 60 direct maps and 70 complete owned CMgr streams. All ten scoped
fresh classes occur through complete containing objects. Six archive versions
straddle the boundary-reference suffix gate. Six scalar/count styles cover
signed extremes, exact IEEE bits, extended CStrings/counts, duplicate entries,
nulls and shared/recursive pointers. Ten independent material-word edit goldens
retain the rest of each complete manager. All fixture bytes regenerated from
an immutable Source copy.

Source replay reproduced the two native managers at 6,367 and 7,794 bytes,
including complete decal spans of 1,440 and 1,241 bytes. It used the actual
supported-manager reader without replacing reader functions. Two Source
regressions passed for body boundaries and nested association arrays. Thirteen
focused Bessemer tests, two scope-restoration tests and the eleven existing
PhotoWorks regressions passed. Three public document tests passed. Two independent native replays agreed
on every status and failure cause: 904 of 906 configurations passed, up from
902, with no regressions. Both decal-bearing parts were added. Source reached
exact EOF across all 83,146,672 resolved-feature bytes for these 904 accepted
configurations. Two native cases remain unsupported; the corpus is incomplete.
The full local `cargo xtask pr` gate passed: 2747 tests, including
1203 library tests and 277 doctests; three external-resource tests are
explicitly ignored in that gate.

This evidence does not establish complete configuration, part, assembly,
drawing or installed-project semantics. The maintainer humanizer skill was
unavailable; prose review remains with review.
