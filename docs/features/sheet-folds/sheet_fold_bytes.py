"""Complete 3D bends, folds and unfolds with native collection framing (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph, WORDS, BITS
from fillet_fixture_bytes import body
from flange_fixture_bytes import FlangeFixtureOptions,group_wire,flange_profile
from sheet_fixture_bytes import component_reference
from fold_fixture_bytes import FoldFixtureOptions,feature_wire
from kernel_geometry_fixture import frame
from weld_corner_bytes import counted

CLASSES=('moSM3dBend_c','moFold_c','moUnFold_c','su_CMapObToInt')
VERSIONS=(0x27cc,0x2f5f,0x335a,0x372f,0x42cb,19000,0xffffffff)


class FoldGraph(EdgeGraph):
    """Share established native dependencies while allocating all tags in wire order."""
    def boundary(self) -> bytes:
        """Use a real complete component-face subtype; e.g. graph.boundary()."""
        return component_reference(self.mapping,self.version,self.ids,'moCompFace_c') if self.style in (1,3,5) else bytes(2)

    def object_map(self, edited: bool) -> bytes:
        """Retain duplicate raw keys and signed values; e.g. graph.object_map(False)."""
        if self.style in (0,4):return counted([],self.style==4)
        values=(12345 if edited else -2147483648,2147483647,-1)
        entries=[self.marker()+struct.pack('<i',values[0]),bytes(2)+struct.pack('<i',values[1]),self.marker()+struct.pack('<i',values[2])]
        return counted(entries,self.style==5)

    def common_fold(self, edited: bool) -> bytes:
        """The direct array follows the complete body base; e.g. graph.common_fold(False)."""
        raw=self.array()+self.boundary()+struct.pack('<i',12345 if edited else WORDS[self.style])+self.component()
        options=FoldFixtureOptions(rich=True,matrix=True,flags=7)
        return raw+(self.shared('fold-transform','moFoldUpDownFeatXformData_c',lambda:feature_wire(self.mapping,self.version,self.ids,options)) if self.style in (1,3,5) else bytes(2))

    def unfold_suffix(self) -> bytes:
        """Use raw compressed bodies without suppression bytes; e.g. graph.unfold_suffix()."""
        count=(-2147483648,-1,0,1,0,2)[self.style]
        raw=self.object_map(False)+struct.pack('<i',count)
        raw+=b''.join(frame(-7-index)[1:] for index in range(max(0,count)))
        words=[(index*0x10203041)&0xffffffff for index in range(30)]
        words[28]=(0,1,2,3,0x100,0xffffff0f)[self.style]
        raw+=struct.pack('<30I',*words)
        edge=component_reference(self.mapping,self.version,self.ids,'moCompEdge_c') if self.style in (1,3,5) else bytes(2)
        return raw+edge+struct.pack('<Q',BITS[self.style])

    def bend_suffix(self, edited: bool) -> bytes:
        """Preserve stored point bits and direct list duplicates; e.g. graph.bend_suffix(False)."""
        raw=flange_profile(self.mapping,self.version,self.ids,True) if self.style in (1,3,5) else bytes(2)
        raw+=struct.pack('<Qi3Q',BITS[self.style],12345 if edited else WORDS[self.style],*BITS[:3])+self.array()+self.boundary()
        raw+=self.shared('line','sgLineHandle',lambda:struct.pack('<H2i',7,-1,2147483647)) if self.style in (1,3,5) else bytes(2)
        return raw+struct.pack('<3i',*WORDS[:3])+self.component()

    def fold_record(self, kind: int, key: str, edited: bool) -> bytes:
        """Only bends include the common bend-group layer; e.g. graph.fold_record(2,'first',False)."""
        def contents() -> bytes:
            if kind==3:return self.object_map(edited)
            raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
            if kind==0:
                return raw+group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions())+self.bend_suffix(edited)
            raw+=self.common_fold(edited)
            return raw+(self.unfold_suffix() if kind==2 else b'')
        return self.shared(key,CLASSES[kind],contents)


def sheet_fold_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep fresh, distinct and repeated complete roots; e.g. sheet_fold_graph(2,19000,5)."""
    graph=FoldGraph(version,style,first_slot)
    def record(key: str) -> bytes:
        return graph.fold_record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1
