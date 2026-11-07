# Reusing generic-wrapper caller fixtures

The existing body-reference fixture builder now accepts an explicit typed pointer
builder. Its original entry point still supplies the original body-reference
grammar. This lets new reviewed wrapper subclasses reuse the complete end,
intersection and external-entity callers without copying their layouts or
replacing any independent reader.

Before adding the derived-reference behavior, three immutable-reference fixture
tests regenerated the original stock and wrapper packets, all primitive
partitions and all 58 owned document fixtures. Every case, slot count, primitive
observation, packet hash and fixture byte matched the previously committed
baselines. This is a fixture-only structural change; production code is unchanged.

The humanizer skill was unavailable; its prose review is left to the maintainer.
