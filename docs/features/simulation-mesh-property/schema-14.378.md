### 14.378 Mesh Property initial layout and versioned dependency map

*Status: study-bound serializer and initial gates recovered; full native Mesh replay remains open · Evidence: C-758*

The study storage reader opens **Mesh** (`181F540A8`), obtains/constructs its object through **1810FE4E0**, stores it at study +1D8, and calls **181109480** with the supplied study version. This storage method independently opens Property. C-758 binds its full native function/decompilation hashes, study caller and **27 numeric version constants**, and inventories all **357 retained Mesh Property streams / 94876 bytes**. These bytes are not yet assigned complete framing by this static checkpoint.

The initial read order is three signed four-byte words (+594/+598/+5DC), three raw doubles (+5A0/+5A8/+5D0), word +284, then counted UTF-16 texts +5F0/+5F8. Continue in the following wire order; destinations are in-memory identifiers, not assigned physical meanings:

| Version condition | Initial continuation |
|---|---|
| >=800005 | two four-byte Windows-long values +708/+70C |
| **800078 <= v < 810000, or v >=810004** | word +37C |
| >=800035 | four doubles +710/+718/+720/+728 |
| >800037 | word +188 |
| >=830018 | doubles +5B0/+5B8 |
| >=830020 | word +5E0 |
| >830033 | four texts +600/+608/+610/+618 |
| >=840018 | word +378 |
| >=890514 | words +730/+734 |
| >=890601 | doubles +560/+568 |
| >=890605 | one word (destination omitted in decompilation) |

The +37C gate is deliberately non-monotonic: the field disappears for 810000..810003 and reappears at 810004. Flattening it to a single minimum would misalign those profiles. Older default assignments and size comparisons mutate in-memory values without consuming the omitted bytes; raw doubles must remain preserved before native normalization.

From **800021**, the next region is a signed count and counted-text list (manager +620), then a virtual +80 delegate on object member **+8A8**, then word +738 from **800040**. This delegate is an inline Property dependency, not a CFB child stream or byte-counted opaque block. From **800074**, another signed count controls records allocated through **1810FDAC0** and serialized by **181119560**. Both dependent layouts must be bounded before the later Property suffix can be replayed universally.

From **810006**, a duration-like field region reads four-byte Windows-long values; before **840017** the native count defaults to one, and from that version an explicit signed count precedes the values. Runtime division/remainder by 60 changes stored in-memory representation but adds no bytes. The physical units/meaning remain unassigned. From **840013** word +9B0 follows. From **850001**, a signed object count controls records of five words, two signed array counts, then the first and second word arrays in sequence; native allocation uses those counts, so independent bounds are required.

Later suffix gates include a temporary word at 890104, word +2AC from 890111, words +89C/+8A0 from 890512, and additional dependent regions from 890703/890907/890909/891103. C-758 retains their native source/gate bytes but this clause does not substitute a guessed constant-length tail. Decode the +8A8 virtual object, 181119560 records and newest regions before granting complete Mesh stream framing.

No target executes. This is static field/dependency evidence only; native replay, historical writer symmetry, populated array bounds, field meanings and complete installed-format coverage remain open. Global semantic coverage credit remains zero.

Evidence: C-758.
