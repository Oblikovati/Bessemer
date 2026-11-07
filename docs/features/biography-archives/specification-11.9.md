### 11.9 Biography records

*Status: Verified layouts; Partial meanings · 1,115 streams · Evidence: [C-23], [C-24], F.12*

`_MO_VERSION_<n>/Biography` contains a `moBiography_c` object record. Its body is i32 `record_count`, followed by that many direct `moBiographyRecord_c` bodies. There are no per-record object tags and no trailing payload in the sample corpus. The record and nested start/end serializers have no archive-version tests in their field-reading paths.

The records describe the product build, operating-system report and environment snapshots associated with loading and saving the document. Their field meanings come from the population routines, not from interpreting plausible sample numbers. Failed or skipped environment queries can leave default or sentinel values; readers retain the stored values. The reported OS version and processor information are API observations from the writer, not an independently verified description of its hardware or actual OS release.

#### 11.9.1 Biography record

*Status: Verified layout; one member meaning open · 82,487 records · Evidence: [C-24], F.12*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `file_version` | u32 | | `moVersionManager_c::getCurrentFileVerion()`. | V |
| 2 | `date_code` | i32 | | Product date code from `su_getFullDateCode()`; not the save timestamp. | V |
| 3 | `unknown_10` | i32 | | The 2026 population routine sets 1; role open (OI-022). | ? |
| 4 | `processor_architecture` | u32 | | `SYSTEM_INFO.wProcessorArchitecture`, widened from u16. | V |
| 5 | `processor_count` | u32 | | `SYSTEM_INFO.dwNumberOfProcessors`. | V |
| 6 | `os_platform_id` | u32 | | `OSVERSIONINFOW.dwPlatformId` returned by `GetVersionExW`. | V |
| 7 | `os_major`, `os_minor`, `os_build` | u32[3] | | `dwMajorVersion`, `dwMinorVersion`, `dwBuildNumber`. | V |
| 8 | `os_csd_version` | CString | | `OSVERSIONINFOW.szCSDVersion`; supports the abbreviation of 11.9.3. | V |
| 9 | `language_id` | u16 | | `GetUserDefaultLangID()` result, retained without normalization. | V |
| 10 | `start_end` | Snapshot[2] | | Direct `moStartEndRecord_c` bodies (11.9.2), at in-memory offsets 0x40 and 0xE8. The loader populates the first and the saver updates the second. | V |

There is no serialized padding corresponding to the class's in-memory alignment gaps. The native baseline's first record stores file version 19000, date code 2026100, architecture 9, processor count 16, OS values (2, 10, 0, 26200), an empty CSD string and language ID 8192. These are fixture values, not required constants.

#### 11.9.2 Environment snapshot

*Status: Verified layout and population sources · Evidence: [C-24], F.12*

The snapshot starts with two u32 words forming `filetime` (low word then high word), followed by u32 `available_page_file_low32`. `GetSystemTimeAsFileTime` supplies the timestamp: a count of 100 ns intervals since 1601-01-01 UTC. `GlobalMemoryStatus().dwAvailPageFile` supplies the available commit-memory observation in bytes; the writer retains only its low 32 bits. This stored word is not a complete 64-bit memory-size measurement [WINAPI].

Six path groups follow, in the order below. Each group is CString `path`, i32 `drive_type`, then two u32 words forming `available_disk_bytes` (low word then high word). `GetDriveTypeW` supplies the drive classification for paths with a recognized drive root. Helper 0x4C72C070 calls `GetDiskFreeSpaceExA` with the stored pair as `lpFreeBytesAvailableToCaller`; the value is bytes available to the calling user, which can differ from total volume free space [WINAPI]. The native collector caches this value by drive root, so repeated paths on one drive need not be independent measurements.

| Group | Path source |
|---|---|
| `system_directory` | `GetSystemDirectoryW` |
| `windows_directory` | `GetWindowsDirectoryW` |
| `current_directory` | `GetCurrentDirectoryW` |
| `temp_environment` | `GetEnvironmentVariableW("TEMP")` |
| `tmp_environment` | `GetEnvironmentVariableW("TMP")` |
| `document_path` | Current container filename from `ZipStorage::getZipFileName()` or the compound-storage filename; its containing directory supplies the drive query. |

The first five paths are normalized with a trailing backslash by the collector. String members occupy in-memory offsets 0x18, 0x30, 0x48, 0x60, 0x78 and 0x90; those offsets do not describe spacing in the stream. Empty strings, unset numeric values and old platform codes remain valid stored observations. They are not inferred from the machine reading the file.

#### 11.9.3 Repeated-string abbreviation

*Status: Verified loader/writer rule · Evidence: [C-23], [C-24], F.12*

For every record after the first, a literal `*` in `os_csd_version` means the preceding record's expanded CSD string. A literal `*` in any snapshot path means the preceding record's expanded path at the same snapshot index and path-group index. The two snapshots do not inherit from one another. Numeric fields are always stored and do not inherit.

The writer substitutes `*` only for a nonempty string equal to that preceding field. An empty string remains empty. With no preceding record, the loader leaves a first-record `*` literal. `biography.read()` preserves all stored spellings; `expanded_records()` returns a separate semantic view and expands chains without changing the raw records or numeric measurements.
