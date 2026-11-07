### 14.347 Shared Simulation Material grammar and delegated table bodies

*Status: both retained mass-child Material payloads complete; other material contexts, populated point tables and physical meanings remain open · Evidence: C-723*

Mass-element storage from 14.346 delegates `Material` to **0x1816E15A0** after construction through **0x1816D5780**. Its nested table groups invoke **0x180FFDFD0** directly. These are CwPersistStream fields, not tagged object archives or another common-load-base body. C-723 binds all three installed function spans/decompilations, eighteen floating threshold constants and three immediate integer thresholds to the verified module hash.

**Header and property records.** Read two persisted strings at runtime **+0x20/+0x28**, signed four-byte **property_count**, then three raw four-byte words **+0x34/+0x38/+0x3C**. Read strings **+0x48/+0x50/+0x58**, word **+0x68**, and strings **+0x60/+0x08**. Runtime clamping of +0x3C to a supported range does not alter the retained stored word.

For each property, read these fields in order: persisted name; signed four-byte kind; one double; auxiliary string; signed four-byte associated-table presence; table-label string; signed four-byte point count; four more four-byte words. If presence is nonzero **and** point count is positive, read that many **pairs of doubles**, interleaved on the wire. At **v >= 800010**, read one additional four-byte word after those pairs, including when the table was absent or empty. The independent parser bounds counts and preserves every declared record/pair in order; it does not apply the native runtime property's lookup or table insertion effects.

Next read a separate direct global table: label string, signed point count, three four-byte words, then that many double pairs. There is no presence word on this global table. The native implementation stores the two coordinates in separate runtime array halves but reads/writes them interleaved as pairs; those memory halves are not two contiguous wire arrays. Auxiliary texts, scalar units and word domains remain unassigned.

**Versioned remainder, in order.**

| Gate | Additional fields |
|---|---|
| **v > 800016** | Two words **+0x3900/+0x3904** |
| **v > 800028** | Word **+0x6C** |
| **v >= 800039** | Words **+0x70/+0x74**, signed group count, then that many direct delegated table bodies |
| **v >= 800049** | Four words **+0x3958/+0x3988/+0x398C/+0x3990**, signed second-group count, then that many delegated bodies |
| **v >= 810004** | String **+0xA0** |
| **v >= 820002** | Word **+0x3994** |
| **v >= 830001** | Word **+0xA8** |
| **830003 <= v < 830028** | One temporary compatibility word, retained even though the native reader does not assign it to a persistent member here |
| **v >= 830014** | Word **+0xB8** |
| **v >= 830016** | Strings **+0xB0/+0xC0/+0xC8** |
| **v >= 850023** | String **+0xD8** |
| **v >= 880011** | Word **+0xE0** |
| **v >= 890003** | Double **+0x78**, words **+0x80/+0x84/+0x88** |
| **v >= 890013** | Words **+0x8C/+0x90** |
| **v >= 890022** | Word **+0x94**, double **+0x98** |
| Otherwise, **890015 <= v < 890022** | Double **+0x98** without the preceding +0x94 word |

Absent-field zeroing, enum repairs and conversion of +0x84 into a default +0x98 value are runtime actions, not extra fields. The table above is the recovered reading grammar; it does not claim all current storing paths reproduce every historical form.

**Delegated body 0x180FFDFD0.** Each body begins with a persisted string at **+0x08**, then seven signed words **+0x20/+0x18/+0xA0/+0xA4/+0x98/+0x9C/+0x1C**. At **v > 800032**, word **+0x100** follows. Read nine doubles **+0x28..+0x68**, then word **+0x90**, double **+0x88**, word **+0x94**, doubles **+0xB0/+0xB8**. Read signed point count **+0x70**; at **v > 820010**, read word **+0xF8**. Then read the counted double pairs, and at **v > 830015** read final string **+0x10**. No tagged class marker or per-body byte-size prefix occurs. The compiled integer comparisons use inclusive thresholds **800033**, **820011** and **830016**, equivalent to the strict gates above.

**Native verification.** C-723 freshly reextracts the two exact `MassElement 1/Material` streams retained as undecoded in C-722, checks their outer/nested hashes and original raw bytes, and completes both **3548-byte payloads / 7096 bytes** at exact EOF under retained candidate **830036**. Each has nine named properties: **EX, NUXY, GXY, DENS, SIGXT, SIGYLD, ALPX, KX, C**. These are literal stored names; no complete physical/unit mapping is inferred from them. Each has ten delegated bodies in the first group and five in the second. All observed property/delegated point tables are empty, so populated point arrays retain static/synthetic evidence only.

`cosworks_material_layout.py` preserves contiguous raw span/reconstruction hashes, count/presence distinctions and every original string/number bit. Negative, over-limit, truncated and trailing data fail explicitly. Four focused tests pass over **41 version profiles**, including all recovered gate boundaries, nonzero-presence versus count-only behavior, populated synthetic point arrays/delegated groups, every-byte truncation/trailing bytes and version/count limits. `cosworks_material_audit.py` retains results in `cosworks-material-native-evidence.json`.

This completes the two mass-child material occurrences, not all Simulation material carriers or all semantics. The C-722 `decoded=false` snapshot is superseded for those exact identities by C-723, while its original bytes/provenance remain retained. No target code or native writer executes. Other contexts, field meanings, runtime material-model behavior, historical variants and full installed-format coverage remain incomplete; global semantic byte credit remains zero.

Evidence: C-723.
