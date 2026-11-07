### 14.6 Composer SMG project containers

*Status: Native writer/header evidence and complete byte partitions for 38 installed samples; member payload meanings remain partial · Evidence: [C-145]*

Composer `.smg` files use a separate container from the part/assembly/drawing format in clause 7. They consist of the three bytes `53 4D 47` (`SMG`), followed by an otherwise unmodified ZIP archive. `litinteropsmg.dll:180012100` (`CLitSMGWriter::WriteSMG`) creates the ZIP with `CZipArchive`, adds files using compression level 9, closes it, then calls `CLitIO::AddSMGHeader`. That helper, `LitFoundations.dll:18016B3A0`, writes the three literal bytes and copies the entire temporary ZIP unchanged. ZIP record offsets consequently count from physical byte **3**.

| Physical region | Layout |
|---|---|
| File prefix | Three literal bytes `SMG`. |
| Local records | Standard `PK 03 04` local file headers, 30 fixed bytes each, then filename and extra field, then compressed data. |
| Central directory | Standard `PK 01 02` headers, 46 fixed bytes each, then filename, extra field and member comment. Local offsets are relative to byte 3. |
| End record | Standard `PK 05 06` end-of-central-directory record, 22 fixed bytes, then the archive comment. Central offset is relative to byte 3. |

The ZIP fixed fields have their standard little-endian order: local version/flags/method/DOS time/date, CRC-32, compressed and decoded sizes, filename length and extra length; central records additionally store creator version, comment length, start disk, internal/external attributes and local offset. The end record stores disk identifiers, per-disk/total counts, central size/offset and comment length. Unlike the clause-7 container, these signatures and filenames are not transformed by a document key.

`composer.py` validates the prefix, single-disk ZIP32 bounds, directory count/span, local/central metadata and filename equality, and an exact physical partition. Duplicate member names remain ordered; string lookup rejects an ambiguous name, while entry-object lookup retains identity. Filenames use UTF-8 when flag `0800` is set and CP437 otherwise. Extra fields retain their `(u16 tag, u16 byte count, payload)` structure; unknown tag payloads remain uninterpreted. Member/archive comments remain raw bytes. Stored and raw-deflate members are supported, with exact decoded length and CRC-32 verification; deflate termination and unused compressed bytes are checked independently of the directory.

Every observed sample has ZIP creator field `002E`, extraction version `0014`, flags `0802`, compression method 8, and no extra fields or comments. No gaps, additional signatures, data descriptors or trailing bytes occur. The reader also tests stored entries, Unicode names, unknown extra fields, comments containing a false end signature and duplicate names using an independent ZIP writer. Encrypted, split, ZIP64 and data-descriptor variants remain explicit unsupported cases; native `WriteSMG` has an optional password branch, so absence of encrypted installed fixtures does not establish that Composer lacks such files.

The installed corpus contains **38** SMG files totaling **44,056,222 bytes**, with **550** members. Their physical partitions comprise 114 prefix bytes, 16,500 local-header bytes, 9,209 local filename bytes, **43,995,054 compressed member-data bytes**, 25,300 central-header bytes, 9,209 central filename bytes and 836 end-record bytes. All members pass exact decompression/CRC checks and agree byte-for-byte with Python's independent `zipfile` reader. The decoded members total **106,834,828 bytes**. Complete container partitioning does not assign application meanings to the compressed member payloads.

Each sample contains `product.smgXml` and `product.smgGeom`, with image/texture and view assets also occurring. All 38 XML roots are `DocumentSMG` with `Version="7.900000"`; decoded XML totals **21,529,648 bytes**. The audit records root attributes, server types and 5,831 observed element paths with attribute counts. This is an observed structure inventory, not a complete XML schema or value mapping. Geometry payload framing is described in 14.7; scene-graph and asset-reference meanings remain incomplete.
