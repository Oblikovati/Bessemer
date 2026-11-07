# Manager feature IDs and material values

Bessemer retains the complete ordinary `moSMAndFPFeatIdPair` and
`moCustomDefinedMaterialProperty_c` bodies. The pair stores three signed words.
Material values store a CString, two binary64 values, a signed word and a final
binary64 value, in that order. No additional base bytes or version-dependent
suffixes are inserted. Feature roles, physical units and material meanings remain
unknown; stored values and identities are preserved without runtime normalization.
The layouts follow specification sections 11.260 and 11.311.

Independent fixtures cover 72 complete object graphs across six version controls
and six styles, with 1,026 primitive observations. Fourteen owned CMgr streams
place actual scalar objects in the existing material and feature-ID maps. They
retain duplicate raw keys, distinct equal values, cached objects, signed extrema,
negative zero, infinities, NaN payloads and short/extended ANSI and UTF-16 names.
Two independent manager edit goldens change only the selected first value.

The fixture builder uses positions recorded by the complete source reader and
profiles actual MFC object calls. Inserting new map values shifts later map slots,
so only recorded later object/class references are relocated. Scalar words and
earlier references stay unchanged. No source reader or primitive method is
replaced. Dedicated tests check short/extended tag relocation and profiler cleanup
after failure, alongside immutable fixture regeneration.

Seven focused Rust tests and three public document tests passed. They compare
every independent primitive, reconstruct full managers, reject all truncations
and appended bytes, verify edit goldens and preserve owned results after source
mutation. Public model loading and save/reopen retain the manager streams exactly.
Typed manager-field editing is tested through the existing internal field encoder;
this change adds no public manager-edit API.

Two fresh native processes matched all 906 statuses and causes. All 883 accepted
configurations remained accepted. Configurations 277 and 498 advanced past the
pair records to the template flat-pattern and gusset surface-ID dependencies;
they remain incomplete and receive no completed-file credit. The aggregate
corpus still fails with 23 explicit stops. The independent reader consumed all
77,262,506 resolved bytes of the 883 accepted inputs through EOF. Separate
complete reads of the two targeted native CMgr streams verified four actual
pair records and all four source-stream hashes. The full local gate passed 2,574
tests across 187 executables, including 1,079 library tests and 277 doctests;
three opt-in corpus tests remained ignored. Supported manager dialects, remaining
schema branches and native application acceptance retain their existing limits.

The humanizer skill was unavailable; its prose review is left to the maintainer.
