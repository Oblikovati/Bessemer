# Earlier viewer-component mode layouts

Reference baseline: specification edition 0.235, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2711, 2724 and 2725. The 94
fixtures provide 18823 independently expected fields. They include all five old
light classes, unshaded roots, empty shaded parts, null component parts, complete
external faces, and populated or empty 2D and 3D buckets. The reference reader
parses every final stream to its end at the declared version.

Viewer components store no `display_mode_data` through `0xAA4`. After that
boundary, mode 6 occupies one i32; every other mode occupies three i32 words.
The fixtures cover the absent record, the sentinel value, and three-word payloads
with signed minimum and maximum values. Null component parts still have viewer
metadata in shaded assemblies. Parts and unshaded assemblies have no affected
viewer record in these fixtures.

The generator locates the tail using a complete independent parse, checks its
actual stored words, and removes or replaces those bytes. The final stream is
parsed again at its declared version. Field traces and forced byte reconstruction
verify the resulting layouts without inserting defaults for absent metadata.

Integration tests check independent edits, exact reverts, source isolation,
truncation failures and incompatible contexts. Changing the sentinel alone fails
when the two extra words are missing or left behind. A complete field splice
converts a frozen one-word archive into the independently built three-word archive
and back, retaining all other fields and exact expected bytes.

Two owned unshaded containers check context resolution at version 2711. Four
complete shaded assembly containers cover versions 2711 and 2725 before and after
edits. They combine populated buckets, non-null component parts and two complete
referenced faces. Mode, polygon and text edits preserve both faces through
replacement, reopen and revert. Their cache version stays 19000 independently of
the display version. Process verifiers compare complete streams, containers,
contexts and linked facet bytes in two fresh processes.

This checkpoint expands archive support to version 2711. Earlier component-property
and line-visibility layouts still need their own coverage. Unresolved inline-facet
payloads remain explicitly unsupported. Byte reconstruction does not establish
geometric validity or native application acceptance. Complete schema parity remains
tracked by #28.
