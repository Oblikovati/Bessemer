"""Exact detail-item and component-detail graphs with explicit current-part indices (#54)."""
from dataclasses import dataclass
from collections.abc import Callable
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_component_references import base
from display_dimension_fixture_bytes import body as strip_tag
from dimension_fixture_bytes import attachment
from owned_name_edit import short_wide_name

VERSIONS=(0x27cc,0x2b60,0x2b61,0x2bb8,0x2bb9,0x2f55,0x2f56,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)

@dataclass(frozen=True)
class CurrentPartIndices:
    root: int
    unit: int


def detail_node(mapping: WireMap, ids: dict[str,int], rich: bool) -> bytes:
    """Reuse actual node-name objects while keeping the modern node prefix; e.g. detail_node(mapping,ids,True)."""
    name=shared_object(mapping,ids,'name','moNodeName_c',lambda:short_wide_name('Ω detail node')) if rich else bytes(2)
    return name+struct.pack('<iIiI',-3,0x89abcdef,-7,0xffffffff)+short_wide_name('Ω node label')


def detail_fields(mapping: WireMap, ids: dict[str,int], style: int, owner: int, edited: bool=False, attachment_factory: Callable[[],bytes] | None=None) -> bytes:
    """Reuse the exact inline detail base with a concrete attachment caller; e.g. detail_fields(mapping,ids,1,2)."""
    rich=style%2==1;raw=detail_node(mapping,ids,rich)+struct.pack('<HH',65535,owner if rich else 0)
    def attachment_pointer() -> bytes:
        if not rich:return bytes(2)
        if attachment_factory is not None:return attachment_factory()
        return shared_object(mapping,ids,'attachment','moAttachPoint_c',lambda:strip_tag(attachment()))
    attach=attachment_pointer()
    raw+=struct.pack('<H',2 if rich else 0)+(attach+struct.pack('<H',ids['attachment']) if rich and attachment_factory is None else attach+attachment_pointer() if rich else b'')
    values=(WORDS[style],0,2147483647) if rich else ()
    return raw+struct.pack('<i',len(values))+b''.join(struct.pack('<i',v) for v in values)+struct.pack('<i',12345 if edited else WORDS[style])+attachment_pointer()


def detail_pointer(mapping: WireMap, ids: dict[str,int], style: int, key: str, edited: bool=False) -> bytes:
    """Retain owned attachments and repeated pointers without a tagged list; e.g. detail_pointer(mapping,ids,3,'first')."""
    return shared_object(mapping,ids,'detail-'+key,'moDetailItem_c',lambda:detail_fields(mapping,ids,style,ids['detail-'+key],edited))


def unit_pointer(mapping: WireMap, ids: dict[str,int], mode: int, indices: CurrentPartIndices | None) -> bytes:
    """Select null, fresh current-part or cached inherited units explicitly; e.g. unit_pointer(mapping,ids,2,indices)."""
    if mode==0:return bytes(2)
    if indices is None:raise ValueError(f'component mode {mode}, indices {indices}; expected verified current-part indices')
    if mode==2:return struct.pack('<H',indices.unit)
    if mode!=1:raise ValueError(f'component mode {mode}; expected 0, 1 or 2')
    return shared_object(mapping,ids,'unit','moUnitComponent_c',lambda:struct.pack('<HB',indices.root,0))


def component_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, mode: int, indices: CurrentPartIndices | None, edited: bool) -> bytes:
    """Keep complete shared detail targets behind a real component prefix; e.g. component_pointer(mapping,ids,19000,2,'first',0,None,False)."""
    def contents() -> bytes:
        raw=unit_pointer(mapping,ids,mode,indices)+base(version,style%2==1)[2:]
        first=bytes(2) if style==4 else detail_pointer(mapping,ids,style,'shared')
        return raw+first+(bytes(2) if style==4 else detail_pointer(mapping,ids,style,'shared'))+struct.pack('<i',12345 if edited else WORDS[style])
    return shared_object(mapping,ids,'component-'+key,'moCompDetailItem_c',contents)


def detail_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, mode: int=0, indices: CurrentPartIndices | None=None) -> tuple[bytes,int]:
    """Keep distinct roots, cached detail targets and repeated first roots; e.g. detail_graph(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def root(key: str) -> bytes:
        if kind==0:return detail_pointer(mapping,ids,style,key,edited and key=='first')
        return component_pointer(mapping,ids,version,style,key,mode,indices,edited and key=='first')
    def array() -> bytes:
        if style==3:items=[detail_pointer(mapping,ids,style,'shared'),root('first')]
        else:items=[root('first'),root('second'),root('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else root('first')
    return raw,mapping.slot+1
