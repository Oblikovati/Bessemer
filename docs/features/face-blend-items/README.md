# Face-blend items and pairs

Bessemer reads, edits and writes `moFaceBlendItem_c` and
`moFaceBlendItemPair_c` in ordinary, non-DB external-XML part archives newer than
`0x27CB`. Specification 11.205 / C-233 binds both native primary dispatches and
their imported `su_CObject` base, which contributes no bytes in this context.
The native evidence file is retained with its source hash; the installed
`sldMoPartu.dll` hash was checked against it.

A pair stores two independent typed item pointers. An item stores typed face
and edge pointers, a raw presence byte and, when that byte is nonzero, three
stored doubles. Null, new and cached references retain their identities. Equal
items remain distinct; repeated item and pair occurrences remain aliases.
Point allocation adds no archive tag or map slot. Raw negative zero, NaN
payloads and infinity survive reconstruction without geometric interpretation.

The independent reference reader consumed 120 complete graphs through exact
EOF, producing 4,947 primitive observations. The corpus covers five versions
including the boundary-reference suffix boundary, both classes, three identity
modes and presence bytes 0, 1, 128 and 255. Eight owned streams begin with the
complete document-derived map of 97 slots; two are independent point-edit
goldens. Two Python tests regenerate every byte, observation and map count.

Four private Rust tests compare the independent field spans and reconstruction,
reject every truncated prefix and extra byte, enforce the old-context boundary,
reject unrelated cached objects at each declared pointer, and check the exact
four/five/29-byte minimum bodies. Three public tests reconstruct the containing
document, check source-mutation isolation, and edit only one of two equal items
against the independent golden. A rejected field-type edit preserves its input.

The reference native sample report contains zero point-presence bytes only.
Populated points are supported by the documented native branch and independent
synthetic fixtures; they are not claimed as observed native samples. Historical,
DB and special contexts, geometric validity, application acceptance and complete
documented-schema parity remain outside this feature's evidence. No dependency,
exported API or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

Two fresh native processes agree on every outcome and cause across all 906
configurations, with a stable executable hash. They preserve all earlier 719
complete cases and complete one additional case. Three more reach later
unsupported records and receive no completion credit. The remaining 186 cases
retain explicit failures, so the broad complete-corpus assertion still fails.
An independent immutable snapshot of 1,184 reference Python modules verifies
all 720 accepted streams through exact EOF: 43,966,522 feature bytes, with all
four original input hashes checked per configuration. The temporary snapshot
and audit script were removed after the audit terminated.

The local `cargo xtask pr` gate passed: 579 library tests, four xtask tests,
all integration tests and 265 doctests. Three opt-in native tests remain ignored
in the default gate; both complete-corpus replays above ran explicitly.
