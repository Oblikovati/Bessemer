# Flow static-settings identity and selection (#241)

`FlowCarrier::static_settings_identity` reads the optional `cfld_uuid` field from
the ordinary header profile in specification 14.32. Record 123 gates record 435.
Missing gated fields return the documented sixteen-byte nil default with
`present=false`; a stored nil GUID has `present=true` and retains its exact
record occurrence. Malformed selected prefixes return errors. The result keeps
the original complete carrier checkpoint and exact optional-field coverage.

`select_flow_static_settings` projects the native cache-selection rule from an
incoming GUID, previous cache GUID and explicit runtime precedence byte. Every
nonzero byte retains FLD-sourced settings. With a zero byte, equal binary GUIDs
skip the loader sequence; different GUIDs require it. Equality uses all sixteen
bytes. Nil keys compare normally. A changed non-nil previous key produces the
overwrite diagnostic, including a transition to nil.

```rust,ignore
let identity = carrier.static_settings_identity()?;
let plan = select_flow_static_settings(identity.guid(), cached_guid, from_fld);
let action = plan.action();
let proposed = plan.cached_after_loaders_return();
```

The result retains all supplied inputs, the selected action, overwrite-warning
decision and proposed cache value after normal loader return. The native consumer
copies the incoming key after its loader calls return normally, without inspecting
their Boolean returns. The API's `ExactNormalReturnSelection` mode and execution
diagnostics preserve this scope. Loader execution, exceptions, settings-body reads,
native cache/model mutation and application acceptance remain separate work.

Identity edits compose with the existing typed record-435 GUID writer. Selection
does not change the input carrier or the caller's cached value. Published identity
and projection results are immutable and `Send + Sync`. Ordinary carrier/index
limitations remain available through the retained source carrier.

The independent packet contains 73,984 selector outcomes: all 256 precedence bytes
across nil and distinguishing GUID values in every byte position. Native identity
checks retain all three installed sample keys, including the physically present
nil in CPT. Tests cover header gating, duplicate selection, all 52 short-prefix
cases, identity-write composition, exact branch coverage and eight shared readers.
Fixture regeneration uses a temporary injected root. Native role evidence is
retained in the neighboring header feature's `settings-role-byte-verification.json`;
it provides static byte authentication, without native execution credit.

```text
cargo test --test flow_settings --test flow_headers --locked
python docs/features/flow-settings/test_selection.py
```

The humanizer skill is unavailable; its prose pass remains deferred to review.
