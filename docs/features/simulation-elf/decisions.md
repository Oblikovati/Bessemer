# ELF implementation boundaries

Tracking: [#115](https://github.com/Oblikovati/Bessemer/issues/115).
These decisions define the implemented profiles and their verification scope.

The complete installed profile is the 1,224-byte wrapped initial image. Its
1,220-byte body has seven header words and a zero continuation. Constructor
parameters in words 3 and 5 are independent raw patterns. Word 5 supplies a
later header-update count; it does not imply that those data records already
exist in the initial file. The native fixture has 100 empty directory pairs.

The complete initializer operation reads, constructs, edits and reopens the
image through a complete CWR while preserving unrelated streams and metadata.
Source hashes and exact initial-byte goldens are frozen. The independent
parameter-A edit changes stream offset 16 to `0x12345678`.

Selected directory traversal is a separate check scope. A positive identifier
requires the caller's explicit map, zero skips, and a negative identifier stops
after its complete record has been read. Unvisited bytes remain source bytes;
selected traversal does not establish whole populated-file acceptance.
Mapping must retain unassigned destinations and visit-order writes. The owned
overwrite case gives the final repeated ID a different payload, so first-write
behavior cannot accidentally match the expected final slot.

The documented index update produces only 1,220 header-body bytes at wrapped
stream offset 4. It clears previous directory entries and padding and may
advertise records that were not written. Keep that proposal distinct from a
completed file operation. Never wrap or publish the proposal as a valid
populated ELF file without establishing the required record bytes and extents.
The portable projection does not execute the native runtime gates.

Each 244-byte record retains its signed identifier and sixty binary32 patterns
in ten groups of six. Preserve NaNs, signed zero and unused cells exactly.
Store-stage construction starts with positive-zero cells; it does not reproduce
caller filtering, numeric calculation, physical component meanings, units or
coordinate frames.

The owned baseline contains four constructor cases, six literal
mapping cases, all 100 header selectors and five independent scalar-store
cases. The populated cases are synthetic. The actual copied reference readers
passed all three freezer checks; no installed populated ELF case or native
application acceptance has been established.

The maintainer humanizer skill was unavailable; the prose pass remains for
review.
