# Flow GUID lookup and related carriers — issue #241

Specification 14.33 assigns the stored header GUIDs lookup roles. CPT lookup uses
record 123's second GUID; FLD lookup uses its first GUID; static-settings lookup
uses record 435's GUID. The caller supplies typed classifications and candidate
order. Carrier version alone does not identify a result type.

`find_flow_carrier_by_guid` retains every supplied candidate and returns the first
eligible match. CPT accepts type 1, FLD accepts type 2, and static-settings lookup
accepts types 1 and 4. The visited-prefix coverage includes skipped types and
counts checked eligible headers separately. Candidates after the first match and
ineligible candidates are not loaded or validated by this lookup.

Only the fresh result-info profile is modeled. Nil or absent candidate GUIDs leave
empty native key strings and cannot match a formatted nil query. For non-nil
GUIDs, exact binary equality is equivalent to comparing their canonical Windows
GUID strings without case sensitivity. Query inputs remain typed sixteen-byte
GUIDs. Reused result-info objects with stale text fields are outside this profile.

`FlowCarrier::related_carriers` first finds a CPT using the seed's second GUID.
The static-settings lookup executes only when a CPT matches, its literal path is
nonempty and the seed has a non-nil settings key. The result retains the seed,
both lookup checkpoints when executed, and a typed settings-stage gate. A skipped
lookup differs from an executed lookup that found no match. Candidate paths stay
literal, including empty paths; no path normalization or filesystem access occurs.

The FLK1 fixture records 21 candidate contexts, 1,260 queries and 21 related-file
selections. It includes the three source-bound native GUID-header selections,
every ordering of three owned candidate types under versions 7 and 8, nil seeds,
empty candidate lists and ineligible-only lists. Queries distinguish all sixteen
GUID byte positions. Expected selections and visited-prefix counts come from
`../inventor-interface/solidworks/flowlinks.py`. Literal tests additionally pin the
type-4 filter, first-match behavior and nil exclusion. Temporary regeneration
verifies source hashes and does not rewrite the frozen fixture.

Run `cargo test --test flow_links --test flow_headers --test flow_settings` and
`python -m unittest discover -s docs/features/flow-links -p test_links.py`.
Directory enumeration, path validity, native execution, result payload loading
and complete Flow acceptance remain separate work. The empty-CPT-path test comes
from the explicit native gate in the specification; the sibling replay assumes
already validated nonempty candidate paths. No additional direct byte ledger or
global schema coverage is claimed. The maintainer humanizer pass remains for
review because that skill is unavailable in this environment.
