# Owned tessellation vectors

These synthetic archives were assembled independently from the documented wire
primitives by [generate_vectors.py](../../../docs/features/face-tessellations/generate_vectors.py).
They contain no installed sample files. Regeneration is deterministic:

```text
python docs/features/face-tessellations/generate_vectors.py
```

`cases.tsv` gives each standalone facet's version and archive mode. The vectors
include intentionally non-geometric values and raw NaN bits to check preservation;
they are archive fixtures, not accepted CAD bodies. Directory fixtures retain
duplicate map keys and high-word-first 64-bit identities. `block.bin` contains one
component with two modern faces. Tests derive expected identities and bit patterns
independently of the Rust writer.
