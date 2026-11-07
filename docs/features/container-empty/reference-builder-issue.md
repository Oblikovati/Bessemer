The current reference solidworks/swcontainer.py build([], key, table, filler=False) raises UnboundLocalError for off at the assignment last_local = off after its empty stream loop. This occurs even though its Document reader accepts the independently assembled documented 30-byte empty container for all 1,000 registered keys.

Reproduction uses the provided cache/keytable.json and the existing Source function with an empty stream list. No native application or binary execution is involved. The reference code and cache must remain independent of the Bessemer fix; do not change the oracle to make Bessemer tests pass.

Track this reference defect separately from the Bessemer reader bug. Empty-wire expected results will be assembled from the documented header/end layout and independently accepted by the Source reader. Fixing the upstream reference helper itself requires its repository workflow; this issue does not authorize silently changing its semantics or coverage.
