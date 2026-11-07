# Reusing the complete One Bend prefix

The One Bend reader now returns the stored signed bend-kind word to a concrete
subclass while its original entry point discards that return value. The shared
reader still emits the same fields, tags, paths and bytes in the same order.
It does not reinterpret unknown kind values or change inherited kernel stops.

Sketch Bend needs this value to distinguish the documented kind-7 owned-data
branch before consuming a suffix. Reading the word twice or finding it later
through emitted field paths would weaken the typed boundary. Returning it at
the original read preserves both the persisted word and its control use.

The previous bitmap full gate passed all existing One Bend fixture comparisons
before this structural change. The established independent edge-flange fixture
builders and frozen baselines are unchanged.
After the change, the focused complete-family regression passed every independent
field, reconstructed byte and object-map slot comparison without baseline updates.

The humanizer skill was unavailable; its prose review is left to the maintainer.
