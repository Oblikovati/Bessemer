### 11.185 Component-reference curves and shared component-feature framing

*Status: B / ? — native modern dispatch and populated containing-root boundaries verified; old branches and raw ID/time semantics retain existing limits.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moCompReferenceCurve_c primary table `0x4CD92880` selects serializer `0x4BBB8C20`. The serializer delegates to shared component-feature helper `0x4BBB9A60` for `v >= 0x2D1`; its alternative historical branch is below the supported minimum. moCompFeature_c's primary serializer is that same helper. Native RTTI establishes moCompRef_c ancestry for both classes. There is no component-reference-curve suffix after the shared helper in this modern branch.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | component-reference base | direct moCompRef_c body | always | Existing 11.17 versioned grammar, including the tagged component, raw fields and conditional reference-info/transform/string fields. Unsupported inherited variants retain explicit stops. | B / ? |
| 2 | feature ID | i32 | always in this ordinary branch | Shared component-feature reference data. Preserve signed value. | V / ? |
| 3 | feature time | u32 | always in this ordinary branch | Four wire bytes, following the existing time representation rather than runtime member size. | V / ? |

The latest-version empty base is 82 bytes, so the null-component, absent-optional component-curve body is 90 bytes before outer class/object framing. Earlier supported versions retain the component base's independent gates; readers SHALL NOT use 90 bytes as an unconditional skip length. The scoped alias uses `modelcomponents.component_feature` and retains the actual runtime class name and archive map identity. Native-proven moCompRef_c ancestry also admits this class as a component-reference pointer in `modelreferences.wrapper`; it does not bypass pointer framing or add a second inherited object-map entry.

The completed-root audit retains **15 component-reference curves**, all referencing moUnitComponent_c. Five distinct stored feature IDs and associated times are observed. These identities are retained without resolving model geometry or asserting universal ID/time domains. Three focused tests cover component-base version transitions, signed ID/unsigned time extrema, cached object reuse, wrapper admission, context/truncation rejection and a source-stream-hash-bound native containing-root span with ID equality.

Evidence: C-213.
