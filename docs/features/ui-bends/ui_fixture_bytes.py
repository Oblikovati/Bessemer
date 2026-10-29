"""Complete UI-bend graphs with direct line bodies and saved helper selection (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body

CLASSES=('moUiBend_c','moRuledBendLineData_c','moBendSketchLineData_c')
VERSIONS=(0x27cc,0x335a,0x372f,0x42cb,0x42cc,19000,0xffffffff)
COUNTS=(-2147483648,-1,0,1,2,3)


class UiGraph(EdgeGraph):
    """Keep direct ruled lines outside the object map and retain raw signed counts."""
    def __init__(self, version: int, style: int, first_slot: int, extended: bool=False) -> None:
        """Keep count selection local to one graph; e.g. UiGraph(19000,3,0,True)."""
        super().__init__(version,style,first_slot,extended)
        self.declared_lines=0

    def ruled_line(self, edited: bool=False) -> bytes:
        """Retain the double's exact bits before its signed word; e.g. graph.ruled_line(True)."""
        return struct.pack('<Qi',BITS[self.style],12345 if edited else WORDS[self.style])

    def sketch_line(self, edited: bool=False) -> bytes:
        """Cross the strict late-word gate; e.g. graph.sketch_line(False)."""
        raw=struct.pack('<2i',12345 if edited else WORDS[self.style],2147483647)
        return raw+(struct.pack('<i',-2147483648) if self.version>0x42cb else b'')

    def alternate(self, edited: bool) -> bytes:
        """Store signed count and direct lines before independent pointer/array fields; e.g. graph.alternate(False)."""
        raw=struct.pack('<2i',12345 if edited else WORDS[self.style],self.declared_lines)
        raw+=b''.join(self.ruled_line() for _ in range(max(0,self.declared_lines)))
        raw+=self.shared('sketch-line',CLASSES[2],self.sketch_line) if self.style not in (0,4) else bytes(2)
        return raw+self.array()

    def ui(self, key: str, edited: bool) -> bytes:
        """The ordinary path contains only one tagged single-bend pointer; e.g. graph.ui('first',False)."""
        def contents() -> bytes:
            raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
            if self.extended:return raw+self.alternate(edited)
            return raw+(self.shared('single-bend','moOneBend_c',lambda:self.one_bend(edited)) if self.style not in (0,4) else bytes(2))
        return self.shared(key,CLASSES[0],contents)

    def record(self, kind: int, key: str, edited: bool) -> bytes:
        """Build each full native binding before computing its map size; e.g. graph.record(1,'first',False)."""
        if kind==0:return self.ui(key,edited)
        return self.shared(key,CLASSES[kind],lambda:self.ruled_line(edited) if kind==1 else self.sketch_line(edited))


def ui_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep fresh, distinct and repeated normal roots; e.g. ui_graph(0,19000,5)."""
    graph=UiGraph(version,style,first_slot)
    def record(key: str) -> bytes:
        return graph.record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1


def active_ui_graph(case: int, version: int, style: int, edited: bool, first_slot: int, extended: bool) -> tuple[bytes,int]:
    """Cross all six signed-count shapes under the actual saved helper; e.g. active_ui_graph(5,19000,3,False,96,True)."""
    graph=UiGraph(version,case,first_slot,extended)
    graph.declared_lines=COUNTS[case]
    raw=graph.ui('first',edited)
    return raw,graph.mapping.slot+1
