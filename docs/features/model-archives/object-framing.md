# Shared MFC framing and model fields

Feature #44 now records typed MFC tag framing through the existing object map.
The shared reader returns the stored word, optional extended word and resolved
object record in one pass. The display decoder builds its existing public tag
type from that result. It no longer reparses the source span or retains a
separate input-buffer field for that second pass.

The shared writer emits those same stored words and class descriptors through
the caller's bounded output. It preserves the earlier display framing, ASCII
name, schema and length checks, including their archive-specific failures.
Declared pointer-role checks are shared too; fix #31's distinction between map
validity and runtime class compatibility remains enforced for aliases.

The private model field builder stores this structured framing, not copied
tag bytes or unresolved object bodies. New object loads have their own budget;
references and nulls do not count as new loads. Imported null references retain
their nonzero physical map tags while resolving to null. Object headers still
require their complete class bodies to be decoded by the containing model
serializer before that root can succeed.

Nine focused object tests passed alongside the nine appearance tests. They
cover new classes, schema `0x1234`, short and wide class reuse, short and wide
aliases, both null forms, imported objects/nulls, incorrect declared roles,
unmapped and truncated tags, object budgets, malformed stored framing and
invalid class descriptors. The owned native node-name object resolves at slot 5
after the explicit root/header map setup and reconstructs all 32 bytes exactly.
Four parallel framing encoders also match after the source buffer is overwritten.
That atomic record test does not establish complete model-root support.

The full local PR gate passed with the shared framing path, including the
existing frozen display fixtures and all 204 doctests. After the small role-helper
cleanup and added ownership test, all 18 focused model tests, formatting and
all-target clippy passed. Public model APIs,
complete section/body decoding, header-derived import planning, scoped maps,
derived section lengths and whole-document model acceptance remain required.
Complete construction must reparse the root with its context and compare field
paths and resolved object metadata; framing syntax alone does not validate the
entire object graph or retain its imported header bodies.

The humanizer skill was unavailable; prose remains subject to maintainer review.
