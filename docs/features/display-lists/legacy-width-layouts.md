# Earlier simple-text width layouts

Reference baseline: specification edition 0.228, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2771, 2795 and 2796. The 34
fixtures provide 4912 independently expected fields. They include all five old
light classes, unshaded roots, empty parts, null component parts and non-null
cache-backed parts. Every fixture is parsed to the end by the reference reader
at its declared version.

Through `0xAEB`, enabled simple-text widths occupy four bytes per character in
the decoded text, following the documented `len(text)` rule and the reference
reader. The stream stores no width count. Later layouts store an explicit i32
count, independent of the text length. Disabled widths store neither a count
nor width words at any version.

Fixtures include astral Unicode, combining characters, Latin-1 strings, empty
text, noncanonical enabled flags and disabled widths. One five-character string
occupies six UTF-16 code units; it requires five implicit widths. Explicit-count
fixtures use three widths for that same text and zero widths for a nonempty
astral string. Negative, zero, minimum and maximum signed-word encodings retain
their exact unsigned word bits.

Integration tests compare independent fields and forced bytes, check text and
width edits against independently built streams, and verify exact reverts and
source isolation. Changing text length alone fails when existing implicit widths
no longer match; explicit counts remain independent. Negative and excessive
explicit counts return typed errors containing the value and expected range.
Incompatible version contexts and truncated streams cannot publish partial fields.

Two owned containers check context resolution at version 2771. Four complete
cache-backed assembly containers cover versions 2771 and 2795 before and after
edits. Both complete referenced faces survive text edits, replacement, reopen
and revert. Their cache version stays 19000 independently of the display version.
Process verifiers compare complete streams, containers, contexts and linked
facet bytes in two fresh processes.

This checkpoint expands archive support to version 2771. Earlier bucket thread-pattern
fields and maps still need their own version coverage. Unresolved inline-facet payloads
remain explicitly unsupported. Byte reconstruction does not establish geometric
validity or native application acceptance. Complete schema parity remains tracked
by issue #28.
