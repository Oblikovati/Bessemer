# Canonical SketchBend owned profile

Feature [#219](https://github.com/Oblikovati/Bessemer/issues/219) adds the explicit
`SketchBendOwnedProfile::CanonicalRuledGrid` context declaration for ordinary
schema-1 kind-7 SketchBend archives. The default context still rejects kind 7
before consuming its owned data. Earlier replay remains available by leaving
the profile unselected. The stored behavior is `C3582-canonical-sketch-owned-v1`.

This profile declares the **current** canonical nonnull ruled-face/grid layout
at the owned serialization checkpoint, with ordinary embedded arrays and their
special `0x10` provider flag clear. Constructor evidence alone does not establish
that every archive meets these conditions: allocation failures, later pointer
replacement, null grids and arbitrary virtual overrides remain excluded.
The declaration is caller-supplied context, not a native object validation or
an inference from absent pointer bytes.

The codec reads the full inherited OneBend, the direct owned grid and face word,
the common tagged sketch/handle/curve suffix, and the required trailing raw kernel
frame. There is no owned pointer tag or presence word. The grid's `+0x138` direct
array and the ruled face's later `+0x138` signed word remain separate fields.
The empty selected owned layout occupies 76 bytes; populated arrays vary in size.

The required `+0xBF0` body uses the existing strict compressed raw-body decoder,
without a suppression byte. Callers must supply a kernel base schema and limits.
Native minimum-body cleanup does not discard the saved frame offline. Other
kernel forms, special array/DB providers and unsupported inherited OneBend raw
branches still stop explicitly. No fallback accepts opaque body bytes.

Select the profile before parsing the source configuration, then use that source
for the dependent feature stream:

```rust,ignore
let context = PartConfigurationContext::from_document(
    &document, id, version, Some(kernel), limits,
)?.with_sketch_bend_owned_profile(SketchBendOwnedProfile::CanonicalRuledGrid);
let source = PartConfigurationArchive::from_bytes(&configuration_bytes, context, limits)?;
let features = PartResolvedFeaturesArchive::from_bytes(
    &feature_bytes, source, &manager_bytes, limits,
)?;
```

The declaration is retained in immutable source contexts and propagated through
configuration construction, feature parsing and edit revalidation. Fresh document
reopens must select the same profile and kernel context again; neither option
is serialized into the file. Missing profile and kernel selections have separate
typed failures. The public profile is `Send` and `Sync` and holds no runtime state.

Specification 0.3100 section 11.249 / C-3582 and the C-291 parent routine bind the
layout. [schema-reference.json](schema-reference.json) records ten verified native
and capture hashes, the schema-1 runtime descriptor/factory and the exact parent
raw-body import bindings. No native DLL or original producer parser was executed.

The literal corpus contains 576 numeric combinations, four containing graphs and
18 owned-document streams across both saved helper states. It covers populated
arrays, nulls, back-references, aliases, strict raw frames, the minimum modern
version and independent scalar/face-word edits. Existing default builders remain
checked against their earlier frozen graphs. Validation outcomes are recorded
separately. Stored-archive acceptance does not certify geometry, native application
opening, all kind-7 records, DB/historical contexts or full schema parity.

The humanizer skill is unavailable here; maintainer prose review remains outstanding.
