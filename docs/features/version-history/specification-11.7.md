### 11.7 Version history streams

*Status: Verified layouts; Partial meanings Ã‚Â· 1,115 streams Ã‚Â· Evidence: [C-20], F.10*

`_MO_VERSION_<n>/History` contains one `moVersionHistory_c` object record (11.5), followed by no trailing data. Its body starts with i32 `state_count`, then that many direct `moVersionState_c` bodies. A state does not have its own object tag. The version conditions below use a state's stored `file_version`, not the version in the stream name.

#### 11.7.1 Version state

*Status: Verified layout; Partial stamp meanings Ã‚Â· 54,323 states Ã‚Â· Evidence: [C-20], F.10*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `stamp` | Stamp | | Direct `moStamp_c` body, 11.7.2. | V |
| 2 | `file_version` | u32 | | Version associated with this state; observed from 38 to 19000 across the corpus. | V |
| 3 | `date_history` | object | `file_version` Ã¢â€°Â¥ 0x178 | `moDateCodeHistory_c` through 11.5. Its body is i32 `count`, then `count` direct u32 date-code states, without per-state object tags. | V |
| 4 | `legacy_date_code` | u32 | 0x175 Ã¢â€°Â¤ `file_version` < 0x178 | Earlier single-date form. No date-code bytes below 0x175. | V |

Each modern date-code state contains a date code in its low 24 bits. The saver sets bit 24 on a selected final date-code state; selection depends on the current save context and is not fully specified here. Other flag meanings remain open (OI-020). The native baseline contains the stored word `0x011EEA74`.

#### 11.7.2 Stamp

*Status: Verified layout; Partial meanings Ã‚Â· Evidence: [C-20], F.10*

The direct stamp body is `revision_status` u16, `unknown_0c` u16, `unknown_10` u16, `time` u32 (`su_CTime` archive value), then `comment` CString. The names with offsets refer to in-memory member offsets and do not imply stream offsets. The serialized time occupies four bytes even though the class holds an eight-byte time member. The second and third fields remain open and receive no semantic coverage credit. The constructor and `updateStamp` identify the first field's enum as `moRevisionStatus_e`; `comment()` identifies the CString. This corrects the field labels in Edition 0.8: the status is first, not third. The enum's complete value domain is not established [C-22].

#### 11.7.3 History tail

*Status: Verified layout; Partial meanings Ã‚Â· Evidence: [C-20], F.10*

After all states, use the last state's `file_version` as `last_version`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `max_failure_severity` | i32 | `last_version` Ã¢â€°Â¥ 0x319 | Writer takes the maximum with the memory monitor's `getFailureSeverity()`. Enum values remain unverified. | B |
| 2 | `min_free_memory` | u32 | `last_version` Ã¢â€°Â¥ 0x319 | Low 32 bits of the writer's minimum free-memory observation (`getMinFreeMemory()`). Unit not established. | B |
| 3 | `unknown_30` | i32 | `last_version` Ã¢â€°Â¥ 0x45C | Value from helper 0x4C72ABF0; meaning open (OI-020). | ? |
| 4 | `unknown_38` | u16 | `last_version` Ã¢â€°Â¥ 0x46C | Meaning open (OI-020). | ? |
| 5 | `unknown_3c` | i32 | `last_version` Ã¢â€°Â¥ 0x46C | Meaning open (OI-020). | ? |
| 6 | `unknown_40` | i32 | `last_version` Ã¢â€°Â¥ 0x46C | Meaning open (OI-020). | ? |

The reader's threshold checks for fields 1Ã¢â‚¬â€œ3 also test that the archive is a standalone stream rather than a database archive. This clause specifies standalone History streams; their database-archive variant is outside the verified scope. All observed final state versions are 9000, 10000, 11000, 18000 or 19000, so the corpus exercises the complete modern tail. The older state-version branches are exercised within the state arrays, not by older history tails.
