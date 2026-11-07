# Logical index replay contract

## Recognition and fallback

Physical version 7/8 framing is validated before replay. Forced scanning never
interprets control payloads. Preferred replay checks the final 12 bytes for
the modern signed eight-byte backward distance and index identifier. The
distance must select a physical index record ending at EOF with a zero
eight-byte marker. A recognition miss falls back to scanning. Once recognized,
a malformed marker, compressed body, table, footer, or selected saved-span
record rejects the preferred read. The four-byte writer footer does not
establish a modern terminal index and falls back to scanning.

## Maps and provenance

A physical scan retains the last record for each ordinary identifier. Earlier
occurrences move into the saved map keyed by their physical start. Every
deleted-record span goes directly into that map.

Terminal replay folds raw index entries in wire order, retaining the last
entry for each identifier, then replaces the index's own entry with its actual
physical span. Raw ordinary target offsets and lengths remain unvalidated.
A saved-span directive must select an exact physical record start, identifier,
and span. Its strict payload is then folded with the last duplicate key winning.
No saved map is reconstructed from unrelated physical occurrences in this mode.

Frozen maps use unsigned key order for deterministic comparison. Raw terminal
entries keep their original order so duplicate precedence can be checked.
Cases cover empty and large tables, duplicate directives, all identifier and
wide-value bit boundaries, malformed recognized controls, and opaque controls
that forced scanning must retain without decoding.

## Acceptance boundary

Completed replay does not establish complete document decoding. Every frozen
projection explicitly reports that ordinary payloads were not decoded,
ordinary indexed targets were not validated, and the complete document schema
was not covered. Native execution and application acceptance are excluded.

The future public reader must preserve these distinctions, expose an explicit
replay policy and bounded options, and publish immutable results with typed
errors. This baseline ships no speculative public reader or editing API.
