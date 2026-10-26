"""Independent cosmetic-thread records and explicitly loaded instance/cache references."""
from fillet_fixture_bytes import WireMap,P,VECTOR
from profile_fixture_bytes import model_feature
from freeze_views import transform
from freeze_geometric_references import wire as geometric,surface
from display_dimension_fixture_bytes import body
from owned_name_edit import short_wide_name as text

LOADED=['moCThreadInstanceId_c','moCThreadCacheData_c']

def data(version:int,count:int,point:int,active:bool,loaded:bool)->bytes:
    out=P('<3iB3did',-2**31,2**31-1,-1,255,-0.0,1.25,-2.5,-1,3.75)+VECTOR*3
    out+=P('<8d2i4dii',-0.0,1,2,3,4,5,6,7,-2**31,2**31-1,-0.0,1.25,2.5,-3.75,-1,count)
    out+=b''.join(transform(bool(i%2),0x2a) for i in range(count))
    out+=P('<'+'H'*count,*([1 if loaded else 0]*count))+P('<'+'i'*count,*[-i-1 for i in range(count)])
    if count:out+=VECTOR
    out+=P('<i',2 if loaded else 0)+(P('<2H',2,2) if loaded else b'')
    out+=P('<id',-1 if active else 0,-0.0)+(P('<2d',1.25,-2.5) if active else b'')+P('<B',point)
    if point==1:out+=VECTOR
    out+=P('<d',-0.0)
    for label in ['Ω 190','Ω 1a8','Ω 1b8']:out+=text(label)+P('<i',-1)
    if version>0x2b6a:out+=text('Ω 198')
    if version>0x3366:out+=P('<d',1.25)
    if version>0x3379:out+=VECTOR
    if version>=0x3749:out+=P('<i',-2**31 if active else 0)+(P('<2d',2.5,-3.75) if active else b'')
    return out

def feature(version:int,rich:bool)->bytes:
    w=WireMap();w.slot=2 if rich else 0
    def suffix():
        out=w.new('moCylinderRef_w',lambda:body(geometric('moCylinderRef_w',version,False,0))) if rich else bytes(2)
        out+=bytes(2)+w.new('moCThreadData_c',lambda:data(version,2 if rich else 0,1 if rich else 255,rich,rich))
        out+=text('Ω cosmetic')+bytes(4)+P('<i',-2**31)
        out+=w.new('moGeneralSurfRef_w',lambda:surface(version,False)) if rich else bytes(2)
        out+=P('<3ii',-2**31,2**31-1,-1,3 if rich else 0)
        if rich:out+=P('<3i',-2**31,2**31-1,-1)
        out+=bytes(2)+P('<dH',-0.0,0)
        if version>0x27cf:out+=bytes(4)
        if version>0x27de:out+=P('<i',-1)
        if version>0x2f6a:out+=P('<i',-2**31)
        if version>0x39d0:out+=P('<2i',-2**31,2**31-1)
        return out
    return w.new('moCosmeticThread_c',lambda:model_feature(w,version)+suffix())

def tagged_data(version:int,count:int,point:int,active:bool,loaded:bool)->bytes:
    w=WireMap();w.slot=2 if loaded else 0
    return w.new('moCThreadData_c',lambda:data(version,count,point,active,loaded))
