# Shell Property behavior profile 1

Feature #159 adopts the documented C-757 reader profile for its first public
Shells/MidSurfShells Property operation. No `SimulationShellProperty` public API
existed in the parent implementation at
`ef62850501d6b0aa3accc85c3d854fc6911adb91`. This adds a stored-framing operation;
it does not replace an earlier published Bessemer shell format or change the
thermal/inertial/mass profiles.

The original C-744/C-745 reports and every source pin remain intact. Their shell
parser hash was
`0b8650a4b40214d0d401b0a7050b73482892a62b5baa3c9bf5b09b2f8e45e31b`.
C-755 added documented shared fifteen-pair curves; C-756 added nonzero virtual
bodies; C-757 supplied the explicit newest layer-presence contract. The selected
parser hash is
`ac700fe228e0a35fe431c1d1a988092ba32f6cb391c240b33afd903201be48c8`.
Exact old/current file-hash pairs are guarded in `shell_property_evidence.py`;
changing any path or either hash fails the reconciliation checks.

The historical shell test hash was
`44e240fc58055b94de94f2cc996a8eb58c58f01cc45ccb1e01813905b621882a`.
The C-756/C-757 test hash is
`703707b838f938638d66a47f9b96b4932d1cb6dbc37e5e5c17891fde4f15cd9c`.
The historical file's bytes are unavailable. They were not guessed or rewritten,
and unchanged execution of that file is not claimed. The selected current tests
exercise the documented bodies, truncation and required runtime context. This
record adopts the current source/test profile while retaining that historical
evidence limitation.

Every full native outcome in C-744/C-745 and C-755/C-756/C-757 replays unchanged,
including failures. The original shell denominator stays 249 identities and 254
arguments; the overlapping refinement reports are not added to that denominator.
All original reports remain selectable evidence inputs in strict replay.

Profile 1 requires exact EOF, independent embedded archive versions, complete
inline Material framing and supplied presence for nonempty newest layers. It
never infers runtime lookup results from remaining bytes. Missing required
presence is an Unsupported error describing incomplete context; malformed lengths
are Invalid. An unused out-of-range terminal mode is accepted only when the curve
count is zero, as documented for the shell reader. Existing strict base callers
retain their original rule and rejection details through a separate callback.

The owned corpus, mutation outcomes, complete primitive/history packets, cap
controls and all-prefix checks are the behavior-1 baselines. Updating a baseline
requires a stated reason and behavior-profile decision. Native writer/application
acceptance, physical meanings and geometry validation remain separate exclusions.
No package-version bump, release tag or publication is authorized by this record.
