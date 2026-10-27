# Weld corners and weld-body data

Bessemer reads, edits and writes `moWeldCornerFeat_c`, `moWeldBodyData_c`,
`moWeldTrimmedBodyData_c` and `moWeldTrimToolData_c` in ordinary non-DB
external-XML part archives newer than `0x27CB`. Specification 11.241 / C-283
binds the four primary serializers, ancestry, constructor-created array types
and the selected helper calls. The supplied evidence is retained with its
source hash, and the installed-module hash was verified.

The feature reuses the complete body-feature base. Its three direct object
arrays have independent packed counts and allocate no array map slots. The
generic reference lies between the first two arrays and the third array. Raw
signed words and the declared point reference retain their stored order; the
final word is present only after version `0x334D`. Earlier versions consume no
word for the native runtime default of one. The ordinary suffix occupies 26 or
30 bytes with empty arrays and null/mapped pointers.

Weld-body data stores a direct object array and two signed words. Trimmed-body
data appends an independent direct unsigned DWORD array. Trimming-tool data
instead appends two stored doubles. Its comparison-derived runtime flag is not
an archive field. The complete empty data bodies occupy 10, 12 and 26 bytes;
other constructor-created containers, pointers and transforms add no fields.

The independent reference reader consumed 120 complete graphs through exact
EOF, producing 20,594 primitive observations. Five versions cover the exact
`0x334D/0x334E` boundary and both supported extremes. Six modes exercise
independently sized arrays, null/fresh/cached references, complete shared
weldment targets, point references, unsigned DWORD extrema, signed zero and NaN
payloads, equal-but-distinct records and repeated aliases. Source-parsed target
identity is checked separately from byte partitions and map counts. Twenty-five
containing streams start with the actual document-derived map of 97 slots,
including an independent one-word edit golden.

Six private tests compare every field span, byte and map count; reject every
prefix and extra byte; check old contexts, declared point roles and impossible
escaped counts; and verify the suffix boundary and data minima without invented
runtime fields. Three public tests reconstruct all containing documents, match
an isolated edit to the independent golden, preserve the second equal record
and shared targets, and verify input isolation after source mutation or a
rejected field-type edit. Two Python tests regenerate every fixture and repeat
the parsed-identity checks.

Raw option domains, runtime trimming/flag updates, geometric validity, historical
or special contexts, application acceptance and complete documented-schema
parity remain unproven. No dependency, exported Rust API or package version
changed. The humanizer skill is unavailable; its prose pass remains with the
maintainer.

Two fresh native processes agree on every outcome and cause across all 906
configurations, with a stable executable hash. Every earlier 734 complete case
remains complete. Four cases advance to the unsupported solid-reference
dependency and receive no additional completion credit. The remaining 172
cases retain explicit failures, so the broad complete-corpus assertion still
fails. An independent immutable snapshot of 1,213 reference Python modules
verifies all 734 accepted streams through exact EOF: 45,307,798 feature bytes,
with all four original input hashes checked per configuration. The temporary
source copy and audit script were removed after their process terminated.

The local `cargo xtask pr` gate passed: 611 library tests, four xtask tests,
all integration tests and 265 doctests. Three native opt-in tests remain ignored
in the default gate; both complete-corpus replays above ran explicitly.
