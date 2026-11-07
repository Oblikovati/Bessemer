# Feature states

Bessemer reads and edits `featureStateData_c` as documented in section 11.332,
C-374. The behavior version is `C374-feature-states-v1`.

Its complete ordinary body contains two signed i32 values, at runtime offsets
`+0x08` and `+0x10`. They occupy eight consecutive archive bytes, without an
inherited node prefix or padding between the words. Their engineering meanings
remain unresolved, so both raw signed values are preserved.

The reader is part of the scoped model-object dispatcher and requires the
ordinary modern part context. It does not infer other layouts or document
contexts from the class name or payload length.

Independent fixtures cover 42 graphs across seven versions and six identity
styles, with 420 primitive observations and seven complete document streams.
They retain distinct/repeated objects and signed extrema. Scalar edits match
independent goldens; failed edits preserve both archive and document inputs.

The independent reader's native dependency-chain regression verified the three
state objects together with their containing root hashes, fields and exact EOF
in two source streams. The combined Bessemer native gate and its exclusions are
recorded with [flatten/process bends](../flatten-bends/README.md); a source-reader
completion does not itself count as a Bessemer completion.

Run `cargo test --lib feature_state_tests` and
`cargo test --test resolved_feature_states` for exact reconstruction and edit
isolation. Run `python -B docs/features/feature-states/run_pinned_tests.py` to
regenerate fixtures using frozen copies of the actual reference readers.

The humanizer skill is unavailable; prose review remains with the maintainer.
