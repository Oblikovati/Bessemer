"""Independent fixed geometry bodies, primitive widths and signed version gates (#54)."""
import struct
from edge_fixture_bytes import BITS,WORDS
from valued_transform_bytes import fixed_words

GEOMETRY_VERSIONS=(-2147483648,-1,0,1,2,3,4,5,6,7,2147483647)


def geometry_bytes(version: int, style: int) -> bytes:
    """Use the stored geometry version independently of archive/feature versions."""
    raw=struct.pack('<i21Q2i2Qi3Q',WORDS[style],*([BITS[style]]*21),-1,2147483647,BITS[style],BITS[(style+1)%6],-1,*([BITS[style]]*3))
    raw+=struct.pack('<16i',*range(-15,0),version)+struct.pack('<10Qi',*([BITS[style]]*10),-1)
    assert len(raw)==372
    if version>0:raw+=struct.pack('<2i',-1,2147483647)
    if version>1:raw+=struct.pack('<2Q',BITS[style],BITS[(style+1)%6])
    if version>2:raw+=struct.pack('<30I',*fixed_words(style))
    if version>3:raw+=struct.pack('<2i6Qi',-1,WORDS[style],*([BITS[style]]*6),2147483647)
    if version>=5:raw+=struct.pack('<Q',BITS[style])
    if version>=6:raw+=struct.pack('<i',-1)
    if version>=7:raw+=struct.pack('<i',2147483647)
    return raw
