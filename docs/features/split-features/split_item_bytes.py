"""Complete split-item names, visuals, compressed bodies and fixed transforms (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from owned_name_edit import short_wide_name
from component_coordinate_bytes import fixed_words
from kernel_geometry_fixture import frame
from solid_reference_bytes import body_attributes


def visual_body(version: int) -> bytes:
    """Keep complete modern material gates without a line-style substitution; e.g. visual_body(19000)."""
    raw=struct.pack('<3I',0xffffffff,0x80000000,0)+short_wide_name('Ω split material')+bytes(1)
    raw+=struct.pack('<6di5i',-0.0,1,2,3,4,5,0,-1,0,1,-2147483648,2147483647)+bytes(1)+struct.pack('<2i',-1,0)
    raw+=bytes(1) if version>0x3368 else b''
    raw+=bytes(4) if version>0x372a else b''
    raw+=bytes(4)+bytes(1)+struct.pack('<2i',-1,1)+bytes(1)+struct.pack('<5i',-1,0,-2147483648,2147483647,-1)
    return raw+(bytes(4) if version>0x3f08 else b'')


def split_item(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool=False) -> bytes:
    """Keep named items distinct while sharing complete visuals and attributes; e.g. split_item(mapping,ids,19000,3,'first')."""
    def contents() -> bytes:
        name=short_wide_name('Ω split item') if style else bytes(1)
        prop=shared_object(mapping,ids,'split-visual','moVisualProperties_c',lambda:visual_body(version)) if style>=2 else bytes(2)
        presence=(0,0,1,-2147483648,0,2147483647)[style]
        raw=name+prop+struct.pack('<i',presence)
        if presence:
            raw+=frame()+ (body_attributes(mapping,ids) if style in (2,5) else bytes(2))
        raw+=struct.pack('<30I',*fixed_words(style>=2,(0,1,2,7,15,11)[style],edited))
        return raw+struct.pack('<30I',*fixed_words(style>=2,(15,11,7,2,1,0)[style]))
    return shared_object(mapping,ids,key,'moSplitItem_c',contents)
