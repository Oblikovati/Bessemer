# Forming tools and inline parts

Issue #54; implementation remains open. The frozen clauses 11.251–11.252
(C-293/C-294) establish the ordinary modern instance, its owned surface-ID
keeper, the tool wrapper and the inline part context. The older prefix-only
description in 11.251 is extended by the complete native evidence in 11.252.

Two native instances contain two complete inline parts on the containing
archive's object map. Each part starts with a bounded lightweight section and
an inline tagged header, followed by a separately bounded resolved section.
Applying external-header imports here would duplicate objects and shift every
later identity. Neither section ends at the containing stream's EOF.

The first embedded body occupies 79,411 bytes, with 8,103 lightweight bytes and
71,300 resolved bytes. The second occupies 134,696 bytes, with 9,052 lightweight
bytes and 125,636 resolved bytes. Both retain their original part suffix selector
of 1. Their feature lists contain 20 and 25 entries. The containing instances
occupy 86,253 and 153,942 bytes and contain four attributed kernel bodies.

`native_reference_tests.py` copies the reference Python sources before importing
them and runs the five actual upstream forming-tool and embedded-part tests.
It requires both native tests to execute without skips and records source hashes
and module origins in `native-reference.json`. No reader function is replaced.
Those five tests passed, including exact native section boundaries and hashes.

Implementation must reuse the established lightweight and resolved codecs while
keeping inline header objects on the shared map. The external part fixtures were pinned unchanged before the shared section
bodies were extracted in a separate structural change. All eighteen existing
part/lightweight tests and workspace Clippy passed; the native source file,
independent edit and primitive oracle retained their original hashes. The tool must select inline-part dispatch only within its own
part pointer and restore caller state after success or failure. Header version
overrides apply only to their documented body fields. A fresh inline header
needs the existing header field recipes, rather than an opaque byte span.

The instance uses compact transforms, a direct keeper without an object tag,
two independent attributed-body lists and an ordered transform map. A suppressed
body still has its direct attribute-list count. Keep signed nonpositive counts,
duplicate entries, raw option words and unknown domains intact. The additional
condition-1 part stage and nonordinary sketch-profile mode remain outside the
reviewed ordinary context and must fail explicitly.

Completion requires independent primitive oracles, exact reconstruction,
edited-byte goldens, malformed boundaries, shared-map identity checks, caller
state restoration and two agreeing native corpus runs. Successful kernel
framing alone does not establish complete kernel semantics or document parity.

The maintainer humanizer skill was unavailable; prose review remains with review.
