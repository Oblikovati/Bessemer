# End-specification point-reference roles

The ordinary extrusion and revolution end-specification readers now accept the
established finite point-reference family: `moPointRef_w`, `moEndPointRef_w` and
`moLoftSynchRef_c`. The fix changes only the two type-three extrusion-side point
roles and the two unconditional revolution-end point roles. Unrelated wrappers
still fail. Other role families and wire layouts remain unchanged.

Specification 11.90 declares the extrusion point pointers; 11.91 declares the
revolution point pointers. Section 11.164 / C-192 proves the endpoint's complete
point-reference base, and 11.204 / C-232 proves loft-sync's endpoint ancestry and
direct base call. The implementation reuses the existing finite family rather
than accepting every wrapper or matching a class-name prefix. The existing
family's comment and plane-role documentation now cite the correct loft-sync
evidence identifier, C-232.

Before the fix, the fresh- and cached-pointer regression tests failed at the
endpoint's point role. After the fix, all four focused private tests pass.
Independent readers observe 96 complete graphs across two owner classes, four
point/wrapper classes, six versions and both fresh/cached modes. The 72 valid
graphs round-trip every primitive, byte and map slot. The 24 unrelated-wrapper
graphs remain invalid. Complete-prefix, trailing-byte and old-context tests
retain explicit failures.

Ten owned streams contain complete boss or revolution features, including their
inherited model/atom/body stages. Eight are positive cases, with two independent
endpoint-coordinate edits; two retain unrelated-class failures. Every graph
imports the actual configuration map's 97 slots and finishes at 107 slots.
Public tests reconstruct whole documents and check isolation after success,
source mutation, rejected field edits and invalid pointer-role parsing.

The existing complete fixture builders gained optional typed callbacks for point
and end references. Thirty-two owned pre-change baselines prove that their
original default calls emit identical bytes and map slots. No earlier fixture or
behavior baseline was replaced. New graphs reuse the complete inherited and
derived builders. Python tests regenerate every fixture and compare the frozen
defaults.

Issue #46 tracks this bug and the broader pointer-role audit. Its runtime files
do not exist on `main`, so this fix accompanies the unreleased interop candidate.
It introduces no version bump, dependency, new runtime function or public API.
Native geometric validity, native application acceptance, historical and DB
archives and full documented schema parity remain outside the evidence here.
The humanizer skill is unavailable; its prose pass remains with the maintainer.

Two fresh native processes report identical outcomes and causes for all 906
configurations. Together with the radiated-edge registration, this batch
preserves the earlier 705 complete results and reaches 708, leaving 198 explicit
stops. All three additional completions previously failed at the end-point role.
The four radiated-edge cases advance to later explicit stops without receiving
completion credit. The executable hash remains unchanged during replay.

An independent immutable snapshot of 1,137 Python modules verifies all 708
accepted streams through exact EOF: 42,423,714 feature bytes, with all four
original input-stream hashes checked for every configuration. The first capture
was rejected when sibling composer modules changed during copying. A fresh
capture passed the stable-source check before the audit began. Temporary source
copies and audit code were removed after their processes terminated. The broad
906-case assertion still fails because the remaining configurations are
unsupported; neither partial results nor that failure were hidden.
