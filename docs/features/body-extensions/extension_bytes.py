"""Complete cavity, sketch-hole and transformed-stock wire compositions (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from body_feature_fixture_bytes import suffix
from freeze_boundary_references import wire as boundary
from display_dimension_fixture_bytes import body as strip_tag
from freeze_model_references import wrapper
from freeze_views import transform
from stock_fixture_bytes import StockGraphBuilder

CLASSES=('moCavity_c','moSketchHole','moXformStock_c')
VERSIONS=(0x27cc,0x2b68,0x2b69,0x2b72,0x2b73,0x2faf,0x2fb0,0x3335,0x3336,0x373b,0x373c,0x3b01,0x3b02,19000,0xffffffff)


class ExtensionGraph(EdgeGraph):
    """Allocate real inherited bodies and nullable typed reference targets."""
    def cavity_payload(self, edited: bool) -> bytes:
        raw=struct.pack('<i3Q2i',12345 if edited else WORDS[self.style],*(BITS[(self.style+i)%6] for i in range(3)),-2147483648,2147483647)
        raw+=self.shared('face','moFaceRef_c',lambda:strip_tag(boundary('moFaceRef_c',self.version,False))) if self.style not in (0,4) else bytes(2)
        raw+=self.shared('coordinate','moCoordSysRef_w',lambda:wrapper(self.version,False)+transform(False,7)) if self.style not in (0,4) else bytes(2)
        return raw

    def feature(self, kind: int, key: str, edited: bool) -> bytes:
        def contents() -> bytes:
            raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
            if kind==0:return raw+self.cavity_payload(edited)
            inherited=suffix(self.mapping,self.version,'moExtrusion_c',self.style in (3,5))
            return raw+inherited[:-4]+struct.pack('<i',12345 if edited else -2147483648)
        return self.shared(key,CLASSES[kind],contents)


def extension_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep inherited serializer bytes complete and reserve aliases before children."""
    if kind==2:
        stock=StockGraphBuilder(0,version,style,edited,first_slot)
        raw=stock.obj('root',CLASSES[kind],stock.stock_body)
        return raw,stock.mapping.slot+1
    graph=ExtensionGraph(version,style,first_slot)
    def roots() -> bytes:
        return struct.pack('<H',3)+graph.feature(kind,'root',edited)+graph.feature(kind,'second',False)+graph.feature(kind,'root',False)
    raw=graph.shared('array','suObArray',roots) if style in (2,5) else graph.feature(kind,'root',edited)
    return raw,graph.mapping.slot+1
