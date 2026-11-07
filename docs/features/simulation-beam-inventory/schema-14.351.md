### 14.351 Reconciled retained BeamManager payload framing

*Status: all C-590 BeamManager stream occurrences have complete retained framing evidence; broader schema/semantics remain incomplete · Evidence: C-727*

A separate generated reconciliation enumerates every retained stream path containing **/BeamManager/** and binds each exact document/path/hash/size to one successful result in C-721, C-724, C-725 or C-726. Missing, extra or duplicate evidence identities fail the check. It verifies successful end/reconstruction hashes and reconstructs the fixed header-word evidence where no generic end record is stored.

| Payload kind | Occurrences | Bytes |
|---|---:|---:|
| Direct manager Property | **353** | **4228** |
| Child beam Property | **163** | **43516** |
| Child beam Material | **150** | **522038** |
| JointGroup | **36** | **29080** |
| JointList | **11** | **1028** |
| Joint | **367** | **20764** |
| Total | **1080** | **620654** |

Every **1080 occurrences / 620654 logical bytes** has complete retained framing evidence. There is no native BeamFolder payload to add to this denominator. The reconciliation does not invent missing material streams or require one per beam merely because 163 Property bodies but only 150 Material streams occur.

`cosworks_beam_inventory_reconciliation.py` produces `cosworks-beam-inventory-reconciliation.json` with the exact identities, source evidence and inventory hashes. This is a retained-outcome reconciliation, not a fresh combined replay against a single implementation. It is separate from the global generated coverage ledger and gives no semantic byte credit. The denominator is C-590's BeamManager payload corpus, not all installed project formats or all historical/context forms. Field meanings, runtime repair/order semantics, native writer equivalence and broader installed-format coverage remain incomplete.

Evidence: C-727.
