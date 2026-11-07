# Flow result-definition profile

Issue: #241. Reference: `../inventor-interface/solidworks/flowresultdefinitions.py`.
The complete record-500 reader and existing-field editor implement this profile.

The documented profile contains one exact zlib stream. Its decoded body starts
with byte version one and contains five consecutive collections. Each collection
stores a signed nonnegative count followed by all signed object identifiers,
then the corresponding GUID and definition bodies. Only the first collection
may be nonempty in this profile. Duplicate identifiers remain physical entries;
they must not be collapsed into a map.

Every definition GUID must equal the documented IMS GUID. Its signed version is
zero through three. Version zero has no shared arrays. Versions one and above
have a first array; versions two and three add a second array and a raw allocation
byte. Version three adds two more arrays. Shared arrays store signed marker one,
an unsigned 64-bit count limited to the signed native-transfer range, and raw
32-bit patterns. All arrays, including empty arrays, retain their intervals and
physical presence. Complete decoded EOF is required.

The reader bounds complete inflated bytes, object count and total array
values independently before allocation. Views retain the complete source,
physical object ordinals, signed identifiers, versions, raw GUID and allocation
byte, array intervals and raw patterns. Existing-field edits preserve count,
GUID and version structure, operate on a copy and reparse the same profile.
Compressed no-ops must retain the original stored payload. Fresh reversals must
restore decoded bytes; they need not reproduce another compressor's encoding.

The independent native manifest contains one record-500 sample from the installed
CFLD carrier: record offsets 65,378 through 72,157, a 6,767-byte stored payload,
one primary IMS version-two definition, allocation byte zero and array lengths
3,876 and 4,056. The freeze authenticates the source, record extent, payload and
complete normalized projection against the native manifest. Generated cases
cover all four definition versions, raw bit patterns, allocation bytes,
multiple and duplicate signed identifiers, empty collections/arrays, compression
variants and resource boundaries. Malformed markers, counts, GUIDs, versions,
secondary nonempty collections, truncations and suffixes must be rejected.

Record 501 is a separate paired profile. Its documented reader requires fresh
objects, IMS version-two definitions and allocation byte zero. A future value
context must retain exact typed definition provenance rather than accepting an
unverified count or copied identifier list. Full record-500 framing must be
proved before extending that paired reader. Native runtime initialization,
engineering field meaning and application acceptance remain separate work.

The humanizer skill is unavailable; maintainer prose review remains pending.
