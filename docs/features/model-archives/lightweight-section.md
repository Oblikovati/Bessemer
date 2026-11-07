# Complete owned part lightweight section

The private part reader now consumes the owned configuration's complete
lightweight section. It starts with the root declaration and declared length,
imports header objects in native order, and retains the shared map through the
node, concept, units, component, appearances, spatial metadata, direct manager,
collections, model tail and part-specific records. Its caller supplies an
explicit `OrdinaryPartExternalXml` context and outer archive version. The
continuation requires a version above `0x27CB`; stored `unknown_ac` does not
select the archive context.

The owned input has a 2,903-byte body and ends at byte 2,921, including the root
declaration and size word. Every byte reconstructs from owned typed fields.
The fixed 16-byte unknown fields and declared dynamic-array payloads are stored
as individual octets with explicit lengths and budget charges. No unread
remainder or source-byte fallback is retained. Null required unit components,
impossible counts, invalid array element sizes, unsupported direct payloads and
unknown new object bodies produce typed failures. A failed reader cannot
publish its earlier fields.

`freeze_native_lightweight.py` reads the owned part through the independent
reference and freezes 477 scalar values, including strings, floating-point
values and fixed unknown octets. Rust checks every frozen value and reconstructs
the full bounded section exactly. Tests overwrite the source after reading,
compare four independent concurrent parses, and reject every shortened body
even when its declared length is adjusted to fit the truncated input. Separate
tests reject wrong context, old context versions and mismatched section sizes.

`freeze_lightweight_sections.py` constructs 57 independent cases and requires
reference EOF before freezing their bytes and 941 scalar values. The cases cover
dynamic arrays, flags, object collections, note lists, model and part tails,
spatial bounds, metadata and direct managers. Sixteen versions cover both sides
of the applicable conditional fields, including the maximum version word.
Every case reconstructs exactly and rejects all truncated prefixes and trailing
bytes. Two fresh reference processes produced identical artifacts; each
reference manifest binds the Python source hashes and generator hash.

This checkpoint verifies the owned lightweight section and the listed section
cases. It does not complete #44. The 21,103-byte resolved section, full-root
construction with derived lengths, isolated full-document edits, revision
history, native corpus acceptance and public API remain open. Nonempty nested
records that need additional complete serializers remain explicit unsupported
failures. Percent-units pointer assignability also remains unresolved before
exporting the model codec.

No release or version change is included. The humanizer skill is unavailable;
the prose pass remains for maintainer review.
