# Binary transmit vectors

`blank-partition.bin` and `blank-deltas.bin` are exact inflated chunks from the
owned blank part's `Contents/Config-0-Partition` stream. The six other binaries
were independently assembled from clause 10, with neutral and bare variants.
They include every primitive kind, fixed and variable arrays, empty arrays,
all four layout-edit operations, fully defined types, pointer-index boundaries,
unknown pointer flags, delta hvecs, and a delta ending after its first list.

`base-schema.s_t` is an independently assembled test schema, not a redistributed
installed schema file. Its PARTITION, PMARK, and WORLD declarations cover the
native blank streams. Its synthetic types exercise primitive handling and
declarations for `q` and `t`, whose transmitted encoding remains unsupported.

`docs/features/parasolid-transmit/generate_fixtures.py` generates the six synthetic
binaries and verifies them with the Python reference parser using an explicit
fixture schema source. Normal Rust tests embed all inputs and do not access an
installed SolidWorks directory. Fixture changes require a reason and the
transmit behavior version in the feature notes.
