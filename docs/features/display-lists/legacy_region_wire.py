"""Reuse complete bucket builders while omitting or retaining the first region word."""
from generate_component_vectors import integers
import legacy_thread_wire as thread_wire


def region_poly_reader(original: object, edited: bool) -> object:
    def encode(version: int, dimensions: int, count: int = 1, kind: int = 1, line: bool = True, smooth: int = -17) -> bytes:
        raw = original(version,dimensions,count,kind,line,smooth)
        if count<=0:return raw
        assert raw[-4:]==integers(0),f"polygon tail {raw[-4:]!r}; expected the zero prototype region word"
        suffix = integers(0 if edited else -2147483648) if version>0xabc else b""
        return raw[:-4]+suffix
    return encode


def region_root(version: int, kind: str, scenario: int, cached: bool = False) -> tuple[bytes,tuple[str,...]]:
    assert 2725<=version<=2749,f"region fixture version {version}; expected 2725..2749"
    original = thread_wire.thread_poly
    edited = scenario==4 if cached else scenario%16==1
    thread_wire.thread_poly = region_poly_reader(original,edited)
    builder = thread_wire.thread_cached_root if cached else thread_wire.thread_root
    try:return builder(version,kind,scenario)
    finally:thread_wire.thread_poly = original
