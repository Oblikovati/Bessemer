"""Independent complete profiles, shared contours and conditional framed geometry."""
from sketch_tail_fixture_bytes import WireMap,P,ordinary
from body_feature_fixture_bytes import items
from freeze_environment import feature
from freeze_views import transform
from owned_name_edit import short_wide_name as text
from kernel_geometry_fixture import frame

def model_feature(w: WireMap,version: int) -> bytes:
    w.slot+=2
    return feature(version,False)+P('<i',-2**31)

def reference_array(size: int, count: int) -> bytes:
    if size==0:return bytes(4)
    return P('<4I',size,0,0xffffffff,count)+b''.join(P('<I',((index*0x12345678)^0xffffffff)&0xffffffff) for index in range(count*(size//4)))

def geometry(rich: bool,kernel: int,size: int=8) -> bytes:
    out=P('<i',-2**31)+(b'\xc3' if kernel==0 else frame())+P('<i',2 if rich else 0)
    if rich:out+=P('<4I',0xffffffff,0x89abcdef,0xffffffff,0x89abcdef)
    out+=items(rich)+reference_array(size if rich else 0,2 if rich else 0)
    if kernel:out+=P('<I',2 if kernel==2 else 0)
    if kernel==2:out+=P('<iB',-2**31,201)+P('<i',2**31-1)+frame(-11)
    return out

def chooser(w: WireMap,rich: bool,owner: int,mask: int=3) -> bytes:
    out=items(rich)+P('<i',-1 if rich else 0)+(P('<H',owner) if rich else b'')
    out+=P('<2iH',-2**31,2**31-1,mask)
    for bit in [1,2]:
        if mask&bit:out+=w.array() if rich else bytes(2)
    return out+P('<i',-1)

def contour(w: WireMap,rich: bool,owner: int) -> bytes:
    return items(rich)*2+P('<i',-1 if rich else 0)+(P('<H',owner) if rich else b'')+P('<5i',-2**31,2**31-1,-1,0,7)

def contours(w: WireMap,rich: bool,owner: int) -> bytes:
    out=P('<I',0xffffffff)+P('<H',2 if rich else 0)
    if not rich:return out
    out+=w.new('moOneContour_c',lambda:contour(w,True,owner))+P('<H3I',3,0xffffffff,0,0x89abcdef)
    return out+P('<HHI',w.slot,1,0xffffffff)

def chain(w: WireMap,rich: bool,mask: int=3) -> bytes:
    out=(P('<HI3I',65535,3,0xffffffff,0,0x89abcdef) if rich else bytes(2))*2
    out+=P('<I3i2I',0xffffffff,-2**31,2**31-1,-1,0x89abcdef,0xffffffff)+text('Ω chain')+P('<H',mask)
    if mask&1:out+=w.handle('sg3DPlaneHandle') if rich else bytes(2)
    if mask&2:
        out+=w.handle() if rich else bytes(2)
        out+=P('<H',w.slot) if rich else bytes(2)
    out+=P('<II',0xffffffff,2 if rich else 0)
    if rich:out+=P('<4I',0xffffffff,7,0xffffffff,7)
    return out+bytes(4)+P('<3i',-2**31,2**31-1,-1)

def chains(w: WireMap,rich: bool) -> bytes:
    out=P('<Ii',0xffffffff,2 if rich else 0)
    if rich:
        identity=w.slot+2
        out+=w.new('moSketchChain_c',lambda:chain(w,True))+P('<H',identity)
    return out

def placement(w: WireMap,rich: bool) -> bytes:
    out=w.handle('sgLineHandle') if rich else bytes(2)
    out+=bytes(2)+transform(rich,0x2a)+P('<H',65535 if rich else 0)
    if rich:out+=transform(True,0x4a)
    return out+P('<HH',65534,0)

def suffix(w: WireMap,version: int,rich: bool,owner: int,final_word: int=-1) -> bytes:
    out=P('<2iIH',-2**31,2**31-1,0xffffffff,65535)
    out+=w.new('moContourChooser_c',lambda:chooser(w,rich,owner)) if rich else bytes(2)
    out+=items(rich)*2
    return out+(P('<i',final_word) if version>=0x2b92 else b'')

def contents(w: WireMap,version: int,rich: bool,kernel: int,reused: bool,owner: int,final_word: int=-1) -> bytes:
    out=model_feature(w,version)+P('<iHi',-2**31,0,-1 if reused else 0)
    if reused:
        out+=P('<H',owner)+bytes(2)+P('<HH',65535,0)
        return out+contours(w,rich,owner)
    out+=w.new('sgSketch',lambda:ordinary(w,version,False)) if rich else bytes(2)
    return out+chains(w,rich)+geometry(rich,kernel)+contours(w,rich,owner)+placement(w,rich)+suffix(w,version,rich,owner,final_word)

def tagged(version: int,name: str,rich: bool,kernel: int=0,reused: bool=False,final_word: int=-1) -> bytes:
    w=WireMap()
    return w.new(name,lambda:contents(w,version,rich,kernel,reused,2,final_word))
