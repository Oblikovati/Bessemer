# Flow feature identities — issue #241

Specification 14.37 defines record 143's identity-list grammar and record 144's
empty/all-null list profile. The frozen FFI1 fixture establishes their independent
wire expectations before a typed reader and editor are introduced.

The 37 cases comprise 22 identity lists and 15 null lists. Two native payloads
account for 219 bytes: the FLD record-143 list contains three objects and an empty
stored tail; its record 144 is empty. The CPT and CFLD corpus sources contain
neither record. Source hashes are checked before native payload extraction.

Owned fixtures cover list versions 0–6, every established factory class and the
unknown fallback, independent header/body version words, full state bytes and
signed tail boundaries. Text cases distinguish an absent terminator from a
counted NUL, preserve unpaired surrogates and surrogate pairs, and retain units
after the first NUL. Logical identity text stops at that first NUL; raw code units
remain complete. GUID-shaped keys stay UTF-16 text and are not binary header GUIDs.

Versions 0–3 have no stored tail count or values. Six explicit context states
retain missing runtime context, zero and negative counts, a positive count and
the maximum signed count. The runtime count describes a virtual fill of value 2;
the fixture neither materializes a huge vector nor invents a serialized tail.

Python regenerates normalized expectations through the sibling
`flowfeatureidentities.py` in temporary directories. Rust independently rebuilds
every payload, validates exact object/text offsets and logical text prefixes, and
checks complete physical carrier retention. Run
`python -m unittest discover -s docs/features/flow-feature-identities -p test_identities.py`
and `cargo test --test flow_feature_identity_fixtures`.

The typed identity reader and editor remain to be implemented. Populated record
144 belongs to the separate mapped-object grammar in specification 14.38. Native
runtime matching, object merging, alternate string modes, registration side
effects and application acceptance remain separate requirements. This fixture
baseline supplies no new production API or global coverage claim. The maintainer
humanizer pass remains for review because that skill is unavailable here.
