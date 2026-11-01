# MeshControl behavior profile 1 (#161)

The first public MeshControl implementation will target the documented shared
base refinements through C-757. There was no public MeshControl codec in Bessemer
at parent `2695049830e733e1bbbc125476de123ef1921a1c`, so this adopts an initial
behavior rather than changing an earlier public profile.

C-748 and C-749 remain the original retained evidence. The C-749 record-reader
pin is `c3a4b38d836ce236c5591e91d7277f2596218549372b12196db917139a01e89e`.
The adopted current reader is
`7181f48e5d14d24770baf78c13c5454236861c7f4611dfd2735c686a6960d155`,
which is independently pinned by the complete C-757 source manifest already
verified for Shell Property. The source gate accepts only this path and exact
old/current pair, rejects substitutions in all three dimensions, and verifies
the existing complete refinement source bindings.

The refinements add bounded nonzero shared-base virtual records and final curve
records. A zero-count final curve table consumes no records and retains its mode
without applying the nonempty-table mode range. Counted MeshControl records still
use their original count prefix, shared-base framing and derived thresholds.
Every original full C-749 native outcome, including record boundaries and span
hashes, remains mandatory. The four original header/record parser tests are
unchanged and execute from the immutable source copy.

The earlier unused `modelboundary.py` pin exception remains restricted to the
existing reviewed source guard; that module must remain unimported. Embedded
archives use the scoped `cosworks_embedded_const_surface.archive_reader` callback.
This adopts neither a general model reader nor a wider embedded version floor.

The writer/read distinction is explicit. Direct headers have historical reader
gates and a static current-writer three-word grammar. Counted records will retain
their declared reader-version layout through reconstruction and typed edits.
Native writer execution, inferred DocumentInfo dispatch, physical field meanings,
geometry acceptance and whole-format completion remain unproven.
