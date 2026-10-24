## Bessemer 

Bessemer is a Rust library being built to read and write SolidWorks documents
(parts .sldprt, assemblies .sldasm, and drawings .slddrw).
CAD kernels use it to exchange data with SolidWorks files. Martensite is its first consumer.

The first implemented layer reads SolidWorks version-4 containers, extracts streams
with size and CRC checks, and replaces existing stream contents. A save without
edits preserves the whole file byte for byte. An edited save preserves every
untouched stream's compressed bytes. `bessemer::block_stream` also reads and writes
the compressed-block carriers used by body partitions and add-ins, with explicit
framing and expansion limits. Typed models, geometry, and semantic editing
are still to be implemented; this is not yet Corliss feature parity.

```rust
let bytes = std::fs::read("part.sldprt")?;
let mut doc = bessemer::Document::from_bytes(&bytes)?;
let preview = doc.read_stream_limited("PreviewPNG", 16 * 1024 * 1024)?;
doc.replace_stream("PreviewPNG", preview)?;
std::fs::write("copy.sldprt", doc.to_bytes()?)?;
```

Run `cargo test` for the owned native fixtures and malformed-input tests.
`cargo xtask pr` runs formatting, clippy, build, tests, and documentation examples.
`cargo xtask full` currently runs those same gates across the CI platform matrix.
See [container evidence and remaining work](docs/features/container/README.md).
The [compressed-block evidence](docs/features/compressed-blocks/README.md) records
the implemented carrier contract and its verification limits.


This library is named after Sir Henry Bessemer (1813–1898) an English engineer, inventor, and entrepreneur 
best known for developing the Bessemer process, the first inexpensive industrial method for the mass production
of steel. Patented in 1856, his technique revolutionised metallurgy and construction by blasting air through 
molten pig iron to rapidly oxidise impurities, dramatically lowering the cost and time required to produce 
high-tensile steel compared to traditional wrought-iron methods. 
Beyond his landmark achievement in steelmaking, Bessemer was a prolific inventor with over 100 patents to his 
name, contributing innovations in fields ranging from steamship design and sugarcane extraction to solar 
furnaces and the manufacture of bronze powder. His pioneering process laid the material foundation for the 
Second Industrial Revolution, enabling the modern expansion of railways, bridges, ocean liners, 
and skyscrapers worldwide.
