# Flow base grid

Issue: #241. Reference: `../inventor-interface/solidworks/flowbasegrid.py`,
specification section 14.41 and the native base-grid corpus.

Record 101 has three signed dimensions followed by three boundary arrays,
each with dimension plus one binary64 words. It has no version word. Zero
dimensions retain one boundary word and produce an empty default tree range.
The baseline preserves every bit, including signed zero and nonfinite values.

The owned fixture contains 56 views: the native CPT payload and all 27 triples
drawn from dimensions zero, one and three, each in raw and zlib transport.
Source file hashes and the native boundary arrays are checked before freezing.
The Rust baseline independently rebuilds all decoded bytes and verifies the
mathematical grid count, default tree bounds, array spans and raw word bits.
Regeneration uses a temporary directory and never overwrites the owned fixture.

This baseline precedes the typed reader and writer. It does not validate units,
boundary monotonicity, model geometry, native refinement, runtime object
indexing or block-vector ordering. Compression context remains explicit.
