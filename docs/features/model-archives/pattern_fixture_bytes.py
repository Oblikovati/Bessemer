"""Independent complete linear patterns, direct variation records and shape branches."""
from fillet_fixture_bytes import WireMap, P, VECTOR, body
from body_feature_fixture_bytes import items
from owned_name_edit import short_wide_name as text
from display_dimension_fixture_bytes import shared
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as tagged_body
from sketch_reference_fixture_bytes import temporary_axis


def matrix(rich: bool) -> bytes:
    return items(rich)*2+P('<i',-2**31)


def header(w: WireMap, version: int, rich: bool) -> bytes:
    out=bytes(4)+P('<iHBiH',-2**31,0,255,2**31-1,0)
    out+=P('<Hi',0,-1) if version>=0x42cf else b'\xfe'
    return out+(w.new('moMatrixIndexHelper_c',lambda:matrix(rich)) if rich else bytes(2))


def collections(rich: bool) -> bytes:
    out=P('<HI',65535,2)+P('<Hi',0,-2**31)*2 if rich else bytes(2)
    out+=P('<HI',65535,2)+bytes(8) if rich else bytes(2)
    return out+b'\xff'+items(rich)


def shape(kind: int, point: bool) -> bytes:
    out=P('<i',kind)
    if kind==0:return out
    if kind==4:out+=bytes(2)
    if kind in [1,4]:out+=bytes(2)
    if kind==3:out+=bytes(2)
    if kind in [2,3]:out+=bytes(2)
    out+=P('<i',-1 if point else 0)+(VECTOR if point else bytes(2))+VECTOR
    if kind!=1:
        if kind not in [2,3]:out+=P('<i',-2**31)
        out+=bytes(2)
    return out+P('<i',2**31-1)


def increment(rich: bool) -> bytes:
    out=P('<BHB',255,0,255 if rich else 0)
    if rich:out+=text('Ω increment')
    return out+P('<3iBHBdiBd',-2**31,2**31-1,-1,254,0,253,-0.0,-2**31,252,-1.25)


def original(rich: bool) -> bytes:
    out=P('<BHB',255,0,255 if rich else 0)
    if rich:out+=text('Ω original')
    out+=P('<BdB2di8iB',254,-0.0,253,1.25,-2.5,-2**31,*range(-8,0),255 if rich else 0)
    if rich:out+=increment(True)
    out+=P('<i',2 if rich else 0)+(bytes(4) if rich else b'')
    return out+P('<i',-2**31)+items(rich)+P('<i',2**31-1)


def variation(rich: bool) -> bytes:
    out=P('<i',2 if rich else 0)+(increment(True)*2 if rich else b'')+b'\xff\xfe'
    for populated in [rich,False]:
        out+=P('<i',1 if populated else 0)+(original(True) if populated else b'')
    return out+original(rich)*2


def tail(version: int, rich: bool, word: int) -> bytes:
    out=P('<Hi',0,word)
    if version>0x2f86:out+=items(rich)
    if version>0x2f93:
        out+=P('<I',3 if rich else 0)+(P('<3i',-2**31,2**31-1,-1) if rich else b'')
    if version>0x3745:out+=b'\xff'
    if version>=0x42dc:out+=b'\xfe'
    return out


def suffix(w: WireMap, version: int, rich: bool, temporary: bool=False) -> bytes:
    dimension=w.new('moDisplayDim_c',lambda:shared(w,version,'moDisplayDim_c',False,0)) if rich else bytes(2)
    out=dimension+bytes(6)+b'\xff\xfe'+bytes(4)
    if temporary:
        out+=w.new('moTempAxisRef_w',lambda:tagged_body(temporary_axis(version,False)))
    else:
        out+=w.new('moLineRef_w',lambda:tagged_body(geometric('moLineRef_w',version,False,0))) if rich else bytes(2)
    out+=bytes(2)+b'\xfd'
    out+=(P('<2i',-2**31,2**31-1)+bytes(4))*2
    out+=P('<2i',-1,-2**31)+bytes(4)+P('<2i',2**31-1,-1)
    return out+(b'\xfc' if version>=0x3f00 else b'')


def linear(version: int, rich: bool, kind: int, point: bool, word: int=-2**31, temporary: bool=False) -> bytes:
    w=WireMap()
    def contents() -> bytes:
        out=body(w,version,rich,2)+header(w,version,rich)+collections(rich)
        out+=shape(kind,point)+b'\xff\xfe'+variation(rich)+tail(version,rich,word)
        return out+suffix(w,version,rich,temporary)
    return w.new('moLPattern_c',contents)
