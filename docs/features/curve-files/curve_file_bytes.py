"""Complete stored curve-file points with the existing reference-curve prefix (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from helix_bytes import HelixOptions,reference_curve
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from owned_name_edit import short_wide_name

VERSIONS=(0x27cc,0x2f85,0x2f86,0x42cd,0x42ce,19000,0xffffffff)
COUNTS=(0,-1,2,3,-2147483648,1)
BITS=(0x8000000000000000,0x7ff8000000000042,0x7ff0000000001234,0x3ff4000000000000,0x7ff0000000000000,0xfff0000000000000)


def curve_file_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete records with signed counts and exact point bits."""
    if kind!=0:raise ValueError(f'curve-file fixture kind {kind}; expected 0')
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents() -> bytes:
        options=HelixOptions(rich=style in (3,5),wire=style in (2,5))
        raw=reference_curve(mapping,ids,version,options)+short_wide_name('C:\\stored\\curves\\Ω.sldcrv')+struct.pack('<i',COUNTS[style])
        for index in range(max(0,COUNTS[style])):
            words=[BITS[(style+index+axis)%6] for axis in range(3)]
            if edited and index==0:words[0]=struct.unpack('<Q',struct.pack('<d',12345.0))[0]
            raw+=struct.pack('<3Q',*words)
        if version>0x42cd:
            raw+=shared_object(mapping,ids,'point','moPointRef_w',lambda:strip_tag(geometric('moPointRef_w',version,False,0))) if style in (1,3,5) else bytes(2)
        return raw
    raw=shared_object(mapping,ids,'root','moCurveInFile_c',contents)
    return raw,mapping.slot+1
