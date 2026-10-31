# SurfaceBody stream boundary

Feature [#153](https://github.com/Oblikovati/Bessemer/issues/153) covers the
separate sheet-body `SurfaceBody` stream in 14.356/C-733 and retains the
historical mismatch investigation in 14.362/C-740. The corpus contains 232
streams /34,672 bytes, with 237 original version arguments: 228 accepted and
nine failed alternatives. Only 226 streams /33,778 bytes have a complete
candidate; the remaining six streams /894 bytes stay incomplete.

The declared grammar owns word70/text68, historical signed and mixed
word/double suffixes in physical order, bounded list108 and gated word144.
Neither a persist-ID prefix nor inherited sheet Property belongs to this
stream. Raw values and text frames remain separate from physical meanings.

At version 860006 the six C-740 cases stop before `words100_94`, which needs
eight bytes while only four remain. Exact consumed prefixes, raw spans,
unclassified bytes, failure messages and version metadata are retained. There
are no complete same-version comparators in the report. These bytes are not
a valid suffix after a complete parse. Do not omit/insert fields, pad, retag,
guess versions or widen tolerance to force successful EOF.

The foundation freshly recovers every identity and original result, checks all
independent historical/raw/list literals and preserves every exact C-740
failure. Full reports, clauses, source/decompilation hashes and installed
image/function/threshold/storage bindings remain retained. Source imports run
from an immutable copy; no target code executes.

Run `python -B -X utf8 docs/features/simulation-surface-body/freeze_surface.py`
for strict replay. `--write` initializes evidence for review. Generated native
binary packets stay under `target/`; original reports retain decoded values
and raw spans as provenance.

Production operations, malformed/resource, ownership/concurrency and
local/platform gates remain pending. Complete independent field and ordered
edit-history expectations cover all 96 owned profiles and 228 accepted native
arguments. Fresh strict replay reproduces them unchanged. Rust operation parity
with those expectations remains unproven. Whole manager
inventory, resolving the six historical failures, native writer/application
and geometric acceptance require separate evidence.

Initial write and strict replay passed all eight checks and their processes
were reaped. All original outcomes, exact historical failures, 96 independent
profiles, installed bindings and three original tests match. See the
[foundation ledger](foundation-verification.json).
