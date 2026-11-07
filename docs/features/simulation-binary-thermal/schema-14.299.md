### 14.299 Embedded wrapper historical tails and first component declarations

*Status: first nested component headers verified for all 48 nonnull archives; wrapper/face-edge bodies remain partial · Evidence: C-675*

**First nested object.** The retained installed wrapper serializer **0x4C2842D0** begins its read branch by restoring a tagged component-reference pointer. A sketch-entity-handle component can require an additional sketch pointer before the common wrapper fields. C-675 freshly reextracts and hash-verifies the **48** nonnull first archives from C-672, enables only a minimal outer face/general-curve reader that invokes this first object read, and leaves all component body readers disabled. The resulting declarations are **46 `moCompFace_c`** and **two `moCompEdge_c`**, each **schema 1**. Every attempt stops immediately after its component class header with the corresponding missing-reader boundary. No model-body context guard is bypassed.

This supplies native nested identity at a proven parse position, rather than interpreting ASCII strings found elsewhere in the payload. The first **46 face wrappers** lead to face components and the two general-curve wrappers lead to edge components in this sample set. It does not establish that arbitrary wrappers are limited to those combinations, or that edge components can be consumed with the face continuation.

**Common wrapper order in the observed version range.** For these nested archive versions (all at least **4307**), after the first component object and any sketch-specific branch, the retained read body consumes: a **u16** to wrapper **+0x60**; a tagged component pointer to **+0x58**; integers **+0x64/+0x08**; a **u16** presence value and optional `mgXform_c` body for **+0x10**; another presence/transform for **+0x18**; integers **+0x0C/+0x20/+0x6C/+0x70**. It then reads the following version-dependent tail in order:

| Order | Condition / wire fields |
|---|---|
| 1 | Version **> 0x2FAF (12207)**: integer **+0x74**. |
| 2 | Integers **+0x4C/+0x68**, present throughout the observed version range. |
| 3 | If archive-helper member **+0x834** is nonzero: one four-byte configuration presence value; if nonzero, a configuration body through the referenced-configuration virtual serializer. |
| 4 | Version **> 0x1FD1 (8145)**: two integers **+0x34/+0x30**. |
| 5 | Version **>= 0x2B73 (11123)**: integer **+0x90**. Older reads derive a runtime value instead of consuming it here. |
| 6 | Version **>= 0x3B02 (15106)**: integer **+0x38**. |

All integer fields above consume four bytes even where the runtime later stores, merges or interprets selected bits. The helper **+0x834** condition is runtime context, not inferable from the object class header or archive version alone. Other helper-driven map registration in this body adds runtime bookkeeping rather than identified bytes.

**Why existing guards remain.** The modern ordinary-context wrapper in 11.16 assumes its documented version range and clear helper **+0x834** flag. Embedded archives at versions such as **4307, 5000, 6000** omit later tail fields that the modern path expects. Their first component bodies also have independent historical branches. A new scoped reader needs evidence for both the helper context and those nested bodies; simply assigning an ordinary-context flag would not establish compatibility. The installed component serializers retained by C-30 are the next source: face/shared boundary body **0x4BBB7E80**, edge body **0x4BBB92E0**, and common component-reference base **0x4BBBA690**.

C-675 retains the installed-wrapper decompilation hash, C-674 static-module evidence hash, fresh native header bytes and offsets, and decoder/source hashes. No component body or whole additional Pressure stream is credited by this header-only pass. No target execution or global semantic byte credit occurs.

Evidence: C-675.
