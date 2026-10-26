# Shared complete-object fixture observer

Circular-pattern fixtures need a complete temporary-axis reader installed on
their individual reference cursor. The mirror fixture observer previously
created and consumed that cursor in one function without a local registration
hook. Copying the observer would duplicate EOF, map-slot and primitive-coverage
checks.

`object_fixture_observer.py` now owns that observation sequence. The existing
mirror helper delegates to it with its original arguments. The shell generator
continues to use the mirror helper. Circular fixtures supply a callback that
registers the existing reference temporary-axis codec on the cursor's private
reader dictionary. No reference source file or global reader registry is edited.

The extraction preserves every byte, primitive span, slot count and recorded
case fact for 46 mirror/shell object cases and nine owned feature streams.
Six existing Python regeneration tests pass. `shared-observer-verification.json`
retains the original facts and fixture hashes for comparison. Generator hashes
and reference-source snapshots were refreshed; existing wire expectations were
unchanged. This structural change is committed separately from circular-pattern
runtime behavior. The humanizer skill is unavailable; its prose pass remains
with the maintainer.
