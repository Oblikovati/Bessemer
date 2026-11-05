# RGB555 preview work

This is the preimplementation design record. The implementation and focused
verification are documented in [README.md](README.md); full gates remain pending.

Feature #210 will add raw-word reading and replacement for 16-bit BI_RGB previews.
The existing pixel enum will remain unchanged. The reconstruction example will
emit little-endian words for this format. BI_BITFIELDS and extended headers remain
separate work; this does not establish complete preview or schema parity.

The first Windows probe checked all 32 levels of each channel in a 32-by-3 image
with every unused high bit set. RGB results matched `(level << 3) | (level >> 2)`;
`native-channel-probe.json` retains the observations. Production word APIs will
preserve every stored bit and will not perform this display conversion.

Before format admission, shared row-layout arithmetic will be extracted and
checked against the unchanged indexed/BGR24/BGRX fixtures. The extraction retains
the existing bit-unit arithmetic for indexed formats and byte-unit arithmetic
for byte-aligned formats, including their checked overflow boundaries. It removes
the superseded private size helpers; it does not add a fallback or pixel format.

Acceptance still requires all raw u16 values, literal original/edited archives,
independent native pixels, both row orientations, odd/even widths, optional
palettes, explicit/zero sizes, full document reopen and fresh reversal, malformed
bounds, isolation, concurrency and process comparisons. Full local and exact-head
remote gates remain required. No native SolidWorks RGB555 population, application
opening, geometric acceptance, dependency, package version or release tag change
is claimed. Humanizer is unavailable; maintainer prose review remains pending.
