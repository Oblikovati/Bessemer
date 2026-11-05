# Flow feature identities — issue #241

Specification 14.37 defines record 143's identity-list grammar and record 144's
empty/all-null list profile. The frozen FFI1 fixture establishes their independent
wire expectations independently of the typed identity reader and editor.

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
The typed API checks run with `cargo test --test flow_feature_identities --test flow_feature_identity_fixtures --test flow_carriers`.

`FlowCarrier::feature_identities` reads the last record 143 with positive object,
total text-unit and stored-tail bounds. It retains the source checkpoint and
revision-bound occurrence, exact entry intervals, raw/identity code units, factory
classification, independent version words, state bytes and stored or virtual
tail state. No runtime vector is allocated from the legacy count.

`replace_object_field` edits an existing header, type code, body word, state byte
or full counted wire string on a copy. Text edits may change length while retaining
unrelated entries and the tail. They validate the total unit budget, terminator
and carrier/payload growth before allocation. `replace_tail_value` edits existing
stored values only; legacy descriptors cannot acquire invented wire values.
The returned `FlowFeatureEdit` retains source/result typed views, exact coverage,
diagnostics and complete physical carrier history. Each complete patched payload
is reparsed and checked before publication. Fresh opposite edits restore exact
original carrier bytes, including after string length changes.

The record-144 null profile remains a fixture baseline; populated record
144 belongs to the separate [mapped-object grammar](../flow-mapped-objects/README.md)
in specification 14.38. Native
runtime matching, object merging, alternate string modes, registration side
effects and application acceptance remain separate requirements. This fixture
baseline supplies no global coverage claim. The maintainer
humanizer pass remains for review because that skill is unavailable here.
