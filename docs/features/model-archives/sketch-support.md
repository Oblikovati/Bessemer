# Sketch support records and dimension text

Feature #44 now reads the documented sketch entity handles, seven backup-data
variants, component sketch and silhouette references, temporary axes, external
references, intersection metadata and slot entities. Escaped handle indices
retain both their escape word and signed value. Shared backup and handle
identities remain tagged references, rather than duplicated objects.

The support packet covers 259 cases and 2,960 independent scalar expectations.
It exercises all nine handle classes, short/escaped index boundaries, signed
extremes, shared references, optional descriptions and the temporary-axis
version boundary. Invalid external-reference modes fail as invalid input;
unsupported minimal-owner slot state fails explicitly as unsupported. Both
diagnostics retain the stored value and expected shape.

Complete dimension-text bodies add 128 cases and 5,760 scalar expectations.
The reader retains stored RTL text, tagged character formats, nested group and
point counts, optional curve arrays and compact/extended string-array counts.
It reuses the existing character-format codec. Counts are checked before array
iteration; negative lengths, impossible extents and sum overflow fail. The owned
populated case uses exactly 19 aggregate array entries: 19 passes and 18 fails.

Both packets reconstruct exactly and reject every truncated prefix and trailing
byte. Handle and text edits match separately assembled, reference-validated
byte fixtures, reparse to the edited fields and leave their sources unchanged.
Source-overwrite and four-writer checks establish ownership and deterministic
encoding for the tested records. No sketch solving or geometry validation is
performed.

These records are dependencies of the complete sketch body. Direct geometry and
dimension collections, patterns and the sketch tail still need composition.
There is no partial `sgSketch` root reader or opaque tail. Native configuration
acceptance remains 866 of 906: 38 sketches and the two tracked role failures are
still incomplete. New native measurements, schema clauses, source hashes and
test results are bound in `sketch-support-verification.json` and the cumulative
corpus checkpoint.

All twelve focused tests, all 182 model tests, all-target Clippy and all 50 Python
tests passed. All five wire fixtures reproduced exactly in fresh processes.
The native corpus reproduced every outcome from the preceding checkpoint.
The full PR gate belongs to
the earlier geometric/kernel checkpoint and must rerun before push. Public
model APIs, operation contracts and the remaining corpus/platform gates remain
open. Humanizer was unavailable; prose remains for maintainer review.
