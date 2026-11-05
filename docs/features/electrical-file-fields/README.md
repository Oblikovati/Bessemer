# Electrical direct file-field transport map (#231)

`ElectricalFileLink::file_read_map` pairs all 38 directly named native field
requests in specification 14.26/C-265 with the original stored values from one
complete file row. Requests retain native call order, exact names, reader kinds
and getter output destinations. The full original record remains available.

```rust,ignore
let mapped = file_link.file_read_map()?;
for request in mapped.reads() {
    let original = &request.value;
    let reader = request.reader;
    let destination = request.destination;
}
```

The map includes 21 Long, three Bool, seven Double, six Str and one Date request.
Reader kinds describe provider operations, not conversions applied by Bessemer.
NULL, integers, real bits, text and blob values retain their SQLite identity.
Every required column must be present; a missing column returns a typed error
without publishing a partial map. Present NULL values remain valid transport
inputs with provider behavior explicitly unresolved.

Destination variants distinguish runtime object byte offsets, the separate
identity temporary and the reused Long/Bool/Double temporaries. Pointer-based
expressions in the retained decompilation are converted to byte offsets using
their documented operand width. These are runtime addresses, never SQLite file
offsets. The identity temporary is passed to a virtual setter; its final member
location is not established by this map. Temporary destinations identify getter
outputs, not every subsequent copy into formatting fields.

The native loader initially zeros shared temporaries but later reuses them and
does not test every getter result. Bessemer therefore applies no independent
zero defaults, guessed NULL conversions, date formats, timezones, units or string
normalization. It does not execute the native loader or its userdata delegate.
The explicit check mode and typed diagnostics retain those exclusions.

All unmatched columns remain in the input record and in stored-order
`unmapped_columns`. Every original native row has `fil_state` and
`fil_updatedate` outside the 38 direct requests. The map does not classify them
as ignored, consumed by a delegate or unimportant. Owned rows also retain an
additional unknown column and deliberately reordered schema columns.

## Independent evidence

The EFF1 packet contains four original template archives, two complete owned
rows and 38 owned missing-column cases. Expectations combine the source-bound
C-265 descriptor inventory with independent SQLite values. Owned complete rows
cover all-NULL fields and deliberately incompatible dynamic types, preserving
the distinction between requested provider kind and stored SQLite type.

The installed DLL identity matches the documented SHA-256. Independent PE
offset reads verified 82 complete byte ranges: the selected loader, five typed
wrappers, all 38 descriptor initializers and all 38 exact UTF-16 literals. The
range hashes and file offsets are retained in `native-byte-verification.json`.
This is static byte evidence; it does not observe provider or application
execution. Native graph/record inputs remain the frozen original archives.

The focused gate passed eight field-map tests, twelve project-record regressions,
three independent Python fixture tests and Clippy with warnings denied. It
includes all missing-column cases, immutable input/result isolation and eight
concurrent replays. Two fresh processes reproduced all carriers, successful maps
and typed missing-column errors byte for byte. `local-verification.json` and
`process-verification.json` retain the exact source, log and output hashes.
The REA target identity matched the same DLL, but its focused byte-read call
timed out; that call contributes no byte evidence. The native session was closed
successfully. `rea-observation.json` records that limitation separately from
the successful offline PE comparisons. Full-suite validation for this new source
remains pending; the active gate covers the earlier project/userdata source.

```powershell
python -m unittest discover -s docs/features/electrical-file-fields -p 'test_*.py' -v
cargo test --test electrical_file_fields --test electrical_records --locked
```

Complete provider conversions, reused-temporary effects after failed native
reads, virtual identity storage, delegated userdata state, database mutation and
native migration/application acceptance remain separate work. Overall documented
schema parity is incomplete. The humanizer skill is unavailable; its prose pass
remains for maintainer review.
