# Documented line-reference roles

Bug #46 includes valid temporary-axis references rejected by fields declared as
`moLineRef_w`. The ordinary temporary-axis serializer calls the line helper,
as recorded in specification evidence C-44; section 11.146 records installed
line-axis data with this concrete reference class. Both complete bodies already
exist in Bessemer. This correction changes the accepted reference types rather
than adding a guessed body or accepting class-name prefixes.

The existing line family first moved from the linear-pattern module to the
geometric-reference module. Six focused tests passed, and all 906 corpus
outcomes and failure causes stayed identical: 418 accepted. That structural
change is separate from the acceptance correction.

The correction uses the same finite family for revolution axes, angle-plane
line references, sketch-pattern directions and intersection lines. The existing
curve-reference family for line-and-point plane data also admits the documented
temporary-axis derivative. Unrelated classes and name suffix lookalikes remain
invalid. Other point, plane and wrapper role findings remain open in #46.

Five independently constructed complete record payloads retain an inherited
line identity, including repeated references in the sketch pattern. The source
reader reaches exact EOF for both line classes without allocating another map
slot. Primitive inventories cover every stored byte. Three Rust tests check
both classes, negative roles, every truncated prefix and trailing bytes.

Complete one-root revolution streams use the owned blank part's actual model
and CMgr context. Null, ordinary-line and temporary-axis cases reconstruct the
whole document byte for byte. The temporary-axis edit matches a separately
constructed complete expected stream, retains one field-history change and
leaves the archive and document inputs unchanged. The independent reader also
checks that the edited temporary-axis parameter equals 3.5.

`reference.json` binds the fixture generators, source readers, owned baseline,
binary fixtures and primitive inventories. Geometry, topology, database paths
and native application acceptance remain outside stored-schema validation.
No dependency or package version changed. The humanizer skill is unavailable
in this session; its prose review remains with the maintainer.

## Verification

The combined local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0:
427 library tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. Two fresh explicit runs reconstructed
419/906 native feature configurations exactly, retaining every previous
success; the broad gate still fails for 487 unsupported cases. Independent
reference reads verified all 419 accepted streams, totaling 16,969,149 feature
bytes, against their four-stream input hashes.
