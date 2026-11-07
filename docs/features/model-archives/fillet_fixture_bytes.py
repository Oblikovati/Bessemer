"""Independent inherited fillets, keepers, caches and ordered maps."""
from body_feature_fixture_bytes import WireMap,P,VECTOR,header,atom_fields,body_suffix,items,nullable
from kernel_geometry_fixture import frame
from display_dimension_fixture_bytes import body as tagged_body,shared
from freeze_component_references import wire as component

def body(w:WireMap,version:int,rich:bool,owner:int)->bytes:
    return header(w,version,rich)+atom_fields(w,rich,owner)+body_suffix(rich)

def keeper(w:WireMap,rich:bool,owner:int)->bytes:
    out=P('<i',2 if rich else 0)
    if rich:
        out+=w.new('moEntityIdRep_c',lambda:P('<HH',owner,0))+P('<H',w.slot)
    return out+items(rich)

def references(w:WireMap,rich:bool,owner:int)->bytes:
    out=keeper(w,rich,owner)+keeper(w,rich,owner)
    out+=bytes(10)+nullable(rich)+items(rich)
    return out+bytes(2)

def settings(rich:bool)->bytes:
    out=bytes(4)+P('<BHBiBH',255,0,254,-2**31,253,0)
    out+=P('<2BHiBi2d3Bi',252,251,65535,2**31-1,250,-1,-0.0,1.25,249,248,247,-1 if rich else 0)
    return out+(bytes(2) if rich else b'')

def cache(rich:bool,kernel:int)->bytes:
    if not rich:return bytes(4)
    out=P('<iiHi',-1,-2**31,0,2)
    out+=frame() if kernel else b'\xc3'
    out+=frame(-11) if kernel else b'\xc9'
    out+=P('<i',2)+bytes(8)+items(rich)+P('<i',2)
    out+=(bytes(2)+P('<i3i',3,-2**31,2**31-1,-1))*2
    return out+P('<4i',-2**31,2**31-1,-1,0)

def tail(version:int,rich:bool,word:int=-2**31)->bytes:
    out=P('<ii',word,2 if rich else 0)
    if rich:out+=P('<2idH',-2**31,2**31-1,-0.0,0)*2
    out+=P('<HI3I',65535,3,0xffffffff,0,0x89abcdef) if rich else bytes(2)
    out+=bytes(4)+P('<B2i',255,-2**31,2**31-1)
    return out+(P('<BH',254,0) if version>0x2b8e else b'')

def fillet(version:int,rich:bool,kernel:int=0,word:int=-2**31)->bytes:
    w=WireMap()
    return w.new('Fillet_c',lambda:body(w,version,rich,2)+references(w,rich,2)+settings(rich)+cache(rich,kernel)+tail(version,rich,word))

def variable(version:int,rich:bool)->bytes:
    w=WireMap()
    def contents():
        out=body(w,version,rich,2)+keeper(w,rich,2)+bytes(4)+P('<2Bi',255,254,-2**31)+bytes(4)+P('<i',2**31-1)
        out+=P('<HI3I',65535,3,0xffffffff,0,0x89abcdef) if rich else bytes(2)
        return out+P('<BiHB',253,-1,0,252)+(P('<B',251) if version>17000 else b'')
    return w.new('VarFillet_c',contents)

def support(version:int,name:str,rich:bool)->bytes:
    w=WireMap()
    if name in ['moFilletSIdKeeper_c','moFaceFilletSIdKeeper_c']:
        return w.new(name,lambda:keeper(w,rich,0))
    def pointer(cls):
        if not rich:return bytes(2)
        if cls=='moDisplayDim_c':return w.new(cls,lambda:shared(w,version,cls,False,0))
        return w.new(cls,lambda:tagged_body(component(cls,version,False,0)))
    def contents():
        if name=='moEdgeDim_c':return pointer('moCompEdge_c')+P('<2i',-2**31,2**31-1)
        if name=='moVertDim_c':return pointer('moDisplayDim_c')+pointer('moCompVertex_c')+P('<B',255)+bytes(4)
        out=pointer('moDisplayDim_c')
        if name=='edgeRadiusObject_c':return out+pointer('moCompEdge_c')+P('<i',-2**31)
        if name=='faceRadiusObject_c':return out+pointer('moCompFace_c')
        return out+pointer('moCompLoop_c')+pointer('moCompEdge_c')
    return w.new(name,contents)
