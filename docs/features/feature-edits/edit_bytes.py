"""Complete move-face and bending bodies with independently assembled wire order (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from body_feature_fixture_bytes import direction
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from freeze_views import transform
from mirror_solid_bytes import matrix_body
from stock_fixture_bytes import StockGraphBuilder,VERSIONS

CLASSES=('moMoveFace_c','moBending_c','moBendingData_c')


class FeatureEditGraph(EdgeGraph):
    """Allocate complete targets before their shared and cyclic occurrences."""
    def vector(self) -> bytes:
        return struct.pack('<3Q',*(BITS[(self.style+i)%6] for i in range(3)))

    def raw_doubles(self, count: int) -> bytes:
        return struct.pack('<'+'Q'*count,*(BITS[(self.style+i)%6] for i in range(count)))

    def direction(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return self.shared('direction','moDirectionSpec_c',lambda:direction(self.mapping,self.style in (3,5)))

    def line(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return self.shared('line','moLineRef_w',lambda:strip_tag(geometric('moLineRef_w',self.version,False,0)))

    def nullable_array(self) -> bytes:
        presence=(0,1,-1,-2147483648,0,2147483647)[self.style]
        return struct.pack('<i',presence)+(self.array() if presence else b'')

    def move_payload(self, edited: bool) -> bytes:
        raw=self.raw_doubles(2)+struct.pack('<i',12345 if edited else WORDS[self.style])+self.direction()
        raw+=struct.pack('<i',-1)+self.nullable_array()+self.line()+self.listing()+self.display()+self.display()
        raw+=struct.pack('<2i',-2147483648,2147483647)+self.vector()*4+self.reference()+self.reference()+self.vector()*2
        raw+=transform(self.style in (2,3,5),(0,7,0x1a,0x2b,0x4c,0x8d)[self.style])+self.vector()+struct.pack('<3i',-1,0,2147483647)
        raw+=self.array()+struct.pack('<i',-32769)+self.raw_doubles(1)+self.reference()+self.point()+self.reference()
        return raw+struct.pack('<i',-1)+self.reference()+self.direction()+struct.pack('<i',2147483647)

    def stock_body(self) -> bytes:
        stock=StockGraphBuilder(1 if self.style==3 else 0,self.version,self.style,False,0)
        stock.mapping=self.mapping;stock.ids['root']=self.ids['stock']
        return stock.stock_body()

    def stock(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        name=('moStock_c','moStock_c','moXformStock_c','moMirrorStock_c','moStock_c','moStock_c')[self.style]
        return self.shared('stock',name,self.stock_body)

    def bending_payload(self, edited: bool) -> bytes:
        raw=self.array()+self.stock()+self.vector()*4+self.raw_doubles(2)
        raw+=struct.pack('<i',12345 if edited else WORDS[self.style])+self.raw_doubles(2)+struct.pack('<i',-2147483648)
        raw+=self.reference()+self.reference()+self.reference()
        for word in (-1,2147483647,-2147483648):raw+=self.display()+struct.pack('<i',word)
        return raw+self.raw_doubles(2)+struct.pack('<i',-32769)

    def payload(self, edited: bool) -> bytes:
        return self.shared('payload',CLASSES[2],lambda:self.bending_payload(edited))

    def feature(self, kind: int, key: str, edited: bool) -> bytes:
        def contents() -> bytes:
            raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
            if kind==0:return raw+self.move_payload(edited)
            raw+=self.payload(edited)
            return raw+(self.shared('matrix','moMatrixIndexHelper_c',lambda:matrix_body(self.mapping,self.ids,self.style)) if self.style not in (0,4) else bytes(2))
        return self.shared(key,CLASSES[kind],contents)


def edit_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Cross complete parent serializers, directly owned arrays and nested stock."""
    graph=FeatureEditGraph(version,style,first_slot)
    def record(key: str, changed: bool) -> bytes:
        return graph.payload(changed) if kind==2 else graph.feature(kind,key,changed)
    def roots() -> bytes:
        first=record('root',edited)
        return struct.pack('<H',3)+first+record('second',False)+record('root',False)
    raw=graph.shared('outer','suObArray',roots) if style in (2,5) else record('root',edited)
    return raw,graph.mapping.slot+1
