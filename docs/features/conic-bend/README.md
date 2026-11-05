# Conic bend data

Feature [#218](https://github.com/Oblikovati/Bessemer/issues/218) adds the ordinary
schema-1 `moConicBendData_c` codec reached by OneBend's existing `+0xAD0` pointer.
The stored behavior is `C3579-conic-bend-v1`. Its 72-byte body contains a point
triple, a vector triple, two binary64 scalars and two signed32 words, in that
order. It adds no base body, version-gated suffix, padding or child map slots.

The source is specification 0.3100, section 11.243 / C-3579.
[schema-reference.json](schema-reference.json) binds the saved clause snapshot
and verifies five retained native/capture hashes. This is static evidence;
the original producer parser and native DLLs were not executed. Native factory
execution and populated native conic samples remain unproven.

The native writer widens each current runtime byte to a signed32 wire word.
The native loader reads the complete word and reduces it to a nonzero byte
state. Bessemer preserves the original signed32 words, including negative and
noncanonical values. Scalar meanings, coordinate frames and engineering roles
remain unknown. Float values retain their exact bits, including signed zero,
infinities and NaN payloads.

The literal corpus contains 576 bare graphs crossing sixteen floating-point
patterns with every pair of six signed-word values. Four complete containing
graphs cover empty, populated and shared OneBend records plus the minimum
modern version. Eighteen complete document streams cover six contexts in the
two saved sheet-metal helper states, with independent state/scalar edits.
Two distinct OneBends can share one conic object, and a repeated root retains
its existing identity.

The existing OneBend fixture assembler accepts an optional conic callback.
Its default output reproduces all 684 earlier edge-flange graphs and all six
active helper-one document streams unchanged. These checks use retained byte
oracles; they do not regenerate expectations with Bessemer's encoder.

Public document tests cover twelve edits and fresh reversals, complete carrier
bytes, revision-bound history, failure/source-mutation isolation and eight
concurrent workers. Private tests cover the numeric matrix, all truncations,
wrong pointer roles, class schemas, modern version boundaries, all five
resource limits and unsupported contexts. Validation outcomes and exclusions
are recorded separately.
Two fresh processes also reconstructed all eighteen owned streams byte-for-byte
against the same independent goldens. The output hashes and cleanup record are
in [process-determinism-verification.json](process-determinism-verification.json).
The retained native compatibility run accepted all 906 selected stream sets
with an explicit installed kernel schema; see
[native-resolved-regression-verification.json](native-resolved-regression-verification.json).

The accepted mode is exact stored-archive reconstruction. It does not certify
geometry or native application opening. DB and historical contexts remain
unsupported. OneBend's positive raw-kernel branches remain explicit stops.
No package version, dependency or release is changed. The humanizer skill is
unavailable here; maintainer prose review remains outstanding.
