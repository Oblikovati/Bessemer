"""Flatten/process bends with independently selected containing-node flags (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS
from fillet_fixture_bytes import body
from flange_fixture_bytes import FlangeFixtureOptions,group_wire
from sheet_fixture_bytes import SheetFixtureOptions,fixed_face
from freeze_node_units import object_tag
from owned_name_edit import short_wide_name
from kernel_geometry_fixture import frame

CLASSES=('moFlattenBends_c','moProcessBends_c')
VERSIONS=(0x27cc,0x335a,0x372f,0x42cb,0x42cc,19000,0xffffffff)
FLAGS=(0,1,0x800,0x40000800,0x80000001,0xffffffff)
PRESENCES=(0,0,0,-1,0,-2147483648)


def containing_flags(raw: bytes, flags: int) -> bytes:
    """Change only the known constructor's flag word, checked again by the source reader; e.g. containing_flags(raw,0x800)."""
    prefix=object_tag('moNodeName_c')+short_wide_name('Ωx')+struct.pack('<i',-3)
    if not raw.startswith(prefix):raise ValueError(f'body prefix {raw[:len(prefix)].hex()}; expected {prefix.hex()}')
    result=bytearray(raw);struct.pack_into('<I',result,len(prefix),flags)
    return bytes(result)


class FlattenGraph(EdgeGraph):
    """Keep node-bit and body-presence selection separate from saved helper state."""
    def base(self, key: str) -> bytes:
        """Use the complete established body and bend-group prefix; e.g. graph.base('first')."""
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
        raw=containing_flags(raw,FLAGS[self.style])
        return raw+group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(extended=self.extended))

    def flatten(self, edited: bool) -> bytes:
        """Omit the presence word completely for unrelated flags; e.g. graph.flatten(False)."""
        raw=struct.pack('<i',12345 if edited else WORDS[self.style])
        if FLAGS[self.style]&0x800:
            presence=PRESENCES[self.style];raw+=struct.pack('<i',presence)
            if presence:raw+=frame()[1:]
        return raw

    def process(self, edited: bool) -> bytes:
        """Use a complete existing fixed-face reader in both saved states; e.g. graph.process(True)."""
        options=SheetFixtureOptions(self.extended,True,0)
        face=self.shared('fixed-face','moFixedFaceInFlat_c',lambda:fixed_face(self.mapping,self.version,self.ids,options)) if self.style not in (0,4) else bytes(2)
        return face+struct.pack('<i',12345 if edited else WORDS[self.style])

    def record(self, kind: int, key: str, edited: bool) -> bytes:
        """Finish one concrete feature before publishing its object-map size; e.g. graph.record(0,'first',False)."""
        return self.shared(key,CLASSES[kind],lambda:self.base(key)+(self.flatten(edited) if kind==0 else self.process(edited)))


def flatten_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, extended: bool=False) -> tuple[bytes,int]:
    """Preserve distinct and repeated roots across both native bindings; e.g. flatten_graph(0,19000,5)."""
    graph=FlattenGraph(version,style,first_slot,extended)
    def record(key: str) -> bytes:
        return graph.record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1
