# Electrical userdata delegate storage (#231)

`ElectricalUserdataState::project_recordset` implements the constructor-shaped
twenty-slot reader described in specification 14.27. Its input is a complete
prior state and explicit post-conversion provider destination effects. An
`Unchanged` effect means the destination is known to remain unchanged; it does
not stand for unknown provider behavior. The method does not convert SQLite
values or infer provider failure defaults.

Null recordsets retain every field. Non-null input assigns the supplied opaque
context word and evaluates the ID gate using a temporary initialized to -1.
Rejected IDs clear all twenty strings while preserving the prior load flag and
object number. Accepted IDs set the flag byte to one and apply the supplied
object-number and string effects. Later provider return values are ignored by
the native reader; callers supply the actual write effects instead.

```rust,ignore
let result = previous.project_recordset(Some(&converted_effects));
assert_eq!(result.before, previous);
let slots = &result.after.slots;
```

Strings use immutable `Arc<[u16]>` storage, preserving embedded NUL and unpaired
surrogates. Results retain complete before/after storage, per-destination history,
attempted typed-read counts, cleared-slot counts and exact supplied-unit error.
The projection shares only immutable string allocations. It performs no external
I/O and cannot publish partial state: every fixed-size input has a complete
projection. It is separate from SQL row existence and stored-database mutation.

The complete 225-byte reader binding and chained unwind ranges are retained in
the neighboring userdata feature's `reader-native-byte-verification.json` (#240).
Native provider conversions, context-helper semantics, malformed native arrays,
whole-application acceptance and the meanings of object-number categories remain
outside this operation. The API supports the documented twenty-slot constructor
shape, rather than emulating native bounds traps on damaged runtime objects.

The unavailable humanizer pass remains deferred to maintainer review.
