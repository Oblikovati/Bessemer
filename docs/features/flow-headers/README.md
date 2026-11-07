# Flow GUID header prefixes (#241)

`FlowCarrier::header_metadata` reads the selected ordinary records described in
specification 14.29. It selects the last record 123 and requires its complete
32-byte prefix before reading two binary GUIDs. Only then does it consider the
last record 435, requiring its four-byte control word and sixteen-byte settings
identity. No record 123 returns `None`, even if an orphan record 435 is truncated.
A malformed selected prefix is an error; the reader does not fall back to an
earlier duplicate or assign a fresh nil value.

The metadata retains the source carrier, exact occurrence IDs and every extension
byte. Missing settings remain `None`, distinct from a physically present nil
`cfld_uuid`. `FlowGuid` preserves sixteen wire-order bytes and formats standard
Windows GUID text with little-endian Data1, Data2 and Data3. Binary equality
never depends on formatting. The settings identity's cache-key role is established
by specification 14.32; the first GUID's lookup role depends on requested result
type, as described in 14.33. The control word's meaning remains unknown.

```rust,ignore
let header = carrier.header_metadata()?.ok_or("missing record 123")?;
let text = header.first_guid.to_string();
let edit = carrier.replace_header_guid(FlowHeaderGuidField::Second, replacement)?;
let checked = edit.result().header_metadata()?;
```

Typed GUID replacement uses the checked physical carrier writer. It changes
exactly the requested sixteen bytes, retains the control word and extensions,
and returns complete carrier history and the original checkpoint. Missing gated
records fail rather than creating new headers. No-ops retain original record
identities; fresh replacement with the original GUID restores original bytes.

Header coverage is explicitly `ExactSelectedGuidPrefixes`: 32 bytes for record
123, plus 20 when record 435 exists, separate from retained opaque extensions.
This is structural prefix coverage. It does not establish static-settings bodies,
cache transitions, related-file enumeration, arbitrary payload cross-references
or native application acceptance. Physical GUID replacement does not repair
opaque indexes or related files. The full Flow schema remains open.

The humanizer skill is unavailable; its prose pass remains deferred to review.
