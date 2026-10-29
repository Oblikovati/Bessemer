### 14.8 Standalone Inspection current encrypted XML envelope

*Status: Native/managed current writer, 42 templates and historical tutorial decoder (14.15); full XML value schema remains partial · Evidence: [C-147], [C-148]*

This clause describes the encrypted save branch in the installed standalone `SOLIDWORKS Inspection.exe`, not every file with the `.ixprj` suffix. The executable is a mixed native/managed PE. Native project writer `0x140812B10` builds XML with `QXmlStreamWriter`, converts the result to a wide string, and invokes the encryption manager through native thunk `0x143153380`. A separate save branch bypasses this encryption manager; its format and selection condition are not specified here.

The encrypted branch passes through managed methods `Encryptor.EncryptorManager.SaveEncryptData` (token `0x06000235`), `EncryptionNative.SaveEncryptData` (`0x06000241`) and `EncryptionCLIWrapper.SaveEncryptData` (`0x0600024A`). Tokens are scoped to executable MVID `780f8a2a-9f78-4205-adde-cb04972b75b0`; they are not format constants. The final method encodes the complete XML string with `System.Text.Encoding.UTF8.GetBytes`, writes it to an encrypting `CryptoStream`, calls `FlushFinalBlock`, and writes the resulting memory-stream bytes directly to a newly created file. This branch adds no magic, length prefix, salt, or separate initialization-vector record.

*Table 14-4 — Current Inspection encrypted writer envelope*

| Item | Encoding or value |
|---|---|
| File bytes | Complete AES ciphertext, a positive multiple of 16 bytes |
| Plaintext | UTF-8 XML bytes from the native project writer |
| Algorithm | `Aes.Create()`, default CBC mode, 128-bit block |
| Key | 16 literal bytes: `01 02 02 03 03 05 05 05 04 04 05 05 03 03 02 01` |
| IV | 16 zero bytes; allocated using the algorithm's IV length and explicitly zero-filled |
| Padding | Default PKCS#7; always present, including a full block when plaintext length is divisible by 16 |
| Authentication | No separate authentication tag or MAC is written by this branch |

`GetKey2020` (`0x06000250`) constructs the key byte by byte; `GetCryptoStream` (`0x0600024E`) supplies that key and the zero IV to `CreateEncryptor` or `CreateDecryptor`. It does not override mode or padding. CBC/PKCS#7 defaults were checked with the installed .NET Framework 4.x implementation. For plaintext bytes `<Project/>`, that implementation produces ciphertext `97 37 E0 4E 6F 9A DA 0B 7C DC FC 03 AB 03 72 3A`. This is an envelope test vector, not a complete native project fixture.

Native loader `0x140813330` invokes manager `GetDecryptData` through thunk `0x143153390` with the historical flag clear, then retries with the flag set on failure. The managed current path decrypts the complete file in chunks of 8,192 bytes, converts the plaintext using UTF-8, and returns a wide string for `QDomDocument::setContent`. The historical path instead calls `SymmetricAlgorithm.Create()` and `GetKey2015`, which constructs nine bytes `01 02 02 03 03 05 05 04 03`. The historical branch is now validated against the installed tutorial in 14.15. Its nine-byte input is accepted by the installed Framework and produces a two-word Rijndael key schedule; substituting a conventional AES key-size check would reject this valid native fixture.

The installed `PDF/Tutorial/cmm data import.ixprj` is **361,536 bytes**, SHA-256 `8a7191837421179a2e02f0cb52256674fe5cce69632d157ed28d62f090745a06`. It fails the current branch's padding check. Its first four bytes, `D1 73 0D 00`, are ciphertext in the historical branch verified in 14.15, not a length prefix. The current-envelope rejection remains an intentional branch distinction; it no longer means that this tutorial cannot be decoded.

`solidworks/inspectionproject.py` implements a bounded current-envelope reader using `pycryptodome`. It checks block length, PKCS#7 padding, strict UTF-8 and well-formed XML, and rejects DTD/entity declarations. These are reader validation choices; the XML checks do not establish native schema conformance or authenticity. The installed `.ixpdot` templates in 14.9 independently validate this envelope against native files. A newly generated current native project save and complete standalone XML value schema remain unavailable. The historical decoder and its original tutorial are verified in 14.15. The embedded drawing XML in 14.4 remains a separate carrier and schema.

### 14.9 Standalone Inspection templates and XML project options

*Status: 42 native template envelopes and observed XML structure; selected native writer fields; complete XML value semantics remain partial · Evidence: [C-148]*

The installed directory `C:/ProgramData/SOLIDWORKS/SOLIDWORKS Inspection 2026 Standalone/Templates` contains **42 `.ixpdot` files**. Every file decodes with 14.8, with no preceding or trailing wrapper bytes. Independent .NET Framework decryption agrees with the Python reader on the complete plaintext length and SHA-256 of both distinct ciphertexts. Repeated language-labelled files do not constitute independent schema variants.

*Table 14-5 — Installed standalone Inspection template payload groups*

| Ciphertext SHA-256 | Files | Bytes per file | XML bytes per file | PKCS#7 bytes per file |
|---|---|---|---|---|
| `84570c788e59d92c80c7009e0ea899cfd19a99e6b2f293740b12100ab870c24b` | 28 | 54,576 | 54,571 | 5 |
| `9c2d689738e9adf5286750fdfd81e2069ae06ab6204125182835beaf5bdc2928` | 14 | 54,848 | 54,837 | 11 |

The 42 files total **2,296,000 stored bytes**, **2,295,706 XML bytes** and **294 padding bytes**. The generated [standalone template schema inventory](inspection-template-schema.md) records **119 observed element paths**, attributes, fixture occurrence counts and lexical forms. `inspection-template-corpus.json` additionally retains all observed leaf and attribute values, per-file hashes and independent framework decode results. XML syntax inventory alone receives no payload semantic credit.

The root expanded name is `{http://www.extensiblecad.com}SOLIDWORKSINSPECTION`. The observed prefix is `iex`; prefix spelling is not the namespace identity. Its `INSPECTION_SHEET` child and all observed descendants are unqualified: a parser MUST NOT apply the root's namespace to those children. This standalone tree differs from the unqualified `ExtInspectionProject` tree stored in drawing stream `ThirdPty/InspectionXpert` (14.4).

*Table 14-6 — Observed standalone Inspection template structures*

| Path below root | Observed structure or values | Scope of evidence |
|---|---|---|
| `INSPECTION_SHEET` | Attributes `DateCreated`, `CreatedByVersion`, `LastSavedByVersion`, `CMMState`, `CapturesVisible` | All 42; version attributes read `2018.0.0.15`, a content value rather than a cipher version |
| `INSPECTION_SHEET/DOCUMENT/DOCUMENT` | Referenced document metadata, source path, model hash/size/version, identifiers; `DefaultGridOptions` and `PageRotation` children | Metadata values are retained; reference resolution, hash input and units are unverified |
| `INSPECTION_SHEET/DOCUMENT/DefaultGridSettings` | `GridOption` attributes and nested `GridRegion` offsets | Names, values and spelling retained; coordinate units remain open |
| `INSPECTION_SHEET/PackageOptions/GeneralOptions` | Ballooning, characteristic defaults, tolerances, export, automatic save, OCR, performance and FAI options | Observed tree only; enum domains and option units are incomplete |
| `INSPECTION_SHEET/PackageOptions/ExportTemplateList` | Repeated `ExportTemplate` with attributes `Active`, `TemplateName`, `TemplateType`, `AllSheets`, `TemplatePath`, `IsImported` | Native writer `0x1408A1650` emits these names and attributes |
| `INSPECTION_SHEET/PackageOptions/ProjectProperties` | Repeated `Property` elements containing `Name`, `Value`, and conditionally `MappingNode`; nested `Sampling` | Native writers `0x1408BC590`, `0x1408BD4E0`, `0x1408BE590` |
| `INSPECTION_SHEET/PackageOptions/IXExtractionSettings` | Characteristic, dimension, note, GD&T, hole-callout, weld, surface-finish, geometry and inspection-criteria options | Native wrapper `0x1408A23D0`; observed settings are not a complete annotation schema |
| `INSPECTION_SHEET/PackageOptions/BalloonSequences` | Repeated `BalloonSequenceItem` containing `Uid`, `Name`, `Value` | Native wrappers `0x14088DC60`, `0x14088E030`; identifier domain remains open |

`PackageOptions` writer `0x1408BADB0` dispatches separate writers for general options, export templates, project properties, display-property settings, extraction settings and balloon sequences. Some subtrees are conditional and not present in these templates. Therefore the observed 119 paths do not define an exhaustive standalone project schema.

The property-list writer emits `Property/Name` and `Property/Value` as text, preserving property order. Its `MappingNode` branch is selected for six literal names: `Part Name`, `Part Number`, `Part Revision`, `Drawing Name`, `Drawing Number`, `Drawing Revision`. These six properties have empty mapping nodes in the templates; other observed properties omit that child. The default/inches payload contains 14 properties and the metric payload contains 16, including different custom property names. These counts are fixture observations, not required cardinalities.

Sampling writer `0x1408BDCB0`, under wrapper `0x1408BE590`, emits text elements in order `LotSize`, `Level`, `Type`, `AQL`. Every template stores `200`, `II`, `NORMAL`, and empty text respectively. Complete level/type/AQL domains and numeric interpretation require consumer evidence. The native strings and write calls establish field identity without granting those unresolved values complete semantic coverage.

The XML-in-attribute value `CMMState` contains an XML declaration followed by two `DMDataSet` fragments after entity decoding. It is retained exactly as text; parsing the concatenation as one well-formed XML document would be incorrect. Date strings, color integers, regex values, misspelled names such as `LengthPrecsionTolerances`, and empty collections are likewise preserved without normalization or inferred constraints. All installed project formats, historical Inspection decoding, populated project annotation collections and complete document bit coverage remain open.

### 14.15 Historical standalone Inspection project envelope

*Status: B / ? — native historical IL, nine independent Framework vectors and complete tutorial plaintext verified; XML value semantics and complete project bit coverage remain partial.*

This resolves the historical tutorial envelope left open in 14.8. The installed native loader calls GetDecryptData with its historical flag clear, then retries with that flag set. Retained IL for EncryptionCLIWrapper.GetDecryptData (0x0600024B) selects GetOldCryptoStream (0x0600024D) and GetKey2015 (0x0600024F) for that historical path. The key factory constructs nine bytes **01 02 02 03 03 05 05 04 03**. GetOldCryptoStream calls SymmetricAlgorithm.Create(), allocates an IV of the algorithm's IV length, zero-fills it, and calls CreateDecryptor with that key/IV. No length prefix is read before the ciphertext.

The installed .NET Framework 4.x resolves that factory to RijndaelManaged in CBC/PKCS7 mode. Crucially, CreateDecryptor accepts this nine-byte argument despite the algorithm's advertised 128/192/256-bit legal sizes. Reflection on the resulting transform verifies **Nb=4, Nk=2, Nr=10**. Key expansion reads two four-byte words, so only the first eight key bytes participate; the ninth is not read. The resulting schedule has 44 words, using the usual rotated/substituted-word/Rcon recurrence every two words, and ten 128-bit Rijndael rounds. This is not standard AES with a padded/truncated-to-128-bit key. Microsoft's [RijndaelManagedTransform reference source](https://github.com/microsoft/referencesource/blob/main/mscorlib/system/security/cryptography/rijndaelmanagedtransform.cs) corroborates integer word-count and round-selection behavior; the installed assembly/IL hashes, reflected fields and complete fixture/vector equality are the local authority.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | stored project | sequence of 16-byte ciphertext blocks | entire file | Historical two-word-key Rijndael CBC stream. No preceding header, explicit IV, salt or separate authentication tag occurs in this branch. | B |
| 2 | IV | implicit 16 zero bytes | always | Supplied by the native helper; absent from stored bytes. | B |
| 3 | padding | PKCS#7 bytes | end of decoded stream | Remove 1–16 identical bytes whose value is their count. A full block is required for a block-aligned plaintext. | B |
| 4 | project plaintext | UTF-8 XML | after padding removal | Native conversion returns a wide string to QDomDocument::setContent. XML parsing does not establish value semantics. | B / ? |

`inspectionlegacy.py` independently implements the recovered fixed-key key expansion, inverse Rijndael rounds and zero-IV CBC processing. It neither invokes Inspection nor depends on Framework decryption. `inspectionproject.decode_legacy` adds bounded input/block/padding, strict UTF-8 and XML checks; it rejects DTD/entity declarations. `decode_current` retains the separate current AES-128 branch and still rejects the historical tutorial. `decode_project` tries current then historical, matching the recovered loader order with stricter XML validation. These validation choices do not establish authenticity or native schema conformance, and suffix alone is not a version discriminator.

The original installed `PDF/Tutorial/cmm data import.ixprj` has 361,536 bytes and stored SHA-256 `8a7191837421179a2e02f0cb52256674fe5cce69632d157ed28d62f090745a06`. Independent Framework and Python decoders agree on **361,531 XML bytes**, **five padding bytes (05 05 05 05 05)** and complete plaintext SHA-256 **d5ce41d90637a180975ac925cabed42c2f87628c446c3276a398e3c40e3d29ce**. The decoded root is `{http://www.extensiblecad.com}SOLIDWORKSINSPECTION`; the observed tree contains **176 element paths**, inventoried in `inspection-tutorial-schema.md`. Only the root is qualified with that namespace. Path counts, lexical forms and attribute names are observations, not cardinality/type constraints or a full project schema.

Nine independent Framework encryption vectors cover plaintext lengths 0,1,15,16,17,31,32,33,256. The Python decoder reproduces every plaintext and every final padding byte. Evidence retains Framework assembly path/hash, module ID, constructor/key-expansion IL and hashes, reflected Nb/Nk/Nr, native Inspection method IL and complete executable hash, and source-bound tutorial identity. Four focused tests cover those vectors, exact native plaintext/hash/XML root, current-to-historical fallback, continued current-branch rejection of the tutorial, current-vector compatibility, bounds/types, malformed blocks, corruption and correctly padded non-XML rejection.

All stored tutorial bytes are now accounted for by this complete envelope decode. This is envelope framing evidence, not semantic coverage credit for the XML values, characteristics, references or embedded assets. Those meanings, newly generated current project fixtures and other installed project families remain unfinished. No part/assembly/drawing feature-frontier totals or whole-document coverage percentages change in this edition.

Evidence: C-254.
