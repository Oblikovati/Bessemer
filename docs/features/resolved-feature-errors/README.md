# Resolved-feature error context

Bug #64 concerns errors from `PartResolvedFeaturesArchive` labeled as
`part configuration` on `Contents/Config-N`. These operations now report
`part resolved features` on `Contents/Config-N-ResolvedFeatures`, using the
already normalized configuration ID. The configuration API retains its
existing error context. Model-context failures during `from_document` retain
their source configuration context.

The selected feature context covers reads, validated field replacements,
reconstruction and revision-bound field lookups. Failure classes and causes
are retained. CMgr parse failures retain `Contents/CMgr` in the cause so a
dependency failure can be distinguished from a feature-body failure. Shared
field lookups no longer add the incorrect model-stream name to their cause.

Six public tests cover truncated feature input with configuration IDs 0 and 7,
same-width invalid field types, out-of-range and foreign-revision handles,
unsupported runtime classes, CMgr dependency failures and unchanged
configuration errors. Existing reconstruction, ownership, field-history and
complete-document edit/reopen/revert tests retain their expected bytes.
The earlier incorrect entity assertion was updated to the documented selected
feature stream; schema, fidelity and geometry checks were not relaxed.

This changes error reporting only. It adds no dependencies, public API, package
version or stored format. Geometry and native application acceptance remain
outside the stored-schema checks. The humanizer skill is unavailable in this
session; its prose review remains with the maintainer.

The combined local PR gate passed with 427 library tests, all integration
suites and 265 doctests. Two fresh 906-case corpus runs retained all 419
successes and every failure cause from the preceding line-reference fix.
The full corpus gate remains incomplete. Reports in this directory record
the executed scope, exclusions and log hashes.
