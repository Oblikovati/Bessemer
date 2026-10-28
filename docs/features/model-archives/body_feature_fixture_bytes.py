"""Independent ordinary inherited body features, maps, chooser cycles and derived suffixes."""
from collections.abc import Callable
from sketch_tail_fixture_bytes import WireMap, P, VECTOR
from freeze_environment import feature
from owned_name_edit import short_wide_name as text
from freeze_views import transform
from freeze_geometric_references import wire as geometric
from freeze_model_references import wrapper
from display_dimension_fixture_bytes import body

CLASSES = ['moExtrusion_c','moBoss_c','moCut_c','moICE_c','moRevolution_c','moRevCut_c','moRib_c']

def items(rich: bool) -> bytes:
    return P('<HI',65535,2)+bytes(4) if rich else bytes(2)

def nullable(rich: bool) -> bytes:
    return P('<i',-1 if rich else 0)+(items(rich) if rich else b'')

def header(w: WireMap, version: int, rich: bool) -> bytes:
    w.slot += 2  # The existing independent feature wire owns its node-name object.
    return feature(version,False)+P('<i',-2**31)+items(rich)*2+bytes(2)

def references(rich: bool) -> bytes:
    out=nullable(rich)*2+bytes(2)+P('<i',3 if rich else 0)
    if rich:out+=P('<3i',-2**31,2**31-1,-1)
    return out+P('<i',2**31-1)+nullable(rich)+P('<i',-1)

def chooser(rich: bool, owner: int) -> bytes:
    return P('<H',65535 if rich else 0)+(items(rich)+P('<H',owner) if rich else b'')

def choosers(rich: bool, owner: int) -> bytes:
    c=chooser(rich,owner)
    return c+P('<H',65534)+c+P('<i',-2**31)+c+c+P('<i',2**31-1)

def maps(w: WireMap, rich: bool) -> bytes:
    out=nullable(rich)+P('<i',-1 if rich else 0)
    if rich:out+=w.new('moPMarkRecord_c',lambda:P('<i',-2**31))+P('<3i',-1,0,2**31-1)
    out+=P('<i',-2**31)
    for edge in [False,True,False]:
        out+=P('<i',2 if rich else 0)
        if rich:out+=(P('<HI',0,0xffffffff) if edge else P('<2I',0xffffffff,0x89abcdef))*2
    return out

def surface_ids() -> bytes:
    return P('<iIi',3,0xffffffff,7)+P('<Iii',0x89abcdef,-1,0)+P('<Iiii3i',0,-2**31,-1,3,-2**31,2**31-1,-1)

def late(rich: bool) -> bytes:
    out=P('<ii',-2**31,2 if rich else 0)
    if rich:out+=P('<iH',-1,0)*2
    out+=P('<i',-1 if rich else 0)+(surface_ids() if rich else b'')
    for face in [True,False]:
        out+=P('<i',2 if rich else 0)
        if rich:out+=(P('<H',0) if face else P('<i',-1))+text('Ω duplicate')
        if rich:out+=(P('<H',0) if face else P('<i',-1))+text('Ω duplicate')
    return out+P('<i',2**31-1)

def atom_fields(w: WireMap, rich: bool, owner: int) -> bytes:
    return references(rich)+choosers(rich,owner)+maps(w,rich)+late(rich)

def body_suffix(rich: bool, final_word: int = -2**31) -> bytes:
    return bytes(4)+items(rich)+P('<i',final_word)

def direction(w: WireMap, rich: bool) -> bytes:
    out=nullable(rich)+P('<iB',-1,255 if rich else 0)
    if rich:out+=VECTOR+P('<B',254)+VECTOR+P('<B',253)+VECTOR
    return out+P('<i',2**31-1)

def from_spec(version: int, kind: int) -> bytes:
    out=P('<i',kind)
    if kind in [1,2]:return out+bytes(2)
    if kind in [3,4,5]:return out+(P('<d',-0.0) if kind==4 else bytes(2))+P('<i',-1)
    return out

def end_spec(w: WireMap, version: int, rich: bool, first: int=3, second: int=7, point_reference: Callable[[], bytes] | None = None, wrapper_reference: Callable[[], bytes] | None = None, from_reference: Callable[[], bytes] | None = None) -> bytes:
    out=w.new('moDirectionSpec_c',lambda:direction(w,rich)) if rich else bytes(2)
    out+=P('<3iH2i',-2**31,2**31-1,-1,0,first,second)+bytes(4)
    for kind in [first,second]:
        if kind==3:out+=point_reference() if point_reference is not None else (w.new('moPointRef_w',lambda:body(geometric('moPointRef_w',version,False,0))) if rich else bytes(2))
        out+=P('<B',255 if rich else 0)+(items(rich) if rich else b'')
    out+=P('<d6i',-0.0,-2**31,2**31-1,-1,0,7,-7)+bytes(4)+P('<4i',-2**31,2**31-1,-1,0)+items(rich)*2+P('<3i',-1,0,7)
    for kind in [first,second]:
        if kind==7:out+=wrapper_reference() if wrapper_reference is not None else (w.new('moRefWrapper_c',lambda:wrapper(version,False)) if rich else bytes(2))
    out+=from_reference() if from_reference is not None else (w.new('moFromEndSpec_c',lambda:from_spec(version,4 if rich else 0)) if rich else bytes(2))
    return out+P('<4i',-2**31,2**31-1,-1,0)+(P('<i',-7) if version>=0x3b01 else b'')

def revolution_end(w: WireMap, version: int, rich: bool, point_reference: Callable[[], bytes] | None = None, wrapper_reference: Callable[[], bytes] | None = None) -> bytes:
    out=P('<5i',-2**31,2**31-1,-1,0,7)
    if wrapper_reference is not None:out+=wrapper_reference()+wrapper_reference()
    else:
        out+=w.new('moRefWrapper_c',lambda:wrapper(version,False)) if rich else bytes(2)
        out+=P('<H',w.slot) if rich else bytes(2)
    out+=point_reference() if point_reference is not None else (w.new('moPointRef_w',lambda:body(geometric('moPointRef_w',version,False,0))) if rich else bytes(2))
    out+=point_reference() if point_reference is not None else (P('<H',w.slot) if rich else bytes(2))
    return out+P('<2d2i',-0.0,1.25,-2**31,2**31-1)+bytes(8)

def suffix(w: WireMap, version: int, name: str, rich: bool, end_reference: Callable[[], bytes] | None = None) -> bytes:
    if name in CLASSES[:4]:
        return (end_reference() if end_reference is not None else (w.new('moEndSpec_c',lambda:end_spec(w,version,rich)) if rich else bytes(2)))+P('<di',-0.0,-2**31)
    if name=='moRib_c':return P('<iI',-2**31,0xffffffff)+bytes(10)
    out=bytes(2)+P('<BH',255,0)+P('<i',-1)+(w.array('suObList') if rich else bytes(2))
    out+=P('<2iH',-2**31,2**31-1,0)
    out+=end_reference() if end_reference is not None else (w.new('moRevEndSpec_c',lambda:revolution_end(w,version,rich)) if rich else bytes(2))
    return out+(P('<B',254) if version>=0x3f05 else b'')

def tagged(version: int, name: str, rich: bool, final_word: int = -2**31) -> bytes:
    w=WireMap()
    return w.new(name,lambda:header(w,version,rich)+atom_fields(w,rich,2)+body_suffix(rich,final_word)+suffix(w,version,name,rich))
