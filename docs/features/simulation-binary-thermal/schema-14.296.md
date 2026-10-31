### 14.296 Native embedded reference archive versions and first-object declarations

*Status: all 49 first archive headers decoded at proven boundaries; reference object bodies remain unresolved · Evidence: C-672*

**Boundary discipline.** `cosworks_reference_archive_headers_audit.py` freshly reextracts the **49** native Pressure streams that stop at embedded object archives in 14.295, verifying both envelope and payload hashes. It begins at the exact offset established by that enclosing parse, reads the provider's four-byte archive version word (14.282), then uses `ObjectArchive` with an empty per-instance body-reader registry to inspect the first object tag/declaration. It does not scan for class-name text, dispatch a model-body reader, inherit another archive's object map or skip unknown body bytes. C-672 retains archive offsets, raw header bytes, schema/class names, next-body boundary and the explicit body-disabled stop.

**First-object results.** All nonnull first objects use the new-class tag **0xFFFF**, schema **1**, and a bounded ASCII class-name declaration. The observed distribution is:

| First object | Native stream count |
|---|---:|
| `moSingleFaceRef_w` | 46 |
| `moGeneralCurveRef_w` | 2 |
| Null tag **0x0000** | 1 |

The **15** observed archive version words are **4307, 4317, 4400, 5000, 6000, 6222, 6247, 7000, 12000, 13000, 14000, 16146, 16148, 17108, 19000**. Version **6000** occurs in **17** first archives. These are nested model-archive versions, distinct from the six-digit Simulation version candidates that select the enclosing reference grammar. Do not substitute either version for the other or infer that a newer outer Simulation stream contains only modern model-archive bodies.

**Two proven null archives.** In `samples/tutorial/api/lineardynamic.sldprt`, the candidate Simulation version is **880012**. At payload offset **344**, the first archive is version **6222** followed by a null object tag; its complete six bytes are **4e1800000000**. The second provider object call begins immediately at offset **350**, independently reads version **6222**, and also has a null object tag, ending at **356**. Both headers are verified with fresh maps. This validates the two-call archive framing for that reference without requiring an object body, but C-672 does not yet claim a complete parse of its enclosing Pressure stream.

**Context boundary.** Existing `modelreferences.py` and `modelcomponents.py` readers cover scoped ordinary model archive contexts and explicitly reject unsupported versions/contexts. Their presence in the repository is not evidence that these embedded Simulation archive bodies have the same flags, owner map or field layout. Many native versions here are below those readers' supported modern range. Do not bypass their context guards merely to force consumption. The next dependency is the version/context-specific `moSingleFaceRef_w` and `moGeneralCurveRef_w` body grammar, together with the nested component references they restore. No native object deserialization or other target code executes; no global semantic byte credit is granted.

Evidence: C-672.
