# Font suffixes, solid fills and temporary facet words

Reference baseline: specification edition 0.246, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 262 complete part and assembly roots cover versions 2011, 2012, 2015, 2016,
2032, 2033 and 2078, with 79851 independently expected traced fields. Font item
kinds 10 and 14 omit their final `unknown_30` double through 2015 (`0x7DF`).
Their preceding double remains stored. Each root adds both kinds with ANSI and
Unicode names; the suffix samples include negative zero and a NaN with a
noncanonical payload. Edited fixtures change font names and both stored doubles
where present. Symbol records retain their own unchanged double layout.

Solid-fill polygon lists are absent through 2032 (`0x7F0`). The generator observes
each bucket's polygon-list calls and identifies the exact list object returned
as `solid_fill`, before removing its complete physical interval. Empty lists and
populated 2D and 3D lists are covered. Primitive list removals introduce no MFC
class or object slots. Every final root is parsed to its end at its declared
version; earlier fixture groups remain unchanged.

Fifty-six inline part and assembly roots each contain two complete facets.
The temporary discarded facet word is present before 2012 (`0x7DC`) in this
interval. Twenty-eight independent facet records check complete typed contents
and forced bytes, including that word. Existing stamp, cone-culling, body,
polygon, component, environment, text and material boundaries remain independent.

Composite edits change fonts together with text, polygons, edges and facet
counts. Tests compare independently built edited roots, authentic reverts and
unchanged sources. Incompatible contexts reject missing font/fill fields or an
extra old facet word before publication. Truncation tests reject partial roots.
Complete externally cached models, empty/null models, all five old-light classes
and nested buckets remain covered.

Four owned containers cover baseline and edited part and assembly streams at
2011. Replacement, reopen and revert preserve unrelated streams. Process
verifiers compare complete roots and whole containers in two fresh processes.

This checkpoint admits archive contexts from 2011. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
