### 14.361 Standalone Component PersistID framing and SolidManager inventory reconciliation

*Status: all retained standalone component IDs complete; six SurfaceBody occurrences remain incomplete in reconciled manager inventory · Evidence: C-738–C-739*

The component storage reader `1813B5FD0`, bound through primary table `18205EB70` slot +28, independently opens **Component PersistID** (`18205EC18`) and delegates member +60 to `CSWPersistID::Serialization`. C-738 checks the installed storage pointer and literal bytes. This is a standalone persist-ID stream, with no component Property text/word tail. Its grammar is the established two words, signed byte count and raw blob, followed by two independent native object archives from outer version 840002. The blob is preserved as bytes; its opaque identifier encoding is not assigned semantic coverage.

`cosworks_component_persistid_layout.py` and its audit freshly hash-check/replay **all 406 retained occurrences / 267786 bytes** to exact EOF and complete reconstruction. Embedded archive handling uses the verified local ID/constant-surface/explicit-4281 profiles without relaxing ordinary model context or unknown-class stops. There are 453 retained version-candidate attempts, 427 complete. Nine tests pass, including three standalone-ID tests for the blob/count/end boundary and required modern archives, plus the existing embedded ID/wrapper historical profiles. Candidate framing still does not establish universal native study-index mapping.

C-739 reconciles **every C-590 SolidManager stream identity** one-to-one to retained evidence, matching path, nested stream, SHA-256, and byte length. There are no missing or duplicate bindings. This is a retained-evidence reconciliation, **not** a fresh combined replay or a global document denominator.

| Payload family | Occurrences | Logical bytes | Current framing evidence |
|---|---:|---:|---|
| Direct manager Property | 361 | 5136 | C-729, complete |
| Component Property | 685 | 33518 | C-730, complete |
| Solid-body Property | 680 | 855786 | C-734, complete |
| Inherited sheet Property | 238 | 186372 | C-737, complete |
| Material below SolidManager | 837 | 2946340 | C-724, complete |
| SurfaceBody | 232 | 34672 | C-733, 226 complete / 33778 bytes |
| Component PersistID | 406 | 267786 | C-738, complete |
| **Total** | **3439** | **4329610** | **3433 complete / 4328716 bytes** |

The remaining **six / 894 bytes** are exactly the candidate-860006 SurfaceBody tail mismatches retained in 14.356; no manager payload family disappears from the count. Empty storages, unrepresented historical profiles, native writer equivalence, business meanings, and non-SolidManager installed formats remain outside this framing reconciliation. No target executes and global semantic coverage credit remains zero. The all-installed-format schema goal remains incomplete.

Evidence: C-738–C-739.
