# Earlier camera, component and simple-text layouts

The complete-root fixtures cover versions 2902, 2932, 2933, 2970, 2971,
2998 and 2999. The reference reader independently parses every fixture to
the end of the stream. The 66 fixtures provide 8192 expected fields across
part and assembly roots, including all five old light classes and assemblies
with explicitly identified external face references.

Simple text stores its 3D flag and optional plane only after `0xB74`.
Viewer components store `unknown_190` from `0xB9B` onward. Views store a
camera name only after `0xBB6`. Absent fields occupy no bytes; the writer
does not insert defaults. Text fixtures retain explicit width counts and
signed width words independently of the string length.

The generator removes fields at offsets observed in a complete reference
parse, then parses the resulting root again at its declared version.
Integration tests compare the reference field traces and force reconstruction
of every byte. The process verifier independently regenerates the frozen
fixtures and compares reconstruction in two fresh processes.

This checkpoint expands archive support to version 2902. It does not resolve
the older inline-facet shapes: those payloads still return a typed unsupported
error. Complete schema parity remains tracked by issue #28.
