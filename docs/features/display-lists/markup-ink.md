# Markup-ink grids

Issue #28 implements the complete `SerializeMarkUpInkViews` body for the currently
supported display contexts. Versions at or below `0x2F80` omit it. Later versions
store signed view and configuration dimensions, followed by each view's name and
ID, its configuration rows, and a final string when the version exceeds `0x332B`.
Each row contains a name, an ID, a suppressed state and an is-shown state.

Both dimensions must be nonnegative. The decoder charges views, the independent
configuration dimension and the product of both dimensions to the shared entry
budget. It checks the product for overflow and against the minimum 13 wire bytes
per configuration before reading any rows. An empty view dimension still permits
a nonzero configuration dimension without requiring nonexistent row bytes (#30).
Signed state values remain raw, including negative and extreme values.

Nine owned complete roots cover populated part and assembly grids at version
19000, a populated part at 18000, an independently encoded state edit, zero views,
zero configurations, both dimensions zero, and minimal empty-string encodings.
The approved sibling reader consumes every root to EOF and independently supplies
104 semantic field expectations. The manifest records source and fixture hashes.
Fixtures prove archive behavior; CAD application validity remains unclaimed.

The native blank roots and the existing 18000 fixture each contain an original
zero-view, one-configuration ink dimension. Replacing that body subtracts its one
entry before adding the new dimensions and rows. The resulting non-ink entry
counts are 52 for both native blank roots and 67 for the 18000 scene fixture.
The assembly display-mode element is fixed-layout, not an extra collection entry.

Seven integration tests compare all ink fields, force exact reconstruction,
check exact global budgets, reject invalid dimensions and every truncated prefix,
retain raw states and string encodings, and exercise a container edit/reopen/revert
with other streams preserved. They also check the grid's physical size before
row reads, omission in older supported layouts, owned source isolation and eight
concurrent edits against the independent edited bytes.

The existing native corpus gate still requires all 1,086 streams and two identical
process results. This family does not complete clause 13; configuration tails,
textured-plane object bodies and other unresolved records remain required work.

The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
