# Polyline reconstruction

The private decoder now reads complete ordinary `moPLine_c` bodies, including
the direct surface-ID keeper, required data pointer and recursive owner.
`moPLineData_c`, `moPLineProject_c` and tagged `su_CWordArray` bodies have complete
readers. The existing body-feature base, entity-ID, nullable-array and numeric
array codecs are reused. The inherited atom-owner role includes the concrete
polyline class. Draft and intersection data bodies remain unsupported.

Four owned fixtures cover base data, projected data, a separately generated
scalar edit and high-bit WORD values. Their 96, 119, 119 and nine primitive
observations independently partition all stored bytes. The projected fixture
contains a tagged WORD array shared by two keeper arrays, recursive feature
owners and a negative nullable-array presence. Five focused Rust tests passed:
independent primitive comparisons, exact reconstruction, all truncated prefixes,
trailing bytes, unsupported versions, required-pointer validation, invalid
surface counts, input isolation and the independent expected edit.

Exact reconstruction increased from 323 to 357 of 906 sample configurations.
Two fresh processes produced identical outcomes. The independent reference
completed all 357 accepted cases against matching hashes for the selected
header, model, manager and feature streams, covering 14,152,243 feature bytes.
The 549 rejected cases remain explicit and the broad parity gate still fails.
These are stored-grammar checks, not native application or geometric acceptance.

The source, fixture and primitive hashes are in `polyline-reference.json`.
`polyline-frontier-checkpoint.json` retains the corpus outcomes and log hashes;
`polyline-accepted-reference.json` binds the independent accepted-subset audit.
Clause 11.145 is retained as a hash-verified raw schema snapshot. No dependency,
version, release, PR or push changed. The humanizer skill is unavailable in this
session; its prose review remains with the maintainer.

Full `cargo test` passed 405 library tests, the integration suites and 247
doctests, with three external gates ignored by default. The explicit sample
gate was executed twice and failed as reported above. Three fast independent
Python polyline tests also passed; their native-sample test was excluded from
that command. All 323 previously accepted samples remain accepted, with 34
additional accepted configurations.
