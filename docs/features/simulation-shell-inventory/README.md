# Combined ShellManager inventory (#160)

This gate binds the complete C-746 denominator to its original C-590 stream
identities and five evidence reports. The fresh reference replay recovers each
document envelope and nested stream, checks exact names, lengths and hashes, then
compares every original reader/version/profile outcome with copied reference code.
No SolidWorks code executes.

The denominator contains 936 unique identities and 1,648,966 logical bytes. There
are 1,008 original arguments: 962 accepted and 46 rejected. Under their explicit
profiles, 935 identities have at least one complete argument. The remaining
4-byte CompositeExtra payload fails both original profiles at version 840030.
Eight other CompositeExtra payloads retain their reader failures and writer
successes. Overlapping refinements add no identities to this denominator.

| Family | Identities | Bytes |
|---|---:|---:|
| Direct manager Property | 361 | 2,888 |
| ShellManagerGroup | 69 | 276 |
| Shells Property | 234 | 826,556 |
| MidSurfShells Property | 15 | 15,770 |
| Material | 237 | 802,740 |
| CompositeFlipTurn | 11 | 220 |
| CompositeExtra | 9 | 516 |

Run the metadata gate from the repository root:

```text
python -B -X utf8 docs/features/simulation-shell-inventory/freeze_inventory_bindings.py
```

The reference gate also needs the installed sample corpus and the same local
reference dependencies as the individual family gates:

```text
python -B -X utf8 docs/features/simulation-shell-inventory/freeze_shell_inventory.py
```

Both commands preserve reviewed baselines. The explicit `--write` option is only
for a reviewed initial freeze or baseline update. The combined gate passed all
11 checks. Its native-reader input packet has 1,844,374 bytes and SHA-256
`a5aeca4bd40e9022e0aa4182b9d7c630f252bcc15c60e3931eab222e785d13a8`.
The packet preserves the family, exact document/stream identity, complete payload
and every original version/profile/success tuple. Shell and midsurface native
arguments use unknown layer-presence context; this replay does not reconstruct
runtime allocations. Source deltas reuse the reviewed Material prefix and
C-757 Shell Property refinements, with all other pins exact.

The complete schema clause is retained in [14.367](schema-14.367.md). Original
arguments and outcomes are retained in `candidate-denominator.json`; metadata
rejection covers missing, extra, duplicate, renamed, resized and rehashed rows.
The source snapshot records the immutable import origins and source hashes.

Combined typed Rust construction, byte/field/span and history validation, owned
platform fixtures, deterministic process exports and full local/platform gates
remain pending for this feature. Existing individual-family acceptance remains
separate. Native writer/application and geometry acceptance, physical field
meanings, unrepresented populated native profiles and whole-format parity remain
unproven. Humanizer was unavailable; maintainer prose review remains pending.
