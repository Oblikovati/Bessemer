# Ordinary text widths and material classification

Reference baseline: specification edition 0.241, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 158 complete part and assembly roots cover versions 2563, 2564, 2565, 2566
and 2596. They provide 43403 independently expected traced fields. Both ordinary
text item kinds, 7 and 15, include empty text with disabled widths and populated
text with enabled widths. Unicode samples contain an astral character, a
combining mark and a CJK character; enabled width arrays use Unicode character
counts. Through 2563, the width flag and array are physically absent. The simple
text width flag remains present independently.

Material classification strings are absent through 2565 and present from 2566.
Their locations are observed in complete material parses before the user-property
continuation. Removing those primitive strings changes no MFC object identities.
Tests check every material definition, including definitions first reached inside
buckets and later referenced by root visual properties.

Eight roots at 2563 retain two complete inline facets each. Separate independently
built facet records check their full typed contents and exact bytes. Composite
edits change ordinary text together with facet stamps, polygons, edges and simple
text. Four owned containers cover part and assembly streams before and after
edits. Tests check replacement, reopen, authentic revert, input isolation and
truncated input; process verifiers compare forced roots and whole containers in
two fresh processes.

This checkpoint admits archive contexts from 2563. Earlier layouts and unresolved
representations remain tracked by #28. Byte fidelity does not establish geometric
validity or native application acceptance.
