# Sheet metal and saved context

Bessemer reads, edits and writes the documented ordinary modern sheet-metal,
template sheet-metal, fixed-face and component sheet-reference bodies. These use
non-DB external-XML framing above version `0x27CB`. The existing complete
body-feature and component-reference readers supply their inherited fields.
The component sheet reference appends one i32; it does not add feature ID/time.

Resolved decoding carries CMgr's validated `persisted_state_650` through a typed
state into every nested reader. The archive helper's native default zero applies
to detached ordinary records. Template records require verified persisted context
before consuming body bytes. State is never inferred from a class name, version,
plausible double or a successful alternative parse. Both valid saved states are
tested through independently patched complete CMgr streams; unsupported state 2
rejects. Public state-mismatch tests reject without a parse-based fallback.

The common bend and allowance bodies are direct members with no extra MFC tags
or map slots. Allowance dynamic arrays retain independent size/capacity/growth/
count fields and all payload bits. Zero size terminates the descriptor; a nonzero
size with zero count remains a distinct descriptor. Nonzero sizes must be
divisible by eight. Payloads are exposed as raw u64 chunks without assuming
each element is a double. Element counts determine the numeric table extent;
stride widths do not multiply that extent. All six grid values in the populated
fixture remain in stored order, including signed zero and NaN payload bits.

Array payload size, matrix extent and aggregate entry budgets are checked before
reading or allocating their contents. The material compatibility word has its
own `0x2BAF..0x2BB9` interval. Direct string arrays retain independent counts and
every CString. Common/derived discarded words and reversed member-offset order
are preserved. Extended fixed faces store a signed bend ID; default fixed faces
store a tagged bend pointer. These are separate grammars chosen by saved state.

Independent readers consumed 316 complete cases to exact EOF, making 89,802
original primitive observations. Twelve versions cover material, body and helper
tail boundaries. Both states, both feature classes, fixed faces and component
references retain shared dimension/geometry identities. Seven owned streams use
the blank part's actual imported 97 slots and complete saved manager state.

The reference reads each dynamic payload as one raw span. A scoped observer
records that exact span after the original function returns, without changing
its reads or values. Only these independently observed spans are projected into
eight-byte bit chunks for typed-field comparison, giving 90,406 observations.
Class names and other raw spans remain untouched. Original inventory hashes,
payload ranges and projected oracle hashes are retained. Projection tests check
every byte and caller provenance, reject partial-span targets and verify hook
restoration after failure.

Six private Rust tests cover every field/byte/slot, all truncated prefixes,
trailing bytes, old contexts, template provenance, malformed extents and unrelated
targets. The resource test requires exactly 16 entries for ten strided payload
words plus six numeric table values. Four public tests reconstruct complete
documents under both saved states, match a grid edit against independent expected
bytes, reject invalid/mismatched state and preserve source inputs. Five Python
tests regenerate the complete fixtures and check raw projection/context rules.

Fresh material-parameter and additional bend-family bodies remain separate
dependencies; no empty placeholder is registered for them. Native engineering
meanings, growth-policy validity, geometry/topology validation, DB/deferred
storage, native application acceptance and full schema parity remain unproven.
`reference.json` binds stable before/after reader hashes, generators and reused
builders. The schema snapshot retains C-182, C-296, C-369 and C-370 clauses.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0:
507 library tests, four xtask tests, every integration suite and 265 doctests.
The three external corpus gates remain ignored by default. Two explicit final
native processes have identical outcomes and causes; all 638 prior complete
configurations remain. Twenty-two failures advance: twenty base flanges, one
flatten-bends record and one environment-table feature. No additional native
configuration completes yet, leaving 268 explicit stops. The frontier record
retains the exact cases and causes; the verification record binds the previous
independent exact-EOF audit of the unchanged accepted inputs.
