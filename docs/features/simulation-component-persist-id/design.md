# Standalone persist-ID operation boundary

Reuse the shared `load_persist_id` reader after the preceding sheet operation's
gate passes. That reader already preserves complete solid-body field/history
exports and reads the same physical prefix in the inherited sheet operation.
Standalone parsing adds mandatory EOF rather than treating following component
or child-storage bytes as part of this stream.

Before outer version 840002, two words and the complete bounded blob make a
mandatory body even when the blob is empty. From that gate exactly two embedded
roots follow. Keep their inner versions/profile guards separate from the outer
version, with no first-success inference or class aliases. Do not interpret the
opaque blob as a GUID or assign meanings to either raw word.

Use complete typed construction, immutable source/replacement/result, exact
coverage/diagnostics and every ordered outer/nested history relation. Pin all
453 actual accepted/failed alternatives and independently expected fields and
edits. Require caller-budget/hard-cap/prefix checks, degenerate/blob/inner-version
cases, source isolation, Send/Sync, concurrent/fresh-process equality and full
local/platform gates before publication.

The C-738 binding is the component reader opening a separately named stream.
Verify its slot and literal bytes rather than applying an inherited sheet +60
storage call merely because the helper grammar is shared. Whole SolidManager
inventory and unresolved SurfaceBody trailers need their own acceptance.
