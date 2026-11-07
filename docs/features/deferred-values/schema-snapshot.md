**MessagePack values in the installed deferred-object writer.** The concrete part path moDBDeferredObjectStore::processDeferredObjects first requires original archive IsStoring nonzero. Each current queued object is serialized through a new moScopedDBArchive, followed by addMsgPackItems. That wrapper's constructor ORs its caller mode with`0x10`; the reached caller supplies0, producing mode`0x10` at the embedded archive constructor checkpoint. Its later helper/callback state is not an immutable mode invariant. This differs from the raw su_CDBArchive constructors described in11.5.

The archive getMsgpack accessor passes current`+0xC8` as a typed std::multimap of MsgPack keys and values to the MsgPack object constructor. The scoped accessor forwards through its embedded archive`+0x28`. The actual deferred caller invokes MsgPack::dump on that result, then forwards the returned byte string toward SerializeDBData. This binds a real part output path, rather than inferring encoding solely from a method name.

All17 installed concrete encoder variants dispatch through actual primary-table slot`0x10`. Public dump`0x180009710` is separate from the private dispatcher`0x180007E70`; an inherited label on one frozen capture is explicitly corrected in the mandatory qualification. The following tag bytes and recursive field order are verified for the installed writers. Multibyte payload ordering is conditional as described after the table.

| Variant | Encoded tag and body |
|---|---|
| Null | `C0` |
| Boolean | `C2` for raw byte0; `C3` otherwise |
| Float | `CA`, then raw4-byte payload |
| Double | `CB`, then raw8-byte payload |
| Int8 | Below -32: `D0` and low8 bits; otherwise low8 bits directly |
| Int16 | Below -128: `D1` and16 bits; otherwise nonpositive uses Int8, positive uses Uint16 |
| Int32 | Below -32768: `D2` and32 bits; otherwise nonpositive uses Int16, positive uses Uint16/Uint32 rules |
| Int64 | Below -2147483648: `D3` and64 bits; otherwise nonpositive uses Int32, positive uses Uint64 |
| Uint8 | Below128: low8 bits; otherwise `CC` and8 bits |
| Uint16 | Below128: low8 bits; below256: `CC` and8 bits; otherwise `CD` and16 bits |
| Uint32 | Below65536: Uint16 rules; otherwise `CE` and32 bits |
| Uint64 | Below65536: Uint16 rules; below2^32: `CE` and32 bits; otherwise `CF` and64 bits |
| String | Length<=31: `A0` OR length; <=255: `D9` and8-bit length; <=65535: `DA` and16-bit length; otherwise `DB` and32-bit length; stored bytes follow |
| Binary | Length<=255: `C4` and8-bit length; <=65535: `C5` and16-bit length; otherwise `C6` and32-bit length; raw bytes follow |
| Array | Count<=15: `90` OR count; <=65535: `DC` and16-bit count; otherwise `DD` and32-bit count; recursive elements in current vector order |
| Object | Count<=15: `80` OR count; <=65535: `DE` and16-bit count; otherwise `DF` and32-bit count; current tree's recursive key/value pairs |
| Extension | Sizes1/2/4/8/16: `D4/D5/D6/D7/D8`; otherwise <=255: `C7` and8-bit size; <=65535: `C8` and16-bit size; otherwise `C9` and32-bit size; raw type byte and payload follow |

Lengths/counts above unsigned32 reach native error/throw requests rather than an admitted larger container encoding. The object encoder does not collapse pairs while walking its current tree; source-tree copy behavior and equal-key ordering are separate. A decoder must retain encoded pairs and their order instead of silently normalizing them into unique-key properties. String encoding writes the stored byte length and retains embedded NUL bytes; this is distinct from the earlier CString-to-DB conversion's first-NUL rule in11.5. Float/Double writers transfer raw payload bits without a reached finite-value filter or arithmetic normalization.

The multibyte primitives index tables using writable control byte`0x180030950`. Value0 reverses the installed x64 little-endian native bytes, producing big-endian payloads. Value1 preserves native byte order. The file image contains0, but that is not proof of current runtime state, an initializer or all-call behavior; other control values are outside the proved table domain. Consequently this is a conditional installed writer grammar, not unconditional MessagePack standards conformance or universal archive admission.

Dump returns a string extracted from actual stream-buffer pointer/length branches. The selected copy uses explicit length, actual memcpy/memmove and an appended terminator, rather than a NUL scan. Valid current variant tables, buffers, terminating recursive traversals, allocation and successful ostream put/write operations remain required. No rollback, complete string after stream failure, valid engineering values or final database page/commit result follows. Additional static import/call-reference reads and the corrected dispatcher label are retained explicitly; they add no byte coverage. Evidence: C-3590.
