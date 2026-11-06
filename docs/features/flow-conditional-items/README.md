# Flow conditional-item records

Issue: #241. References: `flowrecord126.py`, `flowrecord315.py` and
`flowrecord128.py` in `../inventor-interface/solidworks`.

These records share the documented version-zero-through-five item grammar.
Record 126 uses raw transport and three-dimensional items. Record 315 stores its
version-four payload inside one exact zlib stream and uses one-dimensional
items. Record 128 stores its version-six or version-seven prefix outside the
zlib body. Its counted wide-vector elements occupy 72 or 48 bytes, respectively;
the serialized patterns remain individual raw 64-bit words.

The owned FCI1 packets retain original stored bytes, complete decoded bytes,
every field's path, offset, encoding and values, nested scope intervals, count
locations and the reference's mode projections. Empty arrays remain explicit
field descriptors. Omitted counts, absent objects, inactive objects and empty
active collections remain distinct. The packet for each record stays within
the existing 16 MB fixture-decoder bound.

Generated requests cover every activity, presence and stored-mode byte, all six
item versions, conditional legacy vectors and subrecords, all outer modes,
zero/nonzero gates, arbitrary signed subrecord kinds, independent vector
lengths, both block widths and complete raw word/wide patterns. The reference
reader supplies expected values; a separate serializer rebuilds every decoded
byte and checks its item and subrecord boundaries against the reference.

Three native payloads come from the audited installed `1.fld` sample. Record
126 has populated version-two items. Record 128 has populated version-six
block-vector items. Record 315 has populated numeric arrays and empty item
groups. Whole-file and payload hashes, stored/decoded extents and versions are
recorded in `baseline-verification.json`. This evidence does not establish
other native versions or native runtime, inverse-map, engineering or application
acceptance.

Python regenerates the packets in a temporary directory, checks complete field
serialization, mutates every reached compact-profile count, rejects every
proper decoded prefix and trailing byte, and checks exact compressed-stream
boundaries. Rust reconstructs the decoded payloads from their frozen fields and
checks transport, scope/count accounting, raw patterns and mode projections.
There is no typed production API for this family yet; `design.md` records the
requirements for that implementation.
