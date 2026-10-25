# Named-view branch integration

This checkpoint adds owned full-root witnesses for section-plane suffixes,
selection names and selection bitsets. It changes no runtime decoder behavior.
The fixtures cover both document kinds at 8193, 8194, 8195, 8196, 8199, 8200,
8205, 8206, 19000 and the maximum unsigned archive version.

`generate_named_branch_roots.py` embeds bodies built by the independent named-view
writer into accepted full roots. Each manifest row records its source root and
hash. The replacement occupies one collection span and changes no MFC map slots.
Five scenarios cover empty, single, multiple, selected and independently edited
collections. The edit changes only the first section plane's signed word from
-17 to 23.

The 100 roots have 39,740 independent archive fields and 10,698 additional named
view fields. Integration tests force reconstruction, compare both oracle sets,
check unrelated fields and graph identities, reject sampled truncated prefixes,
and edit and revert without changing the source. Four owned whole-file fixtures
exercise edit, reopen and restoration while preserving unrelated streams.

`named-branch-verification.json` records byte reconstruction in two processes.
The two document verification reports record the same check for both original
and edited containers. These witnesses establish integration of the named-view
branches; they do not establish complete SolidWorks schema parity.
