"""Independent complete display-object/item assembly with explicit shared map identities."""
from sketch_tail_fixture_bytes import WireMap, P, VECTOR
from freeze_views import transform
from owned_name_edit import short_wide_name as text


def reference(w: WireMap, component: bool = False) -> bytes:
    name = 'moCompDispObjRef_c' if component else 'moDispObjRef_c'
    return w.new(name, lambda: P('<4i', -2**31, 2**31-1, -1, 0)+(bytes(2) if component else b''))


def pointers(w: WireMap, rich: bool, extended: bool = False) -> bytes:
    if not rich: return bytes(2)
    out = P('<HI', 65535, 2) if extended else P('<H', 2)
    out += reference(w)
    return out+P('<H', w.slot)


def optional_array(w: WireMap, rich: bool) -> bytes:
    return P('<i', -1 if rich else 0)+(pointers(w, rich, rich) if rich else b'')


def object_base(w: WireMap, rich: bool, owner: int) -> bytes:
    out = P('<4Hi', 65535, owner if rich else 0, 0, 0, -2**31)+P('<i', 2**31-1)
    out += P('<2HBIHhf', 65534, 65533, 255, 0x89abcdef, 65532, -123, -0.0)
    out += pointers(w, rich, rich)+pointers(w, rich)
    return out+P('<iHii', -1, 0, 0, -2**31)


def item_prefix(w: WireMap, rich: bool) -> bytes:
    return P('<2i', -2**31, 2**31-1)+VECTOR+optional_array(w, rich)+bytes(4)


def geometry(w: WireMap, rich: bool) -> bytes:
    out = bytes(2)+VECTOR+transform(rich, 0x2a)+P('<i', 3 if rich else 0)
    if rich: out += P('<3i', -2**31, -1, 2**31-1)
    out += P('<B', 255 if rich else 0)+(VECTOR if rich else b'')+P('<di', -0.0, -1 if rich else 0)
    if rich: out += transform(True, 0x4a)
    return out+bytes(4)+bytes(2)+P('<d', -0.0)


def styles(w: WireMap, rich: bool) -> bytes:
    out = P('<i', 2 if rich else 0)
    if rich:
        out += P('<i', -1)+w.new('moArrowSize_c', lambda: P('<3d', -0.0, 1.25, -2.5))+P('<iH', -1, w.slot)
    out += P('<ihf2ihf5i', -1, -123, -0.0, -2**31, 2**31-1, -124, 1.25, -2**31, 2**31-1, -1, 0, 7)
    return out+optional_array(w, rich)+P('<2ihf2i', -1, -2**31, -125, -0.0, 2**31-1, -1)


def item_tail(version: int, rich: bool) -> bytes:
    out = bytes(8)
    if version <= 0x3371: return out
    out += P('<i', 4 if rich else 0)+(VECTOR*4 if rich else b'')
    if version < 0x3374: return out+VECTOR
    return out+P('<i', 2 if rich else 0)+(VECTOR*2 if rich else b'')


def item(w: WireMap, version: int, rich: bool) -> bytes:
    return item_prefix(w, rich)+geometry(w, rich)+styles(w, rich)+item_tail(version, rich)
