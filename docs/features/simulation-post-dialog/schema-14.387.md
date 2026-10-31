### 14.387 Embedded PostInfo block and extended dialog prefix

*Status: prefix through PostInfo validated for all retained dialog streams; PostStyle remains next dependency · Evidence: C-767*

DGXPOST **CDspPostInfo::ReadFromStream**, exported at **180044950**, reads a fixed **0x1CC / 460-byte block** directly into the object for every version except exactly **700002**. At that one historical version it still reads the same block, then one four-byte temporary word, a signed UTF-16 character count, and twice that many text bytes when positive. The legacy string is allocated dynamically and consumed even though it is not retained in the fixed object. This exact-version suffix must not be generalized to adjacent versions.

C-767 retains native function/decompilation hashes for PostInfo and the following **CDspPostStyle::ReadFromStream** at **180044AE0**, tied to the hash-bound DGXPOST provider. The dialog reader's supported minimum is 700335, so the 700002 branch is a standalone historical PostInfo profile, not a reachable branch of these retained dialog fixtures. Its validation is static/synthetic only.

`cosworks_post_info_layout.py` bounds the complete raw block and optional legacy count/text, preserves original bytes, rejects negative or configured-over-limit counts, and supports strict standalone EOF or an explicit offset/prefix mode. The independent safety cap is not asserted as a native schema count limit. The block's width/order is established, but its individual field meanings remain unresolved; it is not promoted to complete semantic coverage merely because the raw bytes are preserved.

`cosworks_post_dialog_info_prefix.py` composes the previously verified seven-string initial prefix with PostInfo, retaining contiguous spans and stopping immediately before PostStyle. Native object offsets +40 for PostInfo and +240 for PostStyle are memory locations, not a request to insert the intervening allocation padding into the wire stream.

Fresh C-767 replay validates **all 224 dialog prefixes / 181714 consumed bytes**, comprising the earlier 78674 initial-prefix bytes plus **103040 PostInfo bytes**. The full streams total 496979 bytes, leaving **315265 bytes** outside this prefix claim. Each result remains **complete=False** with the next dependency explicitly named PostStyle; full dialog completion is not claimed. Five tests cover the exact legacy version, adjacent-version behavior, text/count bounds, offset/EOF boundaries and composition with a following parent body.

PostStyle itself calls a separate **DSPOGL.dll COGLColor** reader and a DGXPOST **CDspLegend** reader, then character-count text and versioned fields. These are the next boundaries; their widths cannot be inferred from the PostInfo allocation or guessed from remaining stream length. Later dialog fields, physical meanings, historical variants and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-767.
