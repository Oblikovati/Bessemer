# Logical index replay contract

## Recognition and fallback

Physical version 7/8 framing is validated before replay. Forced scanning never
interprets control payloads. Preferred replay checks the final 12 bytes for
the modern signed eight-byte backward distance and index identifier. The
distance must select a physical index record ending at EOF. A candidate with
fewer than eight marker bytes rejects the preferred read; a nonzero marker
falls back to scanning. Other recognition misses also fall back to scanning.
After a zero marker is recognized, a malformed compressed body, table, footer,
or selected saved-span record rejects the preferred read. The four-byte writer footer does not
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

The public reader preserves these distinctions through an explicit replay
policy, bounded options, immutable results, and typed errors. It performs no
edits. Physical source identities and selected controls provide replay
provenance; ordinary spans remain raw values rather than validated identities.

Operation-local builders own mutable maps. Publication moves them into shared
immutable maps, and cloning a result retains the same immutable source and
control views. No global state or caches participate in replay. Absolute-start
lookups use the already validated physical ordering instead of allocating a
second record index.

The reader rejects a new distinct key before insertion when its bound is
exhausted. It retains the existing strict control decoder's checks and error
class while naming the logical replay operation at the API boundary. Forced
scans do not validate unused control options, because no control allocation or
decode occurs on that path.
