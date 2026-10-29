"""Complete deform features with independent versions, two-pass matches and direct keepers (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS,BITS
from fillet_fixture_bytes import body
from draft_reference_bytes import DraftOptions,keeper,draft_direction
from sweep_fixture_bytes import endpoint_body
from deform_geometry_bytes import geometry_bytes
from weld_corner_bytes import counted

FEATURE_VERSIONS=(-2147483648,-1,0,1,2,3,4,5,6,7,8,9,10,11,2147483647)
VERSIONS=(0x27cc,0x3746,0x3747,19000)
GEOMETRY_STYLES=(0,7,3,2,4,7)
MATCH_COUNTS=(0,1,1,2,-2147483648,2)
SUBCOUNTS=(0,-1,1,2,0,2)


class DeformGraph(EdgeGraph):
    """Keep operation-local identities shared across both matching passes."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        super().__init__(version,style,first_slot)
        self.kind=kind;self.feature_version=FEATURE_VERSIONS[kind] if kind<15 else 11;self.edited=edited

    def curve_point(self, endpoint: bool=False) -> bytes:
        """Use full endpoint bodies for both proven stored class bindings."""
        name='moEndPointRef_w' if endpoint else 'moCurvePointRef_w'
        return self.shared(name,name,lambda:endpoint_body(self.mapping,self.version,self.ids,self.style in (3,5)))

    def array(self) -> bytes:
        """Distinguish absent, present-empty and noncanonical signed presence words."""
        presence=(0,-1,-2147483648,1,2147483647,255)[self.style]
        if not presence:return bytes(4)
        values=[] if self.style in (1,4) else [self.curve_point(self.style==3),bytes(2),self.curve_point(self.style==3)]
        return struct.pack('<i',presence)+counted(values,self.style in (4,5))

    def subrecord(self) -> bytes:
        """Retain both reference sides and every raw point/vector value."""
        raw=b''
        for side in range(2):
            target=self.curve_point(self.style==3 or self.style==5 and side==1)
            raw+=struct.pack('<i',WORDS[self.style])+target+self.marker()+struct.pack('<6Q',*([BITS[self.style]]*6))
        return raw

    def first_match(self) -> bytes:
        """Emit first-pass sides and signed subcounts without direct-record tags."""
        raw=b''.join(self.array()+struct.pack('<2i',-1,2147483647) for _ in range(2))
        count=SUBCOUNTS[self.style]
        return raw+struct.pack('<i',count)+b''.join(self.subrecord() for _ in range(max(0,count)))

    def second_match(self) -> bytes:
        """Use the saved feature version and original subcount for the later pass."""
        if self.feature_version<9:return b''
        raw=b''.join(self.array()+struct.pack('<i',-1) for _ in range(2))+struct.pack('<i',2147483647)
        if self.feature_version>=10:raw+=struct.pack('<6Q',*([BITS[self.style]]*6))*max(0,SUBCOUNTS[self.style])
        return raw

    def extension_bytes(self) -> bytes:
        """Preserve all recognized payload types and exact nonzero terminators."""
        if self.feature_version<7:return b''
        if self.style==0:return bytes(4)
        if self.style==1:return struct.pack('<3i',0x3d2,-2147483648,-1)
        if self.style==4:return struct.pack('<iQi',0x13e,BITS[self.style],-2147483648)
        raw=struct.pack('<2iiQ',0x3d2,-1,0x13e,BITS[self.style])
        raw+=struct.pack('<i',0x1466)+self.curve_point()+struct.pack('<2i',0x3d2,2147483647)
        return raw+struct.pack('<i',0x12345678 if self.style==2 else -8)

    def versioned_fields(self) -> bytes:
        """Keep the feature's own signed version separate from geometry layout."""
        v=self.feature_version;raw=b''
        if v>=1:raw+=draft_direction(self.mapping,self.ids,DraftOptions(self.style))
        if v>=2:raw+=self.array()
        if v>=3:raw+=self.array()
        if v>=4:raw+=struct.pack('<i',-1)
        if v>=6:raw+=struct.pack('<i',2147483647)
        return raw

    def payload(self) -> bytes:
        """Retain every temporary array, geometry record and second-pass byte."""
        s=self.style;raw=self.display()+self.display(True)+struct.pack('<i',12345 if self.edited else WORDS[s])
        raw+=draft_direction(self.mapping,self.ids,DraftOptions(s))+b''.join(self.array() for _ in range(6))+struct.pack('<i',-1)+b''.join(self.array() for _ in range(5))
        raw+=struct.pack('<3Q',*([BITS[s]]*3));count=-1 if s==0 and self.feature_version<0 else MATCH_COUNTS[s]
        raw+=struct.pack('<i',count)+b''.join(self.first_match() for _ in range(max(0,count)))+struct.pack('<i',self.feature_version)
        raw+=self.versioned_fields()+struct.pack('<3Qi3Q',*([BITS[s]]*3),-1,*([BITS[s]]*3))+geometry_bytes(GEOMETRY_STYLES[s],s)
        if self.feature_version>=8:raw+=self.array()
        raw+=b''.join(self.second_match() for _ in range(max(0,count)))+self.extension_bytes()
        return raw+keeper(self.mapping,self.version,self.ids,DraftOptions(s))

    def contents(self) -> bytes:
        """Use the real full body-feature prefix or the complete curve-point alias."""
        if self.kind==15:
            raw=endpoint_body(self.mapping,self.version,self.ids,self.style in (3,5))
            return raw[:-4]+struct.pack('<i',12345 if self.edited else -2147483648)
        inherited=body(self.mapping,self.version,self.style in (3,5),self.ids['root'])
        return inherited+self.payload()


def deform_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve real root identities before both matching passes; e.g. deform_object(13,19000,2)."""
    graph=DeformGraph(kind,version,style,edited,first_slot)
    name='moDeform_c' if kind<15 else 'moCurvePointRef_w'
    raw=graph.shared('root',name,graph.contents)
    return raw,graph.mapping.slot+1
