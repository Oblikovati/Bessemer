"""Independent ordinary sketch tail wire assembly; map slots are assigned before bodies."""
import struct
from collections.abc import Callable
from freeze_views import object_tag, transform
from owned_name_edit import short_wide_name
from sketch_reference_fixture_bytes import entity
from sketch_dimension_fixture_bytes import logical, logical_body
from freeze_boundary_references import base as boundary_base

P = struct.pack
VECTOR = P('<3d', -0.0, 1.25, -2.5)


class WireMap:
    def __init__(self) -> None:
        self.slot = 0

    def new(self, name: str, body: Callable[[], bytes]) -> bytes:
        self.slot += 2
        return object_tag(name)+body()

    def handle(self, name: str = 'sgPointHandle') -> bytes:
        return self.new(name, lambda: P('<Hiii', 0x777f, -1, -19, 2**31-1))

    def array(self, name: str = 'suObArray', extended: bool = False) -> bytes:
        return self.new(name, lambda: (P('<HI', 65535, 2) if extended else P('<H', 2))+bytes(4))


def points(w: WireMap, rich: bool) -> bytes:
    return P('<3H', 65535, 65534, 32768)+(w.handle() if rich else bytes(2))


def constraint(w: WireMap, version: int, rich: bool) -> bytes:
    out = logical_body(1, version, False, 0)+P('<H', 65535)
    out += w.handle('sgLineHandle') if rich else bytes(2)
    if not rich: return out+bytes(6)
    out += w.handle()
    return out+P('<2H', w.slot, w.slot)


def logical_group(version: int, rich: bool) -> bytes:
    values = [-2**31, 7, 7, 2**31-1] if rich else []
    return logical(version)+P('<H', len(values))+P('<'+'i'*len(values), *values)


def constraints(w: WireMap, version: int, rich: bool) -> bytes:
    out = P('<H', 1)+points(w, rich)+P('<H', 1)+constraint(w, version, rich)
    return out+(P('<H', 1)+logical_group(version, rich) if version > 0x46cb else b'')


def profile(w: WireMap, rich: bool) -> bytes:
    out = P('<H', 2 if rich else 0)
    if rich:
        out += w.array(extended=True)+P('<H', w.slot)
    return out+P('<Hd', 65535, -0.0)


def loop_offset(w: WireMap, rich: bool) -> bytes:
    out = P('<iiH', -2**31, 2**31-1, 2 if rich else 0)
    return out+(w.array()+P('<H', w.slot) if rich else b'')


def loop_edge(w: WireMap, rich: bool) -> bytes:
    return P('<H', 65535)+(w.array(extended=True) if rich else bytes(2))


def sectioned_edge(w: WireMap, rich: bool) -> bytes:
    out = loop_edge(w, rich)+P('<H', 1 if rich else 0)
    return out+(P('<H', w.slot) if rich else b'')


def section_handles(w: WireMap, rich: bool) -> bytes:
    return bytes(4)+(w.array() if rich else bytes(2))


def sketch_offset(w: WireMap, rich: bool) -> bytes:
    out = P('<i', 2 if rich else 0)
    if rich: out += P('<2Iiid', 0xffffffff, 0x89abcdef, -2**31, 2**31-1, -0.0)+VECTOR*2+P('<H', 65535)
    out += w.new('sgPGMProfileData_c', lambda: profile(w, rich)) if rich else bytes(2)
    return out+P('<ii', -2**31, 2**31-1)


def offset(w: WireMap, version: int, rich: bool, kind: int = 7) -> bytes:
    out = P('<H', 3 if rich else 0)
    if rich:
        out += bytes(6)+P('<3I', 0xffffffff, 0, 0x89abcdef)
        out += P('<H', 1)+w.handle()+P('<2H', 2, 65535)
    out += P('<d3H', -0.0, 65535, kind, 65534)+bytes(2)+P('<2H3i', 65533, 65532, -2**31, 2**31-1, -1)+VECTOR*2
    if 7 <= kind <= 9:
        if rich: out += P('<3i', -2**31, 7, 2**31-1)
        out += VECTOR+P('<i', -1)
    return out


def offsets(w: WireMap, version: int, rich: bool) -> bytes:
    return P('<H', 1)+offset(w, version, rich)+P('<H', 1)+sketch_offset(w, rich)


def states(rich: bool) -> bytes:
    return P('<4I3H', 2, 0xffffffff, 0x89abcdef, 3, 0, 32768, 65535) if rich else bytes(4)


def styled(w: WireMap, rich: bool) -> bytes:
    out = P('<i', 2 if rich else 0)
    if rich: out += w.handle()+P('<H', w.slot)
    return out+states(rich)+P('<4iH2d', -2**31, 2**31-1, -1, 0, 65535, -0.0, 1.25)


def dynamic_words(rich: bool, width: int = 8) -> bytes:
    if not rich: return bytes(4)
    count = 2
    return P('<4I', width, 0xffffffff, 0x89abcdef, count)+P('<'+'I'*(count*width//4), *range(count*width//4))


def edge(w: WireMap, version: int, rich: bool) -> bytes:
    out = P('<5i', -2**31, 2**31-1, -1, 0, 7)
    for _ in range(4):
        out += P('<H3i', 65535, -2**31, 2**31-1, -1)
        out += w.new('moLoopRef_c', lambda: boundary_base(version, False)+P('<I', 0xffffffff)) if rich else bytes(2)
        out += bytes(4)
    return out


def intersection(w: WireMap, version: int, rich: bool, count: int = 1) -> bytes:
    out = P('<HIii', 0, 0x89abcdef, count, -1)+bytes(12)
    if count > 0: out += w.handle() if rich else bytes(2)
    out += dynamic_words(rich)
    if count > 0: out += edge(w, version, rich)
    out += P('<B', 255 if rich else 0)+(transform(True, 0x2a) if rich else b'')
    if version > 0x2b71: out += P('<2H', 65535, 65534)+bytes(4)
    if version > 0x2b98:
        out += P('<4H', 65533, 65532, 65531, 65530)+bytes(2)+P('<i', 2 if rich else -1)
        if rich: out += VECTOR*2
    return out+(P('<d', -0.0) if version >= 0x2be0 else b'')


def styled_intersections(w: WireMap, version: int, rich: bool) -> bytes:
    return P('<H', 1)+styled(w, rich)+P('<H', 2)+P('<H', 65535)+intersection(w, version, rich)


def state() -> bytes:
    return P('<2Hi15i4H', 65535, 65534, -2**31, *range(-7, 8), 65533, 65532, 65531, 65530)


def visual(w: WireMap, rich: bool, indicator: int = 0) -> bytes:
    out = short_wide_name('Ω line style')+P('<hf2ii', -123, -0.0, -2**31, 2**31-1, -1)
    out += VECTOR+P('<Iii', 0xffffffff, -2**31, 2 if indicator else 0)
    if indicator:
        out += w.new('moExtensionLineIndicator_c', lambda: P('<HH', indicator, 65535))+P('<H', w.slot)
    return out


def local_relations(w: WireMap, rich: bool) -> bytes:
    out = P('<H', 2 if rich else 0)
    if rich: out += w.array()+P('<H', w.slot)
    out += P('<H', 2 if rich else 0)
    if rich: out += (short_wide_name('Ω duplicate')+P('<d', -0.0))*2
    return out+P('<i', 1 if rich else 0)+(entity(True) if rich else b'')


def coincidences(w: WireMap, rich: bool) -> bytes:
    pairs = P('<i4i', 2, -1, -2**31, -1, -2**31) if rich else bytes(4)
    return pairs*4+(w.array('suObList') if rich else bytes(2))


def silhouettes(version: int, rich: bool) -> bytes:
    if version <= 0x2f50: return b''
    return P('<3H', 2, 1, 65535) if rich else bytes(2)


def handle_data(w: WireMap, rich: bool) -> bytes:
    return P('<i', 1)+w.handle()+bytes(2) if rich else bytes(4)


def blocks(w: WireMap, rich: bool) -> bytes:
    out = P('<i', 2 if rich else 0)
    if rich: out += (P('<iH', -1, 0))*2
    out += P('<i', 2 if rich else 0)
    if rich: out += P('<iII', -1, 0xffffffff, 0x89abcdef)*2
    return out+handle_data(w, rich)


def loops(w: WireMap, rich: bool) -> bytes:
    if not rich: return bytes(4)
    return P('<3i', 2, 0, -1)+P('<ii4i4iidH', -2**31, 2, -1, 7, -1, 7, -2**31, 2**31-1, -1, 0, -1, -0.0, 0)


def slots(w: WireMap, rich: bool) -> bytes:
    return P('<ii', -1, 2)+P('<iH', -7, 0)*2 if rich else P('<ii', -1, 0)


def after_patterns(w: WireMap, version: int, rich: bool, final_word: int = -2**31, indicator: int = 0) -> bytes:
    out = w.array('suObList') if rich else bytes(2)
    out += constraints(w, version, rich)+offsets(w, version, rich)+styled_intersections(w, version, rich)
    out += state()+visual(w, rich, indicator)+local_relations(w, rich)+coincidences(w, rich)+silhouettes(version, rich)
    return out+blocks(w, rich)+loops(w, rich)+slots(w, rich)+P('<i', final_word)


def ordinary(w: WireMap, version: int, rich: bool, final_word: int = -2**31, indicator: int = 0) -> bytes:
    return bytes(52+24)+after_patterns(w, version, rich, final_word, indicator)
