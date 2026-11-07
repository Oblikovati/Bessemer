# PhotoWorks material managers

Issue #54; behavior version 1. Section 11.323 / C-365 binds the optional
configuration +608 branch to a direct PhotoWorks manager. The stored presence
byte stays unchanged. A present manager contains two signed words followed by
a direct packed-count list of nonnull tagged material-property objects. The
manager and its allocated list have no outer MFC header or object identity.

The property-data layout preserves every typed scalar, point, vector and
CString in native order. All historical property gates precede the supported
ordinary modern minimum. Two direct nested lists preserve nulls, cached
identities and fresh unit-component bodies. Fresh unrelated classes stop before
their bodies. Cached references may point back to the material itself; they
retain their original class and identity. Manager entries require the exact
material class even for cached references.

Native load rewrites some default words and material paths. Bessemer retains
the stored values. A zero-valued property body with empty lists is exactly 601
bytes. Material parameter meanings remain unnamed; typed wire coverage does
not establish physical material semantics.

Independent fixtures cover 24 property graphs across four modern versions and
six scalar/list styles, with 2,880 primitive observations. They include signed
extremes, exact IEEE bits, Unicode/extended CStrings, packed/extended counts,
shared units and recursive material aliases. Six direct manager records and
seven complete CMgr streams include a material-edit golden. The actual complete
Source reader reaches exact CMgr EOF and the expected archive-slot count for
every owned stream. All fixture bytes and primitive partitions regenerated from
an immutable Source copy.

A separate Source replay reproduced the three native managers and their 2,512
bytes, including three nested properties totaling 2,448 bytes. Two Source
regressions passed for scalar order, original defaults, count/type/context
rejection and reader restoration. Eleven focused Bessemer tests and three public document tests passed. Two
independent native replays agreed on every status and failure cause: 902 of
906 configurations passed, up from 901, with no regressions. The added case is
bikeframe. Source reached exact EOF across all 82,924,680 resolved-feature bytes
for these 902 accepted configurations. Four native cases remain unsupported;
the corpus is incomplete. The full local `cargo xtask pr` gate passed: 2729 tests, including
1188 library tests and 277 doctests; three external-resource tests are
explicitly ignored in that gate.

This feature does not establish complete configuration, part, assembly,
drawing or installed-project semantics. The maintainer humanizer skill was
unavailable; prose review remains with review.
