"""Independent complete annotation graphs with native mixed-width fields (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS,BITS
from note_fixture_bytes import character
from owned_name_edit import short_wide_name
from sketch_dimension_fixture_bytes import valued,base
from valued_transform_bytes import fixed_words,PRESENCE
from display_dimension_fixture_bytes import shared as display_body
from dimension_fixture_bytes import handle_base
from sketch_tail_fixture_bytes import ordinary
from weld_corner_bytes import counted

CLASSES=('moTextDataBorder_c','moTextDataSymbol_c','sgLinearPattCntDim','sgCircularPattCntDim','moDisplayLinearPattCntDim_c','moDisplayCircularPattCntDim_c','moSkDimHandleLinearPattCnt_c','moSkDimHandleCircularPattCnt_c','moCThreadInstanceId_c','moCTInstId_c')
VERSIONS=(0x27cc,0x27df,0x27e0,0x2bca,0x2bcb,0x2be1,0x2be2,0x42ee,0x42ef,19000,0xffffffff)
SINGLES=(0,0x80000000,0x7f800042,0x7fc01234,0x7f800000,0xff800000)


def text_base(style: int, edited: bool) -> bytes:
    """Use exactly the shared 36 bytes, excluding the character's final u16."""
    raw=character(style,edited)[:-2]
    assert len(raw)==36
    return raw


def symbol(graph: EdgeGraph, edited: bool) -> bytes:
    """Keep both independent native suffix gates and complete strings."""
    v=graph.version;s=graph.style
    raw=text_base(s,edited)+short_wide_name('Ω symbol')+b'\x03abc'+struct.pack('<3Q3I',*([BITS[s]]*3),*([SINGLES[s]]*3))
    if v>0x2bca:raw+=struct.pack('<i',WORDS[s])
    if v>=0x2be2:raw+=struct.pack('<2Q',BITS[(s+1)%6],BITS[(s+2)%6])
    return raw


def text_list(graph: EdgeGraph) -> bytes:
    """Store actual symbols, characters and a cached recursive border root."""
    def elements() -> bytes:
        values=[graph.shared('symbol',CLASSES[1],lambda:symbol(graph,False)),graph.shared('character','moTextDataChar_c',lambda:character(graph.style,False)),struct.pack('<H',graph.ids['root'])]
        return counted(values+[struct.pack('<H',graph.ids['symbol'])],graph.style==5)
    def contents() -> bytes:
        array=graph.shared('text-array','suObArray',elements) if graph.style not in (0,4) else bytes(2)
        return array+struct.pack('<i4Q',-1,*([BITS[graph.style]]*4))
    return graph.shared('text-list','moGenTextDataList_c',contents)


def border(graph: EdgeGraph, edited: bool) -> bytes:
    """Preserve the base, noncanonical u16 words and recursive text-list identity."""
    s=graph.style
    raw=text_base(s,edited)+struct.pack('<6I2H',*([SINGLES[s]]*6),65535,32768)+short_wide_name('Ω border\0')
    raw+=text_list(graph) if s!=0 else bytes(2)
    return raw+struct.pack('<2IQ3I',SINGLES[s],SINGLES[(s+1)%6],BITS[s],*([SINGLES[s]]*3))


def count_dimension(graph: EdgeGraph, edited: bool) -> bytes:
    """Use the entire valued base and fixed archive transform before one word."""
    v=graph.version;s=graph.style;raw=valued(v,s!=0);offset=len(base(v))
    raw=raw[:offset]+struct.pack('<H',PRESENCE[s])+(struct.pack('<30I',*fixed_words(s)) if s!=0 else b'')
    return raw+struct.pack('<i',12345 if edited else WORDS[s])


def sketch_count_handle(graph: EdgeGraph, edited: bool) -> bytes:
    """Use a real complete owned sketch with both optional compact transforms."""
    flags=5 if graph.style in (3,5) else 0
    raw=handle_base(graph.version,False,flags)
    raw+=graph.shared('sketch','sgSketch',lambda:ordinary(graph.mapping,graph.version,False))
    return raw+struct.pack('<i',12345 if edited else WORDS[graph.style])


def thread_item(graph: EdgeGraph, edited: bool) -> bytes:
    """Preserve three independent raw signed words with no extra base bytes."""
    return struct.pack('<3i',12345 if edited else WORDS[graph.style],-1,2147483647)


def threads(graph: EdgeGraph, edited: bool) -> bytes:
    """Keep direct packed counts, null entries and real shared item identities."""
    if graph.style in (0,4):return counted([],graph.style==4)
    first=graph.shared('thread-first',CLASSES[9],lambda:thread_item(graph,edited))
    second=graph.shared('thread-second',CLASSES[9],lambda:thread_item(graph,False))
    return counted([first,bytes(2),second,struct.pack('<H',graph.ids['thread-first'])],graph.style==5)


def contents(graph: EdgeGraph, kind: int, edited: bool) -> bytes:
    """Bind each of ten proven class names to its complete body."""
    if kind==0:return border(graph,edited)
    if kind==1:return symbol(graph,edited)
    if kind in (2,3):return count_dimension(graph,edited)
    if kind in (4,5):return display_body(graph.mapping,graph.version,'moDisplayDim_c',graph.style in (3,5),graph.ids['root'],12345 if edited else WORDS[graph.style])
    if kind in (6,7):return sketch_count_handle(graph,edited)
    return threads(graph,edited) if kind==8 else thread_item(graph,edited)


def annotation_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve the actual root before recursive owned targets; e.g. annotation_object(0,19000,2)."""
    graph=EdgeGraph(version,style,first_slot)
    raw=graph.shared('root',CLASSES[kind],lambda:contents(graph,kind,edited))
    return raw,graph.mapping.slot+1
