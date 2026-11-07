# Modern 3D views

Issue #54; behavior version 1. Section 11.268 / C-310 binds the modern
`mo3dView_c` body from archive version 0x3B07. It retains the complete ordinary
view base and its two compact transforms, then the derived scalar/vector/map
suffix. Both serialized +3FC values remain separate; native input leaves the
second value loaded. Earlier preview and annotation-map branches stay explicit
unsupported contexts here; older manager layouts remain separate work.

Map and item counts store the high u32 first, then the low u32. They are not
little-endian u64 values. Annotation data stores all signed i32 keys before
any groups, then one high/low item count and direct item records per key.
Each direct item has four raw i32 values and no MFC identity. Negative signed
vector/key counts retain their stored value and consume no entries. Duplicate
keys, ordered groups, nulls and cached references remain unchanged. Counts
are bounded against minimum remaining widths before work is allocated.

The configuration-manager reader installs this class. Source does not install
it as a resolved-feature root. An initial fixture attempt used the wrong
entry point and failed before publication. The complete owned fixtures now
insert real view objects into a configuration-manager generic object map,
relocate only observed later MFC references, and pass the actual complete
Source reader without reader replacement. Transient base-view lists continue
to reject fresh helper-state bodies; null and cached targets remain supported.

Independent fixtures contain 36 direct graphs across six modern versions and
six styles, 3,948 primitive observations and seven complete manager streams,
including an edit golden. They cover all-before-groups framing, duplicate keys,
raw IEEE values, high-word overflow, signed empty branches, cached self references
and duplicate +3FC words. Ten focused tests and three public manager tests
passed, including all truncated prefixes, extra bytes, exact reconstruction,
independent editing, save/reopen and source isolation. Immutable regeneration
reproduced every owned byte. The shared public-manager test contract was
extracted separately with all fourteen old scalar fixtures unchanged.

The actual Source reader reproduced all four documented native 3D-view spans,
1,184 bytes in total, without reader replacements. Two native corpus runs
returned identical statuses and diagnostics: 899 of 906 accepted, up from
898, with new case 559 and no regressions. Seven explicit stops remain
incomplete. Source verified exact EOF for all 899 accepted streams, totaling
81,109,538 resolved bytes, after checking four input hashes per configuration.

The Windows Rust 1.99.0 `cargo xtask pr` gate passed: 1,156 library
tests, 277 doctests and 2,686 total passing tests across 198 test
executables, with three intentionally ignored.

This evidence does not
establish legacy-manager, whole-document, assembly, drawing or installed-project
completion. The maintainer humanizer skill was unavailable; prose review
remains with review.
