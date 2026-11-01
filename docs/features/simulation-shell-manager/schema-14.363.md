### 14.363 ShellManager direct Property and empty group archive framing

*Status: all retained direct headers and empty group streams complete; child/composite bodies remain open · Evidence: C-741*

The retained study reader calls **181344D10** for the ShellManager object. This storage method opens direct Property and independently enumerates child storages whose installed literals are **Shells** (`18205B080`), **MidSurfShells** (`18205B0A8`), and **ShellManagerNew** (`18205B0E8`). C-741 binds the function-byte/decompilation hashes, study caller, literal bytes and numeric gate to the installed module. The Shells children are serialized through a separate virtual +260 method; their records are not inline in the direct manager header.

Direct `ShellManager/Property` contains a signed four-byte word to manager +C8. A second signed word to +10C follows only when the supplied version is **strictly greater than 800027** (`181F5AAC8`). The current writer emits both words unconditionally. The independent parser keeps the strict historical gate, validates exact stream length, preserves signed values and requires complete reconstruction. Field meanings are not inferred from native destination offsets.

Within ShellManagerNew, the named **ShellManagerGroup** stream (`18205B108`) is opened through an MFC file/archive path, separately from CwPersistStream Property. The read path first consumes a four-byte signed count. For each positive entry it allocates an object using `180C70BF0`, calls `180C7E650`, then adds it to manager member +180. Those populated record bodies remain separate decoding work. A zero count ends the observed empty stream; the independent parser explicitly rejects negative/populated counts and trailing bytes instead of pretending they have an empty layout. This four-byte count is not a nested native object archive version word.

`cosworks_shell_manager_layout.py` and the C-741 audit freshly hash-check source envelopes and payloads, then replay **361 direct headers / 2888 bytes** and **69 empty group streams / 276 bytes**. All **430 / 3164 bytes** reconstruct exactly and reach EOF. Every native direct header is eight bytes; every retained group stream is a single zero count. Three tests cover the strict 800027/800028 threshold, header lengths, signed values, populated/negative count stops, remainder/truncation, and explicit version validation. Empty native groups do not validate the allocated child serializer.

The retained ShellManager inventory also contains 249 other Property streams / 842326 bytes, 237 Material / 802740 bytes, nine CompositeExtra / 516 bytes, and eleven CompositeFlipTurn / 220 bytes. Material framing is already covered by C-724, but the other child/composite payloads are not declared complete by this header audit. Parent header counts, CFB child names, and separate archive counts must not be substituted for one another.

No target executes. Populated groups, shell/midsurface child properties, composite tails, field meanings and complete installed-format coverage remain open. Global semantic coverage credit remains zero.

Evidence: C-741.
