### 14.365 Shell Property base, derived gates and inline dependency boundaries

*Status: static field order/version constants recovered; native child Property replay remains open · Evidence: C-743*

The bound shell storage method `181345A50` delegates its Property prefix to **180B754B0**, passing object +8 and the unchanged outer version. C-743 retains both function-byte/decompilation hashes and all nineteen referenced numeric version constants. The retained child inventory is **234 Shells Property streams / 826556 bytes** plus **15 MidSurfShells Property streams / 15770 bytes**. This inventory does not prove both families use the identical concrete virtual suffix; the current constructor chain binds Shells, and MidSurfShells dispatch remains separate verification work.

The shared base starts by delegating `CSWPersistReferenceContainer::Serialization`. Do not infer a persist-ID-only prefix: it is a reference **container**, with its own count/reference/tail grammar. The following offsets are relative to the base object pointer passed as outer shell +8, not file offsets. After the container, read word +198; four counted texts (+78, two temporary compatibility strings, +60); words +68/+58; double +F8, word +EC, double +E0, unsigned word +E8, and words +110/+A8/+1E0. Read word +100 only above **800027**, then six doubles starting +118 and six counted texts from the +80 array. The current writer emits +100 unconditionally; historical reader gates remain authoritative for its consumed layout.

Continue in wire order:

| Native version condition | Base continuation |
|---|---|
| >800001 | Two records, each word +218/+228 then presence; nonzero presence calls outer virtual +50 with destination +210/+220 |
| >800002 | word +AC |
| >800037 | word +48 |
| >=800066 | word +230 |
| >810001 and >=870006 | word +C8 |
| >810001 and >=870010 | word +170 |
| >810001 and >=880008 | double +108 |
| >810001 and >=890009 | three words +148/+14C/+150, then texts +158/+160 |
| Previous row and >=890011 | word +2E0, signed count, then conditional delegated 180D59E50 records |

The final count path is conditional on +2E0 lying in the native range -100 through 100. It clamps retained count to 0..50 before calling `180D59E50`; this is not evidence that arbitrary excess serialized records are consumed. Preserve the original count and report unresolved excess instead of silently dropping bytes. Earlier compatibility calls and default assignments consume no additional fields after the listed grammar. Nonzero virtual +50 records and final delegated records remain explicit dependencies.

After the base, the derived shell reader consumes double +328, words +330/+334/+340, texts +300/+348, and two unsigned words copied later to +338/+33C. At **>=820017**, two more unsigned words target +350/+354. At **>830008**, read word +358; inside that branch, **>=840005** adds word +368 then double +360. Native +368 values 1 or 2 overwrite the loaded double with +0.5 or -0.5 in memory; preserve the original wire bits.

When loaded subtype +334 equals **2** and version **>=830010**, read four words +36C/+370/+374/+378. +36C controls the following layer loop: two doubles per layer, then, only when +378 equals zero, a material-presence word and a complete **inline Material body** through `1816E15A0` when nonzero. This body has the shared Material grammar, but is part of Property and is not a sibling Material stream. From **891106**, an existing layer returned by helper `18133E980` also controls reading a flags word; bits 1 and 2 independently introduce records through `180D48D90`. Its version-independent record layout must be decoded before a complete newest-profile claim.

CompositeExtra/CompositeFlipTurn are opened independently during that composite branch and retain 14.364's separate profiles; they add no inline Property bytes. After the layer/auxiliary-stream work, version **>=870022** adds word +480 and **>=890006** adds counted text +488 to Property. Current writer output and loaded-value normalization do not replace any historical reader gate.

C-743 grants no native child-Property completion or global semantic coverage credit. Reference-container framing, nonzero virtual records, inline Material boundaries, newest curve records, MidSurfShells concrete dispatch, and field meanings must be verified in a bounded native replay. No target executes.

Evidence: C-743.
