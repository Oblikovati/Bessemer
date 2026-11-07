# End caps

Feature #54 implements the complete ordinary modern `moEndCap_c` body from
11.348 / C-390. Behavior version: `C390-end-cap-v1`.

The installed part DLL binds primary table `0x1804DF888` to serializer
`0x180389070`. The reader reuses the complete `moBodyFeature_c` body and admits
the documented concrete class through inherited owner roles. It then retains a
tagged `suObList`, four signed words, four tagged distance dimensions, a raw
double, another signed word and the arc-length dimension pointer. Native order
places `+840` before `+838`. All historical suffix gates hold in the supported
ordinary part scope above `0x27CB`.

Null pointers give a forty-byte suffix. Non-null distance dimensions use the
complete existing reader and retain actual shared object identity. A non-null
new `moDisplayArcLengthDim_c` body remains unsupported: the clause names its
pointer role but does not document that body's serializer. The reader neither
aliases it to a distance dimension nor skips bytes to continue.

Independent fixtures cover 306 graphs, 126,039 primitive observations and 21
owned streams across seventeen inherited version boundaries. They retain
signed extrema, exact IEEE bits, direct parent ownership, null/new/cached
pointers, shared dimensions and distinct/repeated roots. Three edit goldens
change only the first suffix word. Tests check every prefix and appended byte,
old contexts, exact suffix width, native pointer order, unrelated cached classes,
unsupported arc-length dispatch and input isolation after accepted/rejected edits.

The native audit retained four end-cap bodies across the two weldment inputs.
Two fresh Bessemer processes matched all 906 outcomes and failure causes, with no
prior success lost. Both targeted inputs now stop at `moGusset_c`; they remain
failures and earn no completion credit. The complete count stays at 819, and a
frozen independent reader reconfirmed exact EOF for those same 819 inputs.

Run `cargo test --lib end_cap_tests`, `cargo test --test resolved_end_caps` and
`cargo xtask pr`. Regenerate with
`python -B -X utf8 docs/features/end-caps/run_pinned_tests.py`. This runner copies
the actual reference sources before importing readers, checks their origins and
hashes, and removes the temporary copy afterward. It avoids mixing reader
revisions while `inventor-interface` is being developed. No reader body is
replaced. The ordinary unittest command remains available for a stable reference
checkout.

Earlier/internal contexts, non-null arc-length bodies, geometry validity, scalar
meanings, application acceptance and whole-schema parity remain incomplete.
Humanizer is unavailable; maintainer prose review remains open.
