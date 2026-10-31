### 14.350 Beam JointGroup, Joint, JointList and static folder header

*Status: all 414 retained joint-related payloads complete; folder header is static/synthetic only and runtime adjacency/order semantics remain partial · Evidence: C-726*

BeamManager enumerates `JointGroups` child storages, constructs a group through **0x180A5F110**, and calls **0x18104B0A0**. This method uses the distinct stream names **JointGroup** and **JointList**, plus child names formatted as **Joint %d** by the writer. Child joint construction through **0x181042660** installs table **0x1820185C0**, whose storage slot **0x1820185E8** selects **0x18104AD80**. C-726 binds these function spans/decompilations, literal bytes, slot pointers and historical gate constants to the installed module hash.

**JointGroup stream.** Read signed word **+0xD0**, persisted string **+0xC8**, double **+0xB8**, and signed words **+0xC0/+0xA8/+0xAC**, in that order. Add the following fields in order:

| Gate | Field |
|---|---|
| **v >= 810023** | Signed word **+0xE0** |
| **v >= 810024** | Signed word **+0xB0** |
| **v >= 810027** | Signed word **+0xB4** |
| **v >= 830029** | Signed word **+0x50** |
| **v >= 810014** | Direct persistent-reference-container body on **+8**, with its versioned reference/archive dependencies |
| **v >= 840023** | Final double **+0xE8** |

The native reader enumerates child joint storages between its reference-container read and final-double read. Those children remain separate storage payloads, not inline bytes inserted into the JointGroup stream. Below the final-double gate it assigns a runtime default without consuming that field. The first/header words are not assumed to be the child-joint count.

**Joint stream.** Read two signed words **+0xF0/+0xF4**. At **v >= 810021**, read the signed two-word identity pair **+0x120/+0x124**. For **810000 <= v <= 810030**, a direct persistent-reference-container body follows; outside that interval it is absent. Then read a signed beam-ID count and that many four-byte IDs. At **v >= 810022**, read three doubles **+0x100/+0x108/+0x110**. At **v >= 810030**, read final double **+0xF8**. Equality at **810030** includes both the older reference container and the final double; at **810031** the reference container is absent.

The native loader searches available beams for each stored beam ID by comparison against beam member **+0xE8**. Missing matches do not justify deleting the stored ID from an independent decoder. This establishes the lookup field in that path, not every ID domain/lifetime or the geometric units of the doubles.

**JointList stream.** Read a signed joint count. For each entry, read a signed neighbor count followed by that many pairs of signed four-byte values. The native loader compares each pair with joint members **+0x120/+0x124** before updating runtime relationships. The independent parser retains every pair and its order without deduplication. The entry index belongs to the native loaded-joint vector; an equivalence to a sorted numeric `Joint N` suffix is not established.

The native reader checks the leading count against its loaded-joint count before reading the grouped pairs. On mismatch or an absent list it may invoke runtime reconstruction depending on saved group state. The bounded parser instead exposes an optional `expected_joint_count` check and fails explicitly on mismatch; it does not synthesize adjacency. All eleven native list counts agree with the independently enumerated direct child-storage counts in C-726. Count agreement alone does not prove matching enumeration order.

**BeamFolder header, without a native fixture.** The manager-selected folder slot **0x181F55660** targets **0x180A5C2A0**. That method first requires a `Beams` child storage, then opens the folder's `Property` and reads a signed word **+0x20** and persisted string **+0x28**. The children use the beam object path from 14.349. There are **zero retained payload occurrences below /BeamManager/BeamFolders/** in C-590; this does not prove empty folder storages never exist. The header has static/synthetic framing evidence only and is not counted as native completion.

**Native replay.** Fresh envelope/nested hash checks and explicit retained candidate arguments complete:

| Carrier | Complete / inventoried streams | Bytes |
|---|---:|---:|
| JointGroup | **36 / 36** | **29080** |
| JointList | **11 / 11** | **1028** |
| Joint | **367 / 367** | **20764** |
| Total | **414 / 414** | **50872** |

All payloads reach exact EOF with contiguous spans and reconstruction hashes. `cosworks_beam_group_layout.py` implements these separate bounded bodies and preserves the inclusive historical gates. Four focused tests pass for gate boundaries, the 810030/810031 overlap change, signed pair retention/count-context agreement, negative-count rejection, all truncations/trailing data, folder-header truncation and unsupported kinds. C-726 retains native attempts, actual JointList child counts and installed/source bindings in `cosworks-beam-group-native-evidence.json`.

No native enumeration, ID lookup, relationship reconstruction or writer executes. Physical values, runtime ordering/repair behavior, unsampled folder/context forms and full installed-format coverage remain incomplete. Global semantic byte credit remains zero.

Evidence: C-726.
