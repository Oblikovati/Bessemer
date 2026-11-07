"""Build complete ordinary note/text graphs with actual typed shared objects (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from display_item_fixture_bytes import object_base, item
from display_dimension_fixture_bytes import body as strip_tag
from freeze_component_references import base as component_base
from freeze_sketch_text import wire as dimension_text
from detail_reference_bytes import detail_fields
from dimension_fixture_bytes import attachment
from owned_name_edit import short_wide_name

CLASSES=('moDisplayNote_c','moCompNote_c','moNoteData_c','moGenericText_c',
         'moGenTextDataList_c','moTextDataLineHead_c','moTextDataChar_c','moAbsolutePoint_c')
VERSIONS=(0x27cc,0x2bbb,0x2bbc,0x2f77,0x2f78,0x2f9a,0x2f9b,0x2fb7,0x2fb8,
          0x3371,0x3372,0x3373,0x3374,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)


def paragraph(style: int) -> bytes:
    """Preserve direct paragraph widths and raw IEEE values; e.g. paragraph(2)."""
    raw=struct.pack('<H3Q2H',65535,*BITS[style:style+1]*3,32768,65534)
    raw+=short_wide_name('Ω paragraph\0') if style%2 else b'\x03abc'
    return raw+struct.pack('<iIi2Qi2Q2i',WORDS[style],0xffffffff,-2,BITS[style],BITS[(style+1)%6],-3,
                          BITS[(style+2)%6],BITS[(style+3)%6],-4,-5)


def character(style: int, edited: bool) -> bytes:
    """Retain signed words, f32 triples and lone UTF-16 units; e.g. character(1,False)."""
    return struct.pack('<2i4IH i I 2H',12345 if edited else WORDS[style],-2,
                       0x80000000,0x7f800042,0xff800000,0x3f800000,65535,-3,0x7fc01234,32768,0xd800)


def suffix(version: int, style: int, edited: bool, favorite: bytes) -> bytes:
    """Keep note chunk framing and all four independent tail gates; e.g. suffix(19000,2,False,b'\0\0')."""
    raw=struct.pack('<3H4Q2iH',65535,32768,7,*([BITS[style]]*4),WORDS[style],-2,0)
    raw+=b'\x01A\x01A\x00'+bytes(4)+struct.pack('<i',style)
    raw+=b''.join(struct.pack('<I',v) for v in (0,0x80000000,0xffffffff,1,0x7fffffff)[:style])
    raw+=struct.pack('<HQ',0,BITS[(style+1)%6])+short_wide_name('Ω note')+favorite
    raw+=struct.pack('<HH2iQiIi',65534,0,-3,-4,BITS[style],-5,0xffffffff,12345 if edited else WORDS[style])
    if version>0x2bbb:raw+=struct.pack('<i',-8)
    if version>0x2f77:raw+=struct.pack('<i',-9)
    if version>0x2f9a:raw+=bytes(2)
    if version>0x2fb7:raw+=bytes(2)
    return raw


class NoteGraphBuilder:
    """Keep operation-local map insertion ahead of recursive children."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.mapping=WireMap();self.mapping.slot=first_slot
        self.ids: dict[str,int]={}
        self.kind,self.version,self.style,self.edited=kind,version,style,edited

    def obj(self, key: str, name: str, build: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,build)

    def edited_word(self, kind: int) -> int:
        return 12345 if self.edited and self.kind==kind else WORDS[self.style]

    def absolute(self) -> bytes:
        return self.obj('absolute','moAbsolutePoint_c',lambda:strip_tag(attachment()))

    def details(self) -> bytes:
        owner=self.ids.get('root-first',0)
        return detail_fields(self.mapping,self.ids,self.style,owner,
                             self.edited and self.kind==2,self.absolute)

    def text_list(self) -> bytes:
        return self.obj('text-list','moGenTextDataList_c',self.text_list_body)

    def text_list_body(self) -> bytes:
        array=self.obj('text-array','suObArray',self.text_elements) if self.style else bytes(2)
        return array+struct.pack('<i4Q',self.edited_word(4),*([BITS[self.style]]*4))

    def text_elements(self) -> bytes:
        values=[self.line(),self.char(),self.char()] if self.style%2 else []
        return struct.pack('<H',len(values))+b''.join(values)

    def line(self) -> bytes:
        return self.obj('line','moTextDataLineHead_c',self.line_body)

    def line_body(self) -> bytes:
        raw=struct.pack('<2i3IH',self.edited_word(5),-2,0x80000000,0x7f800042,0x3f800000,65535)
        raw+=paragraph(self.style)+struct.pack('<3I',0xff800000,0x7fc01234,1)
        return raw+self.text_list()+struct.pack('<iQi',-3,BITS[self.style],-4)

    def char(self) -> bytes:
        return self.obj('character','moTextDataChar_c',
                        lambda:character(self.style,self.edited and self.kind==6))

    def generic(self) -> bytes:
        return self.obj('generic','moGenericText_c',self.generic_body)

    def generic_body(self) -> bytes:
        display=struct.pack('<H',self.ids['root-first']) if self.kind==0 else bytes(2)
        raw=strip_tag(dimension_text(self.version,self.style%4,self.style%4,False))
        return raw+self.text_list()+struct.pack('<i',self.edited_word(3))+display+paragraph(self.style)

    def note_data(self) -> bytes:
        return self.obj('note-data','moNoteData_c',lambda:self.details()+self.generic())

    def component(self) -> bytes:
        return self.obj('component','moCompNote_c',self.component_body)

    def component_body(self) -> bytes:
        first=self.note_data() if self.style%2 else bytes(2)
        second=struct.pack('<H',self.ids['note-data']) if self.style%2 else bytes(2)
        return component_base(self.version,False)+first+second+struct.pack('<i',self.edited_word(1))

    def display_body(self) -> bytes:
        component_wire=self.component()
        base=object_base(self.mapping,self.style%2==1,self.ids['root-first'])
        # Insert the component before children belonging to later display fields.
        base=base[:6]+component_wire+base[8:]
        inherited=base+item(self.mapping,self.version,self.style%2==1)
        favorite=self.obj('favorite','moFavoriteHandle_c',
                          lambda:struct.pack('<iiI',-1,-2,0xffffffff)) if self.style else bytes(2)
        return inherited+suffix(self.version,self.style,self.edited,favorite)

    def root(self) -> bytes:
        if self.kind==0:
            return self.obj('root-first',CLASSES[0],self.display_body)
        return (self.component,self.note_data,self.generic,self.text_list,
                self.line,self.char,self.absolute)[self.kind-1]()


def note_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build typed recursive children before later siblings; e.g. note_graph(0,19000,5)."""
    builder=NoteGraphBuilder(kind,version,style,edited,first_slot)
    raw=builder.root()
    return raw,builder.mapping.slot+1
