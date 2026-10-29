"""Complete sketch bends composed from the independently frozen One Bend grammar (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS
from one_bend_bytes import direct_handle
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag

VERSIONS=(0x27cc,0x334b,0x334c,0x334e,0x334f,0x335a,0x335b,0x3734,0x3735,19000,0xffffffff)


class SketchBendGraph(EdgeGraph):
    """Keep tagged handle temporaries distinct from inherited direct handles."""
    curve_class: str
    curve_mode: int

    def marker(self) -> bytes:
        if self.curve_mode==2:return self.curve()
        return super().marker()

    def tagged_handle(self, point: bool) -> bytes:
        if self.style in (0,4):return bytes(2)
        name='sgPointHandle' if point else 'sgLineHandle'
        return self.shared(name,name,lambda:direct_handle(self.style))

    def curve(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return self.shared('curve',self.curve_class,lambda:strip_tag(geometric(self.curve_class,self.version,False,0)))

    def sketch_bend(self, edited: bool) -> bytes:
        raw=self.one_bend(False)+self.marker()
        raw+=self.tagged_handle(True)+self.tagged_handle(False)+self.tagged_handle(False)
        return raw+self.curve()+struct.pack('<i',12345 if edited else WORDS[self.style])


def sketch_bend_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build the complete inherited body and suffix; e.g. sketch_bend_object(0,19000,3)."""
    if kind not in (0,1,2):raise ValueError(f'sketch bend fixture kind {kind}; expected 0, 1 or 2')
    graph=SketchBendGraph(version,style,first_slot)
    graph.curve_class='moGeneralCurveRef_w' if kind==0 else 'moLineRef_w'
    graph.curve_mode=kind
    raw=graph.shared('root','moSketchBend_c',lambda:graph.sketch_bend(edited))
    return raw,graph.mapping.slot+1
