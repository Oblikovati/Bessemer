# Valued-dimension fixture reuse

The native evidence selects the existing G1/G2/G3 serializers exactly. New
fixtures therefore reuse those complete body builders and retain the derived
class's own tagged identity. The builder now accepts the containing object's
actual map index so its repeated entity handle stays correct inside imported
maps and larger graphs.

Before this argument was added, 442 original default bodies were pinned by byte
length and SHA-256 across all existing classes, rich/minimal bodies and seventeen
versions. The baseline test verifies that the original default remains byte
identical. This preparation changes no production behavior.

Bug #86 / C-395 subsequently corrected the rich transform encoding to the fixed
archive operator. The 442 earlier hashes remain frozen; their test now explicitly
replays the pre-correction compact fixture bytes. That replay is fixture-only and
does not add a production fallback. Positive expectations are checked against the
independent fixed-form reader and native operator evidence.

Humanizer is unavailable; maintainer prose review remains open.
