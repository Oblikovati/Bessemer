# Header vectors

The three `blank-*.bin` files are exact `Header2` streams extracted from the owned
blank SolidWorks documents in the parent directory. The 77 synthetic files were
assembled independently from clause 11.8 with
`docs/features/header-archives/generate_fixtures.py` and checked by the reference
Python reader. They cover both sides of every version boundary and three
temporary body-version overrides. Synthetic paths and names are invented.

The Rust tests force reconstruction from typed fields and compare all bytes with
these independent vectors. The generator also writes `tests/support/header_cases.rs`;
run `cargo fmt` after regenerating it. Fixture updates require the reason and
header behavior version in the feature documentation.
