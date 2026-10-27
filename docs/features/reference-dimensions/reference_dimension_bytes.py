"""Independent ordinary reference-dimension wire assembly (#54)."""
from dataclasses import dataclass
import struct
from dimension_fixture_bytes import dimension_base, handle_base
from freeze_views import transform
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from plane_point_fixture_bytes import point_pointer
from sheet_fixture_bytes import component_reference
from sketch_tail_fixture_bytes import ordinary

CLASSES = ('moRefDim_c', 'moRefDim1E_c', 'moRefDim2E_c', 'moRefDimHandle_c', 'moDimPlaneEntTag_c')
HANDLES = ('moSkDimHandleRadial_c', 'moSkDimHandleOffset_c', 'moSkDimHandleValG2_c', 'moSkDimHandleValG3_c')


@dataclass(frozen=True)
class ReferenceOptions:
    style: int = 0
    edited: bool = False


def plane_body(mapping: WireMap, version: int, ids: dict[str, int], style: int) -> bytes:
    """Preserve stored kind, including unknown null kinds; e.g. plane_body(mapping, 19000, ids, 3)."""
    kind = (0, 1, 2, 65535, 0)[style]
    raw = struct.pack('<H', kind)
    if kind == 2:
        return raw+struct.pack('<6Q', 1 << 63, 0x7ff8000000001234, 1, 2, 3, 4)
    if style == 1:
        return raw+component_reference(mapping, version, ids, 'moCompFace_c')
    return raw+(point_pointer(mapping, version, ids, 0) if style == 4 else bytes(2))


def sketch_handle(mapping: WireMap, version: int, ids: dict[str, int], style: int) -> bytes:
    """Use a complete sketch and derived handle rather than a loaded stub; e.g. sketch_handle(mapping, 19000, ids, 1)."""
    if style == 0:
        return bytes(2)
    def contents() -> bytes:
        prefix = handle_base(version, False, 0)
        sketch = shared_object(mapping, ids, 'sketch', 'sgSketch', lambda: ordinary(mapping, version, False))
        return prefix+sketch+struct.pack('<i', -2147483648)
    return shared_object(mapping, ids, 'sketch-handle', HANDLES[(style-1) % 4], contents)


def reference_base(mapping: WireMap, version: int, ids: dict[str, int], options: ReferenceOptions) -> bytes:
    """Keep raw presence bytes and all direct transform branches; e.g. reference_base(mapping, 19000, ids, options)."""
    style = options.style
    rich = style in (1, 2, 3)
    raw = dimension_base(version, rich)
    mapping.slot += 8 if rich else 0
    raw += transform(style in (1, 3), (0, 0x10, 0x20, 0x40, 0xff)[style])
    for present in ((0, 0), (255, 0), (0, 254), (1, 255), (0, 0))[style]:
        raw += bytes([present])+(struct.pack('<3d', -0.0, 1.25, -2.5) if present else b'')
    raw += struct.pack('<i', -2147483648)
    raw += shared_object(mapping, ids, 'plane', CLASSES[4], lambda: plane_body(mapping, version, ids, style))
    raw += struct.pack('<i', 2147483647)+sketch_handle(mapping, version, ids, style)
    raw += struct.pack('<3i', -1, -32769, 65536)
    return raw+(struct.pack('<i', -2147483648) if version >= 0x46c7 else b'')


def reference_body(mapping: WireMap, version: int, ids: dict[str, int], kind: int, options: ReferenceOptions) -> bytes:
    """Use independently selected one/two-entity version suffixes; e.g. reference_body(mapping, 19000, ids, 2, options)."""
    raw = reference_base(mapping, version, ids, options)
    if kind == 0:
        return raw
    raw += struct.pack('<H', 65535)+point_pointer(mapping, version, ids, 0)
    if kind == 1:
        return raw+struct.pack('<H', 0)+(struct.pack('<H', 65534) if version >= 0x2f6f else b'')
    raw += struct.pack('<H', 0)+point_pointer(mapping, version, ids, 0)
    raw += struct.pack('<5H', 65535, 65534, 32768, 65535 if options.edited else 0, 1)
    return raw+(struct.pack('<H', 32769) if version > 0x3747 else b'')


def reference_object(kind: int, version: int, options: ReferenceOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Count every real map allocation, including dimension children; e.g. reference_object(2, 19000, ReferenceOptions(3))."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        if kind == 4:
            return plane_body(mapping, version, ids, options.style)
        if kind != 3:
            return reference_body(mapping, version, ids, kind, options)
        prefix = handle_base(version, False, 7 if options.style else 0)
        mapping.slot += 2 if options.style else 0
        return prefix+mapping.new(CLASSES[2], lambda: reference_body(mapping, version, ids, 2, options))
    raw = mapping.new(CLASSES[kind], contents)
    return raw, mapping.slot+1
