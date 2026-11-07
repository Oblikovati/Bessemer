#### 7.3.1 Native ZIP timestamp packing and entry-metadata initialization

*Status: static writer path and exact arithmetic verified; timestamp provenance remains partially open — Evidence: C-934*

The installed `sldmfcu.dll` storage commit reaches3CA493C0, which dispatches entry staging through backend slot+40 and commit through+18. The backend factory3CC39970 constructs3CC3A8F0, which installs table3CF51C80. Its slots+18,+20,+40 select3CC3B870,3CC3E570,3CC3F6F0 respectively. Entry staging invokes+20, and entry-open modes1 and2 call wrapper3CC41C70 with a metadata pointer. The wrapper forwards to header core3CC41D80. These bindings distinguish ZIP entry metadata from Header2 document times and filesystem save dates.

The metadata begins with six uint32 words in order: seconds, minutes, hours, day, zero-based month and year. Word at+18 is the packed DOS date-time. The header core uses that packed word unchanged if nonzero. For a nonnull metadata pointer whose packed word is0, it computes the following with uint32 wrapping, unsigned right shift and no per-field range checks:

```
y = year
if y > 1980: y = y - 1980
elif y > 80: y = y - 80
time = ((hours * 64 + minutes) * 32 + (seconds >> 1)) mod 2^32
date = (((y * 16 + month + 1) * 32 + day) * 65536) mod 2^32
packed = time OR date
```

A null metadata pointer selects packed0. Year2026 and year126 both yield adjusted46, but year1980 follows the second branch and yields1900; the exact boundaries must not be replaced with a calendar-normalizing implementation. Overflow in the nominal time part is not masked to16 bits before OR with the date part. The core retains the packed value at backend+40E0 and emits its bytes in the directory date/time fields. `ziptimestamps.native_timestamp` models only these explicit raw inputs, without timezone conversion or wall-clock inference. Four tests cover ordinary dates, both year conventions, exact threshold behavior, noncanonical words, nonzero override and input bounds.

In the identified entry-open mode1 path, the caller passes metadata at stack+60 and zeros its tail at+78/+80; in mode2 it passes metadata at stack+88 and zeros the tail at+A0/+A8. The first24 bytes, containing the six calendar words, have no corresponding assignments in these caller branches. Thus the zero packed word requests conversion of calendar words that this caller has not initialized. Static evidence does not recover their actual prior stack contents, prove every possible indirect effect or predict a particular retained file's malformed word. The writer has no validation in this packing step that would force those values into valid DOS dates.

The C-934 audit replays ten retained native function ranges and verifies the concrete backend slots and packing instructions against installed bytes. This narrows OI-001 to actual input provenance, retained-file attribution, other paths and cross-release behavior. It does not claim deterministic timestamp reconstruction, a save experiment or complete resolution of OI-001. Readers continue to preserve the raw words without depending on their calendar validity; existing writer preservation guidance remains applicable. No coverage ledger credit or target execution is involved.

Evidence: C-934.

#### 7.3.2 Complete installed-corpus timestamp-copy and calendar audit

*Status: complete installed part/assembly/drawing container-header comparison — Evidence: C-935*

C-935 reads every container selected by the installed sample collector:919 parts,176 assemblies and20 drawings,1115 documents in total. For every central-directory entry, it follows that entry's recorded local-header offset, reads both raw timestamp pairs and compares the packed32-bit words. It retains source file hashes, container keys/versions, entry names, both offsets and both words. It does not decompress payloads or claim payload-semantic verification.

| Observation | Count |
|---|---|
| Local/central entry pairs compared | 52,454 |
| Exactly equal packed timestamp words | 52,454 |
| Different local/central timestamp words | 0 |
| Entry words forming an ordinary valid DOS calendar date/time | 2,390 |
| Entry words failing ordinary DOS calendar validation | 50,064 |
| Distinct complete packed words across all entries | 421 |

Calendar classification decodes seconds as twice bits0–4, minutes from bits5–10, hours from bits11–15, day from bits16–20, month from bits21–24 and year1980 plus bits25–31, then checks the resulting date/time. This is an observational classification of these fields, not a document-validity gate, date repair, source-clock inference or deterministic inversion of7.3.1's unchecked uint32 arithmetic. Invalid seconds/minutes/hours and invalid calendar dates are all retained unchanged.

Every document has between3 and9 distinct local timestamp words. The exact document counts by distinct-word count are:3→843,4→194,5→19,6→1,7→1,8→55,9→2. These observations confirm the duplicated-word relation and grouping for this corpus, but do not identify which concrete writer path or prior calendar-word inputs produced an individual value. OI-001 therefore remains open for provenance and cross-release attribution. The1115-document corpus is not a denominator for all installed project formats; no coverage ledger changes or target execution are involved.

Evidence: C-935.
