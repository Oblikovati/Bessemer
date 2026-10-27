# Shared blend and component-curve fixtures

The blend-reference surface imports the same native blend-data serializer as
blend/cut features. The existing Rust reader is now accessible to the new
surface module without changing field scopes or behavior. Its independent byte
builder separates direct blend data from the existing blend/cut suffix.

The component-curve byte builder accepts an optional exact native class name;
its default remains `moCompReferenceCurve_c`. C-382's separately bound
`moCompRefCurve_c` uses that shared grammar without substituting class names
after decoding.

Five existing Python regeneration tests passed unchanged, covering 266 complete
graphs and 12 containing streams. The existing blend reader tests retain their
independent bytes, fields, pointer roles and failures. This refactor is committed
separately from the new runtime classes. The humanizer skill is unavailable;
its prose pass remains with the maintainer.
