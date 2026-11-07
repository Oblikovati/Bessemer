# Bessemer

Bessemer is a Rust library for reading and writing SolidWorks documents:
parts (`.sldprt`), assemblies (`.sldasm`), and drawings (`.slddrw`).
CAD kernels use it to exchange data with SolidWorks files.
Martensite is its first consumer.

The library reads document containers, extracts their streams, and edits
supported layouts while preserving unchanged bytes. Typed views expose
validation coverage and edit history. Support is growing; the documentation
records the file versions and layouts covered by each feature.

## Explore the library

- [Feature reference](docs/feature-index.md) — supported formats and APIs.
- [Examples](examples) — using the library to read and edit documents.
- [Changelog](CHANGELOG.md) — changes across versions.

## The name

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

## License

Bessemer is licensed under [GPL-2.0-only](LICENSE).
