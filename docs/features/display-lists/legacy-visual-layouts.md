# Earlier appearance layouts

Complete part and assembly fixtures cover versions 2891, 2899, 2900, 2901 and
2902. The reference reader parses each fixture to the end of its stream.
The 50 fixtures provide 5260 independently expected fields, including root
appearance colors, add-in fields, lightweight-rendering state and appearance
names. They also include all five old light classes and assemblies with
explicit external face references.

`use_in_sw_lw_rendering` is stored only after `0xB53`. `appearance_name` is
stored only after `0xB55`. The fixtures use noncanonical signed rendering words
and Unicode names so byte reconstruction cannot pass by inserting defaults.
The generator observes both fields in a complete reference parse before
removing them or replacing their values, then parses the final bytes at their
declared version.

Integration tests compare independent fields and forced writes, check edits
against independently built bytes, and verify source isolation and exact
reverts. Incompatible version contexts, sampled truncated prefixes and every
short tail fail without publishing partial results. The process verifier
regenerates the frozen fixtures and reconstructs them in two fresh processes.

Two additional owned containers cover document context resolution at version
2891. Four cache-backed assembly containers cover versions 2891 and 2901,
before and after edits. Their face directory and complete face block retain
version 19000 independently of the older display version. Both referenced
faces remain complete through edits, container replacement, reopen and revert.
Separate process checks compare the full facet bytes and full container bytes.

This checkpoint expands archive support to version 2891, where the documented
single body-property pointer begins. Older inline-facet payloads remain
explicitly unsupported. Native application acceptance and geometric validity
are not inferred from byte reconstruction. Complete schema parity remains
tracked by issue #28.
