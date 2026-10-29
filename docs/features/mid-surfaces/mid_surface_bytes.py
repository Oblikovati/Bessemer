"""Complete mid-surface graphs with direct keeper ownership and backend-selected sheets (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body,keeper
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire
from freeze_geometric_references import wire as geometric
from freeze_component_references import wire as component
from display_dimension_fixture_bytes import body as strip_tag
from kernel_geometry_fixture import frame

CLASSES=('moMidSurface_c','moRefSurface_c','moMidRefSurface_c','moMidSurfData_c','moFacePair_c','moFaceToHeuristicEdges_c','moEdgePair_c','moBendModifiedSurfIdRep_c')
VERSIONS=(0x27cc,0x2bcc,0x2bcd,0x2f85,0x2f86,0x332c,0x332d,19000,0xffffffff)
PRESENCES=(0,1,2,-1,-2147483648,1)
COUNTS=(0,-1,2,1,-2147483648,3)


class MidSurfaceGraph(EdgeGraph):
    """Own actual reference targets and direct arrays in a single MFC map."""
    def single_face(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return self.shared('single-face','moSingleFaceRef_w',lambda:strip_tag(geometric('moSingleFaceRef_w',self.version,False,0)))

    def component(self, edge: bool) -> bytes:
        if self.style in (0,4):return bytes(2)
        name=('moCompSilhEdge_c' if self.style==5 else 'moCompEdge_c') if edge else 'moCompFace_c'
        return self.shared(name,name,lambda:strip_tag(component(name,self.version,False,0)))

    def face_array(self) -> bytes:
        count=COUNTS[self.style]
        return struct.pack('<i',count)+b''.join(self.single_face() for _ in range(max(count,0)))

    def data(self, edited: bool) -> bytes:
        presence=PRESENCES[self.style]
        return struct.pack('<i',presence)+(self.array() if presence else b'')+struct.pack('<i',12345 if edited else WORDS[self.style])

    def direct_keeper(self) -> bytes:
        return keeper(self.mapping,self.style in (1,3,5),self.ids['root'])+self.array()+self.array()

    def mid_payload(self, edited: bool) -> bytes:
        present=PRESENCES[self.style];raw=struct.pack('<i',present)
        if present==1:raw+=frame() if self.style==5 else b'\x01'
        raw+=self.shared('data',CLASSES[3],lambda:self.data(False)) if self.style not in (0,4) else bytes(2)
        return raw+self.direct_keeper()+struct.pack('<i',12345 if edited else WORDS[self.style])+self.face_array()+self.face_array()

    def external_surface(self) -> bytes:
        if self.style in (0,3):
            inline=surface_wire(self.mapping,self.ids,self.version,self.style)
            return bytes(4)+inline[4:]
        count=(0,3,2,0,2147483647,1)[self.style]
        raw=struct.pack('<3i',PRESENCES[self.style],count,1 if self.style>=2 else 0)
        if self.style>=2:raw+=(frame()+self.array()) if self.style in (4,5) else b'\x01'
        if self.version>=0x2bcd:raw+=struct.pack('<i3i',3,-2147483648,0,2147483647)
        return raw

    def surface(self, base: bool, edited: bool) -> bytes:
        empty=lambda:self.shared('base-keeper','moExtendRefSurfIdKeeper_c',lambda:b'')
        raw=geometry_header(self.mapping,self.version,self.style in (3,5),keeper=empty if self.style in (3,5) else None)
        raw+=self.external_surface() if base else surface_wire(self.mapping,self.ids,self.version,4 if self.style==5 else self.style)
        return raw if base else raw+self.mid_payload(edited)

    def contents(self, kind: int, edited: bool) -> bytes:
        if kind==0:return body(self.mapping,self.version,self.style in (3,5),self.ids['root'])+struct.pack('<QB',BITS[self.style],(0,1,255,128,127,254)[self.style])+self.listing()+self.listing()
        if kind in (1,2):return self.surface(kind==1,edited)
        if kind==3:return self.data(edited)
        if kind==4:return self.single_face()+self.single_face()+self.display()
        if kind==5:return self.array()+self.component(False)
        if kind==6:return self.component(True)+self.component(True)
        identity=self.ids['root'] if self.style not in (0,4) else 0
        return struct.pack('<3Hi',identity,0,identity,12345 if edited else WORDS[self.style])


def mid_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose all eight tagged classes and the mid-reference's directly owned keeper."""
    graph=MidSurfaceGraph(version,style,first_slot)
    raw=graph.shared('root',CLASSES[kind],lambda:graph.contents(kind,edited))
    return raw,graph.mapping.slot+1
