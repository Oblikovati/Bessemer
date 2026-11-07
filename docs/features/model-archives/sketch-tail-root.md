# Ordinary sketch tails and complete roots

Feature #44 now composes the documented direct geometry, dimensions, patterns
and tail into the private ordinary `sgSketch` reader. It shares one archive map
through every stage and returns only after the complete supported body. Nested
unsupported classes fail the containing operation; no unread remainder becomes
a successful sketch.

The new tail covers equal-distance constraints and logical groups, ordinary
offsets and PGM profile/loop records, styled curves, zero-marker intersections,
creation IDs, visual state, local relations, coincidence maps, silhouettes,
block/handle-data managers, sparse loops, slots and the final stored word.
It retains duplicate keys, signed sentinel counts, noncanonical presence words,
raw state/allocator metadata and records discarded by native runtime insertion.
The ordinary default helper has profile mode false by constructor. That context
does not consume an invented flag byte or support arbitrary inherited modes.

Zero-marker intersections preserve their handle and edge arrays, dynamic word
groups, optional transform and version-gated references/points. Negative handle
and point counts retain their signed values while selecting empty arrays, as in
the documented loader. Dynamic words require aligned widths and bounded count
products. Two eight-byte elements charge six aggregate entries: two groups and
four words. Six passes and five fails. A populated offset's four three-element
arrays charge twelve entries; twelve passes and eleven fails.

Long zero-marker silhouettes and modern offset kinds 8/9 with undocumented
kernel continuations still stop explicitly. Populated note, display-dimension,
block and handle-data objects require their own implemented readers. Nulls and
references do not substitute empty bodies for unsupported new classes.

The independent packet has 1,038 cases and 33,645 scalar expectations across
fourteen versions. It covers individual records, complete tails, direct roots
and tagged roots. Every case reconstructs exactly, every truncated prefix and
trailing byte fails, and all results survive source-buffer overwrite. A final
integer edit matches independently assembled bytes and reparses with the expected
fields; the original stays unchanged. Parallel writers reproduce the same bytes.
Malformed nested counts, unknown widths and unsupported branches have focused
checks. Three wire fixtures reproduce exactly in two fresh reference processes.

All eleven tail/root tests and all 235 model tests passed on the final source,
along with all-target Clippy and all 51 Python tests. The full PR gate remains
the earlier geometric/kernel checkpoint and must rerun before push.

The scalar oracle excludes only derived completion/presence flags and sketch
cursor/unknown-byte summaries. The first packet accidentally included the
reference reader's `surfaces = True` completion marker. Removing that derived
expectation changed no wire bytes. Stored values remain covered; the final
packet also adds two documented external-reference variants per version.

Full sketch composition exposed Bug #46 at `sgExtEnt_c.reference`: 33 native
cases contain `moSketchExtRef_w` and five contain `moTempAxisRef_w`, both extending
the documented wrapper grammar. A regression failed before admitting these two
classes alongside `moRefWrapper_c`. New-object fixtures and loaded-reference
checks pass; an unrelated class still fails. This finite field-specific change
leaves display-list role validation untouched.

The first complete-tail native run still accepted 866/906 and stopped at these
38 role failures plus the earlier two. After the role fix, acceptance remains
866/906, but all 38 now reach later dependencies: 29 distance display dimensions,
three radial display dimensions, five bosses and one revolution. These first
stops do not prove that every preceding native sketch completed. The two earlier
role failures, selected-configuration diagnostics, public operation results and
history, geometry/native application acceptance and platform gates remain open.
The verification manifest binds source, schema, test and fixture hashes to the
recorded runs. Humanizer was unavailable; prose remains for maintainer review.
