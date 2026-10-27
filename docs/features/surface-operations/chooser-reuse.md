# Direct body chooser reuse

The existing optional body chooser reads a u16 presence tag, then a direct list
and a typed atom-feature owner. Specification 11.222 / C-250 uses that same
direct body without the optional tag. The shared helper preserves the existing
field scopes, primitive widths, limits, pointer roles and map behavior.

The refactor is separate from the new surface readers. Existing owned
body-feature tests exercise populated chooser lists and owner aliases, checking
independent field observations, exact bytes, version gates and failures. No
fixture baseline changes are needed. The humanizer skill is unavailable; its
prose pass remains with the maintainer.
