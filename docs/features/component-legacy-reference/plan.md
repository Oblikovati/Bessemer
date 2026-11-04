# Ordinary direct legacy references

Feature #206 implements the two-entry direct payload selected by nonzero
`unknown_84`. The old field path and its full signed value remain unchanged.
The payload contains a generic object pointer followed by one raw `u32` time
word; it adds no wrapper class or object-map slot.

The first frozen original-reader run passed 270 complete bare cases and 27
complete owned streams. They span the existing shared-prefix gates, all signed
presence extremes, null/self/fresh targets, exact following-field alignment,
and aliases after the containing record. A fresh strict regeneration run passed and reproduced every fixture byte; no
typed implementation acceptance is claimed yet.

Preserve the shared Simulation path: ordinary `base_fields` can read the direct
body, while Simulation's existing `base_head` caller keeps its nonzero-payload
restriction. Share the prefix/presence read rather than duplicating it or adding
a global tagged `moOldRef_c` registration. The separate reference-info body
remains unsupported.

Before changing production behavior, retain a failing public regression. Then
check complete typed construction, expected time-word edits, document reopening,
fresh reversal, class/object identities, every prefix, invalid tags, resource
caps and input isolation. Run existing component and Simulation tests, the native
reconstruction corpus and full PR validation. Keep exact source revisions and
exclusions. Generated positive bodies do not establish positive installed
population, native application opening or geometry validity.
