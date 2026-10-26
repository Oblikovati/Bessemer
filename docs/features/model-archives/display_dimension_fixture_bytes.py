"""Independent display-dimension bases, optional records and all five suffix branches."""
from sketch_tail_fixture_bytes import WireMap, P, VECTOR
from display_item_fixture_bytes import object_base, item, pointers, optional_array, text
from freeze_views import object_tag, transform
from freeze_sketch_text import wire as dimension_text
from dimension_fixture_bytes import dimension, handle_base, attachment
from freeze_node_units import unit_object
from freeze_geometric_references import wire as geometric

CLASSES = ['moDisplayDim_c', 'moDisplayDistanceDim_c', 'moDisplayRadialDim_c', 'moDisplayAngularDim_c', 'moDisplayRevolveDim_c']


def body(raw: bytes) -> bytes:
    return raw[6+int.from_bytes(raw[4:6], 'little'):]


def handles(w: WireMap, version: int, rich: bool) -> bytes:
    if not rich: return bytes(4)
    identity = w.slot+2
    out = w.new('moFeatureDimHandle_c', lambda: handle_base(version, False, 0)+w.new('AngleDim_c', lambda: body(dimension('AngleDim_c', version, 0))))
    return P('<i', 2)+out+P('<H', identity)


def prefix(w: WireMap, version: int, rich: bool) -> bytes:
    out = handles(w, version, rich)+P('<3H', 65535, 65534, 32768)
    out += b''.join(text(value) for value in ['Ω prefix', '', 'suffix', '<RTL> Ω'])+VECTOR
    return out+(P('<Q', 0x7ff000000000002a) if rich else P('<d', -0.0))


def placement(rich: bool) -> bytes:
    out = P('<3d', -0.0, 1.25, -2.5)+VECTOR*5+P('<i', -1 if rich else 0)
    if rich: out += VECTOR*2+P('<d', -0.0)
    return out+P('<7H', 65535, 65534, 32768, 0, 1, 65533, 65532)+VECTOR


def references(w: WireMap, version: int, name: str, rich: bool) -> bytes:
    out = b''
    if name != 'moDisplayDim_c':
        if rich:
            out += w.new('moPointRef_w', lambda: body(geometric('moPointRef_w', version, False, 0)))
            out += P('<H', w.slot)
        else: out += bytes(4)
        out += P('<2d', -0.0, 1.25)
        for present in [rich, False]:
            out += P('<i', -1 if present else 0)+(VECTOR if present else b'')
    return out+P('<2HI', 65535, 65534, 0x89abcdef)+optional_array(w, rich)+optional_array(w, rich)


def parameters(rich: bool) -> bytes:
    return P('<4I3Q', 8, 0xffffffff, 0x89abcdef, 3, 1<<63, 0x7ff000000000002a, 1) if rich else bytes(4)


def units(w: WireMap, version: int, rich: bool) -> bytes:
    out = text('Ω alignment')
    for name in ['moLengthUserUnits_c', 'moAngleUserUnits_c']:
        identity = w.slot+2
        out += w.new(name, lambda name=name, identity=identity: body(unit_object(name, version, identity))) if rich else bytes(2)
    return out+P('<id', -2**31, -0.0)+parameters(rich)+text('Ω units')


def favorite_handle(w: WireMap) -> bytes:
    return w.new('moFavoriteHandle_c', lambda: P('<iiI', -2**31, 2**31-1, 0xffffffff))


def favorite(w: WireMap, rich: bool) -> bytes:
    out = favorite_handle(w) if rich else bytes(2)
    out += P('<d2HI4didHi', -0.0, 65535, 65534, 0x89abcdef, -0.0, 1, 2, 3, -2**31, -0.0, 65533, 2**31-1)
    words = [index*0x1020304 & 0xffffffff for index in range(30)]
    words[28] = 0xffffff0f
    return out+P('<30I', *words)


def attachments(w: WireMap, rich: bool) -> bytes:
    out = P('<H', 2 if rich else 0)
    if rich:
        out += w.new('moAttachPoint_c', lambda: body(attachment()))+P('<H', w.slot)
    return out+pointers(w, rich, rich)+P('<iI', -2**31, 0x89abcdef)


def jogs(rich: bool, owner: int) -> bytes:
    out = bytes(4)+P('<HI', owner if rich else 0, 0x89abcdef)+bytes(4)
    for present in [rich, False]:
        out += P('<i', -1 if present else 0)+(VECTOR if present else b'')
    return out+P('<i', -1)


def late_fields(w: WireMap, version: int, rich: bool, final_word: int = -1) -> bytes:
    out = w.new('moPointBackedUpData_c', lambda: VECTOR) if rich else bytes(2)
    out += P('<5id6i', -2**31, 2**31-1, -1, 0, 7, -0.0, -1, 0, -2**31, 2**31-1, -7, 7)
    out += text('Ω text 808')+text('text 810')+P('<3i', -2**31, 2**31-1, -1)+text('Ω text f60')
    out += P('<2i', -1, 0)+VECTOR+P('<2i', -2**31, final_word)
    return out+(text('Ω f88') if version >= 0x27e0 else b'')


def shared(w: WireMap, version: int, name: str, rich: bool, owner: int, final_word: int = -1) -> bytes:
    out = object_base(w, rich, owner)+item(w, version, rich)
    out += body(dimension_text(version, 2 if rich else 0, 2 if rich else 0, False))
    out += prefix(w, version, rich)+placement(rich)+references(w, version, name, rich)
    out += units(w, version, rich)+favorite(w, rich)+P('<d', -0.0)
    out += b''.join(text(value) for value in ['Ω b70', '', '<RTL> b80', 'b88'])
    return out+attachments(w, rich)+jogs(rich, owner)+late_fields(w, version, rich, final_word)


def callout_data(w: WireMap, rich: bool) -> bytes:
    return bytes(2)+pointers(w, rich)+P('<4i', -2**31, 2**31-1, -1, 0)+text('Ω 40')+text('48')+pointers(w, rich, rich)+P('<i', -1)


def radial(w: WireMap, version: int, rich: bool) -> bytes:
    out = P('<i6d', -2**31, -0.0, 1, 2, 3, 4, 5)+VECTOR*2+P('<i', 4 if rich else 0)
    if rich: out += b''.join(text(value) for value in ['Ω callout', '', '<RTL> x', 'last'])
    out += w.new('moHwCalloutData_c', lambda: callout_data(w, rich)) if rich else bytes(2)
    out += text('Ω radial')+(w.array('suObList') if rich else bytes(2))+P('<i', -1 if rich else 0)
    if rich: out += transform(True, 0x1a)
    out += bytes(2)+P('<i', -2**31)
    if version > 0x333d: out += bytes(2)
    return out+(P('<i', 2**31-1) if version > 0x3b21 else b'')


def suffix(w: WireMap, version: int, name: str, rich: bool) -> bytes:
    if name == CLASSES[0]: return b''
    if name == CLASSES[1]:
        return P('<i2d', -2**31, -0.0, 1.25)+VECTOR+P('<iH', 2**31-1, 0)+(P('<i', -1) if version > 0x3747 else b'')
    if name == CLASSES[3]:
        return P('<2HB', 65535, 65534, 255)+VECTOR*2+P('<4di', -0.0, 1, 2, 3, -2**31)
    out = radial(w, version, rich)
    if name == CLASSES[4]: out += P('<i', -1)+VECTOR*2+P('<4d', -0.0, 1, 2, 3)+transform(True, 0x4a)+P('<i', -2**31)
    return out


def tagged(version: int, name: str, rich: bool, final_word: int = -1) -> bytes:
    w = WireMap()
    return w.new(name, lambda: shared(w, version, name, rich, 2, final_word)+suffix(w, version, name, rich))
