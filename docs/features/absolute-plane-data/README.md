# Absolute plane data

Feature #54 implements `moAbsoluteRefPlnData_c` from 11.243 / C-285. Behavior
version: `C285-absolute-plane-data-v1`.

Native ancestry and virtual-hook evidence bind this class to the existing
`moRefPlnData_c` serializer `0x4C283D30`, with data hook `0x4C2880A0`. The ancestry
excludes the default-plane class, so its extra display-root point is absent.
The reader reuses the complete non-default base and its version gates, with no
absolute-plane suffix or runtime padding. The typed containing-plane role admits
this exact implemented class.

Independent fixtures cover 108 graphs, 16,263 primitive observations and fourteen
owned streams across nine versions. Bare records and complete containing planes
retain new/cached/null references and distinct/repeated roots. Tests compare all
fields, map slots and bytes, every prefix and appended byte, old contexts, the
259/261/265-byte base extents, absence of default-only fields and rejection of
unrelated cached classes. Two point-edit goldens change only the selected first
root; accepted and rejected edits preserve their inputs.

The existing exact plane-role inventory now includes the independently decoded
absolute-plane parent fixture. Its equality check remains exact across all
nineteen implemented plane-data classes; the earlier fixture expectations are
unchanged.

The native audit retained eight absolute-plane bodies in the two weldment
inputs. Together with the finishing readers, both streams now reconstruct
completely. Two fresh processes matched all 906 outcomes and causes, increasing
complete cases from 819 to 821 without losing a prior success. The remaining
85 failures remain explicit.

Run `cargo test --lib absolute_plane_data_tests`,
`cargo test --test resolved_absolute_plane_data` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 docs/features/absolute-plane-data/run_pinned_tests.py`.

The copied clause also covers edge-flange families; this implementation claims
only the absolute-plane class. Earlier/internal contexts, scalar meanings,
geometry validity, application acceptance and whole-schema parity remain
incomplete. Humanizer is unavailable; maintainer prose review remains open.
