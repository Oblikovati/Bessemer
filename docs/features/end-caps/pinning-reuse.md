# Reference runner reuse

The immutable reference runner now lives in
`docs/features/model-archives/pinned_reference_tests.py`. The end-cap command
keeps its existing arguments and fixture expectations. The shared runner accepts
the fixture directory, freeze module and required native reader modules so the
weldment-finishing fixtures can use the same import and hash checks.

The end-cap regeneration tests compare all 306 graphs, primitive observations
and 21 owned streams with their existing goldens. This refactor changes no
reader body or fixture byte. Imported live source modules still fail the check;
temporary sources are removed after verification. Humanizer is unavailable;
maintainer prose review remains open.
