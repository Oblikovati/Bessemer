"""Independent direct reference-plane data, reference frames and complete plane roots."""
from body_feature_fixture_bytes import header, atom_fields, WireMap, P, VECTOR, body
from freeze_views import transform
from freeze_geometric_references import surface, wire as geometric
from freeze_model_references import wrapper

CLASSES=['moRefPlnData_c','moFixedRefPlnData_c','moDefaultRefPlnData_c','moFaceRefPlnData_c',
         'moFacePtRefPlnData_c','moLinePtRefPlnData_c','moPerLinePtRefPlnData_c','moRefAtAngleData_c','moRefThreePtsData_c']

def base(version: int, default: bool) -> bytes:
    out=VECTOR*2+transform(True,0x2a)+P('<4d',-0.0,1.25,-2.5,3.75)+bytes(2)+P('<3iB',-2**31,2**31-1,-1,255)
    if default:out+=VECTOR
    out+=P('<2iH',-1,-2**31,65535)
    if version>0x2b83:out+=bytes(2)
    if version>0x2ba5:out+=bytes(4)
    return out

def plane_reference(version: int) -> bytes:
    return surface(version,False)+VECTOR*2+transform(True,0x1a)+VECTOR

def plane_data_reference(version: int) -> bytes:
    return wrapper(version,False)+transform(False,7)+transform(True,0x4a)+VECTOR*3

def data(w: WireMap, version: int, name: str, rich: bool) -> bytes:
    out=base(version,name=='moDefaultRefPlnData_c')
    if name=='moFaceRefPlnData_c':
        out+=plane_reference(version)+bytes(2)+P('<3i',-2**31,2**31-1,-1 if rich else 0)
        return out+(plane_data_reference(version) if rich else b'')+P('<i',-1)
    if name=='moFacePtRefPlnData_c':return out+bytes(2)+plane_reference(version)+P('<2i',-2**31,2**31-1)
    if name in ['moLinePtRefPlnData_c','moPerLinePtRefPlnData_c']:
        out+=bytes(2)+w.new('moLineRef_w',lambda:body(geometric('moLineRef_w',version,False,0)))+P('<2i',-2**31,2**31-1)
        return out+(P('<Hi',65535,-1) if name=='moPerLinePtRefPlnData_c' else b'')
    if name=='moRefAtAngleData_c':return out+plane_reference(version)+bytes(4)+P('<4i',-2**31,2**31-1,-1,0)
    if name=='moRefThreePtsData_c':return out+bytes(6)
    return out

def tagged(version: int, name: str, rich: bool) -> bytes:
    w=WireMap()
    return w.new(name,lambda:data(w,version,name,rich))

def plane(version: int, rich: bool) -> bytes:
    w=WireMap()
    def contents():
        out=header(w,version,rich)+atom_fields(w,rich,2)+VECTOR+P('<IiH',0x89abcdef,-2**31,0)
        if version>0x2f85:out+=P('<i',2**31-1)
        name='moDefaultRefPlnData_c' if rich else 'moRefPlnData_c'
        return out+w.new(name,lambda:data(w,version,name,rich))
    return w.new('moRefPlane_c',contents)
