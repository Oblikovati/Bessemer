### 14.354 SolidManager component Property and folder reader boundaries

*Status: complete framing for retained component Property streams; folder layout static only · Evidence: C-730*

The bound SolidManager reader `1813CC6C0` enumerates child storages under `Solid Components`, constructs each object with `1813AFD80`, and invokes its primary virtual slot `+28`. The constructor installs table `18205EB70`, whose slot `18205EB98` points to `1813B5FD0`. C-730 verifies constructor instructions, vtable pointers, function-byte hashes, decompilation hashes, and numeric threshold bytes against the installed module. This chain identifies the direct component Property grammar independently of the body serializers in 14.352.

`SolidManager/Solid Components/<child>/Property` contains one counted UTF-16 text (destination +68), then three signed four-byte words (+70/+74/+78), followed by a fourth word (+7C) at **v >= 800066** and a fifth (+C0) at **v >= 840013**. There is no inline collection count or persist-ID prefix in this stream. The current writer emits the text and all five words. Member offsets are destination identifiers; business meanings and enum domains remain unassigned.

The component reader opens a separate stream using literal address `18205EC18` for `CSWPersistID::Serialization` on member +60. It separately opens nested child-storage collections using literals `18205EC40` and `18205EC70`, with distinct object constructors (`1813A9FF0`, `1813302F0`). Their bodies are not appended to the direct component Property. A decoder must not consume a sibling or child-storage payload merely to make a direct Property parse reach EOF.

`cosworks_solid_component_layout.py` and its native audit decode **all 685 retained direct component Property occurrences / 33518 bytes** exactly, preserving every raw span and verifying SHA-256 reconstruction. There are 752 version-candidate attempts: 699 complete and 53 failed alternatives, all retained. Observed stream sizes range from 30 to 112 bytes. At least one candidate succeeds per occurrence; alternative candidates and study-suffix associations remain evidence about framing, not proof of universal native version selection. Three tests cover both threshold boundaries, lossless text/word order, signed values, truncation, remainder, text bounds, and explicit version validation.

The manager's `Folders` child path constructs `1813AFA20`, whose primary table `18205EA88` slot `18205EAB0` binds storage method `1813B5DB0`. Its Property reader consumes: signed word +58, counted text +60, signed count, that many counted texts (list +68), word +A0 at **v >= 840014**, then word +AC at **v >= 870017**. The writer emits both trailing words. The folder read loop consumes no strings for nonpositive counts; an independent safe implementation should reject negative counts rather than reinterpret them as empty. C-590 has **no retained payload streams below SolidManager Folders**; this does not prove empty folder storages are absent. This folder grammar has static evidence only and is not included in the 685 native component successes.

No target was executed. Sibling persist IDs, nested solid/sheet bodies, folder fixtures, field semantics, and historical writer equivalence remain open. C-730 grants zero global coverage credit; full installed-format coverage remains unproven.

Evidence: C-730.
