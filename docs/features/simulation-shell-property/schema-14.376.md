### 14.376 Nonzero shared-base virtual records

*Status: shared virtual body decoded and synthetically bounded; native optional records remain absent · Evidence: C-756*

The shared base's two presence-controlled records call outer virtual **+50** when their presence word is nonzero. All five concrete primary tables checked here bind that slot to **180B72100**: Shells `18205A7D0`, MidSurfShells `18205AB78`, MeshControl `182032468`, ContactComponent `181F5A2C0`, and ContactPair `181F59F30`. C-756 freshly verifies those five installed pointers and retains the delegate's instruction/decompilation hashes. These are **primary** tables; the +8 serialization-subobject tables used for whole records are different.

The delegate consumes, in order:

| Payload | Condition |
|---|---|
| Counted CwPersistStream UTF-16 text | Always |
| Five signed four-byte header words | Always |
| Nine raw doubles / 72 bytes | Only when header word 2 equals 4 |
| One signed four-byte word | Version >=820011 (`181F61F88`) |
| Interleaved double pairs / 16 bytes each | Positive count in header word 5 |

The native allocation stores the pair axes in separate arrays, but reads one first-axis double and one second-axis double per loop, so the **wire order is interleaved**. Native object construction and virtual setter calls consume no additional fields. Header business meanings and actual curve/model interpretation remain unassigned. Independent bounded parsing rejects negative/over-limit counts instead of preserving an unsafe native allocation behavior.

The existing bounded optional-record helper implements this recovered grammar and is now enabled for nonzero shared-base presence in Shells/MidSurfShells Property, MeshControl, ContactComponent and ContactPair. Version threshold, optional nine-double section and following parent-field boundary are explicit; there is no fixed-length skip for a variable body. Synthetic tests cover zero/two pairs, header modes 0/4, negative count rejection, truncation and remainder. The previous unsupported-branch test now checks an actually truncated body because complete nonzero bodies are supported.

C-756 freshly replays **all 378 affected native streams / 1489023 bytes**, proves every prior successful candidate retains its endpoint and reconstruction hash, and records **zero native populated virtual bodies** and **zero native populated base curves**. Twelve focused tests pass. Thus the new branch has static/synthetic validation only; complete native stream counts do not increase merely because an unrepresented branch is implemented.

This resolves the nonzero +50 wire-body boundary retained in earlier clauses for the verified concrete tables. Other concrete overrides, native populated fixtures, newest shell layer flags, historical carrier mismatches and field semantics remain open. No target executes; global semantic coverage credit remains zero and complete installed-format coverage remains unproven.

Evidence: C-756.
